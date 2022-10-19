use bevy::{
    prelude::{EventReader, Query},
    window::CursorMoved,
};
use bevy_mod_raycast::{Intersection, RayCastMethod, RayCastSource};

/// This is a unit struct we will use to mark our generic `RayCastMesh`s and `RayCastSource` as part
/// of the same group, or "RayCastSet". For more complex use cases, you might use this to associate
/// some meshes with one ray casting source, and other meshes with a different ray casting source."
pub struct MouseRaycastSet;

// Update our `RayCastSource` with the current cursor position every frame.
pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<MouseRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
    }
}
