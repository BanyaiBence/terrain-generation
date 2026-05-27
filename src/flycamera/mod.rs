use bevy::prelude::*;

pub mod components;
mod systems;

pub use components::FlyCamera;

pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            systems::camera_movement,
            systems::camera_look,
            systems::cursor_grab,
        ));
    }
}