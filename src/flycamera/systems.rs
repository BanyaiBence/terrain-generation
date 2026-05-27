use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use super::components::FlyCamera;

pub fn camera_look(
    mut mouse_events: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
    cursor_query: Query<&CursorOptions, With<PrimaryWindow>>,
) {
    let Ok(cursor) = cursor_query.single() else { return };

    if cursor.grab_mode == CursorGrabMode::None {
        return;
    }

    for (mut transform, mut cam) in &mut query {
        for event in mouse_events.read() {
            cam.yaw -= event.delta.x * cam.sensitivity;
            cam.pitch -= event.delta.y * cam.sensitivity;

            cam.pitch = cam.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

            transform.rotation = Quat::from_axis_angle(Vec3::Y, cam.yaw)
                * Quat::from_axis_angle(Vec3::X, cam.pitch);
        }
    }
}

pub fn camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FlyCamera)>,
) {
    if query.is_empty() {
        println!("WARNING: No entities match the camera query! Did you spawn FlyCamera?");
    }
    for (mut transform, cam) in &mut query {
        let mut velocity = Vec3::ZERO;

        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0.0, local_z.z).normalize_or_zero();
        let right = Vec3::new(local_z.z, 0.0, -local_z.x).normalize_or_zero();

        if keys.pressed(KeyCode::KeyW) { velocity += forward; }
        if keys.pressed(KeyCode::KeyS) { velocity -= forward; }
        if keys.pressed(KeyCode::KeyD) { velocity += right; }
        if keys.pressed(KeyCode::KeyA) { velocity -= right; }
        if keys.pressed(KeyCode::Space) { velocity += Vec3::Y; }
        if keys.pressed(KeyCode::ShiftLeft) { velocity -= Vec3::Y; }

        let speed_multiplier = if keys.pressed(KeyCode::ControlLeft) { 3.0 } else { 1.0 };

        transform.translation += velocity.normalize_or_zero()
            * cam.speed
            * speed_multiplier
            * time.delta_secs();
    }
}

pub fn cursor_grab(
    mut cursor_query: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    key_btn: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut cursor) = cursor_query.single_mut() else { return };

    if mouse_btn.just_pressed(MouseButton::Left) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }

    if key_btn.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}