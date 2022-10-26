use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

use crate::{utils::error::ErrorMessage, GameState};

use super::creature::{
    creature_plugin::{CreatureType, Monster, Player},
    systems::stats::{BrainState, ConsciousnessStateEnum},
};

#[derive(Default, Component, Debug)]
pub struct Location {
    pub destination: Option<Vec3>,
    pub max_velocity: Option<f32>,
    pub velocity: Option<Vec3>,
    pub position: Option<Vec3>,
}

impl Location {
    pub fn new() -> Location {
        Location { ..default() }
    }
}

pub struct LocationPlugin;
impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(location_system).add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_player_location_from_translation),
        );
    }
}

// Move non player creatures
fn location_system(
    time: Res<Time>,
    mut creatures_query: Query<
        (
            &Transform,
            &mut Velocity,
            &mut Location,
            &BrainState,
            &CreatureType,
        ),
        (With<Monster>, Without<Player>),
    >,
) {
    let mut closure = || {
        for (transform, mut velocity, mut location, brain_state, creature_type) in
            creatures_query.iter_mut()
        {
            // Get entity position
            // Update location from parent translation
            let creature_position = transform.translation;
            location.position = Some(creature_position);

            if let Some(destination) = location.destination {
                // if transform.translation is close enough to destination, remove destination
                if destination.abs_diff_eq(
                    creature_position,
                    creature_type.speed() * time.delta_seconds(),
                ) {
                    location.destination = None;
                    velocity.linvel = Vec3::new(0., 0., 0.);
                    return Ok(());
                }

                // Get normalized vector to destination
                let direction = (destination - creature_position).normalize();
                if brain_state.conscious == ConsciousnessStateEnum::Awake {
                    velocity.linvel = direction * creature_type.speed();
                }
            } else {
                velocity.linvel = Vec3::new(0., 0., 0.);
            }
        }

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while location_system: {}", error);
    }
}

fn update_player_location_from_translation(
    mut player_query: Query<(&Transform, &mut Location), With<Player>>,
) {
    let mut closure = || {
        let (transform, mut location) = player_query.get_single_mut()?;

        location.position = Some(transform.translation);

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!(
            "Error while update_player_location_from_translation: {}",
            error
        );
    }
}

#[cfg(test)]
mod tests {
    // use crate::{plugins::location::Location, *};

    // #[test]
    // fn did_update_sprite_transforms() {
    //     // Setup app
    //     let mut app = App::new();

    //     app.init_resource::<Game>();
    //     app.add_plugins(MinimalPlugins);
    //     app.add_plugin(PopulationPlugin);
    //     app.add_plugin(LocationPlugin);
    //     app.add_startup_system(init_world_map);

    //     // Update system once
    //     app.update();

    //     // Query creatures after the update
    //     let mut creatures_query = app
    //         .world
    //         .query_filtered::<(&Location, &Transform), With<Creature>>();

    //     // Query should not be empty
    //     let is_empty = creatures_query.is_empty(&app.world, 0, 0);
    //     assert_eq!(is_empty, false);

    //     // Expect transform to have been updated according to position
    //     for (location, transform) in creatures_query.iter(&app.world) {
    //         // Every transform.translation should have been updated
    //         assert_eq!(
    //             location.position.unwrap().extend(1.0),
    //             transform.translation
    //         );
    //     }
    // }
}
