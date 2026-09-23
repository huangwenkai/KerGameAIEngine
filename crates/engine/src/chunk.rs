//! Chunk-based pixel world (4px cells, Noita-style)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Material types for 4px cells
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Material {
    Air = 0,
    Sand = 1,
    Stone = 2,
    Water = 3,
    Dirt = 4,
    Grass = 5,
}

impl Material {
    pub fn is_solid(self) -> bool {
        matches!(self, Material::Stone | Material::Dirt | Material::Grass)
    }

    pub fn is_liquid(self) -> bool {
        matches!(self, Material::Water)
    }

    pub fn is_powder(self) -> bool {
        matches!(self, Material::Sand)
    }

    pub fn color_rgba(self) -> [u8; 4] {
        match self {
            Material::Air => [0, 0, 0, 0],
            Material::Sand => [194, 178, 128, 255],
            Material::Stone => [128, 128, 128, 255],
            Material::Water => [64, 164, 223, 200],
            Material::Dirt => [139, 90, 43, 255],
            Material::Grass => [34, 139, 34, 255],
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Air
    }
}

/// Chunk coordinates (world space / CHUNK_SIZE)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_world(world_x: i32, world_y: i32) -> Self {
        Self {
            x: world_x.div_euclid(CHUNK_SIZE as i32),
            y: world_y.div_euclid(CHUNK_SIZE as i32),
        }
    }
}

/// Chunk size (128×128 cells)
pub const CHUNK_SIZE: usize = 128;

/// Single chunk (128×128 cells)
#[derive(Debug, Clone)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub cells: Box<[[Material; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub dirty: bool,
}

impl Chunk {
    pub fn new(coord: ChunkCoord) -> Self {
        Self {
            coord,
            cells: Box::new([[Material::Air; CHUNK_SIZE]; CHUNK_SIZE]),
            dirty: true,
        }
    }

    pub fn get(&self, local_x: usize, local_y: usize) -> Material {
        self.cells[local_y][local_x]
    }

    pub fn set(&mut self, local_x: usize, local_y: usize, material: Material) {
        self.cells[local_y][local_x] = material;
        self.dirty = true;
    }

    pub fn fill(&mut self, material: Material) {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                self.cells[y][x] = material;
            }
        }
        self.dirty = true;
    }
}

/// Chunked world with sparse storage
pub struct ChunkWorld {
    chunks: HashMap<ChunkCoord, Chunk>,
    pub loaded_count: usize,
}

impl ChunkWorld {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            loaded_count: 0,
        }
    }

    /// Get or create chunk
    pub fn get_or_create_chunk(&mut self, coord: ChunkCoord) -> &mut Chunk {
        self.chunks.entry(coord).or_insert_with(|| {
            self.loaded_count += 1;
            Chunk::new(coord)
        })
    }

    /// Get cell (world coordinates)
    pub fn get_cell(&mut self, world_x: i32, world_y: i32) -> Material {
        let chunk_coord = ChunkCoord::from_world(world_x, world_y);
        let local_x = world_x.rem_euclid(CHUNK_SIZE as i32) as usize;
        let local_y = world_y.rem_euclid(CHUNK_SIZE as i32) as usize;

        self.chunks
            .get(&chunk_coord)
            .map(|chunk| chunk.get(local_x, local_y))
            .unwrap_or(Material::Air)
    }

    /// Set cell (world coordinates)
    pub fn set_cell(&mut self, world_x: i32, world_y: i32, material: Material) {
        let chunk_coord = ChunkCoord::from_world(world_x, world_y);
        let local_x = world_x.rem_euclid(CHUNK_SIZE as i32) as usize;
        let local_y = world_y.rem_euclid(CHUNK_SIZE as i32) as usize;

        let chunk = self.get_or_create_chunk(chunk_coord);
        chunk.set(local_x, local_y, material);
    }

    /// Dig (remove material)
    pub fn dig(&mut self, world_x: i32, world_y: i32) {
        self.set_cell(world_x, world_y, Material::Air);
    }

    /// Place (add material)
    pub fn place(&mut self, world_x: i32, world_y: i32, material: Material) {
        self.set_cell(world_x, world_y, material);
    }

    /// Ensure chunks loaded in radius around focus point
    pub fn stream_chunks(&mut self, focus_x: i32, focus_y: i32, radius_chunks: i32) {
        let center = ChunkCoord::from_world(focus_x, focus_y);

        for dy in -radius_chunks..=radius_chunks {
            for dx in -radius_chunks..=radius_chunks {
                let coord = ChunkCoord::new(center.x + dx, center.y + dy);
                self.get_or_create_chunk(coord);
            }
        }
    }

    /// Get chunk count
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Clear all chunks
    pub fn clear(&mut self) {
        self.chunks.clear();
        self.loaded_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_coord_conversion() {
        assert_eq!(ChunkCoord::from_world(0, 0), ChunkCoord::new(0, 0));
        assert_eq!(ChunkCoord::from_world(127, 127), ChunkCoord::new(0, 0));
        assert_eq!(ChunkCoord::from_world(128, 128), ChunkCoord::new(1, 1));
        assert_eq!(ChunkCoord::from_world(-1, -1), ChunkCoord::new(-1, -1));
    }

    #[test]
    fn chunk_get_set() {
        let mut chunk = Chunk::new(ChunkCoord::new(0, 0));
        assert_eq!(chunk.get(0, 0), Material::Air);

        chunk.set(10, 20, Material::Stone);
        assert_eq!(chunk.get(10, 20), Material::Stone);
    }

    #[test]
    fn world_get_set() {
        let mut world = ChunkWorld::new();

        // Set cell in chunk (0, 0)
        world.set_cell(50, 60, Material::Sand);
        assert_eq!(world.get_cell(50, 60), Material::Sand);

        // Set cell in chunk (1, 1)
        world.set_cell(200, 200, Material::Stone);
        assert_eq!(world.get_cell(200, 200), Material::Stone);

        assert_eq!(world.chunk_count(), 2);
    }

    #[test]
    fn chunk_streaming() {
        let mut world = ChunkWorld::new();

        world.stream_chunks(0, 0, 1);
        // Should load 3×3 = 9 chunks around (0,0)
        assert_eq!(world.chunk_count(), 9);

        world.stream_chunks(1000, 1000, 2);
        // Center chunk + 5×5 = 25 total (some overlap with previous)
        assert!(world.chunk_count() >= 25);
    }

    #[test]
    fn material_properties() {
        assert!(Material::Stone.is_solid());
        assert!(!Material::Air.is_solid());
        assert!(Material::Water.is_liquid());
        assert!(Material::Sand.is_powder());
    }
}
