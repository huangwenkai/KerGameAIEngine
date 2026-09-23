//! Physics and collision for character motor

use crate::chunk::{ChunkWorld, Material};

/// AABB (Axis-Aligned Bounding Box)
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl AABB {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn left(&self) -> f32 { self.x }
    pub fn right(&self) -> f32 { self.x + self.width }
    pub fn top(&self) -> f32 { self.y }
    pub fn bottom(&self) -> f32 { self.y + self.height }

    pub fn center_x(&self) -> f32 { self.x + self.width * 0.5 }
    pub fn center_y(&self) -> f32 { self.y + self.height * 0.5 }

    pub fn overlaps(&self, other: &AABB) -> bool {
        self.right() > other.left()
            && self.left() < other.right()
            && self.bottom() > other.top()
            && self.top() < other.bottom()
    }

    pub fn translate(&self, dx: f32, dy: f32) -> AABB {
        AABB::new(self.x + dx, self.y + dy, self.width, self.height)
    }
}

/// Character motor with physics
#[derive(Debug, Clone)]
pub struct CharacterMotor {
    pub aabb: AABB,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub on_ground: bool,
    pub gravity: f32,
    pub max_speed: f32,
    pub jump_strength: f32,
}

impl CharacterMotor {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            aabb: AABB::new(x, y, 12.0, 24.0), // 3×6 cells (12×24 pixels)
            velocity_x: 0.0,
            velocity_y: 0.0,
            on_ground: false,
            gravity: 800.0, // pixels/sec²
            max_speed: 200.0, // pixels/sec
            jump_strength: 400.0, // pixels/sec
        }
    }

    /// Apply movement input
    pub fn move_input(&mut self, direction: f32, dt: f32) {
        let accel = 1000.0 * direction;
        self.velocity_x += accel * dt;
        self.velocity_x = self.velocity_x.clamp(-self.max_speed, self.max_speed);
    }

    /// Apply friction
    pub fn apply_friction(&mut self, dt: f32) {
        let friction = if self.on_ground { 800.0 } else { 200.0 };
        let sign = self.velocity_x.signum();
        self.velocity_x -= sign * friction * dt;
        if self.velocity_x.abs() < 10.0 {
            self.velocity_x = 0.0;
        }
    }

    /// Apply gravity
    pub fn apply_gravity(&mut self, dt: f32) {
        if !self.on_ground {
            self.velocity_y += self.gravity * dt;
            self.velocity_y = self.velocity_y.min(600.0); // Terminal velocity
        }
    }

    /// Jump
    pub fn jump(&mut self) {
        if self.on_ground {
            self.velocity_y = -self.jump_strength;
            self.on_ground = false;
        }
    }

    /// Update position with collision against chunk world
    pub fn update(&mut self, dt: f32, world: &mut ChunkWorld) {
        self.apply_gravity(dt);

        // Move X
        let move_x = self.velocity_x * dt;
        let target_x = self.aabb.translate(move_x, 0.0);
        if !collides_with_world(&target_x, world) {
            self.aabb = target_x;
        } else {
            self.velocity_x = 0.0;
        }

        // Move Y
        let move_y = self.velocity_y * dt;
        let target_y = self.aabb.translate(0.0, move_y);
        if !collides_with_world(&target_y, world) {
            self.aabb = target_y;
            self.on_ground = false;
        } else {
            if self.velocity_y > 0.0 {
                // Hit ground
                self.on_ground = true;
            }
            self.velocity_y = 0.0;
        }

        // Step-up only when grounded and horizontally blocked (prevents A/D looking like jumps)
        if self.on_ground && self.velocity_x.abs() > 10.0 {
            let probe = self.aabb.translate(self.velocity_x.signum() * 2.0, 0.0);
            if collides_with_world(&probe, world) {
                let step_height = 8.0; // 2 cells
                let step_up = self.aabb.translate(0.0, -step_height);
                if !collides_with_world(&step_up, world) {
                    let step_forward = step_up.translate(self.velocity_x.signum() * 4.0, 0.0);
                    if !collides_with_world(&step_forward, world) {
                        self.aabb = step_forward;
                    }
                }
            }
        }
    }
}

/// Check if AABB collides with solid cells in world
pub fn collides_with_world(aabb: &AABB, world: &mut ChunkWorld) -> bool {
    // Sample corners and edges (4px cell size)
    let cell_size = 4.0;
    let left = (aabb.left() / cell_size).floor() as i32;
    let right = (aabb.right() / cell_size).floor() as i32;
    let top = (aabb.top() / cell_size).floor() as i32;
    let bottom = (aabb.bottom() / cell_size).floor() as i32;

    for y in top..=bottom {
        for x in left..=right {
            let material = world.get_cell(x, y);
            if material.is_solid() {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_overlap() {
        let a = AABB::new(0.0, 0.0, 10.0, 10.0);
        let b = AABB::new(5.0, 5.0, 10.0, 10.0);
        let c = AABB::new(20.0, 20.0, 10.0, 10.0);

        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
        assert!(!a.overlaps(&c));
    }

    #[test]
    fn character_gravity() {
        let mut motor = CharacterMotor::new(0.0, 0.0);
        assert_eq!(motor.velocity_y, 0.0);

        motor.apply_gravity(0.1);
        assert!(motor.velocity_y > 0.0);
    }

    #[test]
    fn character_jump() {
        let mut motor = CharacterMotor::new(0.0, 0.0);
        motor.on_ground = true;

        motor.jump();
        assert!(motor.velocity_y < 0.0);
        assert!(!motor.on_ground);
    }

    #[test]
    fn collision_with_solid() {
        let mut world = ChunkWorld::new();
        world.set_cell(0, 0, Material::Stone);

        let aabb = AABB::new(0.0, 0.0, 4.0, 4.0);
        assert!(collides_with_world(&aabb, &mut world));

        let aabb_air = AABB::new(100.0, 100.0, 4.0, 4.0);
        assert!(!collides_with_world(&aabb_air, &mut world));
    }
}
