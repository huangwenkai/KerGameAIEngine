//! Procedural terrain generation with rich biomes, caves, and ores

use crate::chunk::{ChunkWorld, Material, ChunkCoord, CHUNK_SIZE};
use crate::rng::GameRng;
use crate::biomes::{BiomeType, OreType};

/// Simple 2D Perlin-like noise (deterministic from seed)
pub struct NoiseGenerator {
    seed: u64,
}

impl NoiseGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate noise value at (x, y) in range [0, 1]
    pub fn noise_2d(&self, x: f64, y: f64) -> f64 {
        // Simple hash-based noise (not true Perlin but deterministic)
        let ix = x.floor() as i64;
        let iy = y.floor() as i64;
        let fx = x - ix as f64;
        let fy = y - iy as f64;

        // Smooth interpolation
        let u = fx * fx * (3.0 - 2.0 * fx);
        let v = fy * fy * (3.0 - 2.0 * fy);

        let a = self.hash(ix, iy);
        let b = self.hash(ix + 1, iy);
        let c = self.hash(ix, iy + 1);
        let d = self.hash(ix + 1, iy + 1);

        let x1 = a * (1.0 - u) + b * u;
        let x2 = c * (1.0 - u) + d * u;

        x1 * (1.0 - v) + x2 * v
    }

    fn hash(&self, x: i64, y: i64) -> f64 {
        let mut h = self.seed.wrapping_mul(2654435761);
        h = h.wrapping_add(x as u64);
        h ^= h >> 16;
        h = h.wrapping_mul(0x7feb352d);
        h = h.wrapping_add(y as u64);
        h ^= h >> 15;
        h = h.wrapping_mul(0x846ca68b);
        h ^= h >> 16;

        (h % 1000000) as f64 / 1000000.0
    }

    /// Octave noise (multiple frequencies)
    pub fn octave_noise_2d(&self, x: f64, y: f64, octaves: u32, persistence: f64) -> f64 {
        let mut total = 0.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;
        let mut frequency = 1.0;

        for _ in 0..octaves {
            total += self.noise_2d(x * frequency, y * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= 2.0;
        }

        total / max_value
    }
}

/// Rich terrain generator with biomes, caves, and ores
pub struct TerrainGenerator {
    noise: NoiseGenerator,
    seed: u64,
}

