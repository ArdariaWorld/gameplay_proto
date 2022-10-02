use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use super::population::Creature;

#[derive(Default, Component, Debug)]
pub struct Location {
    pub destination: Option<Vec2>,
    pub max_velocity: Option<f32>,
    pub velocity: Option<Vec2>,
    pub position: Option<Vec2>,
}

impl Location {
    pub fn new() -> Location {
        Location { ..default() }
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
    mut creatures_query: Query<(&Parent, &mut Location, &Creature), With<Creature>>,
    mut q_parent: Query<(&Transform, &mut Velocity)>,
) {
    for (parent_entity, mut location, creature) in creatures_query.iter_mut() {
        // Get entity position
        if let Ok((translation, mut velocity)) = q_parent.get_mut(parent_entity.get()) {
            // Update location from parent translation
            let creature_position = translation.translation.truncate();
            location.position = Some(creature_position);

            // Set a velocity if creature has a detination
            if let Some(destination) = location.destination {
                // if transform.translation is close enough to destination, remove destination
                if destination
                    .abs_diff_eq(creature_position, creature.0.speed() * time.delta_seconds())
                {
                    location.destination = None;
                    velocity.linvel = Vec2::new(0., 0.);
                    return;
                }

                // Get normalized vector to destination
                let direction = (destination - creature_position).normalize();
                velocity.linvel = direction * creature.0.speed();
            } else {
                velocity.linvel = Vec2::new(0., 0.);
            }
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
