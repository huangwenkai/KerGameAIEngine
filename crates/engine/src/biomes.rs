//! Biome system for rich world generation

use crate::chunk::Material;
use crate::rng::GameRng;

/// Biome types for world generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BiomeType {
    Grassland,   // 草地: 绿色表层
    Desert,      // 沙漠: 沙子为主
    Jungle,      // 丛林: 密集树木
    Swamp,       // 沼泽: 水+泥
    Mountain,    // 山脉: 石头高地
}

impl BiomeType {
    /// Get surface material for this biome
    pub fn surface_material(&self) -> Material {
        match self {
            BiomeType::Grassland => Material::Grass,
            BiomeType::Desert => Material::Sand,
            BiomeType::Jungle => Material::Grass,
            BiomeType::Swamp => Material::Dirt,
            BiomeType::Mountain => Material::Stone,
        }
    }

    /// Get subsurface material (just below surface)
    pub fn subsurface_material(&self) -> Material {
        match self {
            BiomeType::Grassland => Material::Dirt,
            BiomeType::Desert => Material::Sand,
            BiomeType::Jungle => Material::Dirt,
            BiomeType::Swamp => Material::Dirt,
            BiomeType::Mountain => Material::Stone,
        }
    }

    /// Get average surface height offset for this biome
    pub fn height_offset(&self) -> f64 {
        match self {
            BiomeType::Grassland => 0.0,
            BiomeType::Desert => -10.0,
            BiomeType::Jungle => 5.0,
            BiomeType::Swamp => -20.0,
            BiomeType::Mountain => 40.0,
        }
    }
}

/// Ore types for mining
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OreType {
    Copper,      // 铜: 浅层
    Iron,        // 铁: 中层
    Gold,        // 金: 深层
    MagicCrystal,// 魔法水晶: 稀有
}

impl OreType {
    /// Get material for this ore (we'll use Stone for now, later can add ore materials)
    pub fn material(&self) -> Material {
        // TODO: Add actual ore materials to Material enum
        Material::Stone
    }

    /// Get spawn depth range (min, max)
    pub fn depth_range(&self) -> (i32, i32) {
        match self {
            OreType::Copper => (10, 50),
            OreType::Iron => (30, 80),
            OreType::Gold => (50, 120),
            OreType::MagicCrystal => (60, 150),
        }
    }

    /// Get spawn frequency (higher = more common)
    pub fn frequency(&self) -> f64 {
        match self {
            OreType::Copper => 0.02,
            OreType::Iron => 0.015,
            OreType::Gold => 0.008,
            OreType::MagicCrystal => 0.003,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biome_materials() {
        assert_eq!(BiomeType::Grassland.surface_material(), Material::Grass);
        assert_eq!(BiomeType::Desert.surface_material(), Material::Sand);
    }

    #[test]
    fn ore_depth_ranges() {
        let (min, max) = OreType::Copper.depth_range();
        assert!(min < max);
        assert!(min >= 0);
    }
}
