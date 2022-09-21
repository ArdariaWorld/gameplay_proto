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

enum Creature {
    Human,
    Monster,
}

impl Creature {
    fn color(&self) -> Color {
        match self {
            Creature::Human => Color::rgb(0.4, 0.7, 0.1),
            Creature::Monster => Color::rgb(0.1, 0.7, 0.4),
        }
    }

    fn size(&self) -> Vec3 {
        match self {
            Creature::Human => Vec2::new(17.0, 40.0).extend(1.0),
            Creature::Monster => Vec2::new(25.0, 35.0).extend(1.0),
        }
    }
}

#[derive(Default, Bundle)]
struct CreatureBundle {
    stats: Stats,
    name: Name,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl CreatureBundle {
    fn new(creature: Creature, name_str: String, hp: f32, atk: f32) -> CreatureBundle {
        CreatureBundle {
            stats: Stats { hp, atk },
            name: Name(name_str),
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

// System
fn add_player(mut commands: Commands) -> () {
    commands
        .spawn_bundle(PlayerBundle {
            creature: CreatureBundle::new(Creature::Human, "Jbb".to_string(), 100.0, 20.0),
        })
        .insert(Player);
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

fn main() {
    // main_bis();
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PopulationPlugin)
        .add_startup_system(init_world_map)
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
