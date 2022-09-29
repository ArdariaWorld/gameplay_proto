use super::{
    combat::HitMonsterEvent,
    location::Location,
    population::{Creature, Monster, Player, Stats},
};
use crate::{
    utils::{error::ErrorMessage, vec::RandVec2},
    GameState, HUMAN_MAX_RANGE,
};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    sprite::collide_aabb::collide,
};

pub struct KillPlayerEvent();
pub struct RespawnPlayerEvent();

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KillPlayerEvent>()
            .add_event::<RespawnPlayerEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(mouse_click_system),
            )
            .add_system(kill_player)
            .add_system(respawn_player);
    }
}

fn kill_player(
    mut ev_kill_player: EventReader<KillPlayerEvent>,
    mut state: ResMut<State<GameState>>,
    mut player_query: Query<&mut Location, With<Player>>,
) {
    let mut closure = || {
        for _ in ev_kill_player.iter() {
            // Set GameState as GameOver
            state.set(GameState::GameOver)?;
            let mut location = player_query.get_single_mut()?;
            location.destination = None;
        }
        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("{}", error);
    }
}

fn respawn_player(
    mut ev_respawn_player: EventReader<RespawnPlayerEvent>,
    mut state: ResMut<State<GameState>>,
    mut player_query: Query<(&mut Location, &mut Stats), With<Player>>,
) {
    let mut closure = || {
        for _ in ev_respawn_player.iter() {
            // Set GameState as Playing
            state.set(GameState::Playing)?;

            // Update destination so player stop moving
            let (mut location, mut stats) = player_query.get_single_mut()?;

            // Update player location and stats
            stats.hp = 100.;
            location.position = Some(RandVec2::new());
        }
        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("{}", error);
    }
}

fn mouse_click_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    mut player_query: Query<&mut Location, With<Player>>,
    monsters_query: Query<(&Parent, &Location, &Creature), (With<Monster>, Without<Player>)>,
    camera_query: Query<&Transform, With<Camera>>,
    mut ev_monster_hit: EventWriter<HitMonsterEvent>,
) {
    let mut closure = || {
        // Get camera transform
        let camera_transform = camera_query.get_single()?.translation.truncate();

        for event in mouse_button_input_events.iter() {
            // If not event Pressed we do nothing
            if event.state != ButtonState::Pressed {
                return Ok(());
            };

            let win = windows.get_primary().ok_or(ErrorMessage::NoWindow)?;

            // Get player location
            // Should never fail as player should always have location
            let mut player_location = player_query.get_single_mut()?;

            // Should never happen as cursor_position should always exists when windows is clicked
            let cursor_position = win
                .cursor_position()
                .ok_or(ErrorMessage::NoCursorPosition)?;

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
                    return Ok(());
                }
            }

            // If no hit, we update player destination
            player_location.destination = Some(world_relative_click_position);
        }

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling click: {}", error);
    }
}
