use bevy::prelude::Vec2;
use rand::Rng;

use crate::{PIXEL_PER_METER, WORLD_HEIGHT, WORLD_WIDTH};

pub struct RandVec2 {}

impl RandVec2 {
    pub fn new() -> Vec2 {
        Vec2::new(
            rand::thread_rng().gen_range(-WORLD_WIDTH as i32..WORLD_WIDTH as i32) as f32
                / PIXEL_PER_METER,
            rand::thread_rng().gen_range(-WORLD_HEIGHT as i32..WORLD_HEIGHT as i32) as f32
                / PIXEL_PER_METER,
        )
    }
}
