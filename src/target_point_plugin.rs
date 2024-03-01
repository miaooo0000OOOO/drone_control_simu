use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Debug, Resource)]
pub struct TargetPointRes(pub Vec3);

#[derive(Debug, Component)]
pub struct TargetPoint;

pub struct TargetPointPlugin;

impl Plugin for TargetPointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_target_point);
        // .add_systems(Update, update_taget_point_res);
    }
}

fn setup_target_point(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pos = Vec3::new(0.1, 3., 0.1);

    // Target Point
    commands.spawn((
        RigidBody::Kinematic,
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.1)),
            material: materials.add(Color::rgb(1., 1., 0.)),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..default()
        },
        TargetPoint,
    ));
    commands.insert_resource(TargetPointRes { 0: pos });
}

fn update_taget_point_res(
    query: Query<&Transform, With<TargetPoint>>,
    mut target_point_res: ResMut<TargetPointRes>,
) {
    target_point_res.0 = query.single().translation;
}
