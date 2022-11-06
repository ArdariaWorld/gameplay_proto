use bevy::prelude::{App, Plugin, SystemSet, Vec3};

use crate::GameState;

use super::{
    control::{
        keyboard_actions::equip_item_key, keyboard_movement::wasd_movement,
        mouse_left::mouse_left_click_system, mouse_move::mouse_move_system,
        mouse_right::mouse_right_click_system,
    },
    player_events::{KillPlayerEvent, RespawnPlayerEvent},
};

pub struct MouseMoveEvent(pub Vec3);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KillPlayerEvent>()
            .add_event::<RespawnPlayerEvent>()
            .add_event::<MouseMoveEvent>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(wasd_movement))
            .add_system(mouse_right_click_system)
            .add_system(mouse_left_click_system)
            .add_system(mouse_move_system)
            .add_system(equip_item_key);
        // .add_system(mouse_left_click_system)
        // .add_system(kill_player)
        // .add_system(respawn_player);
    }
}
