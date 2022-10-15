use crate::{PIXEL_PER_METER, WALL_COLOR, WORLD_HEIGHT, WORLD_WIDTH};
use bevy::prelude::*;

#[derive(Default, Bundle)]
pub struct WorldMapBundle {
    world_map: WorldMap,

    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Default, Component)]
struct WorldMap {
    w: f32,
    h: f32,
}

// Render the world
pub fn init_world_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("images/scale_checker.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1920.0, 1080.0), 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            // scale: 1.,
            scale: 2. / PIXEL_PER_METER,
            // scale: 0.02,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(WorldMapBundle {
        world_map: WorldMap {
            h: WORLD_WIDTH,
            w: WORLD_HEIGHT,
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec2::splat(1. / PIXEL_PER_METER).extend(1.)),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        }, // SpriteBundle {
           //     transform: Transform {
           //         // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
           //         // This is used to determine the order of our sprites
           //         translation: Vec2::new(0.0, 0.0).extend(0.0),
           //         // The z-scale of 2D objects must always be 1.0,
           //         // or their ordering will be affected in surprising ways.
           //         // See https://github.com/bevyengine/bevy/issues/4149
           //         scale: Vec2::new(WORLD_WIDTH, WORLD_HEIGHT).extend(1.0),
           //         ..default()
           //     },
           //     sprite: Sprite {
           //         color: WALL_COLOR,
           //         ..default()
           //     },
           //     ..default()
           // },
    });
}
