use crate::screen_to_world::screen_to_world;

use bevy::prelude::*;

pub struct CameraDraggingState {
    cursor_event_reader: EventReader<CursorMoved>,
    camera_entity: Entity,
    prev_cursor_pos: Option<Vec2>,
}

impl CameraDraggingState {
    pub fn new(camera_entity: Entity) -> Self {
        CameraDraggingState {
            cursor_event_reader: Default::default(),
            camera_entity,
            prev_cursor_pos: None,
        }
    }
}

pub fn camera_dragging_system(
    mut state: ResMut<CameraDraggingState>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    transforms: Query<&Transform>,
    translations: Query<&mut Translation>,
) {
    let camera_transform = transforms.get::<Transform>(state.camera_entity).unwrap();
    let mut camera_translation = translations
        .get_mut::<Translation>(state.camera_entity)
        .unwrap();

    if mouse_button_input.pressed(MouseButton::Right) {
        for event in state.cursor_event_reader.iter(&cursor_moved_events) {
            let cursor_pos = screen_to_world(event.position, &camera_transform, &windows);
            let prev_cursor_pos = screen_to_world(
                state.prev_cursor_pos.unwrap_or(event.position),
                &camera_transform,
                &windows,
            );

            let cursor_pos_delta = cursor_pos - prev_cursor_pos;
            *camera_translation = Translation(camera_translation.0 - cursor_pos_delta.extend(0.0));

            state.prev_cursor_pos = Some(event.position);
        }
    } else {
        state.prev_cursor_pos = None;
    }
}
