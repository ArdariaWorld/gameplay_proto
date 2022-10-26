use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{
    plugins::creature::creature_plugin::{CreatureConstructor, CreatureType},
    utils::vec::RandVec3,
};

#[derive(Default, Bundle)]
pub struct PhysicsBundle {
    #[bundle]
    transform: TransformBundle,
}

#[derive(Component, Bundle)]
pub struct CreaturePhysicBundle {
    #[bundle]
    transform_bundle: TransformBundle,
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    velocity: Velocity,
    collider: Collider,
    mass: ColliderMassProperties,
    damping: Damping,
    external_impulse: ExternalImpulse,
    dominance: Dominance,
}

impl CreaturePhysicBundle {
    pub fn new(creature_type: CreatureType, dominance_group: i8) -> Self {
        Self {
            transform_bundle: TransformBundle::from_transform(Transform::from_translation(
                RandVec3::new(),
            )),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity {
                linvel: Vec3::splat(0.),
                angvel: Vec3::splat(0.),
            },
            collider: Collider::cuboid(
                creature_type.size().x / 2.,
                creature_type.size().y / 2.,
                creature_type.size().z / 2.,
            ),
            mass: ColliderMassProperties::Density(2000.0),
            damping: Damping {
                linear_damping: 1.,
                angular_damping: 0.,
            },
            external_impulse: ExternalImpulse::default(),
            dominance: Dominance::group(dominance_group),
        }
    }
}

// ----------------
//
// Physical Body Bundle
pub trait InsertPhysicalBody {
    fn insert_physical_body(&mut self, parent: &mut EntityCommands) -> ();
}

impl InsertPhysicalBody for CreatureConstructor {
    fn insert_physical_body(&mut self, parent: &mut EntityCommands) {
        let physical_bundle = self.physic_bundle.take().unwrap();
        parent.insert_bundle(physical_bundle);

        // Specific groups depending on creature type
        if self.is_player.0 {
            parent.insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2));
        } else {
            parent.insert(ActiveEvents::COLLISION_EVENTS); // Enable events to detect projectile events
            parent.insert(CollisionGroups::new(
                Group::GROUP_2,
                Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3 | Group::GROUP_4,
            ));
        }
    }
}
