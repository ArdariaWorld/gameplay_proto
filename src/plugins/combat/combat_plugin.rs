use bevy::prelude::{App, Component, Plugin};

use super::{
    ai::{
        aggro::monster_aggro_system, monster_attack::monster_fight_system,
        receive_damages::monster_hit_system,
    },
    combat_events::{FireProjectileEvent, HitMonsterEvent, KillMonsterEvent},
    weapons::range::{bow::fire_projectile_system, projectile::projectile_collision_system},
};

#[derive(Component, Default)]
pub struct Projectile;

#[derive(Default)]
pub struct MonstersKilled {
    pub count: i32,
}

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MonstersKilled { count: 0 })
            .add_event::<FireProjectileEvent>()
            .add_event::<HitMonsterEvent>()
            .add_event::<KillMonsterEvent>()
            .add_system(fire_projectile_system)
            .add_system(monster_hit_system)
            .add_system(monster_aggro_system)
            .add_system(monster_fight_system)
            .add_system(fire_projectile_system)
            .add_system(projectile_collision_system);
    }
}
