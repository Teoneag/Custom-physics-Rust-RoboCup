use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands, mut config: ResMut<GizmoConfig>) {
    // to see the debug lines throug objects
    config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };

    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera::default(),
    ));
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    let ground_hx = 10.0;
    let ground_hy = 1.0;
    let ground_hz = 10.0;
    let ground_x = 0.0;
    let ground_y = 0.0;
    let ground_z = 0.0;
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -ground_hx,
                max_x: ground_hx,
                min_y: -ground_hy,
                max_y: ground_hy,
                min_z: -ground_hz,
                max_z: ground_hz,
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            // transform: Transform::from_xyz(ground_x, ground_y, ground_z),
            ..default()
        })
        .insert(Collider::cuboid(ground_hx, ground_hy, ground_hz));
        // .insert(TransformBundle::from(Transform::from_xyz(
        //     ground_x, ground_y, ground_z,
        // )));

    // ball
    let ball_radius = 0.5;
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: ball_radius,
                    subdivisions: 4,
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(ball_radius))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
