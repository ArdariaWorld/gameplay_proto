use super::location::Location;
use bevy::prelude::*;

pub struct PopulationPlugin;
impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player)
            .add_startup_system(add_monsters);
    }
}

#[derive(Component, Default)]
pub struct Name(String);

#[derive(Component, Default)]
pub struct Stats {
    pub hp: f32,
    pub atk: f32,
}

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    creature: CreatureBundle,
    // score
    // inputs
}

#[derive(Default, Bundle)]
pub struct CreatureBundle {
    pub stats: Stats,
    pub name: Name,
    pub location: Location,

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
pub struct Monster;

#[derive(Component)]
pub struct Player;

enum CreatureType {
    Human,
    Monster,
}

#[derive(Default, Component)]
pub struct Creature;

impl CreatureType {
    fn color(&self) -> Color {
        match self {
            CreatureType::Human => Color::rgb(0.4, 0.7, 0.1),
            CreatureType::Monster => Color::rgb(0.7, 0.1, 0.1),
        }
    }

    fn size(&self) -> Vec3 {
        match self {
            CreatureType::Human => Vec2::new(17.0, 40.0).extend(2.0),
            CreatureType::Monster => Vec2::new(25.0, 35.0).extend(1.0),
        }
    }
}

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
