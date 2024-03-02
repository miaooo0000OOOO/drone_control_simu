use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub mod controller;
pub mod drone_plugin;
pub mod environment_plugin;
pub mod target_point_plugin;

use drone_plugin::DronePlugin;
use environment_plugin::EnvPlugin;
use target_point_plugin::TargetPointPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(DronePlugin)
        .add_plugins(TargetPointPlugin)
        .add_plugins(EnvPlugin)
        // .add_plugins(bevy_framepace::FramepacePlugin)
        .run();
}
