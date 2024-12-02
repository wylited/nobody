// main.rs
use bevy::prelude::*;
mod camera;
use camera::{camera_controller, CameraController};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_controller)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(shape::Plane::from_size(20.0).into())),
        MeshMaterial3d(materials.add(Color::rgb_u8(76, 128, 76).into())),
        Transform::from_xyz(0.0, -0.5, 0.0),
        GlobalTransform::default(),
    ));

    // Create several cubes in different positions
    let cube_positions = [
        (0.0, 0.0, 0.0, Color::srgb_u8(255, 0, 0)),    // Red
        (2.0, 1.0, 2.0, Color::srgb_u8(0, 0, 255)),    // Blue
        (-2.0, 0.5, -2.0, Color::srgb_u8(0, 255, 0)),  // Green
        (3.0, 2.0, -1.0, Color::srgb_u8(255, 255, 0)), // Yellow
        (-1.0, 1.5, 3.0, Color::srgb_u8(128, 0, 128)), // Purple
    ];

    for (x, y, z, color) in cube_positions {
        commands.spawn((
            Mesh3d(meshes.add(shape::Cube::new(1.0).into())),
            MeshMaterial3d(materials.add(color.into())),
            Transform::from_xyz(x, y, z),
            GlobalTransform::default(),
        ));
    }

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        CameraController {
            orbit_speed: 1.0,
            pan_speed: 5.0,
            vertical_speed: 5.0,
        },
    ));
}
