use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

use crate::plugins::creature::creature_plugin::CreatureConstructor;

#[derive(Component, Inspectable)]
pub struct PlayerSwordRangeSensor;

// ----------------
//
// Sword range collider
pub trait SpawnSwordRangeColliderChild {
    fn spawn_sword_range_collider_child(&self, parent: &mut EntityCommands) -> ();
}

impl SpawnSwordRangeColliderChild for CreatureConstructor {
    fn spawn_sword_range_collider_child(&self, cmds: &mut EntityCommands) {
        cmds.add_children(|parent| {
            if self.is_player.0 {
                parent
                    .spawn_bundle(TransformBundle::from(Transform {
                        translation: Vec3::new(1.5, 0., 0.),
                        rotation: Quat::from_rotation_z(PI / 2.),
                        ..default()
                    }))
                    .insert(Collider::cone(2., 3.))
                    .insert(Sensor)
                    .insert(PlayerSwordRangeSensor)
                    .insert(CollisionGroups::new(Group::GROUP_3, Group::GROUP_2));
            }
        })
    }
}
