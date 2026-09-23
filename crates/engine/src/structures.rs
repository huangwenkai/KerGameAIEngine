//! Structure generation system (villages, dungeons, etc.)

use crate::chunk::{ChunkWorld, Material};
use crate::rng::GameRng;
use anyhow::Result;

/// Structure type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureType {
    House,
    Shop,
    Altar,
    UndergroundDungeon,
}

/// Structure template
#[derive(Debug, Clone)]
pub struct StructureTemplate {
    pub structure_type: StructureType,
    pub width: i32,
    pub height: i32,
    /// Material grid (relative coords)
    pub blocks: Vec<(i32, i32, Material)>,
    /// NPC spawn points (relative coords, NPC type)
    pub npc_spawns: Vec<(i32, i32, String)>,
}

impl StructureTemplate {
    /// Create a simple house template
    pub fn house() -> Self {
        let mut blocks = Vec::new();
        let width = 12;
        let height = 8;
        
        // Floor
        for x in 0..width {
            blocks.push((x, height - 1, Material::Stone));
        }
        
        // Walls
        for y in 0..height - 1 {
            blocks.push((0, y, Material::Stone));
            blocks.push((width - 1, y, Material::Stone));
        }
        
        // Roof
        for x in 1..width - 1 {
            blocks.push((x, 0, Material::Stone));
        }
        
        // Door (air gap)
        blocks.push((width / 2, height - 2, Material::Air));
        
        Self {
            structure_type: StructureType::House,
            width,
            height,
            blocks,
            npc_spawns: vec![(width / 2, height - 3, "Villager".to_string())],
        }
    }
    
    /// Create a shop template
    pub fn shop() -> Self {
        let mut blocks = Vec::new();
        let width = 16;
        let height = 10;
        
        // Floor
        for x in 0..width {
            blocks.push((x, height - 1, Material::Stone));
        }
        
        // Walls
        for y in 0..height - 1 {
            blocks.push((0, y, Material::Stone));
            blocks.push((width - 1, y, Material::Stone));
        }
        
        // Roof
        for x in 1..width - 1 {
            blocks.push((x, 0, Material::Stone));
        }
        
        // Door
        blocks.push((width / 2, height - 2, Material::Air));
        
        // Counter (stone blocks inside)
        for x in 3..6 {
            blocks.push((x, height - 2, Material::Stone));
        }
        
        Self {
            structure_type: StructureType::Shop,
            width,
            height,
            blocks,
            npc_spawns: vec![(4, height - 3, "Merchant".to_string())],
        }
    }
    
    /// Create an altar template
    pub fn altar() -> Self {
        let mut blocks = Vec::new();
        let width = 8;
        let height = 6;
        
        // Floor
        for x in 0..width {
            blocks.push((x, height - 1, Material::Stone));
        }
        
        // Altar pillar
        for y in height - 4..height - 1 {
            blocks.push((width / 2, y, Material::Stone));
        }
        
        Self {
            structure_type: StructureType::Altar,
            width,
            height,
            blocks,
            npc_spawns: vec![],
        }
    }
    
    /// Create an underground dungeon template
    pub fn underground_dungeon() -> Self {
        let mut blocks = Vec::new();
        let width = 20;
        let height = 12;
        
        // Carve out air space (dungeon room)
        for y in 2..height - 2 {
            for x in 2..width - 2 {
                blocks.push((x, y, Material::Air));
            }
        }
        
        // Stone walls
        for x in 0..width {
            blocks.push((x, 0, Material::Stone));
            blocks.push((x, height - 1, Material::Stone));
        }
        for y in 1..height - 1 {
            blocks.push((0, y, Material::Stone));
            blocks.push((width - 1, y, Material::Stone));
        }
        
        // Entrance tunnel (air)
        blocks.push((width / 2, 0, Material::Air));
        blocks.push((width / 2, 1, Material::Air));
        
        Self {
            structure_type: StructureType::UndergroundDungeon,
            width,
            height,
            blocks,
            npc_spawns: vec![(width / 2, height / 2, "DungeonGuard".to_string())],
        }
    }
}

/// Structure instance in world
#[derive(Debug, Clone)]
pub struct Structure {
    pub template: StructureTemplate,
    pub world_x: i32,
    pub world_y: i32,
}

