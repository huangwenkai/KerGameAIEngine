//! Command pattern for replay and determinism

use serde::{Deserialize, Serialize};

/// All game commands are serializable for replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    /// M0: Placeholder command
    Noop,
    /// Future: Entity spawn, movement, actions, etc.
    #[allow(dead_code)]
    Placeholder(String),
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
}
