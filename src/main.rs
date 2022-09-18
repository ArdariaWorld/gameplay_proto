use bevy::prelude::*;
pub struct PopulationPlugin;
struct GreetTimer(Timer);

impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(add_player)
        .add_startup_system(add_monsters)
        .add_system(combat_system);
    }
}

#[derive(Component)]
struct Monster;

#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
struct Name(String);

#[derive(Component)]
struct Stats {
    hp: f32,
    atk: f32,
    def: f32
}


// System
fn add_player(mut commands: Commands) {
    commands.spawn()
    .insert(Player)
    .insert(Name("SirMashaa".to_string()))
    .insert(Stats{ hp: 100f32, atk: 10f32, def: 0f32 });
}

fn add_monsters(mut commands: Commands) {
    commands.spawn()
    .insert(Monster)
    .insert(Name("Monster 1".to_string()))
    .insert(Stats{ hp: 100f32, atk: 0f32, def: 5f32 });
}

fn compute_new_stats(player_atk :f32, monster_hp: f32) -> f32 {
    monster_hp - player_atk
}

// System
fn combat_system(
    time: Res<Time>, 
    mut timer: ResMut<GreetTimer>, 
    player_query: Query<&Stats, With<Player>>, 
    mut monsters_query: Query<&mut Stats, Without<Monster>>
) {
    if timer.0.tick(time.delta()).just_finished() {

        let player_stats = player_query.single();

        for mut monster_stats in monsters_query.iter_mut() {
            monster_stats.hp = compute_new_stats(player_stats.atk, monster_stats.hp);
            eprintln!("Monster has {} HP.", monster_stats.hp);
        }
    }
}

// System

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(PopulationPlugin)
        .run();
}
