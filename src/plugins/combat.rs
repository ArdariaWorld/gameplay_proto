use crate::{
    MONSTER_AGGRO_DISTANCE, MONSTER_HIT_IMPULSE, MONSTER_MAX_RANGE, PIXEL_PER_METER,
    PROJECTILE_IMPULSE,
};

use super::{location::*, player::KillPlayerEvent, population::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Default)]
pub struct Projectile;
pub struct FireProjectileEvent(pub f32);
pub struct ProjectileHitEvent(pub Entity, pub Entity);

pub struct HitMonsterEvent(pub Entity, pub f32);
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
            .add_system(fire_projectile_system)
            .add_system(print_projectile_stats)
            .add_system(projectile_collision_system);
    }
}

fn print_projectile_stats(mut q_projectile: Query<&mut Velocity, With<Projectile>>) {
    for mut velocity in q_projectile.iter_mut() {
        if velocity.linvel.length() <= 1. {
            velocity.linvel = Vec3::default();
        }
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
        println!("Applied projectile rotation and impulse {:?}", ev.0);

        let vec_destination = Vec2::from_angle(ev.0).normalize();
        let looking_at = Vec3::new(vec_destination.x, 1., vec_destination.y);
        let impulse = Transform::default()
            .looking_at(looking_at, Vec3::ZERO)
            .translation
            * PROJECTILE_IMPULSE;

        commands
            .spawn_bundle(SpatialBundle {
                transform: Transform::from_xyz(0., 0., 2.),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert_bundle(TransformBundle::from(Transform {
                translation: transform.translation,
                rotation: Quat::from_rotation_z(ev.0),
                ..default()
            }))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Collider::cuboid(1.2 / 2., 0.2 / 2., 0.2 / 2.))
            .insert(Velocity::default())
            .insert(Damping {
                linear_damping: 1.,
                ..default()
            })
            .insert(Restitution::coefficient(50.))
            .insert(Dominance::group(2))
            .insert(ActiveEvents::COLLISION_EVENTS) // Enable events to detect projectile events
            .insert(Projectile)
            .insert(ExternalImpulse {
                impulse,
                torque_impulse: Vec3::splat(0.),
            })
            .insert(CollisionGroups::new(Group::GROUP_4, Group::GROUP_2))
            //
            // Add Sprite
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    transform: Transform {
                        scale: Vec3::new(1.2, 0.2, 1.),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::BLUE,
                        ..default()
                    },
                    ..default()
                });
            });
    }
}

// Player hit a monster
fn monster_hit_system(
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

fn projectile_collision_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut ev_monster_hit: EventWriter<HitMonsterEvent>,
    q_projectile: Query<&Transform, With<Projectile>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity_1_ref, entity_2_ref, _) => {
                let entity_1 = commands.entity(*entity_1_ref).id();
                let entity_2 = commands.entity(*entity_2_ref).id();

                let (projectile_entity, monster_entity) = match q_projectile.get(entity_1) {
                    Ok(_) => (entity_1, entity_2),
                    Err(_) => match q_projectile.get(entity_2) {
                        Ok(_) => (entity_2, entity_1),
                        Err(_) => continue, // If no projectile, continue events iteration
                    },
                };

                let projectile_transform =
                    q_projectile.get(projectile_entity).expect("No projectile");

                println!(
                    "Projectile rotation {:?}",
                    projectile_transform.rotation.to_axis_angle()
                );

                ev_monster_hit.send(HitMonsterEvent(
                    monster_entity,
                    projectile_transform.rotation.to_axis_angle().1,
                ));

                commands.entity(projectile_entity).despawn_recursive();
            }
            CollisionEvent::Stopped(_, _, _) => {
                continue;
            }
        }
    }
}

// Monster attack a player
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
