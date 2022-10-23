use std::f32::consts::PI;

use crate::{
    utils::vec::{RandVec2, RandVec3},
    GameState, HUMAN_ATK, HUMAN_MAX_RANGE, HUMAN_STEP_DISTANCE, MONSTER_ATK,
    MONSTER_ATTACK_COOLDOWN, MONSTER_MAX_RANGE, MONSTER_STEP_DISTANCE, MONSTER_STUN_COOLDOWN,
    PIXEL_PER_METER, PIXEL_SCALE,
};

use super::location::Location;
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::*;
use bevy_text_mesh::{TextMesh, TextMeshBundle, TextMeshFont};

pub struct PopulationPlugin;
impl Plugin for PopulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_creatures)
            .add_system(display_hps_system_clone)
            .add_system(change_consciousness_system);
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
pub struct BrainState {
    pub conscious: ConsciousnessStateEnum,
    pub stun_at: Timer,
}

#[derive(Component, Default)]
struct HpsDisplay;

#[derive(Default, Bundle)]
pub struct PhysicsBundle {
    #[bundle]
    transform: TransformBundle,
}

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
    pub brain: BrainState,
}

#[derive(Clone, Default, Debug, Component, PartialEq)]
pub enum ConsciousnessStateEnum {
    #[default]
    Awake,
    Stun,
    Ko,
    Asleep,
    Super,
    Dead,
}

#[derive(Component, Default)]
pub struct ConsciousnessState(pub ConsciousnessStateEnum);

impl CreatureBundle {
    fn new(creature_type: CreatureType, name_str: String, hp: f32, atk: f32) -> CreatureBundle {
        CreatureBundle {
            creature: Creature(creature_type),
            stats: Stats { hp, atk },
            name: Name(name_str),
            location: Location::new(),
            brain: BrainState {
                conscious: ConsciousnessStateEnum::Awake,
                stun_at: Timer::from_seconds(MONSTER_STUN_COOLDOWN, false),
            },
        }
    }
}

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerParent;

#[derive(Component)]
pub struct MonsterParent;

#[derive(Component)]
pub struct CreatureParent;

#[derive(Component, Inspectable)]
pub struct PlayerSwordRangeSensor;

#[derive(Component)]
pub struct CreatureHps(pub Entity);

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

#[derive(Component, Bundle)]
pub struct CreatureHolder {
    ui: UiHolder,
    stats: Stats,
}

#[derive(Component)]
struct UiHolder {
    text_mesh_hp_entity: Option<Entity>,
}

impl CreatureHolder {
    pub fn init(
        &mut self,
        commands: &mut Commands,
        ent: &mut EntityCommands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &mut Res<AssetServer>,
    ) {
        let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Medium.ttf#mesh");

        self.spawn_hp_text_mesh_child(ent, font);
    }
}

trait SpawnHpsTextMeshChild {
    fn spawn_hp_text_mesh_child(
        &mut self,
        parent: &mut EntityCommands,
        font: Handle<TextMeshFont>,
    ) -> ();
}

impl SpawnHpsTextMeshChild for CreatureHolder {
    fn spawn_hp_text_mesh_child(&mut self, cmds: &mut EntityCommands, font: Handle<TextMeshFont>) {
        cmds.add_children(|parent| {
            let mut children = parent.spawn_bundle(TextMeshBundle {
                text_mesh: TextMesh::new_with_color("[hp]", font, Color::WHITE),
                transform: Transform {
                    translation: Vec3::new(-1., 1.75, 0.),
                    scale: Vec3::splat(3.),
                    ..default()
                },
                ..Default::default()
            });

            children.insert(HpsDisplay);

            self.ui.text_mesh_hp_entity = Some(children.id());
        });
    }
}

trait UpdateHpsTextMesh {
    fn update_hps(&mut self, text_mesh_q: &mut Query<&mut TextMesh, With<HpsDisplay>>) -> ();
}

impl UpdateHpsTextMesh for CreatureHolder {
    fn update_hps(&mut self, text_mesh_q: &mut Query<&mut TextMesh, With<HpsDisplay>>) {
        println!("Attempt to update hps {:?}", text_mesh_q.is_empty());

        let mesh_entity = match self.ui.text_mesh_hp_entity {
            Some(mesh) => mesh,
            None => return,
        };

        match text_mesh_q.get_mut(mesh_entity) {
            Ok(mut mesh) => mesh.text = String::from(format!("{}", self.stats.hp)),
            Err(_) => {
                error!("No text mesh for entity {:?}", mesh_entity)
            }
        };
    }
}

impl CreatureType {
    fn color(&self) -> Color {
        match self {
            CreatureType::Human => Color::GREEN,
            CreatureType::Monster => Color::RED,
        }
    }

