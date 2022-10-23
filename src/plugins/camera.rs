use bevy::prelude::*;

use crate::CAMERA_VEC_OFFSET_VEC;

use super::population::PlayerParent;

pub fn camera_follow_player(
    player_q: Query<&Transform, (With<PlayerParent>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<PlayerParent>)>,
) {
    let transform = player_q.get_single().expect("No player location");
    let mut camera_transform = camera_query.get_single_mut().expect("No camera transform");

    camera_transform.translation = transform.translation + CAMERA_VEC_OFFSET_VEC;
    camera_transform.look_at(transform.translation, Vec3::Y);
}
