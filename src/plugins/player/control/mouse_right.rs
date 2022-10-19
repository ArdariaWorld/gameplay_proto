use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::{EventReader, EventWriter, MouseButton, Query, Transform, With},
};
use bevy_mod_raycast::Intersection;

use crate::plugins::{combat::FireProjectileEvent, population::PlayerParent};

use super::mouse::MouseRaycastSet;

pub fn mouse_right_click_system(
    mouse_pos_q: Query<&Intersection<MouseRaycastSet>>,
    player_q: Query<&Transform, With<PlayerParent>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut ev_fire_projectile: EventWriter<FireProjectileEvent>,
) {
    for event in mouse_button_input_events.iter() {
        // If not event Pressed we do nothing
        if event.state == ButtonState::Pressed && event.button == MouseButton::Right {
            let mouse_position = match mouse_pos_q.get_single() {
                Ok(p) => match p.position() {
                    Some(p) => p,
                    None => return,
                },
                Err(_) => return,
            };

            ev_fire_projectile.send(FireProjectileEvent(mouse_position.clone()));
        };
    }
}
