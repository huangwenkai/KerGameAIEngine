//! ECS (Entity Component System) using hecs

use hecs::World as HecsWorld;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Transform component
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
}

/// Velocity component
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

/// Health component
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

/// ECS World wrapper
pub struct EcsWorld {
    world: HecsWorld,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            world: HecsWorld::new(),
        }
    }

    /// Spawn entity with transform, velocity, and health
    pub fn spawn_entity(&mut self, x: f32, y: f32, vx: f32, vy: f32, health: f32) -> hecs::Entity {
        self.world.spawn((
            Transform { x, y },
            Velocity { vx, vy },
            Health {
                current: health,
                max: health,
            },
        ))
    }

    /// Movement system: update positions based on velocities
    pub fn system_movement(&mut self, dt: f32) {
        for (_id, (transform, velocity)) in self.world.query_mut::<(&mut Transform, &Velocity)>() {
            transform.x += velocity.vx * dt;
            transform.y += velocity.vy * dt;
        }
    }

    /// Collision system: simple AABB collision with world bounds
    pub fn system_collision(&mut self, world_size: f32) {
        for (_id, (transform, velocity)) in self.world.query_mut::<(&mut Transform, &mut Velocity)>() {
            // Bounce off world boundaries
            if transform.x < 0.0 || transform.x > world_size {
                velocity.vx = -velocity.vx;
                transform.x = transform.x.clamp(0.0, world_size);
            }
            if transform.y < 0.0 || transform.y > world_size {
                velocity.vy = -velocity.vy;
                transform.y = transform.y.clamp(0.0, world_size);
            }
        }
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.world.len() as usize
    }

    /// Compute state hash for determinism verification
    pub fn state_hash(&self) -> String {
        let mut hasher = Sha256::new();
        
        // Collect all entities and sort by ID for deterministic hashing
        let mut entities: Vec<_> = self
            .world
            .query::<(&Transform, &Velocity, &Health)>()
            .iter()
            .map(|(id, (t, v, h))| (id, *t, *v, *h))
            .collect();
        
        entities.sort_by_key(|(id, _, _, _)| id.to_bits().get());
        
        for (_id, transform, velocity, health) in entities {
            hasher.update(transform.x.to_le_bytes());
            hasher.update(transform.y.to_le_bytes());
            hasher.update(velocity.vx.to_le_bytes());
            hasher.update(velocity.vy.to_le_bytes());
            hasher.update(health.current.to_le_bytes());
            hasher.update(health.max.to_le_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }

    /// Clear all entities
    pub fn clear(&mut self) {
        self.world.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_and_count() {
        let mut ecs = EcsWorld::new();
        assert_eq!(ecs.entity_count(), 0);
        
        ecs.spawn_entity(10.0, 20.0, 1.0, 2.0, 100.0);
        assert_eq!(ecs.entity_count(), 1);
        
        ecs.spawn_entity(30.0, 40.0, -1.0, -2.0, 50.0);
        assert_eq!(ecs.entity_count(), 2);
    }

    #[test]
    fn movement_system() {
        let mut ecs = EcsWorld::new();
        ecs.spawn_entity(0.0, 0.0, 10.0, 5.0, 100.0);
        
        ecs.system_movement(1.0);
        
        // Check position updated
        for (_id, transform) in ecs.world.query_mut::<&Transform>() {
            assert_eq!(transform.x, 10.0);
            assert_eq!(transform.y, 5.0);
        }
    }

    #[test]
    fn collision_system() {
        let mut ecs = EcsWorld::new();
        ecs.spawn_entity(100.0, 100.0, 10.0, 5.0, 100.0);
        
        // Move beyond boundary
        ecs.system_movement(11.0); // x = 210, y = 155
        
        // Apply collision (world_size = 200)
        ecs.system_collision(200.0);
        
        // Velocity should be inverted
        for (_id, velocity) in ecs.world.query_mut::<&Velocity>() {
            assert_eq!(velocity.vx, -10.0);
            // y velocity unchanged (still in bounds)
        }
    }

    #[test]
    fn deterministic_hash() {
        let mut ecs1 = EcsWorld::new();
        let mut ecs2 = EcsWorld::new();
        
        ecs1.spawn_entity(10.0, 20.0, 1.0, 2.0, 100.0);
        ecs2.spawn_entity(10.0, 20.0, 1.0, 2.0, 100.0);
        
        assert_eq!(ecs1.state_hash(), ecs2.state_hash());
    }
}
