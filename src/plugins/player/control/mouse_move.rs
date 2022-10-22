use std::f32::consts::PI;

use bevy::{
    input::mouse::MouseMotion,
    prelude::{EventReader, Query, Transform, Vec3, With},
};
use bevy_mod_raycast::Intersection;

use crate::plugins::population::PlayerParent;

use super::mouse::MouseRaycastSet;

pub fn mouse_move_system(
    mut q_player: Query<&mut Transform, With<PlayerParent>>,
    mouse_pos_q: Query<&Intersection<MouseRaycastSet>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    for _ in mouse_motion_events.iter() {
        let mouse_position = match mouse_pos_q.get_single() {
            Ok(p) => match p.position() {
                Some(p) => p,
                None => return,
            },
            Err(_) => return,
        };

        let mut player_transform = q_player.get_single_mut().expect("No Player found");

        let looking_at = Vec3::new(mouse_position.x, 1., mouse_position.z);
        let mut sword_range_transform = Transform::from_translation(player_transform.translation);
        sword_range_transform.look_at(looking_at, Vec3::Y * 2.);
        sword_range_transform.rotate_y(PI / 2.);
        player_transform.rotation = sword_range_transform.rotation;
    }
}
