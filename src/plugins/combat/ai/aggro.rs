use bevy::prelude::*;

use crate::{
    plugins::{
        location::Location,
        population::{Monster, Player},
    },
    MONSTER_AGGRO_DISTANCE,
};

pub fn monster_aggro_system(
    mut monsters_query: Query<&mut Location, (With<Monster>, Without<Player>)>,
    player_query: Query<&Location, With<Player>>,
) {
    // Get player position
    let player_position = match player_query.get_single() {
        Ok(location) => match location.position {
            Some(position) => position,
            None => return,
        },
        Err(_) => return,
    };

    // for each monster -> get distance from player
    for mut location in monsters_query.iter_mut() {
        let position = match location.position {
            Some(position) => position,
            None => continue,
        };

        // if distance <= AGGRO_DISTANCE
        if position.abs_diff_eq(player_position, MONSTER_AGGRO_DISTANCE) {
            // monster location.destination = player.location.position
            location.destination = Some(player_position);
        }
    }
}
