use crate::{WALL_COLOR, WORLD_HEIGHT, WORLD_WIDTH};
use bevy::prelude::*;

#[derive(Default, Bundle)]
pub struct WorldMapBundle {
    world_map: WorldMap,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

#[derive(Default, Component)]
struct WorldMap {
    w: f32,
    h: f32,
}

// Render the world
pub fn init_world_map(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(WorldMapBundle {
        world_map: WorldMap {
            h: WORLD_WIDTH,
            w: WORLD_HEIGHT,
        },
        sprite_bundle: SpriteBundle {
            transform: Transform {
                // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                // This is used to determine the order of our sprites
                translation: Vec2::new(0.0, 0.0).extend(-3.0),
                // The z-scale of 2D objects must always be 1.0,
                // or their ordering will be affected in surprising ways.
                // See https://github.com/bevyengine/bevy/issues/4149
                scale: Vec2::new(WORLD_WIDTH, WORLD_HEIGHT).extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
    });
}
