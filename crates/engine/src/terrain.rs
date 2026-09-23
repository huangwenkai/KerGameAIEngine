//! Procedural terrain generation with Perlin noise

use crate::chunk::{ChunkWorld, Material, ChunkCoord, CHUNK_SIZE};
use crate::rng::GameRng;

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

/// Terrain generator
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

    /// Generate terrain for a chunk
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

    /// Determine material at world coordinates
    fn get_material_at(&self, x: i32, y: i32) -> Material {
        let scale = 0.01; // Noise frequency
        let nx = x as f64 * scale;
        let ny = y as f64 * scale;

        // Surface height (varies by x)
        let height_noise = self.noise.octave_noise_2d(nx * 2.0, 0.0, 4, 0.5);
        let surface_y = (height_noise * 50.0) as i32;

        // Depth from surface
        let depth = y - surface_y;

        if depth < -5 {
            // Sky
            Material::Air
        } else if depth < 0 {
            // Above surface
            Material::Air
        } else if depth == 0 {
            // Surface layer
            Material::Grass
        } else if depth < 10 {
            // Dirt layer
            let dirt_noise = self.noise.noise_2d(nx * 5.0, ny * 5.0);
            if dirt_noise > 0.7 {
                Material::Stone // Occasional stone in dirt
            } else {
                Material::Dirt
            }
        } else if depth < 50 {
            // Stone layer
            let cave_noise = self.noise.octave_noise_2d(nx * 3.0, ny * 3.0, 3, 0.6);
            if cave_noise > 0.6 {
                Material::Air // Cave
            } else {
                Material::Stone
            }
        } else if depth < 100 {
            // Deep stone
            Material::Stone
        } else {
            // Very deep
            Material::Stone
        }
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
}
