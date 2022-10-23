use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::{EventReader, EventWriter, MouseButton, Query},
};
use bevy_mod_raycast::Intersection;

use crate::plugins::combat::combat_events::FireProjectileEvent;

use super::mouse::MouseRaycastSet;

pub fn mouse_right_click_system(
    mouse_pos_q: Query<&Intersection<MouseRaycastSet>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
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
        };
    }
}
