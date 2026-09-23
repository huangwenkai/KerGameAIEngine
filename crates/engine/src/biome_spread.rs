//! Biome spread system - lightweight cellular conversion at boundaries

use crate::chunk::{ChunkWorld, Material};
use crate::rng::GameRng;

/// Biome spread configuration
#[derive(Debug, Clone)]
pub struct BiomeSpread {
    /// Enable spreading
    pub enabled: bool,
    /// Spread rate (cells per second)
    pub spread_rate: f32,
    /// Accumulated time for spread ticks
    accumulator: f32,
}

impl BiomeSpread {
    pub fn new(spread_rate: f32) -> Self {
        Self {
            enabled: true,
            spread_rate,
            accumulator: 0.0,
        }
    }
    
    /// Update spread (call every frame with dt)
    pub fn update(&mut self, dt: f32, world: &mut ChunkWorld, rng: &mut GameRng) {
        if !self.enabled {
            return;
        }
        
        self.accumulator += dt;
        
        // Spread 1 cell when accumulator exceeds threshold
        let threshold = 1.0 / self.spread_rate;
        if self.accumulator >= threshold {
            self.accumulator -= threshold;
            self.try_spread_one_cell(world, rng);
        }
    }
    
    fn try_spread_one_cell(&self, world: &mut ChunkWorld, rng: &mut GameRng) {
        // Pick random location to check for spread
        let x = rng.gen_range(-50..50);
        let y = rng.gen_range(-30..30);
        
        let current = world.get_cell(x, y);
        
        // Only spread to convertible materials
        if !self.is_convertible(current) {
            return;
        }
        
        // Check neighbors for "corrupt" biome (stone → corrupt stone)
        // or "crimson" (dirt → crimson dirt)
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x + dx;
            let ny = y + dy;
            let neighbor = world.get_cell(nx, ny);
            
            match neighbor {
                Material::Stone if current == Material::Dirt => {
                    // Corrupt spread: dirt near stone → corrupt (grass becomes dark)
                    if rng.gen_f32() < 0.3 {
                        world.set_cell(x, y, Material::Grass); // Reuse grass as "corrupt" visual
                    }
                }
                Material::Dirt if current == Material::Stone => {
                    // Crimson spread: stone near dirt → crimson stone (darker stone)
                    if rng.gen_f32() < 0.2 {
                        world.set_cell(x, y, Material::Water); // Reuse water as "crimson" visual marker
                    }
                }
                _ => {}
            }
        }
    }
    
    fn is_convertible(&self, material: Material) -> bool {
        matches!(material, Material::Dirt | Material::Stone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn biome_spread_accumulates() {
        let mut spread = BiomeSpread::new(1.0); // 1 cell/second
        let mut world = ChunkWorld::new();
        let mut rng = GameRng::new(42);
        
        // Seed some terrain
        for x in -10..10 {
            for y in -10..10 {
                world.set_cell(x, y, if (x + y) % 2 == 0 { Material::Dirt } else { Material::Stone });
            }
        }
        
        let initial_dirt = count_material(&mut world, Material::Dirt, -10, 10, -10, 10);
        
        // Run for 5 seconds
        for _ in 0..300 {
            spread.update(1.0 / 60.0, &mut world, &mut rng);
        }
        
        let final_dirt = count_material(&mut world, Material::Dirt, -10, 10, -10, 10);
        
        // Some conversion should have occurred
        assert_ne!(initial_dirt, final_dirt);
    }
    
    fn count_material(world: &mut ChunkWorld, material: Material, x1: i32, x2: i32, y1: i32, y2: i32) -> usize {
        let mut count = 0;
        for x in x1..x2 {
            for y in y1..y2 {
                if world.get_cell(x, y) == material {
                    count += 1;
                }
            }
        }
        count
    }
}
