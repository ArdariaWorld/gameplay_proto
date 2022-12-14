use bevy::prelude::*;

use crate::{
    plugins::items::items_plugin::{ActivationTimer, Item, ItemType},
    SWORD_SLASH_TIME,
};

#[derive(Default, Component)]
pub struct Sword;

pub fn create_sword(
    mut commands: Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn()
        .insert(Item {
            item_type: ItemType::Sword,
            cooldown_timer: ActivationTimer(Timer::from_seconds(SWORD_SLASH_TIME, false)),
            // pbr_bundle: PbrBundle {
            //     mesh: meshes.add(Mesh::from(shape::Box::new(0.2, 1.3, 0.2))),
            //     material: materials.add(Color::PURPLE.into()),
            //     transform: Transform::from_xyz(0., 0., 0.),
            //     ..default()
            // },
        })
        .insert(Sword);
}

/**
 * Receive an event with the
 */
pub fn slash_sword(item_bundle: &Item, time: Res<Time>) {}
