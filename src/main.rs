use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub mod drone_plugin;
pub mod environment_plugin;

use drone_plugin::DronePlugin;
use environment_plugin::EnvPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(DronePlugin)
        .add_plugins(EnvPlugin)
        .run();
}
