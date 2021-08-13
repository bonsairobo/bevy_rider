use crate::screen_to_world::screen_to_world;

use bevy::prelude::*;

pub struct CameraDraggingState {
    camera_entity: Entity,
    prev_cursor_pos: Option<Vec2>,
}

impl CameraDraggingState {
    pub fn new(camera_entity: Entity) -> Self {
        CameraDraggingState {
            camera_entity,
            prev_cursor_pos: None,
        }
    }
}

pub fn camera_dragging_system(
    mut state: ResMut<CameraDraggingState>,
    mut cursor_event_reader: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut transforms: Query<&mut Transform>,
) {
    let mut camera_transform = transforms.get_mut(state.camera_entity).unwrap();

    if mouse_button_input.pressed(MouseButton::Right) {
        for event in cursor_event_reader.iter() {
            let cursor_pos = screen_to_world(event.position, &camera_transform, &windows);
            let prev_cursor_pos = screen_to_world(
                state.prev_cursor_pos.unwrap_or(event.position),
                &camera_transform,
                &windows,
            );

            let cursor_pos_delta = cursor_pos - prev_cursor_pos;
            *camera_transform.translation = *(camera_transform.translation - cursor_pos_delta.extend(0.0));

            state.prev_cursor_pos = Some(event.position);
        }
    } else {
        state.prev_cursor_pos = None;
    }
}
