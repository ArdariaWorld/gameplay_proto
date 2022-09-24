use bevy::prelude::Vec2;
use rand::Rng;

use crate::{WORLD_HEIGHT, WORLD_WIDTH};

pub struct RandVec2 {
    x: f32,
    y: f32,
}

impl RandVec2 {
    pub fn new() -> Vec2 {
        Vec2::new(
            rand::thread_rng().gen_range(0..WORLD_WIDTH as i32) as f32,
            rand::thread_rng().gen_range(0..WORLD_HEIGHT as i32) as f32,
        )
    }
}
