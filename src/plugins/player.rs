use super::{
    location::Location,
    population::{Creature, Monster, Player, PlayerSwordRange, PlayerSwordRangeSensor, Stats},
};
use crate::{
    utils::{error::ErrorMessage, vec::RandVec2},
    GameState, HUMAN_MAX_RANGE, HUMAN_STEP_DISTANCE,
};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    sprite::collide_aabb::collide,
};
use bevy_rapier3d::prelude::*;

pub struct KillPlayerEvent();
pub struct RespawnPlayerEvent();

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KillPlayerEvent>()
            .add_event::<RespawnPlayerEvent>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(wasd_movement))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_location_from_translation),
            );
        // .add_system(mouse_left_click_system)
        // .add_system(mouse_right_click_system)
        // .add_system(kill_player)
        // .add_system(respawn_player);
    }
}

fn wasd_movement(
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

fn update_location_from_translation(
    mut q_parent: Query<&Transform>,
    mut player_query: Query<(&Parent, &mut Location), With<Player>>,
) {
    let mut closure = || {
        let (player_parent, mut location) = player_query.get_single_mut()?;
        let transform = q_parent.get_mut(player_parent.get())?;

        location.position = Some(transform.translation);

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling click: {}", error);
    }
}
