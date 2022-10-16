use crate::{
    utils::vec::RandVec2, GameState, HUMAN_ATK, HUMAN_MAX_RANGE, HUMAN_STEP_DISTANCE, MONSTER_ATK,
    MONSTER_ATTACK_COOLDOWN, MONSTER_MAX_RANGE, MONSTER_STEP_DISTANCE, PIXEL_PER_METER,
    PIXEL_SCALE,
};

use super::location::Location;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

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
            CreatureType::Human => Color::GREEN,
            CreatureType::Monster => Color::RED,
        }
    }

    pub fn size(&self) -> Vec3 {
        match self {
            CreatureType::Human => Vec2::new(0.9, 1.8).extend(2.0),
            CreatureType::Monster => Vec2::new(1.2, 2.5).extend(1.0),
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

    return;
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
        Vect::new(0., 0.) * HUMAN_MAX_RANGE,
        Vect::new(0., 1.) * HUMAN_MAX_RANGE,
        Vect::new(0.5, 0.866) * HUMAN_MAX_RANGE,
        Vect::new(0.707, 0.707) * HUMAN_MAX_RANGE,
        Vect::new(0.866, 0.5) * HUMAN_MAX_RANGE,
        Vect::new(1., 0.) * HUMAN_MAX_RANGE,
    ]));

    // Setup the sprite sheet
    let texture_handle = asset_server.load("images/hitZone.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(300.0, 300.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let convex_polyline = match convex_polyline_opt {
        Some(it) => it,
        _ => return,
    };

    let mut ent = commands.spawn_bundle(SpatialBundle {
        transform: Transform::from_xyz(0., 0., 2.),
        ..default()
    });

    ent.insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(RandVec2::new().extend(2.)),
        ))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        })
        .insert(Collider::cuboid(
            creature_type.size().x / 2.,
            creature_type.size().y / 2.,
        ))
        .insert(ColliderMassProperties::Density(1000.0))
        // .insert(Damping {
        // linear_damping: 50.,
        // angular_damping: 0.,
        // })
        // .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(3.))
        .insert(Dominance::group(dominance_group));

    if is_player {
        ent.insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2));
    } else {
        ent.insert(ActiveEvents::COLLISION_EVENTS); // Enable events to detect projectile events
        ent.insert(CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_4,
        ));
    }

    //
    // Sword range collider
    ent.with_children(|parent| {
        if is_player {
            let mut ent = parent.spawn();
            ent.insert(convex_polyline)
                .insert_bundle(TransformBundle::from(Transform::from_rotation(
                    Quat::from_rotation_z(0.),
                )))
                .insert(Sensor)
                .insert(PlayerSwordRangeSensor)
                .insert(CollisionGroups::new(Group::GROUP_3, Group::GROUP_2));
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
    //
    //Sword range
    .with_children(|parent| {
        if is_player {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        scale: Vec2::splat(1. / PIXEL_PER_METER).extend(1.),
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
                scale: Vec3::new(creature_type.size().x, creature_type.size().y, 1.),
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
        parent
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    100.0.to_string(),
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                    },
                ),
                transform: Transform {
                    translation: Vec3::new(
                        -creature_type.size().x / 2. - 0.1,
                        creature_type.size().y,
                        0.,
                    ),
                    scale: Vec2::splat(PIXEL_SCALE).extend(1.),
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
