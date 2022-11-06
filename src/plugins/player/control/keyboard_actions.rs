use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::{
    creature::creature_plugin::Player,
    items::items_plugin::{EquipItemEvent, Inventory, Item, ItemType, PickUpItemEvent, Pickable},
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
