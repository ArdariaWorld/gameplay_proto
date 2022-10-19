fn mouse_right_click_system(
    windows: Res<Windows>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut ev_fire_projectile: EventWriter<FireProjectileEvent>,
) {
    for event in mouse_button_input_events.iter() {
        // If not event Pressed we do nothing
        if event.state == ButtonState::Pressed && event.button == MouseButton::Right {
            let win = windows
                .get_primary()
                .ok_or(ErrorMessage::NoWindow)
                .expect("No window");

            // get angle from click position
            // Should never happen as cursor_position should always exists when windows is clicked
            let cursor_position = win.cursor_position().expect("No cursor position");

            // Correct the mouse position with windows size (0,0 at center)
            let centered_cursor_position = Vec2::new(
                cursor_position.x - win.requested_width() / 2.,
                cursor_position.y - win.requested_height() / 2.,
            );

            let mouse_angle = Vec2::new(1., 0.).angle_between(centered_cursor_position);

            ev_fire_projectile.send(FireProjectileEvent(mouse_angle));
        };
    }
}
