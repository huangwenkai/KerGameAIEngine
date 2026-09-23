//! Tile-based lighting system for 4px chunk world

use crate::chunk::{ChunkWorld, Material};
use std::collections::{HashMap, VecDeque};

/// Light intensity (0-255)
pub type LightLevel = u8;

/// Maximum light intensity
pub const MAX_LIGHT: LightLevel = 255;

/// Minimum visible light
pub const MIN_LIGHT: LightLevel = 16;

/// Light propagation falloff per cell
pub const LIGHT_FALLOFF: u8 = 12;

/// Point light source
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub x: i32,
    pub y: i32,
    pub intensity: LightLevel,
    pub enabled: bool,
}

impl PointLight {
    pub fn new(x: i32, y: i32, intensity: LightLevel) -> Self {
        Self { x, y, intensity, enabled: true }
    }
}

/// Light map for a chunk region
pub struct LightMap {
    /// Light levels per cell (world coords -> light level)
    cells: HashMap<(i32, i32), LightLevel>,
    
    /// Dirty cells needing light recalculation
    dirty: Vec<(i32, i32)>,
    
    /// Point light sources
    lights: Vec<PointLight>,
    
    /// Ambient/sky light level
    ambient: LightLevel,
}

impl LightMap {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            dirty: Vec::new(),
            lights: Vec::new(),
            ambient: 64, // Default ambient
        }
    }

    pub fn set_ambient(&mut self, level: LightLevel) {
        self.ambient = level;
    }

    pub fn add_light(&mut self, light: PointLight) -> usize {
        let id = self.lights.len();
        self.lights.push(light);
        self.mark_dirty(light.x, light.y);
        id
    }

    pub fn remove_light(&mut self, id: usize) {
        if id < self.lights.len() {
            let light = self.lights[id];
            self.lights[id].enabled = false;
            self.mark_dirty(light.x, light.y);
        }
    }

    pub fn update_light(&mut self, id: usize, x: i32, y: i32, intensity: LightLevel) {
        if id < self.lights.len() {
            let old = self.lights[id];
            self.mark_dirty(old.x, old.y);
            self.lights[id].x = x;
            self.lights[id].y = y;
            self.lights[id].intensity = intensity;
            self.mark_dirty(x, y);
        }
    }

    pub fn get_light(&self, x: i32, y: i32) -> LightLevel {
        self.cells.get(&(x, y)).copied().unwrap_or(self.ambient)
    }

    pub fn mark_dirty(&mut self, x: i32, y: i32) {
        // Mark this cell and neighbors for recalculation
        for dy in -1..=1 {
            for dx in -1..=1 {
                self.dirty.push((x + dx, y + dy));
            }
        }
    }

    /// Mark region dirty (e.g., after dig/place)
    pub fn mark_region_dirty(&mut self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                self.mark_dirty(x, y);
            }
        }
    }

    /// Update light propagation incrementally (only dirty regions)
    pub fn update(&mut self, world: &mut ChunkWorld, max_updates: usize) {
        if self.dirty.is_empty() {
            return;
        }

        // Deduplicate dirty cells
        self.dirty.sort_unstable();
        self.dirty.dedup();

        // Process up to max_updates cells
        let to_process = self.dirty.len().min(max_updates);
        let dirty_batch: Vec<_> = self.dirty.drain(..to_process).collect();

        for (x, y) in dirty_batch {
            self.propagate_light(world, x, y);
        }
    }

    /// Propagate light from a single cell using BFS
    fn propagate_light(&mut self, world: &mut ChunkWorld, start_x: i32, start_y: i32) {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        // Find light sources affecting this region
        let mut max_intensity = self.ambient;
        
        for light in &self.lights {
            if !light.enabled {
                continue;
            }
            
            let dx = (light.x - start_x).abs();
            let dy = (light.y - start_y).abs();
            let dist = dx.max(dy) as u32;
            
            if dist < 20 {
                // Light might affect this region
                queue.push_back((light.x, light.y, light.intensity));
                visited.insert((light.x, light.y), light.intensity);
                max_intensity = max_intensity.max(light.intensity);
            }
        }

        // BFS light propagation
        while let Some((x, y, intensity)) = queue.pop_front() {
            if intensity < MIN_LIGHT {
                continue;
            }

            // Update cell light
            let current = self.cells.entry((x, y)).or_insert(0);
            if intensity > *current {
                *current = intensity;
            }

            // Check material blocking
            let material = world.get_cell(x, y);
            let blocked = material.is_solid();

            if blocked {
                // Solid materials block most light
                continue;
            }

            // Propagate to neighbors
            let next_intensity = intensity.saturating_sub(LIGHT_FALLOFF);
            if next_intensity < MIN_LIGHT {
                continue;
            }

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = x + dx;
                    let ny = y + dy;
                    let key = (nx, ny);

                    // Check if target cell blocks light
                    let target_material = world.get_cell(nx, ny);
                    if target_material.is_solid() {
                        // Still light up the solid cell itself (so walls are slightly lit)
                        // but don't propagate beyond it
                        let lit_intensity = next_intensity.saturating_sub(LIGHT_FALLOFF * 2);
                        if lit_intensity >= MIN_LIGHT {
                            let wall_light = self.cells.entry((nx, ny)).or_insert(0);
                            if lit_intensity > *wall_light {
                                *wall_light = lit_intensity;
                            }
                        }
                        continue;
                    }

                    if let Some(&prev) = visited.get(&key) {
                        if prev >= next_intensity {
                            continue; // Already visited with higher intensity
                        }
                    }

                    visited.insert(key, next_intensity);
                    queue.push_back((nx, ny, next_intensity));
                }
            }
        }
    }

    /// Force full recalculation (expensive, use sparingly)
    pub fn full_recalculate(&mut self, world: &mut ChunkWorld, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
        self.cells.clear();
        
        // Set ambient baseline
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                self.cells.insert((x, y), self.ambient);
            }
        }

        // Collect lights to propagate (to avoid borrow checker issues)
        let lights_to_process: Vec<_> = self.lights.iter()
            .filter(|l| l.enabled)
            .map(|l| (l.x, l.y))
            .collect();

        // Propagate all lights
        for (x, y) in lights_to_process {
            self.propagate_light(world, x, y);
        }

        self.dirty.clear();
    }

    pub fn light_count(&self) -> usize {
        self.lights.iter().filter(|l| l.enabled).count()
    }

    pub fn dirty_count(&self) -> usize {
        self.dirty.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_creation() {
        let mut map = LightMap::new();
        assert_eq!(map.light_count(), 0);

        let id = map.add_light(PointLight::new(10, 10, 200));
        assert_eq!(map.light_count(), 1);

        map.remove_light(id);
        assert_eq!(map.light_count(), 0);
    }

    #[test]
    fn ambient_light() {
        let mut map = LightMap::new();
        map.set_ambient(100);

        let mut world = ChunkWorld::new();
        let light = map.get_light(50, 50);
        assert_eq!(light, 100);
    }

    #[test]
    fn light_propagation_simple() {
        let mut map = LightMap::new();
        map.set_ambient(0);

        let mut world = ChunkWorld::new();
        map.add_light(PointLight::new(10, 10, MAX_LIGHT));
        
        map.update(&mut world, 1000);

        // Center should be bright
        let center = map.get_light(10, 10);
        assert!(center > 200, "Center should be bright");

        // Adjacent should be dimmer
        let adjacent = map.get_light(11, 10);
        assert!(adjacent < center, "Adjacent should be dimmer");
        assert!(adjacent > 100, "Adjacent should still be lit");
    }

    #[test]
    fn dirty_tracking() {
        let mut map = LightMap::new();
        assert_eq!(map.dirty_count(), 0);

        map.mark_dirty(10, 10);
        assert!(map.dirty_count() > 0);

        let mut world = ChunkWorld::new();
        map.update(&mut world, 100);

        // Should process some dirty cells
        assert!(map.dirty_count() < 10);
    }

    #[test]
    fn multiple_lights() {
        let mut map = LightMap::new();
        map.set_ambient(0);

        let mut world = ChunkWorld::new();
        map.add_light(PointLight::new(5, 10, 200));
        map.add_light(PointLight::new(15, 10, 200));

        map.full_recalculate(&mut world, 0, 0, 20, 20);

        // Middle should be lit by both
        let middle = map.get_light(10, 10);
        assert!(middle > 100, "Middle should be lit by both sources");

        // Far left should be lit by left light only
        let far_left = map.get_light(2, 10);
        assert!(far_left > 50, "Far left should be lit");

        // Far right should be lit by right light only
        let far_right = map.get_light(18, 10);
        assert!(far_right > 50, "Far right should be lit");
    }
}
