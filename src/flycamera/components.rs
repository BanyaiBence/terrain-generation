use bevy::prelude::*;

#[derive(Component)]
pub struct FlyCamera {
    pub speed: f32,
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            speed: 20.0,
            sensitivity: 0.002,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}