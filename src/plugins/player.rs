use super::{
    combat::HitMonsterEvent,
    location::Location,
    population::{Creature, Monster, Player, PlayerSwordRange, Stats},
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
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, Velocity};

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
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(wasd_movement))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_location_from_translation),
            )
            .add_system(mouse_left_click_system)
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

fn wasd_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_parent: Query<&mut Velocity>,
    player_query: Query<&Parent, With<Player>>,
) {
    let mut closure = || {
        let player_parent = player_query.get_single()?;
        let mut velocity = q_parent.get_mut(player_parent.get())?;

        let mut velocity_vector = Vec2::splat(0.);

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            velocity_vector.x = -1.;
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            velocity_vector.x = 1.;
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            velocity_vector.y = 1.;
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            velocity_vector.y = -1.;
        }

        velocity.linvel = velocity_vector * 150.;
        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling click: {}", error);
    }
}

fn update_location_from_translation(
    mut q_parent: Query<&Transform>,
    mut player_query: Query<(&Parent, &mut Location), With<Player>>,
) {
    let mut closure = || {
        let (player_parent, mut location) = player_query.get_single_mut()?;
        let transform = q_parent.get_mut(player_parent.get())?;

        location.position = Some(transform.translation.truncate());

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling click: {}", error);
    }
}

fn mouse_left_click_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
    mut collider_query: Query<
        (&mut Transform, &Collider, Entity, &Parent),
        (With<Collider>, With<PlayerSwordRange>),
    >,
    q_parent: Query<&Transform, Without<PlayerSwordRange>>,
) {
    let mut closure = || {
        for event in mouse_button_input_events.iter() {
            // If not event Pressed we do nothing
            if event.state != ButtonState::Pressed || event.button != MouseButton::Left {
                return Ok(());
            };

            let win = windows.get_primary().ok_or(ErrorMessage::NoWindow)?;

            // get angle from click position
            // Should never happen as cursor_position should always exists when windows is clicked
            let cursor_position = win
                .cursor_position()
                .ok_or(ErrorMessage::NoCursorPosition)?;

            // Correct the mouse position with windows size (0,0 at center)
            let centered_cursor_position = Vec2::new(
                cursor_position.x - win.requested_width() / 2.,
                cursor_position.y - win.requested_height() / 2.,
            );

            let mouse_angle = Vec2::splat(1.).angle_between(centered_cursor_position);
            let (mut position, collider, entity, parent_entity) = collider_query
                .get_single_mut()
                .expect("No collider position");

            position.rotation = Quat::from_rotation_z(mouse_angle);
            println!("New rotation {}", position.rotation);

            let transform = q_parent
                .get(parent_entity.get())
                .expect("No parent transform");

            println!("Parent translation {}", transform.translation);

            // // TODO how to exclude more than 1 collider?
            let filter = QueryFilter::default()
                .exclude_collider(entity)
                .exclude_rigid_body(parent_entity.get());

            // query collider
            rapier_context.intersections_with_shape(
                transform.translation.truncate(),
                mouse_angle,
                &collider,
                filter,
                |entity| {
                    println!("The entity {:?} intersects our shape.", entity);
                    true // Return `false` instead if we want to stop searching for other colliders that contain this point.
                },
            );
        }

        Ok::<(), ErrorMessage>(())
    };

    if let Err(error) = closure() {
        println!("Error while handling click: {}", error);
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
