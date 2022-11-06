use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, Sensor};

use crate::{
    plugins::{
        creature::creature_plugin::Player,
        items::items_plugin::{Item, ItemType, PickUpItemEvent, Pickable},
    },
    MONSTER_GROUP, PICKABLE_GROUP, PLAYER_GROUP,
};

/**
 * DEV system used to populate the game with some items to equip
 */
pub fn dev_init_items_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create Item
    let mut item_entity = commands.spawn();
    item_entity.insert(Item {
        item_type: ItemType::Sword,
        ..default()
    });

    // DropItemEvent
    item_entity.insert_bundle(SpatialBundle {
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..default()
    });

    item_entity.add_children(|parent| {
        let mut transform = Transform::from_xyz(1., 0., 0.5);
        transform.rotate_z(PI / 2.);

        parent
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(ItemType::Sword.mesh()),
                material: materials.add(ItemType::Sword.color().into()),
                transform,
                ..default()
            })
            .insert(Collider::capsule(Vec3::Y / 2., Vec3::ZERO, 1.))
            .insert(Sensor)
            .insert(Pickable)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(CollisionGroups::new(
                PICKABLE_GROUP,
                PLAYER_GROUP | MONSTER_GROUP,
            ));
    });
}
