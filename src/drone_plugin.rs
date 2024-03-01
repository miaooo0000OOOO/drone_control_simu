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
            .add_systems(Update, (update_drone_force));
    }
}

fn add_drone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Drone
    commands.spawn((
        RigidBody::Dynamic,
        AngularVelocity(Vec3::new(2.5, 3.4, 1.6)),
        Collider::cuboid(1.0, 1.0, 1.0),
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
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
        // f.apply_force_at_point(
        //     Vec3::new(0., DRONE_THRUST, 0.),
        //     Vec3::new(x, y, z),
        //     Vec3::ZERO,
        // );
        // f.apply_force_at_point(force, point, center_of_mass)
    }
}
