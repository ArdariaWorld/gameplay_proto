use std::sync::Arc;

use crate::{
    utils::vec::RandVec2, GameState, HUMAN_ATK, HUMAN_MAX_RANGE, HUMAN_STEP_DISTANCE, MONSTER_ATK,
    MONSTER_ATTACK_COOLDOWN, MONSTER_MAX_RANGE, MONSTER_STEP_DISTANCE,
};

use super::{location::Location, player};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::{
    prelude::*,
    rapier::prelude::{ColliderHandle, RoundShape, Shape, SharedShape},
};

pub struct PopulationPlugin;
impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_creatures)
            .add_system(display_hps_system)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(display_events));
    }
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

#[derive(Component, Default)]
pub struct Name(String);

#[derive(Component)]
pub struct LastAttack(pub Timer);

#[derive(Component, Default)]
pub struct Stats {
    pub hp: f32,
    pub atk: f32,
}

#[derive(Component, Default)]
struct HpsDisplay;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    creature: CreatureBundle,
}

#[derive(Default, Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub stats: Stats,
    pub name: Name,
    pub location: Location,
}

impl CreatureBundle {
    fn new(creature_type: CreatureType, name_str: String, hp: f32, atk: f32) -> CreatureBundle {
        CreatureBundle {
            creature: Creature(creature_type),
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

#[derive(Component, Inspectable)]
pub struct PlayerSwordRangeSensor;

#[derive(Component)]
pub struct PlayerSwordRange;

#[derive(Clone, Default, Debug)]
pub enum CreatureType {
    #[default]
    Human,
    Monster,
}

#[derive(Default, Component)]
pub struct Creature(pub CreatureType);

impl CreatureType {
    fn color(&self) -> Color {
        match self {
            CreatureType::Human => Color::rgb(0.4, 0.7, 0.1),
            CreatureType::Monster => Color::rgb(0.7, 0.1, 0.1),
        }
    }

    pub fn size(&self) -> Vec3 {
        match self {
            CreatureType::Human => Vec2::new(17.0, 40.0).extend(2.0),
            CreatureType::Monster => Vec2::new(25.0, 35.0).extend(1.0),
        }
    }

    pub fn speed(&self) -> f32 {
        match self {
            CreatureType::Human => HUMAN_STEP_DISTANCE,
            CreatureType::Monster => MONSTER_STEP_DISTANCE,
        }
    }

    pub fn range(&self) -> f32 {
        match self {
            CreatureType::Human => HUMAN_MAX_RANGE,
            CreatureType::Monster => MONSTER_MAX_RANGE,
        }
    }

    fn attack(&self) -> f32 {
        match self {
            CreatureType::Human => HUMAN_ATK,
            CreatureType::Monster => MONSTER_ATK,
        }
    }
}

fn spawn_creatures(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> () {
    add_creature(&mut commands, &mut asset_server, &mut texture_atlases, true);

    for _ in 0..10 {
        add_creature(
            &mut commands,
            &mut asset_server,
            &mut texture_atlases,
            false,
        );
    }
}

fn add_creature(
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    is_player: bool,
) -> () {
    let creature_type = match is_player {
        true => CreatureType::Human,
        false => CreatureType::Monster,
    };

    let dominance_group = match is_player {
        true => 1,
        false => 0,
    };

    let convex_polyline_opt = Collider::convex_polyline(Vec::from([
        Vect::new(0., 0.) * 100.,
        Vect::new(0., 1.) * 100.,
        Vect::new(0.5, 0.866) * 100.,
        Vect::new(0.707, 0.707) * 100.,
        Vect::new(0.866, 0.5) * 100.,
        Vect::new(1., 0.) * 100.,
    ]));

    let convex_hull_opt = Collider::round_convex_hull(
        &[
            Vect::new(0., 0.),
            Vect::new(-100., 0.),
            Vect::new(-80., -80.),
            Vect::new(0., -100.),
        ],
        1.,
    );

    // Setup the sprite sheet
    let texture_handle = asset_server.load("images/hitZone.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(300.0, 300.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let convex_polyline = match convex_polyline_opt {
        Some(it) => it,
        _ => return,
    };

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_translation(RandVec2::new().extend(1.)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        })
        //
        // Box creature collider
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(
                    creature_type.size().x,
                    creature_type.size().x,
                ))
                .insert(Damping {
                    linear_damping: 0.5,
                    angular_damping: 0.,
                })
                .insert(Friction::coefficient(0.7))
                .insert(Restitution::coefficient(3.))
                .insert(Dominance::group(dominance_group));
        })
        //
        // Sword range collider
        .with_children(|parent| {
            if is_player {
                let mut ent = parent.spawn();
                ent.insert(convex_polyline)
                    .insert_bundle(TransformBundle::from(Transform::from_rotation(
                        Quat::from_rotation_z(0.),
                    )))
                    .insert(Sensor)
                    .insert(PlayerSwordRangeSensor);
            }
        })
        //
        // Add Creature
        .with_children(|parent| {
            let mut ent = parent.spawn_bundle(CreatureBundle::new(
                creature_type.clone(),
                "Jbb".to_string(),
                100.0,
                creature_type.attack(),
            ));

            if is_player {
                ent.insert(Player);
            } else {
                ent.insert(Monster);
                ent.insert(LastAttack(Timer::from_seconds(
                    MONSTER_ATTACK_COOLDOWN,
                    false,
                )));
            }
        })
        .with_children(|parent| {
            if is_player {
                parent
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform {
                            scale: Vec3::splat(0.65),
                            ..default()
                        },
                        sprite: TextureAtlasSprite::new(0),
                        ..Default::default()
                    })
                    .insert(PlayerSwordRange);
            }
        })
        //
        // Add Sprite
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., creature_type.size().y / 2., -1.1),
                    ..default()
                },
                sprite: Sprite {
                    color: creature_type.color(),
                    custom_size: Some(creature_type.size().truncate() * 2.),
                    ..default()
                },
                ..default()
            });
        })
        //
        // Add Text
        .with_children(|parent| {
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::from_section(
                        100.0.to_string(),
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(1., 1., 1.0),
                            font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                        },
                    ),
                    transform: Transform {
                        translation: Vec3::new(-25., 60., -20.),
                        ..default()
                    },
                    ..default()
                })
                .insert(HpsDisplay);
        });
}

fn get_child_hps(
    children: &Children,
    creatures_query: &Query<&Stats, With<Creature>>,
) -> Option<f32> {
    for &child in children.iter() {
        match creatures_query.get(child) {
            Ok(stats) => return Some(stats.hp),
            Err(_) => None::<f32>,
        };
    }

    None
}

fn display_hps_system(
    entity_query: Query<(Entity, &Children)>,
    creatures_query: Query<&Stats, With<Creature>>,
    mut hps_display_query: Query<&mut Text, With<HpsDisplay>>,
) {
    // get the event
    for (_, children) in entity_query.iter() {
        if let Some(hps) = get_child_hps(&children, &creatures_query) {
            for &child in children.iter() {
                match hps_display_query.get_mut(child) {
                    Ok(mut text) => {
                        text.sections[0].value = format!("{}", hps);
                    }
                    Err(_) => (),
                };
            }
        }
    }
}
