use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::plugins::creature::creature_plugin::CreatureConstructor;

// ----------------
//
// Pbr Mesh Bundle
pub trait SpawnBodyMeshChild {
    fn spawn_body_mesh_child(
        &self,
        parent: &mut EntityCommands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> ();
}

impl SpawnBodyMeshChild for CreatureConstructor {
    fn spawn_body_mesh_child(
        &self,
        cmds: &mut EntityCommands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        cmds.add_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(
                    self.creature_type.size().x,
                    self.creature_type.size().y,
                    self.creature_type.size().z,
                ))),
                material: materials.add(self.creature_type.color().into()),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            });
        });
    }
}
