use crate::HUMAN_MAX_RANGE;

use super::{
    combat::HitMonsterEvent,
    location::Location,
    population::{Creature, Monster, Player},
};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    sprite::collide_aabb::collide,
};

pub fn handle_mouse_click(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    mut player_query: Query<&mut Location, With<Player>>,
    monsters_query: Query<(&Parent, &Location, &Creature), (With<Monster>, Without<Player>)>,
    camera_query: Query<&Transform, With<Camera>>,
    mut ev_monster_hit: EventWriter<HitMonsterEvent>,
) {
    let camera_transform = camera_query
        .get_single()
        .expect("No camera transform")
        .translation
        .truncate();

    for event in mouse_button_input_events.iter() {
        // If not event Pressed we do nothing
        if event.state != ButtonState::Pressed {
            return;
        };

        let win = windows.get_primary().expect("no primary window");

        // Get player location
        // Should never fail as player should always have location
        let mut player_location = match player_query.get_single_mut() {
            Ok(location) => location,
            Err(_) => return,
        };

        // Should never happen as cursor_position should always exists when windows is clicked
        let cursor_position = match win.cursor_position() {
            Some(position) => position,
            None => return,
        };

        // Get relative position including camera transform and coordinates interpolation
        let world_relative_click_position = cursor_position
            - Vec2::new(win.requested_width() / 2., win.requested_height() / 2.)
            + camera_transform;

        // Check every monster position against player position
        for (parent, monster_location, creature) in monsters_query.iter() {
            //
            // Get the monster location
            // Should never fail as monster.location.position should always exists
            let monster_position = match monster_location.position {
                Some(position) => position,
                None => continue,
            };

            // If not colliding, continue
            match collide(
                monster_position.extend(1.),
                creature.0.size().truncate(),
                world_relative_click_position.extend(1.),
                Vec2::splat(1.),
            ) {
                Some(_) => (),
                None => continue,
            };

            // Get player position
            // Should never fail as player.location.position should always exists
            let player_position = match player_location.position {
                Some(position) => position,
                None => continue,
            };

            // If monster is close enough from player, we hit
            if player_position.abs_diff_eq(monster_position, HUMAN_MAX_RANGE) {
                println!("Hit monster !");
                ev_monster_hit.send(HitMonsterEvent(parent.get()));
                return;
            }
        }

        // If no hit, we update player destination
        player_location.destination = Some(world_relative_click_position);
    }
}
