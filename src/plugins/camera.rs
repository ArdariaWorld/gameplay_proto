use bevy::prelude::{Camera, Query, Transform, With};

use super::{location::Location, population::Player};

pub fn camera_follow_player(
    player_query: Query<&Location, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let player_location = player_query.get_single().expect("No player location");
    let mut camera_transform = camera_query.get_single_mut().expect("No camera transform");

    if let Some(position) = player_location.position {
        camera_transform.translation = position.extend(1.);
    }
}
