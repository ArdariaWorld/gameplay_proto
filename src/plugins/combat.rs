use super::{location::*, population::*};
use bevy::prelude::*;

pub struct KillMonsterEvent(Entity);

#[derive(Default)]
pub struct MonstersKilled {
    pub count: i32,
}

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MonstersKilled { count: 0 })
            .add_event::<KillMonsterEvent>();
    }
}

fn compute_new_hps(player_stats: &Stats, monster_stats: &Stats) -> f32 {
    monster_stats.hp - player_stats.atk
}
// System
fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(&Stats, &Location), With<Player>>,
    mut monsters_query: Query<(Entity, &Name, (&mut Stats, &Location)), Without<Player>>,
) {
    let player = player_query.single();

    if monsters_query.is_empty() {
        eprintln!("No monster to fight");
    }

    for (entity_id, name, mut monster) in monsters_query.iter_mut() {
        eprintln!(
            "Player in {:?} will attack monster in {:?}",
            player.1.position, monster.1.position
        );

        // Todo move into single function and unit test
        attack(&mut commands, entity_id, &mut monster.0, player.0, name);
    }
}

fn despawn(commands: &mut Commands, entity_id: Entity) -> () {
    commands.entity(entity_id).despawn();
}

// Attack system
// check it despawn a dead ennemy
fn attack(
    commands: &mut Commands,
    entity_id: Entity,
    monster_stats: &mut Stats,
    player_stats: &Stats,
    monster_name: &Name,
) -> () {
    monster_stats.hp = compute_new_hps(&player_stats, &monster_stats);
    if monster_stats.hp <= 0.0 {
        despawn(commands, entity_id);
        // commands.entity(entity_id).despawn();
        eprintln!("Monster {} has been killed", monster_name);
    }
    eprintln!("Monster {} has {} HP.", monster_name, monster_stats.hp);
}

#[cfg(test)]
mod tests {
    use crate::plugins::{combat::compute_new_hps, population::Stats};

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