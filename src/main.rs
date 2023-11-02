use bevy::{ecs::query, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::{na::ComplexField, prelude::*};
mod consts;
use consts::*;

mod box_spawner;
use box_spawner::*;

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

#[derive(Component)]
struct Ball {}

#[derive(Component)]
struct Shooter {}

// get target speed (fw, sideways, turning)
fn move_robot(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Robot, &mut ExternalForce, &Transform)>,
    mut query_ball: Query<(&Ball, &Transform)>,
) {
    let (ball, ball_transform) = query_ball.single_mut();

    for (robot, mut ext_force, transform) in query.iter_mut() {
        if robot.id != 5 {
            continue;
        }
        let mut direction = Vec3::ZERO;
        let mut torque = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Up) {
            direction.x += 0.05;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.x -= 0.05;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            direction.z -= 0.05;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.z += 0.05;
        }
        if keyboard_input.pressed(KeyCode::A) {
            torque.y += 0.003;
        }
        if keyboard_input.pressed(KeyCode::D) {
            torque.y -= 0.003;
        }
        ext_force.force = transform.rotation.mul_vec3(direction);
        ext_force.torque = torque;



        // if the ball is in the rectangle of the shooter,
        // always oriented in the relative front of the robot

        // let robot_pos = transform.translation;

        // let robot_rot = transform.rotation;

        // let ball_pos = ball_transform.translation;

        // let ball_robot_pos = Vec3::new(ball_pos.x - robot_pos.x, 0.0, ball_pos.z - robot_pos.z);

        // if ball_robot_pos.x > 0.0 && ball_robot_pos.x < SHOOTER_LENGTH
        //     && ball_robot_pos.z.abs() < SHOOTER_HALF_WIDTH
        // {
        //     println!("Ball in range");
        // }

        // let shooter_x = robot_pos.x + SHOOTER_HALF_WIDTH * robot_rot.cos();
        // let shooter_y = robot_pos.y + SHOOTER_HALF_WIDTH * robot_rot.sin();

        // // Calculate the rotated coordinates of the ball
        // let rotated_ball_x = (ball_pos.x - robot_pos.x) * robot_rot.cos()
        //     - (ball_pos.y - robot_pos.y) * robot_rot.sin()
        //     + robot_pos.x;
        // let rotated_ball_y = (ball_pos.x - robot_pos.x) * robot_rot.sin()
        //     + (ball_pos.y - robot_pos.y) * robot_rot.cos()
        //     + robot_pos.y;

        // // Check if the rotated ball coordinates are within the bounds of the shooter rectangle
        // let in_x_range = (rotated_ball_x >= shooter_x - SHOOTER_HALF_WIDTH)
        //     && (rotated_ball_x <= shooter_x + SHOOTER_HALF_WIDTH);
        // let in_y_range = (rotated_ball_y >= shooter_y - SHOOTER_HALF_LENGTH)
        //     && (rotated_ball_y <= shooter_y + SHOOTER_HALF_LENGTH);

        // if in_x_range && in_y_range {
        //     print!("Ball in range: ");
        // }

        // // print distance between robot and ball ignoring the y axis
        // let ball_robot_dist = ball_robot_pos.length();
        // // println!("Distance: {}", dist_real_pos);

        // // print ball_robot_dist * ball_rotation
        // let ball_robot_dist_rot = ball_robot_dist * ball_transform.rotation;

        // // print the angle between rel_pos and the x axis
        // let angle_real_robot_ball_x = ball_robot_pos.angle_between(Vec3::X);
        // // println!("Angle: {}", angle_real_robot_ball_x);

        // // print the angle between robot_rot and the x axis
        // let angle_robot_x = robot_rot.angle_between(Quat::from_rotation_x(0.0));
        // // println!("Angle: {}", angle_robot_x);

        // // print the angle between the robot_rot and the rel_pos
        // let angle_robot_ball = angle_robot_x - angle_real_robot_ball_x;
        // // println!("Angle: {}", angle_robot_ball);

        // // if distance < 0.2 && abs(angle) < 0.1 print angle & dist
        // if ball_robot_dist < 0.2 && angle_robot_ball.abs() < 0.1 {
        //     println!("Angle: {}; Distance: {}", angle_robot_ball, ball_robot_dist);
        // }
    }
}

