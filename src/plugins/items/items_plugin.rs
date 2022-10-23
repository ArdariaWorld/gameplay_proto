use super::{
    systems::{
        create_items::create_items_system,
        equip_item::{display_equiped_item, equip_item_system},
        update_items::update_items_system,
    },
    weapons::melee::sword::slash_sword,
};

use bevy::prelude::*;

// Equip an ItemBundle on a specific Entity
pub struct EquipItemEvent(pub Entity, pub Entity);

// Activate the equipped item of the Entity (Creature)
pub struct ActivateItemEvent(pub Entity);

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_items_system)
            .add_system(equip_item_system)
            .add_system(update_items_system)
            .add_system(display_equiped_item)
            .add_event::<EquipItemEvent>()
            .add_event::<ActivateItemEvent>();
    }
}

#[derive(Default, Component)]
pub enum ItemType {
    #[default]
    Sword,
    Shovel,
}

#[derive(Default, Component)]
pub struct ActivationTimer(pub Timer);

#[derive(Default, Bundle)]
pub struct ItemBundle {
    pub item_type: ItemType,
    pub activation_timer: ActivationTimer,

    #[bundle]
    pub pbr_bundle: MaterialMeshBundle<StandardMaterial>,
}

impl ItemBundle {
    fn activate(&mut self) -> () {
        self.activation_timer.0.reset();
    }
}

trait UpdateItem {
    fn update(&self, time: Res<Time>) -> ();
}

impl UpdateItem for ItemBundle {
    fn update(&self, time: Res<Time>) -> () {
        match self.item_type {
            ItemType::Sword => slash_sword(&self, time),
            ItemType::Shovel => slash_sword(&self, time),
        }
    }
}

#[derive(Component)]
pub struct EquippedItem(pub Entity);

#[derive(Component)]
pub struct DroppedItem(pub Entity);
