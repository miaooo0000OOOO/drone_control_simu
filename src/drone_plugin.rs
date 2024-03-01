use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub const DRONE_HEIGHT: f32 = 1.0;
pub const DRONE_WIDTH: f32 = 0.5;
pub const DRONE_THRUST: f32 = 9.5 / 4.0;

pub struct DronePlugin;

#[derive(Component, Debug)]
pub struct Drone;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_drone)
            .add_systems(Update, update_drone_force);
    }
}

fn add_drone(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
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
    ));
}

fn update_drone_force(mut query: Query<(&Transform, &mut ExternalForce), With<Drone>>) {
    for (t, mut f) in query.iter_mut() {
        f.clear();
        for (x, z) in [(1., 1.), (1., -1.), (-1., 1.), (-1., -1.)].iter() {
            f.apply_force_at_point(
                t.rotation * Vec3::new(0., DRONE_THRUST, 0.),
                Vec3::new(x * DRONE_WIDTH / 2., 0., z * DRONE_WIDTH / 2.),
                Vec3::ZERO,
            );
        }
    }
}
