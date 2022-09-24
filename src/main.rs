#![allow(dead_code)]
pub mod plugins;
pub mod utils;

use bevy::prelude::*;
use plugins::{
    camera::camera_follow_player,
    inputs::handle_mouse_click,
    location::LocationPlugin,
    population::PopulationPlugin,
    world::{init_world_map, WorldMapBundle},
};

struct GreetTimer(Timer);
struct LocationTimer(Timer);

pub const STEP_DISTANCE: f32 = 150.;
pub const WORLD_WIDTH: f32 = 800.0;
pub const WORLD_HEIGHT: f32 = 400.0;
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.4, 0.2);

#[derive(Default)]
pub struct Game {
    // here add game state
    world_bundle: WorldMapBundle,
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PopulationPlugin)
        .add_system(handle_mouse_click)
        .add_plugin(LocationPlugin)
        .add_system(camera_follow_player)
        .add_startup_system(init_world_map)
        .add_system(bevy::window::close_on_esc)
        .run();
}
