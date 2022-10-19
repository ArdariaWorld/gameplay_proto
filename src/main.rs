pub mod plugins;
pub mod utils;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::*;
use plugins::{
    camera::camera_follow_player, combat::CombatPlugin, location::LocationPlugin,
    player::PlayerPlugin, population::PopulationPlugin,
};

pub const CAMERA_VEC_OFFSET: Vec3 = Vec3::new(0., 25., 25.0);

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

// fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
//     // Set gravity to x and spawn camera.
//     rapier_config.gravity = Vec2::new(0.0, 0.0);
// }

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0., 25., 25.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 300000.,
            range: 90.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 50.0, 4.0),
        ..default()
    });
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(30.0, 0.1, 30.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(60., 0.2, 60.))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert_bundle(PickableBundle::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ardaria Server Prototype".to_string(),
            width: 1080.,
            height: 720.,
            ..default()
        })
        // .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_state(GameState::Playing)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(PopulationPlugin)
        .add_system(camera_follow_player)
        .add_plugin(PlayerPlugin)
        .add_plugin(LocationPlugin)
        .add_plugin(CombatPlugin)
        // .add_plugin(UiPlugin)
        // .add_plugin(HudPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
