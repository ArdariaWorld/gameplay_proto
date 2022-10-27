use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::{
    plugins::{
        combat::combat_events::HitMonsterEvent,
        creature::{
            creature_plugin::{Monster, Player},
            systems::{inventory, sensors::PlayerSwordRangeSensor},
        },
        items::items_plugin::{AnimateVisualItem, EquippedItem, Inventory, Item, ItemMesh},
    },
    utils::error::ErrorMessage,
};

pub fn mouse_left_click_system(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    rapier_context: Res<RapierContext>,
    mut player_q: Query<(&Transform, &mut EquippedItem), (With<Player>, Without<ItemMesh>)>,
    sword_q: Query<Entity, With<PlayerSwordRangeSensor>>,
    monster_q: Query<Entity, With<Monster>>,
    mut ev_hit_monster: EventWriter<HitMonsterEvent>,
) {
    let mut closure = || {
        for event in mouse_button_input_events.iter() {
            // If not event Pressed we do nothing
            if event.state != ButtonState::Pressed || event.button != MouseButton::Left {
                return Ok(());
            };

            let (transform, mut equipped_item_entity) = player_q
                .get_single_mut()
                .expect("No Player found in left click");

            let collider = sword_q.get_single().expect("No sword collider found");

            let equipped_visual_item_entity = match equipped_item_entity.0 {
                Some(i) => i,
                None => continue,
            };

            println!(
                "equipped_visual_item_entity -- {:?}",
                equipped_visual_item_entity
            );

            // TODO if !cooldown && !animation running

            // Start animation for visual_equipped_item
            commands
                .entity(equipped_visual_item_entity)
                .insert(AnimateVisualItem);

            /* Iterate through all the intersection pairs involving a specific collider. */
            for (collider1, collider2, intersecting) in rapier_context.intersections_with(collider)
            {
                if intersecting {
                    let victim = match monster_q.get(collider1) {
                        Ok(m) => m,
                        Err(_) => match monster_q.get(collider2) {
                            Ok(m) => m,
                            Err(_) => continue,
                        },
                    };

                    // Hit monster
                    ev_hit_monster.send(HitMonsterEvent(
                        victim,
                        transform.rotation.to_axis_angle().1,
                    ));
                }
            }
        }

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling left click: {}", error);
    }
}
