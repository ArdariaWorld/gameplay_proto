pub mod plugins;
pub mod utils;

use bevy::prelude::*;
use plugins::{
    camera::camera_follow_player,
    combat::CombatPlugin,
    hud::HudPlugin,
    inputs::handle_mouse_click,
    location::LocationPlugin,
    population::PopulationPlugin,
    ui::UiPlugin,
    world::{init_world_map, WorldMapBundle},
};

pub const HUMAN_STEP_DISTANCE: f32 = 150.;
pub const MONSTER_STEP_DISTANCE: f32 = 100.;

pub const HUMAN_MAX_RANGE: f32 = 200.;
pub const MONSTER_MAX_RANGE: f32 = 170.;

pub const HUMAN_ATK: f32 = 20.;
pub const MONSTER_ATK: f32 = 7.;

pub const MONSTER_ATTACK_COOLDOWN: f32 = 2.;
pub const MONSTER_AGGRO_DISTANCE: f32 = 200.;

pub const WORLD_WIDTH: f32 = 800.0;
pub const WORLD_HEIGHT: f32 = 400.0;
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.4, 0.2);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

#[derive(Default)]
pub struct Game {
    // here add game state
    world_bundle: WorldMapBundle,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_plugin(CombatPlugin)
        .add_plugin(PopulationPlugin)
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(handle_mouse_click))
        .add_plugin(LocationPlugin)
        .add_plugin(HudPlugin)
        .add_plugin(UiPlugin)
        .add_system(camera_follow_player)
        .add_startup_system(init_world_map)
        .add_system(bevy::window::close_on_esc)
        .run();
}
