use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::Camera};

const ZOOM_SENSITIVITY: f32 = 0.1;

pub fn camera_zooming_system(
    mut mouse_wheel_event_reader: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let mut zoom_scalar = 1.0;
    for mouse_wheel_event in mouse_wheel_event_reader.iter() {
        zoom_scalar *= 1.0 - ZOOM_SENSITIVITY * mouse_wheel_event.y;
    }

    for (_, mut transform) in query.iter_mut() {
        // BUG: for some reason, when camera scale < 1.0, things just disappear!
        let zoomed = transform.scale * zoom_scalar;
        let limited = Vec3::new(zoomed.x.max(1.0), zoomed.y.max(1.0), zoomed.z.max(1.0));
        *transform.scale = *limited;
    }
}
