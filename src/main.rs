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
        .add_systems(Startup, spawn_robots)
        .add_systems(Update, move_robot)
        .run();
}

#[derive(Component)]
struct Robot {
    id: u16,
}

// get target speed (fw, sideways, turning)
fn move_robot(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&Robot>,
    mut ext_forces: Query<&mut ExternalForce>,
) {
    for robot in query.iter_mut() {
        if robot.id != 5 {
            continue;
        }
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Up) {
            direction.z -= 0.1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.z += 0.1;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 0.1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 0.1;
        }
        // transform.translation += direction * TIME_STEP;
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = direction;
        }
    }
}

fn spawn_robots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..12 {
        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(
                        shape::Cylinder {
                            radius: ROB_R,
                            height: ROB_H,
                            resolution: 10,
                            segments: 10,
                        }
                        .into(),
                    ),
                    transform: Transform::from_xyz(ROB_START_POS[i][0], 0.0, ROB_START_POS[i][1]),
                    material: materials.add(ROB_COL.into()),
                    ..default()
                },
                Robot { id: i as u16 },
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::cylinder(ROB_H / 2.0, ROB_R))
            // .insert(Restitution::coefficient(REST_COEF))
            .insert(ColliderMassProperties::Density(1.0))
            .insert(TransformBundle::from(Transform::from_xyz(
                ROB_START_POS[i][0],
                0.0,
                ROB_START_POS[i][1],
            )))
            .insert(ExternalForce { force: Vec3::ZERO, ..Default::default() });
    }
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let restitution = Restitution::coefficient(REST_COEF);

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
            material: materials.add(BALL_COL.into()),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(BALL_RADIUS))
        .insert(ColliderMassProperties::Density(1.0));
    // .insert(restitution);
    // .insert(Friction {
    //     coefficient: FRICTION_COEF,
    //     ..Default::default()
    // });

    // terain (box: 13.4 x 0.1 x 10.4)
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Mesh::from(shape::Box {
                min_x: -TERR_H_X,
                max_x: TERR_H_X,
                min_y: -0.1,
                max_y: 0.1,
                min_z: -TERR_H_Z,
                max_z: TERR_H_Z,
            }))),
            material: materials.add(TERR_COL.into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(13.4 / 2.0, 0.1, 10.4 / 2.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0))); // .insert(Friction {
                                                                             //     coefficient: FRICTION_COEF,
                                                                             //     ..Default::default()
                                                                             // });

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
