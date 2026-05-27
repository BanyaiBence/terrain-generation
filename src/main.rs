use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};
mod terrain;
mod flycamera;

use terrain::TerrainPlugin;
use flycamera::{FlyCamera, FlyCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TerrainPlugin)
        .add_plugins(FlyCameraPlugin)
        .add_plugins(WireframePlugin::default())
        .add_systems(Startup, setup_environment)
        .add_systems(Update, toggle_wireframe)
        .run();
}

    fn setup_environment(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 40.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCamera::default(),
    ));
}

fn toggle_wireframe(
    keys: Res<ButtonInput<KeyCode>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        wireframe_config.global = !wireframe_config.global;
    }
}