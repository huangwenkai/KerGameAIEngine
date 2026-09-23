//! Command pattern for replay and determinism

use serde::{Deserialize, Serialize};
use crate::state::{EntityId, World};

/// All game commands are serializable for replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    /// Spawn an entity at position
    SpawnEntity { x: f32, y: f32 },
    
    /// Set entity velocity
    SetVelocity { entity_id: EntityId, vx: f32, vy: f32 },
    
    /// Apply impulse to entity
    ApplyImpulse { entity_id: EntityId, dx: f32, dy: f32 },
    
    /// No-op command for testing
    Noop,
}

impl Command {
    /// Execute command on world state
    pub fn apply(&self, world: &mut World) {
        match self {
            Command::SpawnEntity { x, y } => {
                world.spawn_entity(*x, *y);
            }
            Command::SetVelocity { entity_id, vx, vy } => {
                if let Some(entity) = world.get_entity_mut(*entity_id) {
                    entity.vx = *vx;
                    entity.vy = *vy;
                }
            }
            Command::ApplyImpulse { entity_id, dx, dy } => {
                if let Some(entity) = world.get_entity_mut(*entity_id) {
                    entity.vx += *dx;
                    entity.vy += *dy;
                }
            }
            Command::Noop => {}
        }
    }
}

/// Command buffer collects commands per tick
pub struct CommandBuffer {
    commands: Vec<Command>,
}

impl CommandBuffer {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn push(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Command> + '_ {
        self.commands.drain(..)
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_command() {
        let mut world = World::new();
        let cmd = Command::SpawnEntity { x: 10.0, y: 20.0 };
        
        cmd.apply(&mut world);
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn velocity_command() {
        let mut world = World::new();
        let id = world.spawn_entity(0.0, 0.0);
        
        let cmd = Command::SetVelocity {
            entity_id: id,
            vx: 5.0,
            vy: 10.0,
        };
        
        cmd.apply(&mut world);
        let entity = world.get_entity(id).unwrap();
        assert_eq!(entity.vx, 5.0);
        assert_eq!(entity.vy, 10.0);
    }
}
