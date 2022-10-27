use bevy::prelude::*;

use crate::plugins::{
    creature::creature_plugin::Player,
    items::items_plugin::{Item, ItemType, PickUpItemEvent},
};

/**
 * DEV system used to populate the game with some items to equip
 */
pub fn dev_init_items_system(
    mut commands: Commands,
    player_q: Query<Entity, With<Player>>,
    mut ev_pickup_item: EventWriter<PickUpItemEvent>,
) {
    let item = commands.spawn_bundle(Item {
        item_type: ItemType::Sword,
        ..default()
    });
    let player = player_q.get_single().expect("No Player found");
    ev_pickup_item.send(PickUpItemEvent(player, item.id()));
}
