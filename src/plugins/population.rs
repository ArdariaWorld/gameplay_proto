use super::location::Location;
use bevy::prelude::*;

pub struct PopulationPlugin;
impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_creatures);
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
}

#[derive(Default, Bundle)]
pub struct CreatureBundle {
    pub stats: Stats,
    pub name: Name,
    pub location: Location,
}

impl CreatureBundle {
    fn new(name_str: String, hp: f32, atk: f32) -> CreatureBundle {
        CreatureBundle {
            stats: Stats { hp, atk },
            name: Name(name_str),
            location: Location::new(),
        }
    }
}

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Player;

#[derive(Clone)]
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

fn spawn_creatures(mut commands: Commands, mut asset_server: Res<AssetServer>) -> () {
    add_creature(&mut commands, &mut asset_server, true);

    for _ in 0..10 {
        add_creature(&mut commands, &mut asset_server, false);
    }
}

fn add_creature(
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    is_player: bool,
) -> () {
    let creature_type = match is_player {
        true => CreatureType::Human,
        false => CreatureType::Monster,
    };

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_scale(Vec3::splat(1.)),
            ..default()
        })
        //
        // Add Creature
        .with_children(|parent| {
            let mut ent = parent.spawn_bundle(CreatureBundle::new("Jbb".to_string(), 100.0, 20.0));

            ent.insert(Creature);

            if is_player {
                ent.insert(Player);
            }
        })
        //
        // Add Sprite
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                transform: Transform {
                    scale: creature_type.size(),
                    translation: Vec2::splat(0.).extend(-1.),
                    ..default()
                },
                sprite: Sprite {
                    color: creature_type.color(),
                    ..default()
                },
                ..default()
            });
        })
        //
        // Add Text
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    100.0.to_string(),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(1., 1., 1.0),
                        font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                    },
                ),
                transform: Transform {
                    translation: Vec3::new(-25., 60., -1.),
                    ..default()
                },
                ..default()
            });
        });
}
