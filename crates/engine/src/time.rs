//! Fixed timestep time management
//! Simulation runs at fixed rate, decoupled from render rate

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct TimeState {
    pub tick_count: u64,
    pub sim_time: Duration,
    start_instant: Instant,
}

impl TimeState {
    pub fn new() -> Self {
        Self {
            tick_count: 0,
            sim_time: Duration::ZERO,
            start_instant: Instant::now(),
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        self.tick_count += 1;
        self.sim_time += dt;
    }

    pub fn elapsed_real(&self) -> Duration {
        self.start_instant.elapsed()
    }
}
