use bevy::prelude::{App, Plugin, SystemSet};

use crate::GameState;

use super::{
    control::keyboard_movement::wasd_movement,
    player_events::{KillPlayerEvent, RespawnPlayerEvent},
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KillPlayerEvent>()
            .add_event::<RespawnPlayerEvent>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(wasd_movement));
        // .add_system(mouse_left_click_system)
        // .add_system(mouse_right_click_system)
        // .add_system(kill_player)
        // .add_system(respawn_player);
    }
}
