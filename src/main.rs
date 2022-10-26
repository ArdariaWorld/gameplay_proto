pub mod plugins;
pub mod utils;

use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_raycast::{
    DefaultPluginState, DefaultRaycastingPlugin, RayCastMesh, RayCastSource, RaycastSystem,
};
use bevy_rapier3d::{
    prelude::{Collider, NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_text_mesh::TextMeshPlugin;
use plugins::{
    camera::camera_follow_player,
    creature::creature_plugin::CreaturePlugin,
    player::{
        control::mouse::{update_raycast_with_cursor, MouseRaycastSet},
        player_plugin::PlayerPlugin,
    },
    ui::UiPlugin,
};

pub const CAMERA_VEC_OFFSET: f32 = 30.;
pub const CAMERA_VEC_OFFSET_VEC: Vec3 = Vec3::new(0., CAMERA_VEC_OFFSET, CAMERA_VEC_OFFSET);

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
            transform: Transform::from_xyz(0., 15., 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(RayCastSource::<MouseRaycastSet>::new());

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
        .insert(Collider::cuboid(300.0, 0.1, 300.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(600., 0.2, 600.))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(RayCastMesh::<MouseRaycastSet>::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Ardaria Server Prototype".to_string(),
            width: 1080.,
            height: 720.,
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .insert_resource(DefaultPluginState::<MouseRaycastSet>::default())
        // .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultRaycastingPlugin::<MouseRaycastSet>::default())
        //
        // You will need to pay attention to what order you add systems! Putting them in the wrong
        // order can result in multiple frames of latency. Ray casting should probably happen near
        // start of the frame. For example, we want to be sure this system runs before we construct
        // any rays, hence the ".before(...)". You can use these provided RaycastSystem labels to
        // order your systems with the ones provided by the raycasting plugin.
        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MouseRaycastSet>),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(TextMeshPlugin)
        .add_state(GameState::Playing)
        .add_plugin(CreaturePlugin)
        .add_plugin(PlayerPlugin)
        .add_system(camera_follow_player)
        // .add_plugin(LocationPlugin)
        // .add_plugin(CombatPlugin)
        // .add_plugin(UiPlugin)
        // .add_plugin(HudPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
