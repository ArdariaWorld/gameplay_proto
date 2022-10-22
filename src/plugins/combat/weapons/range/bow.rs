use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::{
    combat::{combat_events::FireProjectileEvent, combat_plugin::Projectile},
    population::PlayerParent,
};

pub fn fire_projectile_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_fire_projectile: EventReader<FireProjectileEvent>,
    q_player: Query<&Transform, With<PlayerParent>>,
) {
    for ev in ev_fire_projectile.iter() {
        let player_transform = q_player.get_single().expect("No Player found");
        let mut projectile_transform = Transform {
            translation: player_transform.translation,
            ..default()
        };

        let looking_at = Vec3::new(ev.0.x, 2., ev.0.z);
        projectile_transform.look_at(looking_at, Vec3::Y);

        commands
            .spawn_bundle(SpatialBundle {
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert_bundle(TransformBundle::from_transform(projectile_transform))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Collider::cuboid(0.2 / 2., 0.2 / 2., 1.2 / 2.))
            .insert(Velocity::default())
            .insert(Damping {
                linear_damping: 1.,
                ..default()
            })
            .insert(Restitution::coefficient(50.))
            .insert(Dominance::group(2))
            .insert(ActiveEvents::COLLISION_EVENTS) // Enable events to detect projectile events
            .insert(Projectile)
            .insert(ExternalImpulse {
                impulse: projectile_transform.forward().normalize() * 10.,
                torque_impulse: Vec3::splat(0.),
            })
            .insert(CollisionGroups::new(Group::GROUP_4, Group::GROUP_2))
            //
            // Add Sprite
            .with_children(|parent| {
                parent.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.2, 0.2, 1.2))),
                    material: materials.add(Color::BLUE.into()),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..default()
                });
            });
    }
}
