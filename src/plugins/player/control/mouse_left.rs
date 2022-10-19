fn mouse_left_click_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
    mut collider_query: Query<
        (&mut Transform, &Collider, Entity, &Parent),
        (
            With<Collider>,
            With<PlayerSwordRangeSensor>,
            Without<PlayerSwordRange>,
        ),
    >,
    mut sprite_range_query: Query<
        &mut Transform,
        (With<PlayerSwordRange>, Without<PlayerSwordRangeSensor>),
    >,
    q_parent: Query<&Transform, (Without<PlayerSwordRangeSensor>, Without<PlayerSwordRange>)>,
    mut ev_monster_hit: EventWriter<HitMonsterEvent>,
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

            let mut sprite_transform = sprite_range_query
                .get_single_mut()
                .expect("No sprite transform");
            sprite_transform.rotation = Quat::from_rotation_z(mouse_angle);

            let transform = q_parent
                .get(parent_entity.get())
                .expect("No parent transform");

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
                    ev_monster_hit.send(HitMonsterEvent(entity, mouse_angle));
                    // println!("The entity {:?} intersects our shape.", entity);
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
