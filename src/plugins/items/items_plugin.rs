use crate::SWORD_SLASH_TIME;

use super::{
    systems::{
        create_items::dev_init_items_system,
        dropped_items::dropped_items_collision_system,
        equip_item::{
            display_equiped_item, equip_item_system, pickup_item_system, unequip_item_system,
        },
        update_items::{animate_items_system, start_items_animation_system},
    },
    weapons::melee::sword::slash_sword,
};

use bevy::{ecs::bundle, prelude::*};

// Equip an ItemBundle on a specific Entity
pub struct EquipItemEvent(pub Entity);

/// PickUp an item from the world and equip it in the Entity inventory
/// ### Param1 - {Entity} - The entity whom pickup the item
/// ### Param2 - {Entity} - The item which is picked up by the entity
pub struct PickUpItemEvent(pub Entity, pub Entity);

/// Drop an item on the floor
/// ### Param 1 - {Entity} - The Item entity to drop
/// ### Param 2 - {Tranform} - The global transform to drop the item
pub struct DropItemEvent(pub Entity, pub Transform);

// Activate the equipped item of the Entity (Creature)
pub struct ActivateItemEvent(pub Entity);

#[derive(Component)]
pub struct Pickable;

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickUpItemEvent>()
            .add_event::<EquipItemEvent>()
            .add_event::<ActivateItemEvent>()
            .add_startup_system(dev_init_items_system)
            .add_system(pickup_item_system)
            .add_system(equip_item_system)
            .add_system(unequip_item_system)
            .add_system(start_items_animation_system)
            .add_system(display_equiped_item)
            .add_system(dropped_items_collision_system)
            .add_system(animate_items_system);
    }
}

#[derive(Default, Component)]
pub struct ItemMesh;

#[derive(Default, Component)]
pub enum ItemType {
    #[default]
    Sword,
    Shovel,
}

impl ItemType {
    pub fn dimensions(&self) -> Vec3 {
        match self {
            ItemType::Sword => Vec3::new(0.2, 1.3, 0.2),
            ItemType::Shovel => Vec3::new(0.2, 1.3, 0.2),
        }
    }

    pub fn mesh(&self) -> Mesh {
        match self {
            ItemType::Sword => Mesh::from(shape::Box::new(0.2, 1.3, 0.2)),
            ItemType::Shovel => Mesh::from(shape::Box::new(0.4, 1.3, 0.4)),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ItemType::Sword => Color::PURPLE,
            ItemType::Shovel => Color::SEA_GREEN,
        }
    }

    pub fn animation_timer(&self) -> AnimationTimer {
        match self {
            ItemType::Sword => AnimationTimer(Timer::from_seconds(SWORD_SLASH_TIME, false)),
            ItemType::Shovel => AnimationTimer(Timer::from_seconds(SWORD_SLASH_TIME, false)),
        }
    }

    pub fn cooldown_timer(&self) -> ActivationTimer {
        match self {
            ItemType::Sword => ActivationTimer(Timer::from_seconds(SWORD_SLASH_TIME, false)),
            ItemType::Shovel => ActivationTimer(Timer::from_seconds(SWORD_SLASH_TIME, false)),
        }
    }
}

#[derive(Default, Component)]
pub struct ActivationTimer(pub Timer);

#[derive(Default, Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Default)]
pub struct Item {
    pub item_type: ItemType,
    pub cooldown_timer: ActivationTimer,
}

#[derive(Bundle, Component, Default)]
pub struct VisualItem {
    #[bundle]
    pub mesh: MaterialMeshBundle<StandardMaterial>,
    pub animation_timer: AnimationTimer,
}

#[derive(Bundle, Default)]
pub struct DroppedItem {
    #[bundle]
    pub mesh: MaterialMeshBundle<StandardMaterial>,
    // pub collider: Collider,
    // pub sensor: Sensor,
    // .insert(Collider::cone(2., 3.))
    // .insert(Sensor)
    // .insert(PlayerSwordRangeSensor)
    // .insert(CollisionGroups::new(Group::GROUP_3, Group::GROUP_2));

    // Collider
}

#[derive(Component, Default)]
pub struct AnimateVisualItem;

impl Item {
    fn primary(&mut self) -> () {
        self.cooldown_timer.0.reset();
        // insert Animate Component
    }

    fn activate(&mut self) -> () {
        self.cooldown_timer.0.reset();
    }
}

trait UpdateItem {
    fn update(&self, time: Res<Time>) -> ();
}

impl UpdateItem for Item {
    fn update(&self, time: Res<Time>) -> () {
        match self.item_type {
            ItemType::Sword => slash_sword(&self, time),
            ItemType::Shovel => slash_sword(&self, time),
        }
    }
}

trait EquipItem {
    fn equip(&self, commands: Commands, inventory_entity: Entity) -> ();
}

impl EquipItem for Item {
    fn equip(&self, mut commands: Commands, inventory_entity: Entity) {
        let cmds = commands.entity(inventory_entity);
        // cmds.insert_bundle(bundle)
    }
}

#[derive(Component, Default)]
pub struct EquippedItem(pub Option<Entity>);

#[derive(Bundle, Component)]
pub struct InventoryBundle {
    inventory: Inventory,
    belt: Belt,
}

impl InventoryBundle {
    pub fn new() -> Self {
        Self {
            inventory: Inventory(Vec::new()),
            belt: Belt(Vec::new()),
        }
    }

    // pickup ?
}

#[derive(Component, Default)]
pub struct Inventory(pub Vec<Entity>);

#[derive(Component, Default)]
pub struct Belt(pub Vec<Entity>);
