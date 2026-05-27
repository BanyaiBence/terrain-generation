use bevy::prelude::*;

pub mod components;
mod systems;

use components::TerrainConfig;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TerrainConfig>()

            .add_systems(Startup, (
                systems::spawn_terrain,
            ));
    }
}