    pub fn size(&self) -> Vec3 {
        match self {
            CreatureType::Human => Vec3::new(0.9, 1.8, 0.9),
            CreatureType::Monster => Vec3::new(1.2, 2.5, 1.2),
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> () {
    // add_creature(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     &mut asset_server,
    //     &mut texture_atlases,
    //     true,
    // );

    // let mut creature_holder = CreatureHolder {
    // spatial_bundle: SpatialBundle {
    // transform: Transform::from_xyz(0., 0., 0.),
    // ..default()
    // },
    // ui: UiHolder {
    // text_mesh_hp_entity: None,
    // },
    // stats: Stats { atk: 1., hp: 100. },
    // };

    // Spawn SpatialBundle which will hold everything
    let mut ent = commands.spawn_bundle(CreatureHolder {
        spatial_bundle: SpatialBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        ui: UiHolder {
            text_mesh_hp_entity: None,
        },
        stats: Stats { atk: 1., hp: 100. },
    });

    ent.insert(creature_holder);

    creature_holder.init(
        &mut commands,
        &mut ent,
        &mut meshes,
        &mut materials,
        &mut asset_server,
    );
    return;

    for _ in 0..100 {
        add_creature(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut asset_server,
            &mut texture_atlases,
            false,
        );
    }
}

fn add_creature(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
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

    // Setup the sprite sheet
    let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Medium.ttf#mesh");

    // Spawn SpatialBundle which will hold everything
    let mut ent = commands.spawn_bundle(SpatialBundle { ..default() });

    // Add the Creature related stuff
    ent.insert_bundle(CreatureBundle::new(
        creature_type.clone(),
        "Jbb".to_string(),
        100.0,
        creature_type.attack(),
    ))
    .insert(CreatureParent);

    // Add specific Components depending on the creature type
    if is_player {
        ent.insert(PlayerParent);
    } else {
        ent.insert(MonsterParent);
    }

    // Add the creature physics bundle
    ent.insert_bundle(TransformBundle::from_transform(
        Transform::from_translation(RandVec3::new()),
    ))
    .insert(RigidBody::Dynamic)
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Velocity {
        linvel: Vec3::splat(0.),
        angvel: Vec3::splat(0.),
    })
    .insert(Collider::cuboid(
        creature_type.size().x / 2.,
        creature_type.size().y / 2.,
        creature_type.size().z / 2.,
    ))
    .insert(ColliderMassProperties::Density(2000.0))
    .insert(Damping {
        linear_damping: 1.,
        ..default()
    })
    .insert(ExternalImpulse::default())
    .insert(Dominance::group(dominance_group));

    // Specific groups depending on creature type
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
            ent.insert(Collider::cone(2., 3.))
                .insert_bundle(TransformBundle::from(Transform {
                    translation: Vec3::new(1.5, 0., 0.),
                    rotation: Quat::from_rotation_z(PI / 2.),
                    ..default()
                }))
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
    // .with_children(|parent| {
    //     if is_player {
    //         parent
    //             .spawn_bundle(SpriteSheetBundle {
    //                 texture_atlas: texture_atlas_handle.clone(),
    //                 transform: Transform {
    //                     scale: Vec2::splat(1. / PIXEL_PER_METER).extend(1.),
    //                     ..default()
    //                 },
    //                 sprite: TextureAtlasSprite::new(0),
    //                 ..Default::default()
    //             })
    //             .insert(PlayerSwordRange);
    //     }
    // })
    //Sword range
    // .with_children(|parent| {
    //     if is_player {
    //         parent
    //             .spawn_bundle(PbrBundle {
    //                 mesh: meshes.add(Mesh::from(shape::Box::new(
    //                     creature_type.size().x,
    //                     creature_type.size().y,
    //                     creature_type.size().z,
    //                 ))),
    //                 material: materials.add(creature_type.color().into()),
    //                 transform: Transform::from_xyz(0., 0., 0.),
    //                 ..default()
    //             })
    //             .insert(PlayerSwordRange);
    //     }
    // })
    //
    // Add Sprite
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                creature_type.size().x,
                creature_type.size().y,
                creature_type.size().z,
            ))),
            material: materials.add(creature_type.color().into()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        });
    })
    //
    // Text
    .with_children(|parent| {
        parent
            .spawn_bundle(TextMeshBundle {
                text_mesh: TextMesh::new_with_color("[hp]", font, Color::WHITE),
                transform: Transform {
                    translation: Vec3::new(-1., 1.75, 0.),
                    scale: Vec3::splat(3.),
                    ..default()
                },
                ..Default::default()
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

fn display_hps_system(mut stats_q: Query<(&Stats, &mut TextMesh), With<Creature>>) {
    for (stat, mut tex_mesh) in stats_q.iter_mut() {
        tex_mesh.text = String::from(format!("{}", stat.hp));
    }
}

fn display_hps_system_clone(
    mut creature_q: Query<(&mut CreatureHolder, Entity), With<Creature>>,
    mut text_mesh_q: Query<&mut TextMesh, With<HpsDisplay>>,
) {
    for (mut creature, entity) in creature_q.iter_mut() {
        // for mut child in children.iter_mut() {
        creature.update_hps(&mut text_mesh_q);
        // }
        // tex_mesh.text = String::from(format!("{}", stat.hp));
    }
}

fn change_consciousness_system(
    time: Res<Time>,
    mut creatures_q: Query<&mut BrainState, With<Monster>>,
) {
    for mut creature_brain_state in creatures_q.iter_mut() {
        if creature_brain_state.stun_at.tick(time.delta()).finished() {
            creature_brain_state.conscious = ConsciousnessStateEnum::Awake;
        }
    }
}
