use std::ops::Range;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use controller::Controller;

use crate::{controller, target_point_plugin::TargetPointRes};

pub const DRONE_HEIGHT: f32 = 1.0;
pub const DRONE_WIDTH: f32 = 0.5;
pub const DRONE_THRUST: f32 = 9.5 / 4.0;

pub const DRONE_J_WING: f32 = 0.01;

pub const DRONE_START_POS: Vec3 = Vec3::new(0.0, 7.0, 0.0);

// pub const DRONE_THRUST_RANGE: Range<f32> = -5.0..5.0;
pub const DRONE_THRUST_RANGE: Range<f32> = -5.0..5.0;

pub struct DronePlugin;

#[derive(Component, Debug)]
pub struct Drone;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_drone)
            .add_systems(Update, update_drone)
            // .add_systems(Update, restraint_drone)
            // .add_systems(Update, log_drone)
            ;
    }
}

fn add_drone(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Drone
    let drone = commands
        .spawn((
            RigidBody::Dynamic,
            AngularVelocity(Vec3::new(0., 0., 0.)),
            // Collider::cuboid(DRONE_WIDTH, DRONE_HEIGHT, DRONE_WIDTH),
            Collider::sphere(DRONE_WIDTH),
            // ColliderDensity(0.0),
            SceneBundle {
                scene: asset_server.load("Drone.glb#Scene0"),
                transform: Transform::from_xyz(
                    DRONE_START_POS.x,
                    DRONE_START_POS.y,
                    DRONE_START_POS.z,
                ),
                ..default()
            },
            ExternalForce::default(),
            ExternalTorque::default(),
            Drone,
            Controller::new(),
            GravityScale(0.), // Mass(10.)
        ))
        .id();

    let fixed_point = commands
        .spawn((
            RigidBody::Static,
            Position::from_xyz(DRONE_START_POS.x, DRONE_START_POS.y, DRONE_START_POS.z),
        ))
        .id();

    // commands.spawn(SphericalJoint::new(drone, fixed_point));
}

// fn restraint_drone(mut query: Query<&mut Transform, With<Drone>>) {
//     let mut t = query.single_mut();
//     t.translation = DRONE_START_POS;
// }

fn update_drone(
    mut query: Query<
        (
            &Transform,
            &AngularVelocity,
            &mut ExternalForce,
            &mut ExternalTorque,
            &mut Controller,
        ),
        With<Drone>,
    >,
    time: Res<Time>,
    target_point: Res<TargetPointRes>,
) {
    let (t, w, mut f, mut tor, mut c) = query.single_mut();

    let target_pos = target_point.0;

    f.clear();
    let dt = time.delta_seconds();
    if dt == 0. {
        return;
    }
    let thrusts: Vec<f32> = c.ctrl_drone(&target_pos, &t, dt);
    for (i, (x, z)) in [(1., 1.), (1., -1.), (-1., 1.), (-1., -1.)]
        .iter()
        .enumerate()
    {
        f.apply_force_at_point(
            t.rotation * Vec3::new(0., restraint_in_range(thrusts[i], DRONE_THRUST_RANGE), 0.),
            Vec3::new(x * DRONE_WIDTH / 2., 0., z * DRONE_WIDTH / 2.),
            Vec3::ZERO,
        );
    }
    // f.apply_force(Vec3::new(0.,9.81,0.));

    tor.clear();

    let rotate_speed = [-thrusts[0], thrusts[1], thrusts[2], -thrusts[3]];

    // tor.apply_torque(Vec3::new(
    //     DRONE_J_WING
    //         * w.z
    //         * (rotate_speed[0] - rotate_speed[1] + rotate_speed[2] - rotate_speed[3]),
    //     0.,
    //     DRONE_J_WING
    //         * w.x
    //         * (-rotate_speed[0] + rotate_speed[1] - rotate_speed[2] + rotate_speed[3]),
    // )); // 陀螺力矩
    // tor.apply_torque(Vec3::new(0., rotate_speed.iter().sum(), 0.)); // 螺旋桨力矩
}

fn log_drone(
    query_drone: Query<(Entity, &Transform, &ExternalForce, &ExternalTorque), With<Drone>>,
) {
    let (e, t, f, tor) = query_drone.single();
    println!("Transform: {:?}", t);
    println!("EF: {:?}", f);
    println!("Entity: {:?}", e);
    println!("ET: {:?}", tor);
}

pub fn restraint_in_range(x: f32, range: Range<f32>) -> f32 {
    if range.end < x {
        range.end
    } else if x < range.start {
        range.start
    } else {
        x
    }
    // 0.0
}
