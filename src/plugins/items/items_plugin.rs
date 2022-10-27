use crate::SWORD_SLASH_TIME;

use super::{
    systems::{
        equip_item::{
            display_equiped_item, equip_item_system, pickup_item_system, unequip_item_system,
        },
        update_items::update_items_system,
    },
    weapons::melee::sword::slash_sword,
};

use bevy::prelude::*;

// Equip an ItemBundle on a specific Entity
pub struct EquipItemEvent(pub Entity);

/// PickUp an item from the world and equip it in the Entity inventory
/// ### Param1 - {Entity} - The entity whom pickup the item
/// ### Param2 - {Entity} - The item which is picked up by the entity
pub struct PickUpItemEvent(pub Entity, pub Entity);

// Activate the equipped item of the Entity (Creature)
pub struct ActivateItemEvent(pub Entity);

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickUpItemEvent>()
            .add_event::<EquipItemEvent>()
            .add_event::<ActivateItemEvent>()
            .add_system(pickup_item_system)
            .add_system(equip_item_system)
            .add_system(unequip_item_system)
            .add_system(update_items_system)
            .add_system(display_equiped_item);
    }
}

#[derive(Default, Component)]
pub enum ItemType {
    #[default]
    Sword,
    Shovel,
}

impl ItemType {
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

    pub fn activation_timer(&self) -> f32 {
        match self {
            ItemType::Sword => SWORD_SLASH_TIME,
            ItemType::Shovel => SWORD_SLASH_TIME,
        }
    }
}

#[derive(Default, Component)]
pub struct ActivationTimer(pub Timer);

#[derive(Component, Default)]
pub struct Item {
    pub item_type: ItemType,
    pub activation_timer: ActivationTimer,
}

impl Item {
    fn primary(&mut self) -> () {
        self.activation_timer.0.reset();
        // insert Animate Component
    }

    fn activate(&mut self) -> () {
        self.activation_timer.0.reset();
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

#[derive(Component)]
pub struct EquippedItem;

#[derive(Component)]
pub struct DroppedItem(pub Entity);

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
