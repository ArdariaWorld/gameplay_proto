use bevy::prelude::*;
use bevy_text_mesh::TextMeshFont;

use crate::{
    plugins::{
        items::items_plugin::{EquippedItem, Inventory, InventoryBundle},
        location::Location,
    },
    SystemsLabel, HUMAN_ATK, HUMAN_MAX_RANGE, HUMAN_STEP_DISTANCE, MONSTER_ATK, MONSTER_MAX_RANGE,
    MONSTER_STEP_DISTANCE,
};

use super::systems::{
    physical::{CreaturePhysicBundle, InsertPhysicalBody},
    sensors::SpawnSwordRangeColliderChild,
    stats::{change_consciousness_system, BrainState, CreatureName, Stats},
    ui::{display_hps_system, SpawnHpsTextMeshChild},
    visual::SpawnBodyMeshChild,
};

pub struct CreaturePlugin;
impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            spawn_creatures
                .label(SystemsLabel::Creatures)
                .before(SystemsLabel::Items),
        )
        .add_system(display_hps_system)
        .add_system(change_consciousness_system);
    }
}

#[derive(Bundle, Default)]
pub struct CreatureBundle {
    pub creature_type: CreatureType,
    pub stats: Stats,
    pub brain_state: BrainState,
    pub name: CreatureName,
    pub location: Location,
    pub inventory: Inventory,
    pub equipped_item: EquippedItem,
}

impl CreatureBundle {
    pub fn new(creature_type: CreatureType, stats: Stats, name: CreatureName) -> Self {
        Self {
            creature_type,
            stats,
            name,
            ..default()
        }
    }
}

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Player;

// ------------------
//
// CreatureType
#[derive(Clone, Copy, Component, Default, Debug)]
pub enum CreatureType {
    #[default]
    Human,
    Monster,
}

impl CreatureType {
    pub fn color(&self) -> Color {
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

    pub fn attack(&self) -> f32 {
        match self {
            CreatureType::Human => HUMAN_ATK,
            CreatureType::Monster => MONSTER_ATK,
        }
    }
}

#[derive(Default, Component)]
pub struct IsPlayer(pub bool);

#[derive(Component)]
pub struct CreatureConstructor {
    pub creature_type: CreatureType,
    pub stats: Stats,
    pub name: CreatureName,
    pub is_player: IsPlayer,
    pub physic_bundle: Option<CreaturePhysicBundle>,
}

impl CreatureConstructor {
    pub const fn new(
        creature_type: CreatureType,
        stats: Stats,
        name: CreatureName,
        is_player: IsPlayer,
        physic_bundle: CreaturePhysicBundle,
    ) -> Self {
        Self {
            creature_type,
            stats,
            name,
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

        parent.insert(Creature);

        // Insert base components
        parent.insert_bundle(CreatureBundle::new(
            self.creature_type,
            self.stats,
            self.name.clone(),
        ));

        if self.is_player.0 {
            parent.insert(Player);
        } else {
            parent.insert(Monster);
        }

        parent.insert_bundle(InventoryBundle::new());

        // Add Physical body
        self.insert_physical_body(&mut parent);

        // Spawn children
        self.spawn_sword_range_collider_child(&mut parent);
        self.spawn_hp_text_mesh_child(&mut parent, font);
        self.spawn_body_mesh_child(&mut parent, meshes, materials);
        // self.spawn_inventory_bundle(&mut parent);
    }
}

fn spawn_creatures(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> () {
    let creature_type = CreatureType::Human;
    CreatureConstructor::new(
        creature_type.clone(),
        Stats { atk: 1., hp: 100. },
        CreatureName("Moi".into()),
        IsPlayer(true),
        CreaturePhysicBundle::new(creature_type, 0),
    )
    .init(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut asset_server,
    );

    return;
    // Spawn monsters
    for _ in 0..100 {
        CreatureConstructor::new(
            CreatureType::Monster,
            Stats { atk: 1., hp: 100. },
            CreatureName("Monstre".into()),
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
