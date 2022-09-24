use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};

use super::{location::Location, population::Player};

pub fn handle_mouse_click(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    mut player_query: Query<&mut Location, With<Player>>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let camera_transform = camera_query
        .get_single()
        .expect("No camera transform")
        .translation
        .truncate();

    for event in mouse_button_input_events.iter() {
        if event.state == ButtonState::Pressed {
            let win = windows.get_primary().expect("no primary window");

            // Change destination one player clicks somewhere on the map
            if let (Ok(mut location), Some(cursor_position)) =
                (player_query.get_single_mut(), win.cursor_position())
            {
                println!("Cursor {:?}", cursor_position);
                location.destination = Some(
                    cursor_position
                        - Vec2::new(win.requested_width() / 2., win.requested_height() / 2.)
                        + camera_transform,
                );
            }
        }
    }
}
