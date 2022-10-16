pub mod plugins;
pub mod utils;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use plugins::{
    camera::camera_follow_player,
    combat::CombatPlugin,
    hud::HudPlugin,
    location::LocationPlugin,
    player::PlayerPlugin,
    population::PopulationPlugin,
    ui::UiPlugin,
    world::{init_world_map, WorldMapBundle},
};

use bevy_rapier2d::prelude::*;

pub const HUMAN_STEP_DISTANCE: f32 = 10.;
pub const MONSTER_STEP_DISTANCE: f32 = 5.;

pub const HUMAN_MAX_RANGE: f32 = 3.;
pub const MONSTER_MAX_RANGE: f32 = 3.;

pub const HUMAN_ATK: f32 = 20.;
pub const MONSTER_ATK: f32 = 7.;
pub const PROJECTILE_IMPULSE: f32 = 0.9;
pub const MONSTER_HIT_IMPULSE: f32 = 20.;

pub const MONSTER_ATTACK_COOLDOWN: f32 = 2.;
pub const MONSTER_STUN_COOLDOWN: f32 = 2.;
pub const MONSTER_AGGRO_DISTANCE: f32 = 16.;

pub const WORLD_WIDTH: f32 = 800.0;
pub const WORLD_HEIGHT: f32 = 400.0;
pub const WALL_COLOR: Color = Color::BLUE;

pub const PIXEL_PER_METER: f32 = 50.;
pub const PIXEL_SCALE: f32 = 1. / 50.;

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

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    // Set gravity to x and spawn camera.
    rapier_config.gravity = Vec2::new(0.0, 0.0);
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ardaria Server Prototype".to_string(),
            width: 1080.,
            height: 720.,
            ..default()
        })
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXEL_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state(GameState::Playing)
        .add_plugin(CombatPlugin)
        .add_plugin(PopulationPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(LocationPlugin)
        .add_plugin(HudPlugin)
        .add_system(camera_follow_player)
        .add_startup_system(setup_rapier)
        .add_startup_system(init_world_map)
        .add_system(bevy::window::close_on_esc)
        .run();
}
