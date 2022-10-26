use bevy::prelude::*;

use crate::{
    plugins::{
        creature::{
            creature_plugin::{Monster, Player},
            systems::stats::{LastAttack, Stats},
        },
        location::Location,
        player::player_events::KillPlayerEvent,
    },
    MONSTER_MAX_RANGE,
};
// Monster attack a player
pub fn monster_fight_system(
    time: Res<Time>,
    mut monsters_query: Query<
        (&Location, &Stats, &mut LastAttack),
        (With<Monster>, Without<Player>),
    >,
    mut player_query: Query<(&Location, &mut Stats), With<Player>>,
    mut ev_kill_player: EventWriter<KillPlayerEvent>,
) {
    // Get player position
    let (player_position, mut player_stats) = match player_query.get_single_mut() {
        Ok(tuple) => match tuple.0.position {
            Some(position) => (position, tuple.1),
            None => return,
        },
        Err(_) => return,
    };

    // for each monster -> get distance from player
    for (location, stats, mut last_attack) in monsters_query.iter_mut() {
        let position = match location.position {
            Some(position) => position,
            None => continue,
        };

        // if distance <= MONSTER_MAX_RANGE
        if position.abs_diff_eq(player_position, MONSTER_MAX_RANGE)
            && last_attack.0.tick(time.delta()).finished()
        {
            // Reset monster timer
            last_attack.0.reset();

            // monster to attack player
            player_stats.hp -= stats.atk;

            if player_stats.hp <= 0. {
                player_stats.hp = 0.;
                ev_kill_player.send(KillPlayerEvent());
            }
        }
    }
}
