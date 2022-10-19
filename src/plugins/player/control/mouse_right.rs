use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    math::Vec3Swizzles,
    prelude::{EventReader, EventWriter, MouseButton, Query, Res, Transform, Vec2, With},
    window::Windows,
};
use bevy_mod_raycast::Intersection;

use crate::{
    plugins::{combat::FireProjectileEvent, population::PlayerParent},
    utils::error::ErrorMessage,
};

use super::mouse::MouseRaycastSet;

pub fn mouse_right_click_system(
    windows: Res<Windows>,
    mouse_pos_q: Query<&Intersection<MouseRaycastSet>>,
    player_q: Query<&Transform, With<PlayerParent>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut ev_fire_projectile: EventWriter<FireProjectileEvent>,
) {
    let mouse_position = match mouse_pos_q.get_single() {
        Ok(p) => match p.position() {
            Some(p) => p,
            None => return,
        },
        Err(_) => return,
    };

    let player_translation = player_q
        .get_single()
        .expect("No player position")
        .translation;

    for event in mouse_button_input_events.iter() {
        // If not event Pressed we do nothing
        if event.state == ButtonState::Pressed && event.button == MouseButton::Right {
            let mouse_angle = player_translation.xz().angle_between(mouse_position.xz());
            ev_fire_projectile.send(FireProjectileEvent(mouse_angle));
        };
    }
}
