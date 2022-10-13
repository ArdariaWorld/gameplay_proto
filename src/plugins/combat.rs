use crate::{MONSTER_AGGRO_DISTANCE, MONSTER_MAX_RANGE};

use super::{location::*, player::KillPlayerEvent, population::*};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Projectile;
pub struct FireProjectileEvent(pub f32);
pub struct ProjectileHitEvent(pub Entity, pub Entity);

pub struct HitMonsterEvent(pub Entity);
pub struct KillMonsterEvent(pub Entity);

#[derive(Default)]
pub struct MonstersKilled {
    pub count: i32,
}

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MonstersKilled { count: 0 })
            .add_event::<FireProjectileEvent>()
            .add_event::<HitMonsterEvent>()
            .add_event::<KillMonsterEvent>()
            .add_system(monster_hit_system)
            .add_system(monster_aggro_system)
            .add_system(monster_fight_system)
            .add_system(fire_projectile_system);
    }
}

fn fire_projectile_system(
    mut commands: Commands,
    mut ev_fire_projectile: EventReader<FireProjectileEvent>,
    player_query: Query<&Parent, With<Player>>,
    q_parent: Query<&Transform>,
) {
    let parent_entity = player_query.get_single().expect("No Player found");
    let transform = q_parent
        .get(parent_entity.get())
        .expect("No parent transform");

    for ev in ev_fire_projectile.iter() {
        commands
            .spawn()
            .insert_bundle(TransformBundle::from(Transform::from_translation(
                transform.translation,
            )))
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Velocity::zero())
            .insert(Collider::cuboid(20., 10.))
            .insert(Friction::coefficient(0.7))
            .insert(Restitution::coefficient(5.))
            .insert(Dominance::group(2))
            .insert(ExternalImpulse {
                impulse: Vec2::from_angle(ev.0) * 50.,
                torque_impulse: 14.0,
            })
            .insert(CollisionGroups::new(Group::GROUP_4, Group::GROUP_2));
    }
}

fn monster_hit_system(
    mut commands: Commands,
    mut entity_query: Query<(Entity, &Children)>,
    mut monsters_query: Query<&mut Stats, Without<Player>>,
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
        let (entity, children) = match entity_query.get_mut(ev.0) {
            Ok(result) => result,
            Err(e) => {
                println!("Error entity not found {:?}", e);
                continue;
            }
        };

        for &child in children.iter() {
            match monsters_query.get_mut(child) {
                Ok(mut stats) => {
                    stats.hp -= player_stats.atk;

                    if stats.hp <= 0. {
                        ev_kill_monster.send(KillMonsterEvent(entity));
                        commands.entity(entity).despawn_recursive();
                    }
                }
                Err(_) => (),
            };
        }
    }
}

fn monster_aggro_system(
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

fn monster_fight_system(
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

#[cfg(test)]
mod tests {
    use crate::plugins::population::Stats;

    fn compute_new_hps(player_stats: &Stats, monster_stats: &Stats) -> f32 {
        monster_stats.hp - player_stats.atk
    }

    #[test]
    fn test_attack() {
        let player_stats = Stats {
            hp: 100f32,
            atk: 10f32,
        };

        let monster_stats = Stats {
            hp: 100f32,
            atk: 100f32,
        };

        let new_monster_hps = compute_new_hps(&player_stats, &monster_stats);
        assert_eq!(new_monster_hps, 90f32);
    }
}
