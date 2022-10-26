use crate::{
    plugins::{
        combat::combat_events::{HitMonsterEvent, KillMonsterEvent},
        creature::{
            creature_plugin::{Monster, Player},
            systems::stats::{BrainState, ConsciousnessStateEnum, Stats},
        },
    },
    MONSTER_HIT_IMPULSE,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Player hit a monster
pub fn monster_hit_system(
    mut commands: Commands,

    mut monsters_q: Query<
        (&mut Stats, &mut BrainState, &mut ExternalImpulse),
        (With<Monster>, Without<Player>),
    >,
    player_q: Query<&Stats, (With<Player>, Without<Monster>)>,

    mut ev_hit_monster: EventReader<HitMonsterEvent>,
    mut ev_kill_monster: EventWriter<KillMonsterEvent>,
) {
    // Get player stats
    let player_stats = match player_q.get_single() {
        Ok(stats) => stats,
        Err(_) => return,
    };

    for ev in ev_hit_monster.iter() {
        println!("event hit monster");

        let (mut stats, mut brain_state, mut external_impulse) = match monsters_q.get_mut(ev.0) {
            Ok(tupl) => tupl,
            Err(_) => return,
        };

        println!(
            "Applied impulse vector {:?}",
            Vec2::from_angle(ev.1).normalize()
        );

        let vec_destination = Vec2::from_angle(ev.1).normalize();
        let looking_at = Vec3::new(vec_destination.x, 1., vec_destination.y);
        let impulse = Transform::default()
            .looking_at(looking_at, Vec3::ZERO)
            .translation
            * MONSTER_HIT_IMPULSE;

        external_impulse.impulse = impulse;
        stats.hp -= player_stats.atk;

        brain_state.conscious = ConsciousnessStateEnum::Stun;
        brain_state.stun_at.reset();

        if stats.hp <= 0. {
            ev_kill_monster.send(KillMonsterEvent(ev.0));
            brain_state.conscious = ConsciousnessStateEnum::Ko;
            commands.entity(ev.0).despawn_recursive();
        }
    }
}
