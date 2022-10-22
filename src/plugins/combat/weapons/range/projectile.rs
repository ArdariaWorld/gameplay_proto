use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::combat::{combat_events::HitMonsterEvent, combat_plugin::Projectile};

pub fn projectile_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut ev_monster_hit: EventWriter<HitMonsterEvent>,
    q_projectile: Query<&Transform, With<Projectile>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1_ref, entity_2_ref, _) => {
                let entity_1 = commands.entity(*entity_1_ref).id();
                let entity_2 = commands.entity(*entity_2_ref).id();

                let (projectile_entity, monster_entity) = match q_projectile.get(entity_1) {
                    Ok(_) => (entity_1, entity_2),
                    Err(_) => match q_projectile.get(entity_2) {
                        Ok(_) => (entity_2, entity_1),
                        Err(_) => continue, // If no projectile, continue events iteration
                    },
                };

                let projectile_transform =
                    q_projectile.get(projectile_entity).expect("No projectile");

                // println!(
                //     "Projectile rotation {:?}",
                //     projectile_transform.rotation.to_axis_angle()
                // );

                ev_monster_hit.send(HitMonsterEvent(
                    monster_entity,
                    projectile_transform.rotation.to_axis_angle().1,
                ));

                commands.entity(projectile_entity).despawn_recursive();
            }
            CollisionEvent::Stopped(_, _, _) => {
                continue;
            }
        }
    }
}
