use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::Camera};

#[derive(Default)]
pub struct CameraZoomingState {
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

const ZOOM_SENSITIVITY: f32 = 0.1;

// BUG: for some reason, when camera scale < 1.0, things just disappear!

pub fn camera_zooming_system(
    mut state: Local<CameraZoomingState>,
    mouse_wheel: Res<Events<MouseWheel>>,
    mut query: Query<(&Camera, &mut Scale)>,
) {
    let mut zoom_scalar = 1.0;
    for mouse_wheel_event in state.mouse_wheel_event_reader.iter(&mouse_wheel) {
        zoom_scalar *= 1.0 - ZOOM_SENSITIVITY * mouse_wheel_event.y;
    }

    for (_, mut scale) in &mut query.iter() {
        *scale = Scale(scale.0 * zoom_scalar);
    }
}
