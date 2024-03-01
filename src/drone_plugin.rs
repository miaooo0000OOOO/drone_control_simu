use std::ops::Range;

use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use controller::Controller;

use crate::{controller, target_point_plugin::TargetPointRes};

pub const DRONE_HEIGHT: f32 = 1.0;
pub const DRONE_WIDTH: f32 = 0.5;
pub const DRONE_THRUST: f32 = 9.5 / 4.0;

pub const DRONE_START_POS: Vec3 = Vec3::new(0.0, 4.0, 0.0);

pub const DRONE_THRUST_RANGE: Range<f32> = -2.5..2.5;

pub struct DronePlugin;

#[derive(Component, Debug)]
pub struct Drone;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_drone)
            // .add_systems(Update, update_drone_force)
            .add_systems(Update, log_drone);
    }
}

fn add_drone(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Drone
    commands.spawn((
        RigidBody::Dynamic,
        AngularVelocity(Vec3::new(0., 0., 0.)),
        Collider::cuboid(DRONE_WIDTH, DRONE_HEIGHT, DRONE_WIDTH),
        // Collider::sphere(DRONE_WIDTH),
        SceneBundle {
            scene: asset_server.load("Drone.glb#Scene0"),
            transform: Transform::from_xyz(DRONE_START_POS.x, DRONE_START_POS.y, DRONE_START_POS.z),
            ..default()
        },
        ExternalForce::default(),
        Drone,
        Controller::new(),
    ));
}

fn update_drone_force(
    mut query: Query<(&Transform, &mut ExternalForce, &mut Controller), With<Drone>>,
    time: Res<Time>,
    target_point: Res<TargetPointRes>,
) {
    let (t, mut f, mut c) = query.single_mut();

    let target_pos = target_point.0;

    f.clear();
    let dt = time.delta_seconds();
    if dt == 0. {
        return;
    }
    let thrusts: Vec<f32> = c.ctrl_drone(&target_pos, &t, dt);
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

fn log_drone(query_drone: Query<(Entity, &Transform, &ExternalForce), With<Drone>>) {
    let (e, t, f) = query_drone.iter().next().unwrap();
    println!("Transform: {:?}", t);
    println!("EF: {:?}", f);
    println!("Entity: {:?}", e);
}

fn restraint_in_range(x: f32, range: Range<f32>) -> f32 {
    if x > range.end {
        range.end
    } else if range.start < x {
        range.start
    } else {
        x
    }
    // 0.0
}