fn spawn_robots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..12 {
        // if i != 5 {
        //     continue;
        // }
        let transform = TransformBundle::from(Transform::from_xyz(
            ROB_START_POS[i][0],
            0.0,
            ROB_START_POS[i][1],
        ));

        // commands
        //     .spawn((
        //         PbrBundle {
        //             mesh: meshes.add(
        //                 shape::Box {
        //                     min_x: -SHOOTER_HALF_WIDTH,
        //                     min_y: 0.0,
        //                     min_z: -0.1,
        //                     max_x: SHOOTER_LENGTH,
        //                     max_y: 0.1,
        //                     max_z: SHOOTER_HALF_WIDTH,
        //                 }
        //                 .into(),
        //             ),
        //             material: materials.add(SHOOTER_COL.into()),
        //             ..default()
        //         },
        //         Shooter {},
        //     ))
        //     .insert(transform);

        let robot: Entity = commands
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
                    material: materials.add(ROB_COL.into()),
                    ..default()
                },
                Robot { id: i as u16 },
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::cylinder(ROB_H / 2.0, ROB_R))
            .insert(ColliderMassProperties::Density(1.0))
            .insert(ExternalForce {
                ..Default::default()
            })
            .insert(Friction {
                coefficient: FRICTION_COEF_ROB,
                combine_rule: CoefficientCombineRule::Multiply,
                ..Default::default()
            })
            .insert(transform)
            .id();

        // let joint = PrismaticJointBuilder::new(Vec3::X)
        //     .local_anchor1(Vec3::new(
        //         ROB_R / 2.0,
        //         -ROB_H / 2.0 + SHOOTER_H_S + 2.0 * SHOOTER_POS_Y,
        //         0.0,
        //     )) // robot anchor
        //     .local_anchor2(Vec3::new(-SHOOTER_H_S / 2.0, 0.0, 0.0)) // shoter anchor
        //     .limits([0.0, ROB_R + SHOOTER_H_S + SHOOTER_RANGE]);

        // commands
        //     // .spawn(PbrBundle {
        //     //     mesh: meshes.add(
        //     //         shape::Cube {
        //     //             size: 2.0 * SHOOTER_H_S,
        //     //         }
        //     //         .into(),
        //     //     ),
        //     //     material: materials.add(SHOOTER_COL.into()),
        //     //     ..default()
        //     // })
        //     // .spawn(RigidBody::Dynamic)
        //     .spawn(Collider::cuboid(SHOOTER_H_S, SHOOTER_H_S, SHOOTER_H_S))
        //     .insert(Sensor)
        //     .insert(transform)
        //     .insert(ImpulseJoint::new(robot, joint));
    }
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ball
    commands
        .spawn((
            PbrBundle {
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
            },
            Ball {},
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(BALL_RADIUS))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Friction {
            coefficient: FRICTION_COEF_BALL,
            combine_rule: CoefficientCombineRule::Multiply,
            ..Default::default()
        })
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        });

    // terain (box: 13.4 x 0.1 x 10.4)
    spawn_box_1(
        &mut commands,
        &mut meshes,
        &mut materials,
        TERR_H_X,
        0.1,
        TERR_H_Z,
        0.0,
        -0.1,
        0.0,
    );

    // Left wall
    spawn_box_1(
        &mut commands,
        &mut meshes,
        &mut materials,
        WALL_H_X,
        WALL_H_Y,
        TERR_H_Z,
        -(TERR_H_X + WALL_H_X),
        0.1,
        0.0,
    );

    // Right wall
    spawn_box_1(
        &mut commands,
        &mut meshes,
        &mut materials,
        WALL_H_X,
        WALL_H_Y,
        TERR_H_Z,
        TERR_H_X + WALL_H_X,
        0.1,
        0.0,
    );

    // Top wall
    spawn_box_1(
        &mut commands,
        &mut meshes,
        &mut materials,
        TERR_H_X,
        WALL_H_Y,
        WALL_H_X,
        0.0,
        0.1,
        TERR_H_Z + WALL_H_X,
    );

    // Bottom wall
    spawn_box_1(
        &mut commands,
        &mut meshes,
        &mut materials,
        TERR_H_X,
        WALL_H_Y,
        WALL_H_X,
        0.0,
        0.1,
        -(TERR_H_Z + WALL_H_X),
    );
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
