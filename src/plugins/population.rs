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
            .add_system(display_hps_system)
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

#[derive(Component, Default, Debug)]
struct HpsDisplay(pub f32);

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

#[derive(Default, Component)]
pub struct IsPlayer(pub bool);

#[derive(Component, Bundle)]
pub struct CreaturePhysicBundle {
    #[bundle]
    transform_bundle: TransformBundle,
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    velocity: Velocity,
    collider: Collider,
    mass: ColliderMassProperties,
    damping: Damping,
    external_impulse: ExternalImpulse,
    dominance: Dominance,
}

impl CreaturePhysicBundle {
    pub fn new(creature_type: CreatureType, dominance_group: i8) -> Self {
        Self {
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                RandVec3::new(),
            )),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity {
                linvel: Vec3::splat(0.),
                angvel: Vec3::splat(0.),
            },
            collider: Collider::cuboid(
                creature_type.size().x / 2.,
                creature_type.size().y / 2.,
                creature_type.size().z / 2.,
            ),
            mass: ColliderMassProperties::Density(2000.0),
            damping: Damping {
                linear_damping: 1.,
                angular_damping: 0.,
            },
            external_impulse: ExternalImpulse::default(),
            dominance: Dominance::group(dominance_group),
        }
    }
}

#[derive(Component)]
pub struct CreatureHolder {
    pub creature_type: CreatureType,
    pub stats: Option<Stats>,
    pub is_player: IsPlayer,
    pub physic_bundle: Option<CreaturePhysicBundle>,
}

#[derive(Component)]
pub struct UiHolder {
    text_mesh_hp_entity: Option<Entity>,
}

impl CreatureHolder {
    pub const fn new(
        creature_type: CreatureType,
        stats: Stats,
        is_player: IsPlayer,
        physic_bundle: CreaturePhysicBundle,
    ) -> Self {
        Self {
            creature_type,
            stats: Some(stats),
            is_player,
            physic_bundle: Some(physic_bundle),
        }
    }

    pub fn init(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &mut Res<AssetServer>,
    ) {
        let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Medium.ttf#mesh");

        let mut parent = commands.spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        });

        // Insert base components
        parent.insert(CreatureParent);
        if self.is_player.0 {
            parent.insert(PlayerParent);
        } else {
            parent.insert(MonsterParent);
        }

        // Add Stats
        parent.insert(self.stats.take().unwrap());

        // Add Physical body
        self.insert_physical_body(&mut parent);

        // Spawn children
        self.spawn_sword_range_collider_child(&mut parent);
        self.spawn_hp_text_mesh_child(&mut parent, font);
        self.spawn_body_mesh_child(&mut parent, meshes, materials);
    }
}

// ----------------
//
// Physical Body Bundle
trait InsertPhysicalBody {
    fn insert_physical_body(&mut self, parent: &mut EntityCommands) -> ();
}

impl InsertPhysicalBody for CreatureHolder {
    fn insert_physical_body(&mut self, parent: &mut EntityCommands) {
        let physical_bundle = self.physic_bundle.take().unwrap();
        parent.insert_bundle(physical_bundle);

        // Specific groups depending on creature type
        if self.is_player.0 {
            parent.insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2));
        } else {
            parent.insert(ActiveEvents::COLLISION_EVENTS); // Enable events to detect projectile events
            parent.insert(CollisionGroups::new(
                Group::GROUP_2,
                Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_4,
            ));
        }
    }
}

// ----------------
//
// Pbr Mesh Bundle
trait SpawnSwordRangeColliderChild {
    fn spawn_sword_range_collider_child(&self, parent: &mut EntityCommands) -> ();
}

impl SpawnSwordRangeColliderChild for CreatureHolder {
    fn spawn_sword_range_collider_child(&self, cmds: &mut EntityCommands) {
        cmds.add_children(|parent| {
            if self.is_player.0 {
                parent
                    .spawn_bundle(TransformBundle::from(Transform {
                        translation: Vec3::new(1.5, 0., 0.),
                        rotation: Quat::from_rotation_z(PI / 2.),
                        ..default()
                    }))
                    .insert(Collider::cone(2., 3.))
                    .insert(Sensor)
                    .insert(PlayerSwordRangeSensor)
                    .insert(CollisionGroups::new(Group::GROUP_3, Group::GROUP_2));
            }
        })
    }
}

// ----------------
//
// Pbr Mesh Bundle
trait SpawnBodyMeshChild {
    fn spawn_body_mesh_child(
        &self,
        parent: &mut EntityCommands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> ();
}

impl SpawnBodyMeshChild for CreatureHolder {
    fn spawn_body_mesh_child(
        &self,
        cmds: &mut EntityCommands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        cmds.add_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(
                    self.creature_type.size().x,
                    self.creature_type.size().y,
                    self.creature_type.size().z,
                ))),
                material: materials.add(self.creature_type.color().into()),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            });
        });
    }
}

// ----------------
//
// Text Mesh Bundle
trait SpawnHpsTextMeshChild {
    fn spawn_hp_text_mesh_child(
        &self,
        parent: &mut EntityCommands,
        font: Handle<TextMeshFont>,
    ) -> ();
}

impl SpawnHpsTextMeshChild for CreatureHolder {
    fn spawn_hp_text_mesh_child(&self, cmds: &mut EntityCommands, font: Handle<TextMeshFont>) {
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

            children.insert(HpsDisplay(100.));
        });
    }
}

trait UpdateHpsTextMesh {
    fn update_hps(
        creature_q: Query<&Stats, With<CreatureParent>>,
        text_mesh_q: Query<(&Parent, &mut TextMesh), With<HpsDisplay>>,
    ) -> ();
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
    let creature_type = CreatureType::Human;
    CreatureHolder::new(
        creature_type.clone(),
        Stats { atk: 1., hp: 100. },
        IsPlayer(true),
        CreaturePhysicBundle::new(creature_type, 0),
    )
    .init(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut asset_server,
    );

    for _ in 0..100 {
        CreatureHolder::new(
            CreatureType::Monster,
            Stats { atk: 1., hp: 100. },
            IsPlayer(false),
            CreaturePhysicBundle::new(CreatureType::Monster, 0),
        )
        .init(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut asset_server,
        );
    }
}

// --------------
//
// Display HPs
impl UpdateHpsTextMesh for HpsDisplay {
    fn update_hps(
        creature_q: Query<&Stats, With<CreatureParent>>,
        mut text_mesh_q: Query<(&Parent, &mut TextMesh), With<HpsDisplay>>,
    ) {
        for (parent, mut text_mesh) in text_mesh_q.iter_mut() {
            match creature_q.get(parent.get()) {
                Ok(stats) => text_mesh.text = String::from(format!("{}", stats.hp)),
                Err(_) => continue,
            };
        }
    }
}
fn display_hps_system(
    creature_q: Query<&Stats, With<CreatureParent>>,
    text_mesh_q: Query<(&Parent, &mut TextMesh), With<HpsDisplay>>,
) {
    HpsDisplay::update_hps(creature_q, text_mesh_q);
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
