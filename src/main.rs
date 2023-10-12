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
            transform: Transform::from_xyz(-3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    let restitution = Restitution::coefficient(0.7);
    // dimensions are in m

    // robot (cilinder)
    let robot_radius = 0.15 / 2.0;
    let robot_height = 0.18;
    let black = Color::rgb(0.0, 0.0, 0.0).into();
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                shape::Cylinder {
                    radius: robot_radius,
                    height: robot_height,
                    resolution: 10,
                    segments: 10,
                }
                .into(),
            ),
            material: materials.add(black),
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(robot_height / 2.0, robot_radius))
        .insert(restitution)
        .insert(TransformBundle::from(Transform::from_xyz(1.0, 0.0, 0.0)));

    // ball
    let ball_radius = 0.04267 / 2.0; // 42.67 mm diameter
    let orange = Color::rgb(1.0, 0.5, 0.0).into();
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
            material: materials.add(orange),
            // transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(ball_radius))
        .insert(restitution);
        // .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));

    // terain (box: 13.4 x 0.1 x 10.4)
    let ground_hx = 13.4 / 2.0;
    let ground_hy = 0.1;
    let ground_hz = 10.4 / 2.0;
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Mesh::from(shape::Box {
                min_x: -ground_hx,
                max_x: ground_hx,
                min_y: -ground_hy,
                max_y: ground_hy,
                min_z: -ground_hz,
                max_z: ground_hz,
            }))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(13.4 / 2.0, 0.1, 10.4 / 2.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0)));

    // ground
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Plane::from_size(20.0))),
    //         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //         transform: Transform::from_xyz(0.0, -0.1, 0.0),
    //         ..default()
    //     })
    //     .insert(Collider::cuboid(10.0, 0.1, 10.0))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0)));

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
