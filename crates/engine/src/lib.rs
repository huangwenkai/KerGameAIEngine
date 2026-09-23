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
pub mod rng;
pub mod state;
pub mod ecs;
pub mod render;
pub mod chunk;
pub mod terrain;
pub mod biomes;
pub mod physics;
pub mod automata;
pub mod lighting;
pub mod items;
pub mod combat;
pub mod magic;
pub mod npc;
pub mod story;
pub mod audio;
pub mod save;
pub mod animation;
pub mod character;
pub mod structures;
pub mod needs;
pub mod enemy_ai;
pub mod day_night;
pub mod biome_spread;

#[cfg(test)]
mod replay_tests;

#[cfg(test)]
mod m5_tests;

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
    pub time: time::TimeState,
    command_buffer: commands::CommandBuffer,
    replay_hasher: replay::ReplayHasher,
    pub rng: rng::GameRng,
    pub world: state::World,
    pub ecs: ecs::EcsWorld,
    pub chunk_world: chunk::ChunkWorld,
    pub physics_sim: automata::PhysicsSimulator,
    pub light_map: lighting::LightMap,
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
            rng: rng::GameRng::new(seed),
            world: state::World::new(),
            ecs: ecs::EcsWorld::new(),
            chunk_world: chunk::ChunkWorld::new(),
            physics_sim: automata::PhysicsSimulator::new(10_000),
            light_map: lighting::LightMap::new(),
        }
    }

    /// Run a single simulation tick
    pub fn tick(&mut self) -> Result<()> {
        self.time.tick(self.config.fixed_timestep);
        
        // Process commands for this tick
        for cmd in self.command_buffer.drain() {
            self.replay_hasher.hash_command(&cmd);
            cmd.apply(&mut self.world, &mut self.chunk_world);
            
            // Wake cells affected by dig/place commands
            if let commands::Command::DigCell { x, y } = cmd {
                self.physics_sim.wake_cell(x, y);
                self.light_map.mark_dirty(x, y);
            } else if let commands::Command::PlaceCell { x, y, .. } = cmd {
                self.physics_sim.wake_cell(x, y);
                self.light_map.mark_dirty(x, y);
            }
        }
        
        // Update old world physics (M1)
        let dt = self.config.fixed_timestep.as_secs_f32();
        self.world.update_all(dt);
        
        // Update ECS systems (M2)
        self.ecs.system_movement(dt);
        self.ecs.system_collision(1000.0); // World bounds: 1000x1000
        
        // Update cellular automata physics (M6)
        self.physics_sim.update(&mut self.chunk_world);
        
        // Update lighting (M7) - incremental updates
        self.light_map.update(&mut self.chunk_world, 500);
        
        Ok(())
    }

    /// Get current tick count
    pub fn tick_count(&self) -> u64 {
        self.time.tick_count
    }

    /// Get replay hash (combines command hash + state hashes)
    pub fn replay_hash(&self) -> String {
        format!("{}:{}:{}", 
                self.replay_hasher.finalize(), 
                self.world.state_hash(),
                self.ecs.state_hash())
    }

    /// Queue a command for next tick
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

    #[test]
    fn deterministic_simulation() {
        let config = EngineConfig {
            seed: 42,
            ..Default::default()
        };
        
        let mut engine1 = Engine::new(config.clone());
        let mut engine2 = Engine::new(config);
        
        // Run identical command sequence
        for _ in 0..10 {
            let x = engine1.rng.gen_f32() * 100.0;
            let y = engine1.rng.gen_f32() * 100.0;
            engine1.queue_command(commands::Command::SpawnEntity { x, y });
        }
        
        // Reset RNG for engine2
        engine2.rng = rng::GameRng::new(42);
        for _ in 0..10 {
            let x = engine2.rng.gen_f32() * 100.0;
            let y = engine2.rng.gen_f32() * 100.0;
            engine2.queue_command(commands::Command::SpawnEntity { x, y });
        }
        
        engine1.tick().unwrap();
        engine2.tick().unwrap();
        
        assert_eq!(engine1.replay_hash(), engine2.replay_hash());
    }
}
