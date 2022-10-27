use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::plugins::{
    creature::creature_plugin::CreatureConstructor, items::items_plugin::InventoryBundle,
};

pub trait SpawnInventoryChildBundle {
    fn spawn_inventory_bundle(&self, parent: &mut EntityCommands) -> ();
}

impl SpawnInventoryChildBundle for CreatureConstructor {
    fn spawn_inventory_bundle(&self, cmds: &mut EntityCommands) {
        cmds.add_children(|parent| {
            parent.spawn_bundle(InventoryBundle::new());
        })
    }
}