/// Structure generator
pub struct StructureGenerator {
    seed: u64,
}

impl StructureGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
    
    /// Find surface positions for structures
    fn find_surface_positions(&self, world: &mut ChunkWorld, count: usize, rng: &mut GameRng) -> Vec<(i32, i32)> {
        let mut positions = Vec::new();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 1000;
        
        while positions.len() < count && attempts < MAX_ATTEMPTS {
            attempts += 1;
            
            // Random X position
            let x = rng.gen_range(-100..100);
            
            // Find ground level (scan downward)
            let mut ground_y = None;
            for y in -20..50 {
                let mat = world.get_cell(x, y);
                if mat.is_solid() {
                    // Check if air above
                    let above1 = world.get_cell(x, y - 1);
                    let above2 = world.get_cell(x, y - 2);
                    if !above1.is_solid() && !above2.is_solid() {
                        ground_y = Some(y);
                        break;
                    }
                }
            }
            
            if let Some(y) = ground_y {
                // Check min spacing from existing structures
                let min_spacing = 40;
                let too_close = positions.iter().any(|(px, _py)| (x - px).abs() < min_spacing);
                
                if !too_close {
                    positions.push((x, y - 8)); // Place structure above ground
                }
            }
        }
        
        positions
    }
    
    /// Find underground positions for dungeons
    fn find_underground_positions(&self, world: &mut ChunkWorld, count: usize, rng: &mut GameRng) -> Vec<(i32, i32)> {
        let mut positions = Vec::new();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 1000;
        
        while positions.len() < count && attempts < MAX_ATTEMPTS {
            attempts += 1;
            
            let x = rng.gen_range(-80..80);
            let y = rng.gen_range(20..60); // Deep underground
            
            // Check if area is mostly solid (for carving)
            let mut solid_count = 0;
            for dy in 0..12 {
                for dx in 0..20 {
                    if world.get_cell(x + dx, y + dy).is_solid() {
                        solid_count += 1;
                    }
                }
            }
            
            // Need mostly solid for carving dungeon
            if solid_count > 200 {
                let min_spacing = 60;
                let too_close = positions.iter().any(|(px, _py)| (x - px).abs() < min_spacing);
                
                if !too_close {
                    positions.push((x, y));
                }
            }
        }
        
        positions
    }
    
    /// Generate structures in world
    pub fn generate_structures(&self, world: &mut ChunkWorld, rng: &mut GameRng) -> Vec<Structure> {
        let mut structures = Vec::new();
        
        // Surface structures
        let surface_pos = self.find_surface_positions(world, 3, rng);
        
        for (i, (x, y)) in surface_pos.iter().enumerate() {
            let template = match i % 3 {
                0 => StructureTemplate::house(),
                1 => StructureTemplate::shop(),
                _ => StructureTemplate::altar(),
            };
            
            structures.push(Structure {
                template,
                world_x: *x,
                world_y: *y,
            });
        }
        
        // Underground dungeons
        let underground_pos = self.find_underground_positions(world, 1, rng);
        
        for (x, y) in underground_pos {
            let template = StructureTemplate::underground_dungeon();
            structures.push(Structure {
                template,
                world_x: x,
                world_y: y,
            });
        }
        
        structures
    }
    
    /// Place structure in world
    pub fn place_structure(&self, world: &mut ChunkWorld, structure: &Structure) {
        for (rel_x, rel_y, material) in &structure.template.blocks {
            let world_x = structure.world_x + rel_x;
            let world_y = structure.world_y + rel_y;
            world.set_cell(world_x, world_y, *material);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn structure_templates() {
        let house = StructureTemplate::house();
        assert_eq!(house.structure_type, StructureType::House);
        assert!(house.width > 0);
        assert!(house.height > 0);
        assert!(!house.blocks.is_empty());
        
        let shop = StructureTemplate::shop();
        assert_eq!(shop.structure_type, StructureType::Shop);
        
        let altar = StructureTemplate::altar();
        assert_eq!(altar.structure_type, StructureType::Altar);
        
        let dungeon = StructureTemplate::underground_dungeon();
        assert_eq!(dungeon.structure_type, StructureType::UndergroundDungeon);
    }
}
