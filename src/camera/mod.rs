// camera.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController {
    pub orbit_speed: f32,
    pub pan_speed: f32,
    pub vertical_speed: f32,
}

pub fn camera_controller(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CameraController)>,
) {
    let (mut transform, controller) = query.single_mut();
    let dt = time.delta_seconds();

    // Orbit (Q/E)
    if keyboard.pressed(KeyCode::Q) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(controller.orbit_speed * dt),
        );
    }
    if keyboard.pressed(KeyCode::E) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(-controller.orbit_speed * dt),
        );
    }

    // Pan (WASD)
    let forward = transform.forward();
    let right = transform.right();
    let mut movement = Vec3::ZERO;

    if keyboard.pressed(KeyCode::W) {
        movement += forward;
    }
    if keyboard.pressed(KeyCode::S) {
        movement -= forward;
    }
    if keyboard.pressed(KeyCode::D) {
        movement += right;
    }
    if keyboard.pressed(KeyCode::A) {
        movement -= right;
    }

    // Vertical movement (Space/LShift)
    if keyboard.pressed(KeyCode::Space) {
        movement += Vec3::Y;
    }
    if keyboard.pressed(KeyCode::ShiftLeft) {
        movement -= Vec3::Y;
    }

    if movement != Vec3::ZERO {
        transform.translation += movement.normalize() * controller.pan_speed * dt;
    }

    // Make camera look at center
    let look_target = Vec3::ZERO;
    transform.look_at(look_target, Vec3::Y);
}
