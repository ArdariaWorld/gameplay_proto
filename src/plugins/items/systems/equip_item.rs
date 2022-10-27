use crate::plugins::{
    creature::creature_plugin::Creature,
    items::items_plugin::{
        ActivationTimer, EquipItemEvent, EquippedItem, Inventory, Item, ItemType, PickUpItemEvent,
    },
};
use bevy::prelude::*;

/// # equip_item_system
/// Get the bundle item from available item bundles  
///
/// Add Equipped component to this item bundle  
///
/// Remove Equiped component from previous equiped item bundle  
///
pub fn equip_item_system(
    mut commands: Commands,
    creature_q: Query<(Entity, &Inventory), With<Creature>>,
    item_q: Query<&Item>,
    mut ev_equip_item: EventReader<EquipItemEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_equip_item.iter() {
        let (entity, inventory) = creature_q.get(ev.0).expect("No creature found");

        println!("inventory found");

        let item_entity = inventory.0.first().take().unwrap();
        let item = item_q
            .get(item_entity.clone())
            .expect("Cant find given item entity");

        // Add holder entity reference in EquippedItem ?
        // commands.entity(*item_entity).insert(EquippedItem);

        // Add EquippedItem component to inventory with Item entity reference

        commands.entity(entity).add_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(item.item_type.mesh()),
                material: materials.add(item.item_type.color().into()),
                transform: Transform::from_xyz(1., 0., 0.5),
                ..default()
            });
        });
    }
}

/// # unequip_item_system
///
///
pub fn unequip_item_system() {}

/**
 * Display the equipped bundle item on the creature
 */
pub fn display_equiped_item(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut changed_items_q: Query<(Entity, &ItemType), Added<EquippedItem>>,
) {
    for (entity, item) in changed_items_q.iter_mut() {
        println!("Item changed");
        // item.activation_timer = ActivationTimer(Timer::from_seconds(
        //     item.item_type.activation_timer(),
        //     false,
        // ));

        // commands.entity(entity).insert_bundle(PbrBundle {
        //     mesh: meshes.add(item.item_type.shape()),
        //     material: materials.add(item.item_type.color().into()),
        //     transform: Transform::from_xyz(1., 0., 1.),
        //     ..default()
        // });
    }
}

/**
 * Receive a PickUpItem event and place item into entity inventory
 */
pub fn pickup_item_system(
    mut inventory_q: Query<&mut Inventory, With<Creature>>,
    mut ev_pickup_item: EventReader<PickUpItemEvent>,
) {
    for ev in ev_pickup_item.iter() {
        println!("PickUpItemEvent {:?} - {:?}", ev.0, ev.1);

        let mut inventory = match inventory_q.get_mut(ev.0) {
            Ok(i) => i,
            Err(_) => continue,
        };

        println!("Inventory found {:?}", inventory.0.len());

        inventory.0.push(ev.1)
    }
}
