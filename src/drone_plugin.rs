use std::ops::Range;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use controller::Controller;

use crate::{controller, environment_plugin::TargetPoint};

pub const DRONE_HEIGHT: f32 = 1.0;
pub const DRONE_WIDTH: f32 = 0.5;
pub const DRONE_THRUST: f32 = 9.5 / 4.0;

pub const DRONE_THRUST_RANGE: Range<f32> = -5.0..5.0;

pub struct DronePlugin;

#[derive(Component, Debug)]
pub struct Drone;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_drone)
            .add_systems(Update, (update_drone_force, log_drone));
    }
}

fn add_drone(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Drone
    commands.spawn((
        RigidBody::Dynamic,
        AngularVelocity(Vec3::new(2.5, 3.4, 1.6)),
        Collider::cuboid(1.0, 1.0, 1.0),
        SceneBundle {
            scene: asset_server.load("Drone.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
        ExternalForce::default(),
        Drone,
        Controller::new(),
    ));
}

fn update_drone_force(
    mut query_drone: Query<(&Transform, &mut ExternalForce, &mut Controller), With<Drone>>,
    query_taget_point: Query<&Transform, With<TargetPoint>>,
    time: Res<Time>,
) {
    let (t, mut f, mut c) = query_drone.iter_mut().next().unwrap();
    let target_pos = query_taget_point.iter().next().unwrap().translation;

    f.clear();
    let dt = time.delta_seconds();
    let thrusts: Vec<f32> = c.ctrl_drone(&target_pos, t, dt);
    for (i, (x, z)) in [(1., 1.), (-1., 1.), (1., -1.), (-1., -1.)]
        .iter()
        .enumerate()
    {
        f.apply_force_at_point(
            t.rotation * Vec3::new(0., restraint_in_range(thrusts[i], DRONE_THRUST_RANGE), 0.),
            Vec3::new(x * DRONE_WIDTH / 2., 0., z * DRONE_WIDTH / 2.),
            Vec3::ZERO,
        );
    }
}

fn log_drone(query_drone: Query<(&Transform, &ExternalForce), With<Drone>>) {
    let (t, f) = query_drone.iter().next().unwrap();
    println!("Transform: {:?}", t);
    println!("EF: {:?}", f);
}

fn restraint_in_range(x: f32, range: Range<f32>) -> f32 {
    if x > range.end {
        range.end
    } else if range.start < x {
        range.start
    } else {
        x
    }
}
