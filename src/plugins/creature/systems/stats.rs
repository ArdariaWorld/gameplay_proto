use bevy::prelude::*;

use crate::{plugins::creature::creature_plugin::CreatureEntity, MONSTER_STUN_COOLDOWN};

#[derive(Clone, Component, Default)]
pub struct CreatureName(pub String);

#[derive(Component)]
pub struct LastAttack(pub Timer);

#[derive(Copy, Clone, Component, Default)]
pub struct Stats {
    pub hp: f32,
    pub atk: f32,
}

#[derive(Component, Default)]
pub struct BrainState {
    pub conscious: ConsciousnessStateEnum,
    pub stun_at: Timer,
}

impl BrainState {
    pub fn new() -> Self {
        Self {
            conscious: ConsciousnessStateEnum::Awake,
            stun_at: Timer::from_seconds(MONSTER_STUN_COOLDOWN, false),
        }
    }
}

#[derive(Clone, Default, Debug, Component, PartialEq)]
pub enum ConsciousnessStateEnum {
    #[default]
    Awake,
    Stun,
    Ko,
    Asleep,
    Super,
    Dead,
}

#[derive(Component, Default)]
pub struct ConsciousnessState(pub ConsciousnessStateEnum);

pub fn change_consciousness_system(
    time: Res<Time>,
    mut creatures_q: Query<&mut BrainState, With<CreatureEntity>>,
) {
    for mut creature_brain_state in creatures_q.iter_mut() {
        if creature_brain_state.stun_at.tick(time.delta()).finished() {
            creature_brain_state.conscious = ConsciousnessStateEnum::Awake;
        }
    }
}
