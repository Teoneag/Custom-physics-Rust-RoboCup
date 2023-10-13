use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::prelude::*;
mod consts;
use consts::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, move_robot1)
        .run();
}

#[derive(Component)]
struct Robot1;
#[derive(Component)]
struct Robot2;
#[derive(Component)]
struct Robot3;
#[derive(Component)]
struct Robot4;
#[derive(Component)]
struct Robot5;

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let restitution = Restitution::coefficient(RESTITUTION_COEFFICIENT);

    // robot 1 (cilinder)
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(
                    shape::Cylinder {
                        radius: ROBOT_RADIUS,
                        height: ROBOT_HEIGHT,
                        resolution: 10,
                        segments: 10,
                    }
                    .into(),
                ),
                material: materials.add(ROBOT_COLOR.into()),
                transform: Transform::from_xyz(1.0, 0.0, 0.0),
                ..default()
            },
            Robot1,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(ROBOT_HEIGHT / 2.0, ROBOT_RADIUS))
        .insert(Restitution::coefficient(RESTITUTION_COEFFICIENT))
        .insert(TransformBundle::from(Transform::from_xyz(1.0, 0.0, 0.0)));

    // ball
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: BALL_RADIUS,
                    subdivisions: 4,
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(BALL_COLOR.into()),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(BALL_RADIUS))
        .insert(restitution)
        .insert(Friction {
            coefficient: 0.5,
            ..Default::default()
        });

    // terain (box: 13.4 x 0.1 x 10.4)
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Mesh::from(shape::Box {
                min_x: -TERRAIN_HALF_X,
                max_x: TERRAIN_HALF_X,
                min_y: -0.1,
                max_y: 0.1,
                min_z: -TERRAIN_HALF_Z,
                max_z: TERRAIN_HALF_Z,
            }))),
            material: materials.add(TERRAIN_COLOR.into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(13.4 / 2.0, 0.1, 10.4 / 2.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0)))
        .insert(Friction {
            coefficient: 0.5,
            ..Default::default()
        });

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
}

fn move_robot1(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Robot1>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Up) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        transform.translation += direction * TIME_STEP;
    }
}

fn setup_graphics(mut commands: Commands, mut config: ResMut<GizmoConfig>) {
    // to see the debug lines throug objects
    config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };

    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera::default(),
    ));

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
