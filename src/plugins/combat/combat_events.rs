use bevy::prelude::*;

pub struct FireProjectileEvent(pub Vec3);
pub struct ProjectileHitEvent(pub Entity, pub Entity);

pub struct HitMonsterEvent(pub Entity, pub f32);
pub struct KillMonsterEvent(pub Entity);
