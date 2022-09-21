use bevy::prelude::*;
pub struct PopulationPlugin;
struct GreetTimer(Timer);

impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(0.2, true)))
            .add_startup_system(add_player)
            .add_startup_system(add_monsters)
            .add_system(combat_system);
    }
}

#[derive(Component)]
struct Monster;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Stats {
    hp: f32,
    atk: f32,
}

// System
fn add_player(mut commands: Commands) -> () {
    commands
        .spawn()
        .insert(Player)
        .insert(Name("SirMashaa".to_string()))
        .insert(Stats {
            hp: 100f32,
            atk: 10f32,
        });
}

fn add_monsters(mut commands: Commands) -> () {
    commands
        .spawn()
        .insert(Monster)
        .insert(Name("Monster 1".to_string()))
        .insert(Stats {
            hp: 100f32,
            atk: 0f32,
        });
}

fn compute_new_hps(player_stats: &Stats, monster_stats: &Stats) -> f32 {
    monster_stats.hp - player_stats.atk
}

// System
fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    player_query: Query<&Stats, With<Player>>,
    mut monsters_query: Query<(Entity, &Name, &mut Stats), Without<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let player_stats = player_query.single();

        if monsters_query.is_empty() {
            eprintln!("No monster to fight");
        }

        for (entity_id, name, mut monster_stats) in monsters_query.iter_mut() {
            // Todo move into single function and unit test
            attack(
                &mut commands,
                entity_id,
                &mut monster_stats,
                player_stats,
                name,
            );
        }
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
        eprintln!("Monster {} has been killed", monster_name.0);
    }
    eprintln!("Monster {} has {} HP.", monster_name.0, monster_stats.hp);
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(PopulationPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use crate::*;

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
