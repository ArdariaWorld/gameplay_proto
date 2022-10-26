use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_text_mesh::{TextMesh, TextMeshBundle, TextMeshFont};

use crate::plugins::creature::creature_plugin::{CreatureConstructor, CreatureEntity};

use super::stats::Stats;

#[derive(Component, Default, Debug)]
pub struct HpsDisplay(pub f32);

#[derive(Component)]
pub struct CreatureHps(pub Entity);

// ----------------
//
// Text Mesh Bundle
pub trait SpawnHpsTextMeshChild {
    fn spawn_hp_text_mesh_child(
        &self,
        parent: &mut EntityCommands,
        font: Handle<TextMeshFont>,
    ) -> ();
}

impl SpawnHpsTextMeshChild for CreatureConstructor {
    fn spawn_hp_text_mesh_child(&self, cmds: &mut EntityCommands, font: Handle<TextMeshFont>) {
        cmds.add_children(|parent| {
            let mut children = parent.spawn_bundle(TextMeshBundle {
                text_mesh: TextMesh::new_with_color("[hp]", font, Color::WHITE),
                transform: Transform {
                    translation: Vec3::new(-1., 1.75, 0.),
                    scale: Vec3::splat(3.),
                    ..default()
                },
                ..Default::default()
            });

            children.insert(HpsDisplay(100.));
        });
    }
}

pub trait UpdateHpsTextMesh {
    fn update_hps(
        creature_q: Query<&Stats, With<CreatureEntity>>,
        text_mesh_q: Query<(&Parent, &mut TextMesh), With<HpsDisplay>>,
    ) -> ();
}

// --------------
//
// Display HPs
impl UpdateHpsTextMesh for HpsDisplay {
    fn update_hps(
        creature_q: Query<&Stats, With<CreatureEntity>>,
        mut text_mesh_q: Query<(&Parent, &mut TextMesh), With<HpsDisplay>>,
    ) {
        for (parent, mut text_mesh) in text_mesh_q.iter_mut() {
            match creature_q.get(parent.get()) {
                Ok(stats) => text_mesh.text = String::from(format!("{}", stats.hp)),
                Err(_) => continue,
            };
        }
    }
}
pub fn display_hps_system(
    creature_q: Query<&Stats, With<CreatureEntity>>,
    text_mesh_q: Query<(&Parent, &mut TextMesh), With<HpsDisplay>>,
) {
    HpsDisplay::update_hps(creature_q, text_mesh_q);
}
