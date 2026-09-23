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

/// Demo registry
pub struct DemoRegistry {
    demos: Vec<Box<dyn Demo>>,
}

impl DemoRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            demos: Vec::new(),
        };
        
        // Register M0 demo
        registry.demos.push(Box::new(M0Demo));
        
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
