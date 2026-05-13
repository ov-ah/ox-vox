use bevy::prelude::*;
use std::f32::consts::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //camera
    commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // cube
    commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
            Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn update_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    // orbit the camera, transform pos X, Z and rotate X to pan at object
    for mut transform in &mut query {
        let radius = 10.0;
        let speed  = 1.0;
        let angle = time.elapsed_secs() * speed;

        transform.translation.x = radius * angle.cos();
        transform.translation.z = radius * angle.sin();
        
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
