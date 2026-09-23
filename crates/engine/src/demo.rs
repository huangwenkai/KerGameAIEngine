//! Demo system for stress testing and validation

use crate::{Engine, EngineConfig};
use crate::report::DemoReport;
use anyhow::Result;
use std::time::Instant;

pub trait Demo {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn run(&self, engine: &mut Engine) -> Result<()>;
}

/// M0 demo: Basic scaffold validation
pub struct M0Demo;

impl Demo for M0Demo {
    fn id(&self) -> &str {
        "M0"
    }

    fn description(&self) -> &str {
        "M0 scaffold validation: CLI + headless harness + report generation"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M0 demo: {}", self.description());
        
        // Run 600 ticks (10 seconds at 60 TPS)
        for _ in 0..600 {
            engine.tick()?;
        }
        
        log::info!("M0 demo completed: {} ticks", engine.tick_count());
        Ok(())
    }
}

/// M1 demo: Command execution and replay validation
pub struct M1Demo;

impl Demo for M1Demo {
    fn id(&self) -> &str {
        "M1"
    }

    fn description(&self) -> &str {
        "M1 command replay: 1000 commands with deterministic execution"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M1 demo: {}", self.description());
        
        // Phase 1: Spawn 100 entities with random positions
        log::info!("Phase 1: Spawning 100 entities");
        for _ in 0..100 {
            let x = engine.rng.gen_range(0.0..1000.0);
            let y = engine.rng.gen_range(0.0..1000.0);
            engine.queue_command(crate::commands::Command::SpawnEntity { x, y });
        }
        engine.tick()?;
        
        assert_eq!(engine.world.entity_count(), 100, "Should have 100 entities");
        
        // Phase 2: Apply random velocities to entities (900 commands)
        log::info!("Phase 2: Applying velocities");
        for tick in 0..30 {
            for entity_id in 1..=100 {
                if tick % 3 == 0 {
                    let vx = engine.rng.gen_range(-10.0..10.0);
                    let vy = engine.rng.gen_range(-10.0..10.0);
                    engine.queue_command(crate::commands::Command::SetVelocity {
                        entity_id,
                        vx,
                        vy,
                    });
                }
            }
            engine.tick()?;
        }
        
        // Phase 3: Simulate for 10 more seconds (600 ticks)
        log::info!("Phase 3: Simulating movement");
        for _ in 0..600 {
            engine.tick()?;
        }
        
        log::info!("M1 demo completed: {} ticks, {} entities, final state hash: {}",
                   engine.tick_count(),
                   engine.world.entity_count(),
                   engine.world.state_hash());
        
        // Verify state is non-trivial
        assert!(engine.world.entity_count() > 0, "World should have entities");
        
        Ok(())
    }
}

/// M2 demo: ECS stress test with 10k entities
pub struct M2Demo;

impl Demo for M2Demo {
    fn id(&self) -> &str {
        "M2"
    }

    fn description(&self) -> &str {
        "M2 ECS stress: 10,000 entities with movement + collision systems"
    }

    fn run(&self, engine: &mut Engine) -> Result<()> {
        log::info!("Running M2 demo: {}", self.description());
        
        // Spawn 10,000 entities with random positions and velocities
        log::info!("Spawning 10,000 entities...");
        for _ in 0..10_000 {
            let x = engine.rng.gen_range(0.0..1000.0);
            let y = engine.rng.gen_range(0.0..1000.0);
            let vx = engine.rng.gen_range(-50.0..50.0);
            let vy = engine.rng.gen_range(-50.0..50.0);
            let health = engine.rng.gen_range(50.0..200.0);
            
            engine.ecs.spawn_entity(x, y, vx, vy, health);
        }
        
        log::info!("Entities spawned: {}", engine.ecs.entity_count());
        assert_eq!(engine.ecs.entity_count(), 10_000);
        
        // Simulate for 600 ticks (10 seconds @ 60 TPS)
        log::info!("Simulating 600 ticks...");
        let start = std::time::Instant::now();
        
        for tick in 0..600 {
            engine.tick()?;
            
            if (tick + 1) % 100 == 0 {
                log::info!("Tick {}/600 ({}ms elapsed)", 
                           tick + 1, 
                           start.elapsed().as_millis());
            }
        }
        
        let elapsed = start.elapsed();
        let avg_tick_ms = elapsed.as_secs_f64() * 1000.0 / 600.0;
        
        log::info!("M2 demo completed: {} ticks, {} entities",
                   engine.tick_count(),
                   engine.ecs.entity_count());
        log::info!("Performance: {:.3}ms per tick (avg), {:.1} FPS capable",
                   avg_tick_ms,
                   1000.0 / avg_tick_ms);
        log::info!("ECS state hash: {}", engine.ecs.state_hash());
        
        // Verify performance target: < 2ms per tick
        assert!(avg_tick_ms < 2.0, 
                "Performance target not met: {:.3}ms > 2.0ms per tick", 
                avg_tick_ms);
        
        Ok(())
    }
}

/// Demo registry
pub struct DemoRegistry {
    demos: Vec<Box<dyn Demo>>,
}

impl DemoRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            demos: Vec::new(),
        };
        
        // Register demos
        registry.demos.push(Box::new(M0Demo));
        registry.demos.push(Box::new(M1Demo));
        registry.demos.push(Box::new(M2Demo));
        
        registry
    }

    pub fn get(&self, id: &str) -> Option<&dyn Demo> {
        self.demos.iter()
            .find(|d| d.id() == id)
            .map(|b| b.as_ref())
    }

    pub fn list(&self) -> Vec<&dyn Demo> {
        self.demos.iter().map(|b| b.as_ref()).collect()
    }
}

/// Run a demo and generate report
pub fn run_demo(demo_id: &str, seed: u64, headless: bool) -> Result<DemoReport> {
    let registry = DemoRegistry::new();
    let demo = registry.get(demo_id)
        .ok_or_else(|| anyhow::anyhow!("Demo '{}' not found", demo_id))?;

    let config = EngineConfig {
        headless,
        seed,
        ..Default::default()
    };

    let mut engine = Engine::new(config);
    let start = Instant::now();
    
    demo.run(&mut engine)?;
    
    let report = DemoReport::success(
        demo_id.to_string(),
        seed,
        engine.tick_count(),
        start.elapsed(),
        engine.time.sim_time,
        engine.replay_hash(),
    );

    Ok(report)
}
