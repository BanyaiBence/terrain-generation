use bevy::prelude::*;

#[derive(Resource)]
pub struct TerrainConfig {
    pub size: u32,
    pub scale: f64,
    pub height_multiplier: f32,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            size: 500,
            scale: 0.05,
            height_multiplier: 5.0,
        }
    }
}

#[derive(Component)]
pub struct TerrainChunk;