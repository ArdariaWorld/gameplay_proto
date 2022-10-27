use bevy::prelude::{default, Commands, Entity, EventWriter, Input, KeyCode, Query, Res, With};

use crate::plugins::{
    creature::creature_plugin::Player,
    items::items_plugin::{EquipItemEvent, Inventory, Item, ItemEntity, ItemType, PickUpItemEvent},
};

pub fn equip_item_key(
    keyboard_input: Res<Input<KeyCode>>,
    player_q: Query<Entity, With<Player>>,
    mut ev_equip_item: EventWriter<EquipItemEvent>,
) {
    if keyboard_input.pressed(KeyCode::E) {
        let player = player_q.get_single().expect("No Player found");

        ev_equip_item.send(EquipItemEvent(player));
    }
}

pub fn dev_spawn_item_key(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_q: Query<(Entity, &Inventory), With<Player>>,
    mut ev_pickup_item: EventWriter<PickUpItemEvent>,
) {
    if keyboard_input.pressed(KeyCode::X) {
        let (player, inventory) = player_q.get_single().expect("No Player found");

        if inventory.0.len() == 0 {
            println!("Inventory empty -> creating item");
            let mut item = commands.spawn_bundle(Item {
                item_type: ItemType::Sword,
                ..default()
            });
            item.insert(ItemEntity);

            println!(
                "Inventory empty -> sending event {:?} -- {:?}",
                player,
                item.id()
            );
            ev_pickup_item.send(PickUpItemEvent(player, item.id()));
        }
    }
}
