use bevy::{math::Vec3A, prelude::*};

use crate::{utils::vec::RandVec2, STEP_DISTANCE};

use super::population::Creature;

#[derive(Default, Component)]
pub struct Location {
    pub destination: Option<Vec2>,
    pub max_velocity: Option<f32>,
    pub velocity: Option<Vec2>,
    pub position: Option<Vec2>,
}

impl Location {
    pub fn new() -> Location {
        Location {
            position: Some(RandVec2::new()),
            ..default()
        }
    }
}

pub struct LocationPlugin;
impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(location_system);
    }
}

fn location_system(
    time: Res<Time>,
    mut creatures_query: Query<(&Parent, &mut Location), With<Creature>>,
    mut q_parent: Query<&mut Transform>,
) {
    for (parent_entity, mut location) in creatures_query.iter_mut() {
        // Update location if entity have a destination
        if let (Some(destination), Some(position)) = (location.destination, location.position) {
            // compute vector from position to destination
            let delta_v = destination - position;
            let new_positon = position + delta_v.normalize() * STEP_DISTANCE * time.delta_seconds();

            // apply new vector to position to get new_posittion
            if !destination.abs_diff_eq(new_positon, STEP_DISTANCE * time.delta_seconds()) {
                location.position = Some(new_positon);
            }
        }

        // Update parent transform from creature computed position
        if let (Ok(mut parent_transform), Some(position)) =
            (q_parent.get_mut(parent_entity.get()), location.position)
        {
            parent_transform.translation = position.extend(2.);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        plugins::{location::Location, population::Creature},
        *,
    };

    #[test]
    fn did_update_sprite_transforms() {
        // Setup app
        let mut app = App::new();

        app.init_resource::<Game>();
        app.add_plugins(MinimalPlugins);
        app.add_plugin(PopulationPlugin);
        app.add_plugin(LocationPlugin);
        app.add_startup_system(init_world_map);

        // Update system once
        app.update();

        // Query creatures after the update
        let mut creatures_query = app
            .world
            .query_filtered::<(&Location, &Transform), With<Creature>>();

        // Query should not be empty
        let is_empty = creatures_query.is_empty(&app.world, 0, 0);
        assert_eq!(is_empty, false);

        // Expect transform to have been updated according to position
        for (location, transform) in creatures_query.iter(&app.world) {
            // Every transform.translation should have been updated
            assert_eq!(
                location.position.unwrap().extend(1.0),
                transform.translation
            );
        }
    }
}
