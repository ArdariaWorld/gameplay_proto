use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::{
    plugins::creature::{
        creature_plugin::{Monster, Player},
        systems::sensors::PlayerSwordRangeSensor,
    },
    utils::error::ErrorMessage,
};

pub fn mouse_left_click_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    rapier_context: Res<RapierContext>,
    q_monster: Query<Entity, (With<Monster>, Without<Player>)>,
    q_player: Query<&Transform, With<Player>>,
    collider_query: Query<Entity, (With<Collider>, With<PlayerSwordRangeSensor>)>,
) {
    let mut closure = || {
        for event in mouse_button_input_events.iter() {
            // If not event Pressed we do nothing
            if event.state != ButtonState::Pressed || event.button != MouseButton::Left {
                return Ok(());
            };

            let player_transform = q_player.get_single().expect("No Player found");

            let entity = collider_query.get_single().expect("No collider position");

            /* Iterate through all the intersection pairs involving a specific collider. */
            for (collider1, collider2, intersecting) in rapier_context.intersections_with(entity) {
                if intersecting {
                    let other = if entity == collider1 {
                        collider2
                    } else {
                        collider1
                    };

                    println!("The other {:?}", other);
                    println!("monsters {}", q_monster.is_empty());

                    // Detect if entity is a monster
                }
            }
        }

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling click: {}", error);
    }
}
