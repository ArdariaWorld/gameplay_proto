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
            systems::sensors::PlayerSwordRangeSensor,
        },
    },
    utils::error::ErrorMessage,
};

pub fn mouse_left_click_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    rapier_context: Res<RapierContext>,
    player_q: Query<&Transform, With<Player>>,
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

            let transform = player_q
                .get_single()
                .expect("No Player found in left click");

            let collider = sword_q.get_single().expect("No sword collider found");

            /* Iterate through all the intersection pairs involving a specific collider. */
            for (collider1, collider2, intersecting) in rapier_context.intersections_with(collider)
            {
                if intersecting {
                    match monster_q.get(collider1) {
                        Ok(m) => ev_hit_monster
                            .send(HitMonsterEvent(m, transform.rotation.to_axis_angle().1)),
                        Err(_) => match monster_q.get(collider2) {
                            Ok(m) => ev_hit_monster
                                .send(HitMonsterEvent(m, transform.rotation.to_axis_angle().1)),
                            Err(_) => continue,
                        },
                    };
                }
            }
        }

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling left click: {}", error);
    }
}
