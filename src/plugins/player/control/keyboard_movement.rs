use bevy::prelude::{Input, KeyCode, Parent, Query, Res, Vec3, With};
use bevy_rapier3d::prelude::Velocity;

use crate::{plugins::population::Player, utils::error::ErrorMessage, HUMAN_STEP_DISTANCE};

pub fn wasd_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_parent: Query<&mut Velocity>,
    player_query: Query<&Parent, With<Player>>,
) {
    let mut closure = || {
        let player_parent = player_query.get_single()?;
        let mut velocity = q_parent.get_mut(player_parent.get())?;

        let mut velocity_vector = Vec3::splat(0.);

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            velocity_vector.x = -1.;
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            velocity_vector.x = 1.;
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            velocity_vector.z = -1.;
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            velocity_vector.z = 1.;
        }

        velocity.linvel = velocity_vector * HUMAN_STEP_DISTANCE;
        // println!("Linear velocity is {:?}", velocity.linvel);
        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling click: {}", error);
    }
}
