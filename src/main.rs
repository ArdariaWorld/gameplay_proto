#![allow(dead_code)]
mod game_cake;
use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        ButtonState,
    },
    prelude::*,
};
use game_cake::main_bis;
use rand::Rng;

struct GreetTimer(Timer);
struct LocationTimer(Timer);

pub struct PopulationPlugin;
impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(0.2, true)))
            .add_startup_system(add_player)
            .add_startup_system(add_monsters);
    }
}

pub struct LocationPlugin;
impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(location_system);
    }
}

fn location_system(
    time: Res<Time>,
    mut creatures_query: Query<(&mut Location, &mut Transform), With<Creature>>,
) {
    for (mut location, mut transform) in creatures_query.iter_mut() {
        // Update location if entity have a destination
        if let (Some(destination), Some(position)) = (location.destination, location.position) {
            // compute vector from position to destination
            let delta_v = destination - position;
            println!("{:?}", delta_v);
            location.position = Some(position + delta_v.normalize() * 100.0 * time.delta_seconds());
            // reduce vector to maximum velocity
            // apply new vector to position to get new_posittion
        }
        // transform.translation.x += 10.0 * time.delta_seconds();

        // Update sprite transform from entity position
        if let Some(position) = location.position {
            transform.translation = position.extend(1.0);
        }
    }
}

fn handle_mouse_click(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    mut player_query: Query<&mut Location, With<Player>>,
) {
    // eprintln!("Mouse event system {}", player_query.is_empty());
    for event in mouse_button_input_events.iter() {
        if event.state == ButtonState::Pressed {
            let win = windows.get_primary().expect("no primary window");
            // println!("{:?}", win.cursor_position());
            if let (Ok(mut location), Some(cursor_position)) =
                (player_query.get_single_mut(), win.cursor_position())
            {
                location.destination = Some(cursor_position);
            }
        }
    }
}

fn compute_new_hps(player_stats: &Stats, monster_stats: &Stats) -> f32 {
    monster_stats.hp - player_stats.atk
}
// System
fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    player_query: Query<(&Stats, &Location), With<Player>>,
    mut monsters_query: Query<(Entity, &Name, (&mut Stats, &Location)), Without<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
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

// -------------------
// -------------------
// -------------------
// WORLD

const WORLD_WIDTH: f32 = 800.0;
const WORLD_HEIGHT: f32 = 400.0;
const WALL_COLOR: Color = Color::rgb(0.8, 0.4, 0.2);

// Render the world
fn init_world_map(mut commands: Commands, mut game: ResMut<Game>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(WorldMapBundle {
        world_map: WorldMap {
            h: WORLD_WIDTH,
            w: WORLD_HEIGHT,
        },
        sprite_bundle: SpriteBundle {
            transform: Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: Vec2::new(0.0, 0.0).extend(0.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: Vec2::new(WORLD_WIDTH, WORLD_HEIGHT).extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
    });
}

#[derive(Default)]
struct Game {
    // here add game state
    world_bundle: WorldMapBundle,
}

#[derive(Default, Component)]
struct WorldMap {
    w: f32,
    h: f32,
}

#[derive(Default, Bundle)]
struct WorldMapBundle {
    world_map: WorldMap,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

// -------------------
// -------------------
// -------------------
// CREATURE

#[derive(Component, Default)]
struct Name(String);

#[derive(Component, Default)]
struct Stats {
    hp: f32,
    atk: f32,
}

#[derive(Default, Bundle)]
struct PlayerBundle {
    #[bundle]
    creature: CreatureBundle,
    // score
    // inputs
}

#[derive(Default, Component)]
struct Location {
    destination: Option<Vec2>,
    max_velocity: Option<f32>,
    velocity: Option<Vec2>,
    position: Option<Vec2>,
}

struct RandVec2 {
    x: f32,
    y: f32,
}

impl RandVec2 {
    fn new() -> Vec2 {
        Vec2::new(
            rand::thread_rng().gen_range(0..WORLD_WIDTH as i32) as f32,
            rand::thread_rng().gen_range(0..WORLD_HEIGHT as i32) as f32,
        )
    }
}

impl Location {
    fn new() -> Location {
        Location {
            position: Some(RandVec2::new()),
            ..default()
        }
    }
}

#[derive(Default, Bundle)]
struct CreatureBundle {
    stats: Stats,
    name: Name,
    location: Location,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl CreatureBundle {
    fn new(creature: CreatureType, name_str: String, hp: f32, atk: f32) -> CreatureBundle {
        CreatureBundle {
            stats: Stats { hp, atk },
            name: Name(name_str),
            location: Location::new(),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec2::new(0.0, 0.0).extend(1.0),
                    scale: creature.size(),
                    ..default()
                },
                sprite: Sprite {
                    color: creature.color(),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
struct Monster;

#[derive(Component)]
struct Player;

enum CreatureType {
    Human,
    Monster,
}

#[derive(Default, Component)]
struct Creature;

impl CreatureType {
    fn color(&self) -> Color {
        match self {
            CreatureType::Human => Color::rgb(0.4, 0.7, 0.1),
            CreatureType::Monster => Color::rgb(0.1, 0.7, 0.4),
        }
    }

    fn size(&self) -> Vec3 {
        match self {
            CreatureType::Human => Vec2::new(17.0, 40.0).extend(1.0),
            CreatureType::Monster => Vec2::new(25.0, 35.0).extend(1.0),
        }
    }
}

// System
fn add_player(mut commands: Commands) -> () {
    commands
        .spawn_bundle(PlayerBundle {
            creature: CreatureBundle::new(CreatureType::Human, "Jbb".to_string(), 100.0, 20.0),
        })
        .insert(Creature)
        .insert(Player);
}

fn add_monsters(mut commands: Commands) -> () {
    commands
        .spawn_bundle(CreatureBundle::new(
            CreatureType::Monster,
            "Monster 1".to_string(),
            90.0,
            10.0,
        ))
        .insert(Creature)
        .insert(Monster);
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PopulationPlugin)
        .add_system(handle_mouse_click)
        .add_plugin(LocationPlugin)
        .add_startup_system(init_world_map)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

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

    #[test]
    fn did_update_sprite_transforms() {
        // Setup app
        let mut app = App::new();

        app.init_resource::<Game>();
        app.add_plugins(MinimalPlugins);
        app.add_plugin(PopulationPlugin);
        app.add_plugin(LocationPlugin);
        app.add_startup_system(init_world_map);

        // Update system once
        app.update();

        // Query creatures after the update
        let mut creatures_query = app
            .world
            .query_filtered::<(&Location, &Transform), With<Creature>>();

        // Query should not be empty
        let is_empty = creatures_query.is_empty(&app.world, 0, 0);
        assert_eq!(is_empty, false);

        // Expect transform to have been updated according to position
        for (location, transform) in creatures_query.iter(&app.world) {
            // Every transform.translation should have been updated
            assert_eq!(
                location.position.unwrap().extend(1.0),
                transform.translation
            );
        }
    }
}
