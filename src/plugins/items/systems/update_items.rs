use bevy::prelude::*;

use crate::plugins::items::items_plugin::{AnimateVisualItem, AnimationTimer};

/**
 * Query all equiped items and call their update function with the current delta time
 */
pub fn start_items_animation_system(
    mut animating_items_q: Query<&mut AnimationTimer, Added<AnimateVisualItem>>,
) {
    for mut animation_timer in animating_items_q.iter_mut() {
        println!("Starting animation");
        animation_timer.0.reset();
    }
}

pub fn animate_items_system(
    time: Res<Time>,
    mut commands: Commands,
    mut animating_items_q: Query<
        (Entity, &mut Transform, &mut AnimationTimer),
        With<AnimateVisualItem>,
    >,
) {
    for (entity, mut transform, mut timer) in animating_items_q.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            println!("Animation finished!");
            commands.entity(entity).remove::<AnimateVisualItem>();
            transform.rotation = Quat::default();
            timer.0.reset();
        } else {
            println!("animate_items_system {:?}", timer.0.percent());
            transform.rotation = transform
                .rotation
                .lerp(Quat::from_rotation_x(2.), timer.0.percent());
        }
    }
}
