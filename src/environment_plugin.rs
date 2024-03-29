use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::target_point_plugin::TARGET_POSITION;

pub struct EnvPlugin;

impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, env_setup)
        // .add_systems(Startup, set_frame)
        ;
    }
}

fn env_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(8.0, 0.002, 8.0),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(8.0, 8.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-1.0, 13.0, -1.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-11.0, 13.5, 20.0)
            .looking_at(TARGET_POSITION - Vec3::new(0., 3., 0.), Vec3::Y),
        ..default()
    });
}

// fn set_frame(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
//     use bevy_framepace::Limiter;
//     settings.limiter = Limiter::from_framerate(30.0);
// }