impl TerrainGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            noise: NoiseGenerator::new(seed),
            seed,
        }
    }

    /// Generate terrain for a chunk with all features
    pub fn generate_chunk(&self, world: &mut ChunkWorld, coord: ChunkCoord) {
        let chunk = world.get_or_create_chunk(coord);
        let base_x = coord.x * CHUNK_SIZE as i32;
        let base_y = coord.y * CHUNK_SIZE as i32;

        for local_y in 0..CHUNK_SIZE {
            for local_x in 0..CHUNK_SIZE {
                let world_x = base_x + local_x as i32;
                let world_y = base_y + local_y as i32;

                let material = self.get_material_at(world_x, world_y);
                chunk.set(local_x, local_y, material);
            }
        }
    }

    /// Get biome at world X coordinate (exposed for M16 demo)
    pub fn get_biome_at(&self, x: i32) -> BiomeType {
        self.get_biome(x)
    }
    
    /// Determine biome at world X coordinate
    fn get_biome(&self, x: i32) -> BiomeType {
        let scale = 0.002; // Large-scale biome transitions
        let biome_noise = self.noise.noise_2d(x as f64 * scale, 0.0);

        if biome_noise < 0.15 {
            BiomeType::Desert
        } else if biome_noise < 0.35 {
            BiomeType::Swamp
        } else if biome_noise < 0.60 {
            BiomeType::Grassland
        } else if biome_noise < 0.80 {
            BiomeType::Jungle
        } else {
            BiomeType::Mountain
        }
    }

    /// Check if position is inside a cave
    fn is_cave(&self, x: i32, y: i32) -> bool {
        // Only generate caves underground (y > 10)
        if y < 10 {
            return false;
        }

        // 3D cave noise (worm-like caves)
        let scale = 0.05;
        let nx = x as f64 * scale;
        let ny = y as f64 * scale;

        let cave_noise = self.noise.octave_noise_2d(nx, ny, 4, 0.5);

        // Threshold for cave: between 0.45 and 0.55 creates winding tunnels
        cave_noise > 0.45 && cave_noise < 0.55
    }

    /// Check if position should have ore vein
    fn get_ore(&self, x: i32, y: i32, depth: i32) -> Option<OreType> {
        let ores = [
            (OreType::Copper, 1.0),
            (OreType::Iron, 2.0),
            (OreType::Gold, 3.0),
            (OreType::MagicCrystal, 4.0),
        ];

        for (ore, offset) in &ores {
            let (min_depth, max_depth) = ore.depth_range();
            if depth >= min_depth && depth <= max_depth {
                // Use noise to determine if ore spawns here (unique offset per ore type)
                let ore_noise = self.noise.noise_2d(
                    x as f64 * 0.1 + offset * 1000.0,
                    y as f64 * 0.1 + offset * 500.0,
                );

                if ore_noise < ore.frequency() {
                    return Some(*ore);
                }
            }
        }

        None
    }

    /// Determine material at world coordinates with all features
    fn get_material_at(&self, x: i32, y: i32) -> Material {
        let biome = self.get_biome(x);
        let scale = 0.01;
        let nx = x as f64 * scale;
        let ny = y as f64 * scale;

        // Surface height with biome variation
        let height_noise = self.noise.octave_noise_2d(nx * 2.0, 0.0, 4, 0.5);
        let biome_offset = biome.height_offset();
        let surface_y = (height_noise * 50.0 + biome_offset) as i32;

        let depth = y - surface_y;

        // Air above surface
        if depth < -5 {
            return Material::Air;
        } else if depth < 0 {
            return Material::Air;
        }

        // Check for caves first (highest priority)
        if self.is_cave(x, y) {
            // Lava at very deep caves
            if y > 150 && self.noise.noise_2d(x as f64 * 0.2, y as f64 * 0.2) > 0.7 {
                return Material::Water; // TODO: Add lava material
            }
            return Material::Air;
        }

        // Surface layer
        if depth == 0 {
            return biome.surface_material();
        }

        // Check for ores
        if depth > 5 {
            if let Some(ore) = self.get_ore(x, y, depth) {
                return ore.material(); // Returns Stone for now, will be ore materials later
            }
        }

        // Subsurface layers
        if depth < 10 {
            return biome.subsurface_material();
        }

        // Check for underground lakes/water
        if depth > 20 && depth < 40 {
            let water_noise = self.noise.noise_2d(nx * 3.0, ny * 3.0);
            if water_noise > 0.75 {
                return Material::Water;
            }
        }

        // Deep stone layer
        Material::Stone
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_deterministic() {
        let gen1 = NoiseGenerator::new(42);
        let gen2 = NoiseGenerator::new(42);

        assert_eq!(gen1.noise_2d(10.5, 20.3), gen2.noise_2d(10.5, 20.3));
    }

    #[test]
    fn noise_different_seeds() {
        let gen1 = NoiseGenerator::new(42);
        let gen2 = NoiseGenerator::new(123);

        assert_ne!(gen1.noise_2d(10.5, 20.3), gen2.noise_2d(10.5, 20.3));
    }

    #[test]
    fn terrain_generation() {
        let mut world = ChunkWorld::new();
        let gen = TerrainGenerator::new(42);

        gen.generate_chunk(&mut world, ChunkCoord::new(0, 0));

        // Should have materials other than Air
        let mut found_solid = false;
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let mat = world.get_cell(x as i32, y as i32);
                if mat != Material::Air {
                    found_solid = true;
                    break;
                }
            }
        }

        assert!(found_solid, "Terrain should have non-Air materials");
    }

    #[test]
    fn terrain_deterministic() {
        let mut world1 = ChunkWorld::new();
        let mut world2 = ChunkWorld::new();
        let gen = TerrainGenerator::new(42);

        gen.generate_chunk(&mut world1, ChunkCoord::new(0, 0));
        gen.generate_chunk(&mut world2, ChunkCoord::new(0, 0));

        // Same seed should produce identical terrain
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                assert_eq!(
                    world1.get_cell(x as i32, y as i32),
                    world2.get_cell(x as i32, y as i32)
                );
            }
        }
    }

    #[test]
    fn biome_diversity() {
        let gen = TerrainGenerator::new(42);

        let mut biomes = std::collections::HashSet::new();
        for x in (-500..500).step_by(100) {
            biomes.insert(gen.get_biome(x));
        }

        // Should have multiple biomes in range
        assert!(biomes.len() >= 3, "Should have at least 3 biome types in test range");
    }

    #[test]
    fn caves_exist() {
        let gen = TerrainGenerator::new(42);

        let mut found_cave = false;
        for y in 10..100 {
            for x in 0..100 {
                if gen.is_cave(x, y) {
                    found_cave = true;
                    break;
                }
            }
            if found_cave {
                break;
            }
        }

        assert!(found_cave, "Should find at least one cave in test range");
    }
}
