use bevy::prelude::*;

pub fn screen_to_world(p: Vec2, camera_transform: &Transform, windows: &Windows) -> Vec2 {
    let w = windows.get_primary().unwrap();
    let resolution = Vec2::new(w.width as f32, w.height as f32);
    let p_ndc = p - resolution / 2.0;
    let p_world = camera_transform.value * p_ndc.extend(0.0).extend(1.0);

    p_world.truncate().truncate()
}
