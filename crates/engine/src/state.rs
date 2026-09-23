//! Game state for simulation

use serde::{Deserialize, Serialize};

/// Entity ID
pub type EntityId = u32;

/// Simple entity with position and velocity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Entity {
    pub fn new(id: EntityId, x: f32, y: f32) -> Self {
        Self {
            id,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
        }
    }

    /// Update position based on velocity (fixed timestep)
    pub fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }
}

/// World state
#[derive(Debug)]
pub struct World {
    entities: Vec<Entity>,
    next_entity_id: EntityId,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_entity_id: 1,
        }
    }

    pub fn spawn_entity(&mut self, x: f32, y: f32) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(Entity::new(id, x, y));
        id
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn update_all(&mut self, dt: f32) {
        for entity in &mut self.entities {
            entity.update(dt);
        }
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Compute state hash for verification
    pub fn state_hash(&self) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        
        for entity in &self.entities {
            hasher.update(entity.id.to_le_bytes());
            hasher.update(entity.x.to_le_bytes());
            hasher.update(entity.y.to_le_bytes());
            hasher.update(entity.vx.to_le_bytes());
            hasher.update(entity.vy.to_le_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_entity() {
        let mut world = World::new();
        let id = world.spawn_entity(10.0, 20.0);
        
        assert_eq!(world.entity_count(), 1);
        let entity = world.get_entity(id).unwrap();
        assert_eq!(entity.x, 10.0);
        assert_eq!(entity.y, 20.0);
    }

    #[test]
    fn entity_movement() {
        let mut entity = Entity::new(1, 0.0, 0.0);
        entity.vx = 10.0;
        entity.vy = 5.0;
        
        entity.update(1.0);
        assert_eq!(entity.x, 10.0);
        assert_eq!(entity.y, 5.0);
    }

    #[test]
    fn deterministic_state_hash() {
        let mut world1 = World::new();
        let mut world2 = World::new();
        
        world1.spawn_entity(10.0, 20.0);
        world2.spawn_entity(10.0, 20.0);
        
        assert_eq!(world1.state_hash(), world2.state_hash());
    }
}
