use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::{
    plugins::{
        combat::combat_events::HitMonsterEvent,
        population::{
            MonsterParent, Player, PlayerParent, PlayerSwordRange, PlayerSwordRangeSensor,
        },
    },
    utils::error::ErrorMessage,
};

pub fn mouse_left_click_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    rapier_context: Res<RapierContext>,
    q_monster: Query<Entity, (With<MonsterParent>, Without<Player>)>,
    q_player: Query<&Transform, With<PlayerParent>>,
    collider_query: Query<
        Entity,
        (
            With<Collider>,
            With<PlayerSwordRangeSensor>,
            Without<PlayerSwordRange>,
        ),
    >,

    mut ev_monster_hit: EventWriter<HitMonsterEvent>,
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
                    match q_monster.get(other) {
                        Ok(monster) => ev_monster_hit
                            .send(HitMonsterEvent(monster, player_transform.rotation.xyz().y)),
                        Err(_) => continue,
                    };
                }
            }
        }

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling click: {}", error);
    }
}
