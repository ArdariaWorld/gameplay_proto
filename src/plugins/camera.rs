use bevy::prelude::{Camera, Parent, Query, Transform, With, Without};

use super::population::Player;

pub fn camera_follow_player(
    player_query: Query<&Parent, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut q_parent: Query<&Transform, Without<Camera>>,
) {
    let parent_entity = player_query.get_single().expect("No player location");
    let mut camera_transform = camera_query.get_single_mut().expect("No camera transform");

    if let Ok(transform) = q_parent.get_mut(parent_entity.get()) {
        // camera_transform.translation = transform.translation;
    }
}
