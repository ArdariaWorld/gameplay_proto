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

#[derive(Component)]
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

fn compute_new_hps(player_atk :f32, monster_hp: f32) -> f32 {
    monster_hp - player_atk
}

// System
fn combat_system(
    mut commands: Commands,
    time: Res<Time>, 
    mut timer: ResMut<GreetTimer>, 
    player_query: Query<&Stats, With<Player>>, 
    mut monsters_query: Query<(Entity, &Name, &mut Stats), Without<Player>>
) {
    if timer.0.tick(time.delta()).just_finished() {

        let player_stats = player_query.single();

        if monsters_query.is_empty() {
            eprintln!("No monster to fight");
        }

        for (entity_id, name, mut monster_stats) in monsters_query.iter_mut() {

            // Todo move into single function and unit test
            monster_stats.hp = compute_new_hps(player_stats.atk, monster_stats.hp);
            if monster_stats.hp <= 0.0 {
                commands.entity(entity_id).despawn();
                eprintln!("Monster {} has been killed", name.0);
            }


            eprintln!("Monster {} has {} HP.", name.0, monster_stats.hp);
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
