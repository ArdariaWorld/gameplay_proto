use crate::{
    plugins::{
        combat::combat_events::{HitMonsterEvent, KillMonsterEvent},
        population::*,
    },
    MONSTER_HIT_IMPULSE,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Player hit a monster
pub fn monster_hit_system(
    mut commands: Commands,
    mut entity_query: Query<(Entity, &Children)>,
    mut monsters_query: Query<
        (&Parent, &mut Stats, &mut BrainState),
        (With<Monster>, Without<Player>),
    >,
    mut q_monster_parent: Query<&mut ExternalImpulse, With<Collider>>,
    player_query: Query<&Stats, With<Player>>,
    mut ev_hit_monster: EventReader<HitMonsterEvent>,
    mut ev_kill_monster: EventWriter<KillMonsterEvent>,
) {
    // Get player stats
    let player_stats = match player_query.get_single() {
        Ok(stats) => stats,
        Err(_) => return,
    };

    for ev in ev_hit_monster.iter() {
        println!("event hit monster");

        let (entity, children) = match entity_query.get_mut(ev.0) {
            Ok(result) => result,
            Err(e) => {
                println!("Error entity not found {:?}", e);
                continue;
            }
        };

        for &child in children.iter() {
            match monsters_query.get_mut(child) {
                Ok((parent, mut stats, mut brain_state)) => {
                    let mut external_impulse = q_monster_parent
                        .get_mut(parent.get())
                        .expect("No creature external impulse");

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
                        ev_kill_monster.send(KillMonsterEvent(entity));
                        brain_state.conscious = ConsciousnessStateEnum::Ko;
                        commands.entity(entity).despawn_recursive();
                    }
                }
                Err(_) => (),
            };
        }
    }
}
