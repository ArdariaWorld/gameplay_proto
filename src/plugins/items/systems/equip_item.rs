use crate::plugins::{
    creature::creature_plugin::Creature,
    items::items_plugin::{
        EquipItemEvent, EquippedItem, Inventory, Item, ItemMesh, ItemType, PickUpItemEvent,
        VisualItem,
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
    mut creature_q: Query<(Entity, &Inventory, &mut EquippedItem), With<Creature>>,
    item_q: Query<&Item>,
    mut ev_equip_item: EventReader<EquipItemEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ev in ev_equip_item.iter() {
        let (creature_entity, inventory, mut equipped_item) =
            creature_q.get_mut(ev.0).expect("No creature found");

        println!("inventory found");

        let item_entity = inventory.0.first().take().unwrap();
        let item = item_q
            .get(item_entity.clone())
            .expect("Cant find given item entity");

        if let Some(mut item) = equipped_item.0 {
            println!("Item already equipped");
            continue;
        }

        // Add EquippedItem component to inventory with Item entity reference
        // And spawn pbr bundle
        commands.entity(creature_entity).add_children(|parent| {
            let mut child = parent.spawn_bundle(VisualItem {
                mesh: PbrBundle {
                    mesh: meshes.add(item.item_type.mesh()),
                    material: materials.add(item.item_type.color().into()),
                    transform: Transform::from_xyz(1., 0., 0.5),
                    ..default()
                },
                animation_timer: item.item_type.animation_timer(),
            });

            child.insert(ItemMesh);

            equipped_item.0 = Some(child.id());
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
