use bevy:: {
    prelude::*,
    color::palettes::css::*,
    pbr::wireframe::{
        NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin,
    },
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
};
use bevy::window::{PresentMode, Window, WindowPlugin};

mod ui;
use ui::FpsCounterPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            WireframePlugin::default(),
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: RED.into(),
            ..default()
        })
        .add_plugins(FpsCounterPlugin)
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
            Transform::from_xyz(0.0, 7.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // light
    commands.spawn((
            PointLight {
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4.0, 13.0, 4.0),
    ));

    // base-cubes
    let grid_size = 5;
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let offset = (grid_size as f32 - 1.0) / 2.0;
    for x in 0..grid_size {
        for y in 0..grid_size {
            for z in 0..grid_size {
                let t = (x + y + z) as f32 / ((grid_size - 1) * 3) as f32;
                let shade = (60.0 + t * 180.0) as u8;

                commands.spawn((
                    Mesh3d(cube_mesh.clone()),
                    MeshMaterial3d(materials.add(Color::srgb_u8(shade, shade, shade))),
                    Transform::from_xyz(
                        x as f32 - offset,
                        y as f32 - offset,
                        z as f32 - offset,
                    ),
                ));
            }
        }
    }

    // top-cube
    let top_y = grid_size as f32 - offset;
    commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
            Transform::from_xyz(0.0, top_y, 0.0),
    ));
}

fn update_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    // orbit the camera, transform pos X
    for mut transform in &mut query {
        let radius = 15.0;
        let speed  = 1.0;
        let angle = time.elapsed_secs() * speed;

        transform.translation.x = radius * angle.cos();
        transform.translation.z = radius * angle.sin();
        
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
