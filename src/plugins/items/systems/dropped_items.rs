use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::items::items_plugin::{PickUpItemEvent, Pickable};

pub fn dropped_items_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut ev_pick_up_item: EventWriter<PickUpItemEvent>,
    q_pickable: Query<&Transform, With<Pickable>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1_ref, entity_2_ref, _) => {
                println!("toto");

                let entity_1 = commands.entity(*entity_1_ref).id();
                let entity_2 = commands.entity(*entity_2_ref).id();

                // Projectile should react only with monsters (see CollisionGroups) so other entity is always a monster
                let (pickable_entity, creature_entity) = match q_pickable.get(entity_1) {
                    Ok(_) => (entity_1, entity_2),
                    Err(_) => match q_pickable.get(entity_2) {
                        Ok(_) => (entity_2, entity_1),
                        Err(_) => continue, // If no projectile, continue events iteration
                    },
                };

                ev_pick_up_item.send(PickUpItemEvent(creature_entity, pickable_entity));
            }
            CollisionEvent::Stopped(_, _, _) => {
                continue;
            }
        }
    }
}
