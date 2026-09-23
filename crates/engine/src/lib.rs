//! KerGameAIEngine - Custom Rust game engine
//! 
//! Core features:
//! - Fixed timestep simulation decoupled from rendering
//! - Seeded RNG for deterministic behavior
//! - Command pattern for replay/hash verification
//! - Headless execution support

pub mod demo;
pub mod time;
pub mod commands;
pub mod replay;
pub mod report;

use anyhow::Result;
use std::time::Duration;

/// Engine configuration
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub headless: bool,
    pub seed: u64,
    pub fixed_timestep: Duration,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            headless: false,
            seed: 0,
            fixed_timestep: Duration::from_secs_f64(1.0 / 60.0), // 60 TPS
        }
    }
}

/// Main engine context
pub struct Engine {
    config: EngineConfig,
    time: time::TimeState,
    command_buffer: commands::CommandBuffer,
    replay_hasher: replay::ReplayHasher,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        log::info!("Initializing engine (headless: {}, seed: {})", 
                   config.headless, config.seed);
        
        let seed = config.seed;
        Self {
            config,
            time: time::TimeState::new(),
            command_buffer: commands::CommandBuffer::new(),
            replay_hasher: replay::ReplayHasher::new(seed),
        }
    }

    /// Run a single simulation tick
    pub fn tick(&mut self) -> Result<()> {
        self.time.tick(self.config.fixed_timestep);
        
        // Process commands for this tick
        for cmd in self.command_buffer.drain() {
            self.replay_hasher.hash_command(&cmd);
            // Commands would be executed here in later milestones
        }
        
        Ok(())
    }

    /// Get current tick count
    pub fn tick_count(&self) -> u64 {
        self.time.tick_count
    }

    /// Get replay hash
    pub fn replay_hash(&self) -> String {
        self.replay_hasher.finalize()
    }

    /// Queue a command
    pub fn queue_command(&mut self, cmd: commands::Command) {
        self.command_buffer.push(cmd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_creation() {
        let config = EngineConfig::default();
        let engine = Engine::new(config);
        assert_eq!(engine.tick_count(), 0);
    }

    #[test]
    fn engine_ticks() {
        let config = EngineConfig::default();
        let mut engine = Engine::new(config);
        
        engine.tick().unwrap();
        assert_eq!(engine.tick_count(), 1);
        
        engine.tick().unwrap();
        assert_eq!(engine.tick_count(), 2);
    }
}
