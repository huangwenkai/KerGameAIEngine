//! Cellular automata physics for falling sand and fluids

use crate::chunk::{ChunkWorld, Material};
use std::collections::HashSet;

/// Wake set: tracks active cells that need physics updates
pub struct WakeSet {
    active: HashSet<(i32, i32)>,
    next: HashSet<(i32, i32)>,
}

impl WakeSet {
    pub fn new() -> Self {
        Self {
            active: HashSet::new(),
            next: HashSet::new(),
        }
    }

    pub fn wake(&mut self, x: i32, y: i32) {
        self.active.insert((x, y));
    }

    pub fn wake_neighbors(&mut self, x: i32, y: i32) {
        for dy in -1..=1 {
            for dx in -1..=1 {
                self.next.insert((x + dx, y + dy));
            }
        }
    }

    pub fn step(&mut self) {
        std::mem::swap(&mut self.active, &mut self.next);
        self.next.clear();
    }

    pub fn active_cells(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.active.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.active.len()
    }
}

/// Material physics properties
impl Material {
    /// Check if material can fall (affected by gravity)
    pub fn can_fall(self) -> bool {
        matches!(self, Material::Sand | Material::Water)
    }

    /// Density for material displacement (higher = sinks lower)
    pub fn density(self) -> u8 {
        match self {
            Material::Air => 0,
            Material::Water => 1,
            Material::Sand => 2,
            Material::Stone => 255,
            Material::Dirt => 2,
            Material::Grass => 2,
        }
    }
}

/// Cellular automata physics simulator
pub struct PhysicsSimulator {
    wake_set: WakeSet,
    update_budget: usize,
    updates_this_tick: usize,
}

impl PhysicsSimulator {
    pub fn new(update_budget: usize) -> Self {
        Self {
            wake_set: WakeSet::new(),
            update_budget,
            updates_this_tick: 0,
        }
    }

    /// Wake a cell for physics updates
    pub fn wake_cell(&mut self, x: i32, y: i32) {
        self.wake_set.wake(x, y);
    }

    /// Wake a region (e.g., after terrain generation)
    pub fn wake_region(&mut self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                self.wake_set.wake(x, y);
            }
        }
    }

    /// Update physics for one simulation tick
    pub fn update(&mut self, world: &mut ChunkWorld) {
        self.updates_this_tick = 0;

        // Process active cells with budget limit
        let cells: Vec<_> = self.wake_set.active_cells().collect();
        
        for (x, y) in cells {
            if self.updates_this_tick >= self.update_budget {
                // Budget exhausted, defer remaining cells to next tick
                self.wake_set.wake(x, y);
                break;
            }

            let material = world.get_cell(x, y);
            
            if material.can_fall() {
                if self.try_fall(world, x, y, material) {
                    self.updates_this_tick += 1;
                }
            }
        }

        self.wake_set.step();
    }

    /// Try to move a falling material down
    fn try_fall(&mut self, world: &mut ChunkWorld, x: i32, y: i32, material: Material) -> bool {
        let below = world.get_cell(x, y + 1);

        // Try straight down
        if below == Material::Air || (material.density() > below.density()) {
            world.set_cell(x, y, below);
            world.set_cell(x, y + 1, material);
            self.wake_set.wake_neighbors(x, y + 1);
            return true;
        }

        // Liquids spread sideways
        if material.is_liquid() {
            // Try left
            let left = world.get_cell(x - 1, y);
            if left == Material::Air || (material.density() > left.density()) {
                world.set_cell(x, y, left);
                world.set_cell(x - 1, y, material);
                self.wake_set.wake_neighbors(x - 1, y);
                return true;
            }

            // Try right
            let right = world.get_cell(x + 1, y);
            if right == Material::Air || (material.density() > right.density()) {
                world.set_cell(x, y, right);
                world.set_cell(x + 1, y, material);
                self.wake_set.wake_neighbors(x + 1, y);
                return true;
            }
        }

        // Sand slides diagonally
        if material == Material::Sand {
            // Try down-left
            let down_left = world.get_cell(x - 1, y + 1);
            if down_left == Material::Air {
                world.set_cell(x, y, Material::Air);
                world.set_cell(x - 1, y + 1, material);
                self.wake_set.wake_neighbors(x - 1, y + 1);
                return true;
            }

            // Try down-right
            let down_right = world.get_cell(x + 1, y + 1);
            if down_right == Material::Air {
                world.set_cell(x, y, Material::Air);
                world.set_cell(x + 1, y + 1, material);
                self.wake_set.wake_neighbors(x + 1, y + 1);
                return true;
            }
        }

        false
    }

    pub fn active_cell_count(&self) -> usize {
        self.wake_set.len()
    }

    pub fn updates_this_tick(&self) -> usize {
        self.updates_this_tick
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wake_set_basic() {
        let mut wake = WakeSet::new();
        wake.wake(10, 20);
        assert_eq!(wake.len(), 1);

        wake.step();
        assert_eq!(wake.len(), 0);
    }

    #[test]
    fn wake_neighbors() {
        let mut wake = WakeSet::new();
        wake.wake_neighbors(10, 10);
        wake.step(); // Move next -> active
        assert_eq!(wake.len(), 9); // 3x3 grid
    }

    #[test]
    fn sand_falls() {
        let mut world = ChunkWorld::new();
        let mut sim = PhysicsSimulator::new(1000);

        // Place sand above air
        world.set_cell(10, 10, Material::Sand);
        sim.wake_cell(10, 10);

        // Simulate one tick
        sim.update(&mut world);

        // Sand should have fallen
        assert_eq!(world.get_cell(10, 10), Material::Air);
        assert_eq!(world.get_cell(10, 11), Material::Sand);
    }

    #[test]
    fn material_density() {
        assert!(Material::Sand.density() > Material::Water.density());
        assert!(Material::Stone.density() > Material::Sand.density());
        assert_eq!(Material::Air.density(), 0);
    }

    #[test]
    fn budget_limit() {
        let mut world = ChunkWorld::new();
        let mut sim = PhysicsSimulator::new(5); // Low budget

        // Place 10 sand cells
        for i in 0..10 {
            world.set_cell(i, 10, Material::Sand);
            sim.wake_cell(i, 10);
        }

        sim.update(&mut world);

        // Should have processed only 5 updates (budget)
        assert!(sim.updates_this_tick() <= 5);
    }
}
