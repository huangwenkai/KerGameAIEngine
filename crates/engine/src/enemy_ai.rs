//! Enemy AI behaviors for different enemy types

use crate::physics::AABB;
use crate::rng::GameRng;

/// Enemy type with distinct behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Slime,      // Ground hopper
    Flyer,      // Flying pursuer
    Crawler,    // Wall climber
}

/// Enemy AI state
#[derive(Debug, Clone)]
pub struct EnemyAI {
    pub enemy_type: EnemyType,
    pub aabb: AABB,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub on_ground: bool,
    pub chase_cooldown: f32,
    pub hop_cooldown: f32,
}

impl EnemyAI {
    pub fn new(enemy_type: EnemyType, x: f32, y: f32) -> Self {
        let (width, height) = match enemy_type {
            EnemyType::Slime => (12.0, 12.0),
            EnemyType::Flyer => (10.0, 8.0),
            EnemyType::Crawler => (8.0, 6.0),
        };
        
        Self {
            enemy_type,
            aabb: AABB::new(x, y, width, height),
            velocity_x: 0.0,
            velocity_y: 0.0,
            on_ground: false,
            chase_cooldown: 0.0,
            hop_cooldown: 0.0,
        }
    }
    
    /// Update AI behavior toward player
    pub fn update(&mut self, dt: f32, player_x: f32, player_y: f32, world: &mut crate::chunk::ChunkWorld) {
        self.chase_cooldown = (self.chase_cooldown - dt).max(0.0);
        self.hop_cooldown = (self.hop_cooldown - dt).max(0.0);
        
        let dx = player_x - self.aabb.center_x();
        let dy = player_y - self.aabb.center_y();
        let dist = (dx * dx + dy * dy).sqrt();
        
        match self.enemy_type {
            EnemyType::Slime => self.update_slime(dt, dx, dy, dist, world),
            EnemyType::Flyer => self.update_flyer(dt, dx, dy, dist),
            EnemyType::Crawler => self.update_crawler(dt, dx, dy, world),
        }
        
        // Apply physics
        self.apply_physics(dt, world);
    }
    
    fn update_slime(&mut self, _dt: f32, dx: f32, _dy: f32, dist: f32, _world: &mut crate::chunk::ChunkWorld) {
        // Hop toward player when in range
        if dist < 100.0 && self.on_ground && self.hop_cooldown <= 0.0 {
            self.velocity_x = (dx.signum() * 30.0).clamp(-30.0, 30.0);
            self.velocity_y = -80.0; // Hop up
            self.hop_cooldown = 1.0;
        }
    }
    
    fn update_flyer(&mut self, _dt: f32, dx: f32, dy: f32, dist: f32) {
        // Fly toward player
        if dist < 150.0 && dist > 1.0 {
            let speed = 20.0;
            self.velocity_x = (dx / dist) * speed;
            self.velocity_y = (dy / dist) * speed * 0.5; // Less vertical movement
        } else {
            // Hover
            self.velocity_x *= 0.95;
            self.velocity_y *= 0.95;
        }
    }
    
    fn update_crawler(&mut self, _dt: f32, dx: f32, _dy: f32, _world: &mut crate::chunk::ChunkWorld) {
        // Walk toward player on ground
        if dx.abs() > 5.0 {
            self.velocity_x = dx.signum() * 15.0;
        } else {
            self.velocity_x *= 0.9;
        }
    }
    
    fn apply_physics(&mut self, dt: f32, world: &mut crate::chunk::ChunkWorld) {
        match self.enemy_type {
            EnemyType::Flyer => {
                // Flyers ignore gravity
                let new_aabb = self.aabb.translate(self.velocity_x * dt, self.velocity_y * dt);
                self.aabb = new_aabb;
            }
            _ => {
                // Ground enemies have gravity
                self.velocity_y += 150.0 * dt; // Gravity
                
                // Move X
                let target_x = self.aabb.translate(self.velocity_x * dt, 0.0);
                if !crate::physics::collides_with_world(&target_x, world) {
                    self.aabb = target_x;
                } else {
                    self.velocity_x = 0.0;
                }
                
                // Move Y
                let target_y = self.aabb.translate(0.0, self.velocity_y * dt);
                if !crate::physics::collides_with_world(&target_y, world) {
                    self.aabb = target_y;
                    self.on_ground = false;
                } else {
                    if self.velocity_y > 0.0 {
                        self.on_ground = true;
                    }
                    self.velocity_y = 0.0;
                }
            }
        }
    }
    
    pub fn get_color(&self) -> [f32; 4] {
        match self.enemy_type {
            EnemyType::Slime => [0.2, 1.0, 0.3, 1.0],     // Green
            EnemyType::Flyer => [1.0, 0.5, 0.0, 1.0],     // Orange
            EnemyType::Crawler => [0.6, 0.3, 0.8, 1.0],   // Purple
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn enemy_types_distinct() {
        let slime = EnemyAI::new(EnemyType::Slime, 0.0, 0.0);
        let flyer = EnemyAI::new(EnemyType::Flyer, 0.0, 0.0);
        let crawler = EnemyAI::new(EnemyType::Crawler, 0.0, 0.0);
        
        assert_ne!(slime.get_color(), flyer.get_color());
        assert_ne!(flyer.get_color(), crawler.get_color());
    }
}
