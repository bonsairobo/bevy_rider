use crate::screen_to_world::screen_to_world;

use bevy::prelude::*;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

pub struct SledSpawnerState {
    cursor_event_reader: EventReader<CursorMoved>,
    camera_entity: Entity,
    last_cursor_pos: Option<Vec2>,
}

impl SledSpawnerState {
    pub fn new(camera_entity: Entity) -> Self {
        SledSpawnerState {
            cursor_event_reader: Default::default(),
            camera_entity,
            last_cursor_pos: None,
        }
    }
}

#[derive(Default)]
pub struct SledMaterial(pub Handle<ColorMaterial>);

pub fn sled_spawner_system(
    mut commands: Commands,
    mut state: ResMut<SledSpawnerState>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    sled_material: Res<SledMaterial>,
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    transforms: Query<&Transform>,
) {
    let camera_transform = transforms.get::<Transform>(state.camera_entity).unwrap();

    state.last_cursor_pos = state
        .cursor_event_reader
        .latest(&cursor_moved_events)
        .map(|event| screen_to_world(event.position, &camera_transform, &windows))
        .or(state.last_cursor_pos);
    if let Some(cursor_pos) = state.last_cursor_pos {
        if keyboard_input.just_pressed(KeyCode::Space) {
            let sled_size = Vec2::new(50.0, 10.0);
            spawn_sled(cursor_pos, sled_size, sled_material.0, &mut commands);
        }
    }
}

fn spawn_sled(
    position: Vec2,
    size: Vec2,
    material: Handle<ColorMaterial>,
    commands: &mut Commands,
) {
    commands
        .spawn(SpriteComponents {
            material,
            sprite: Sprite { size },
            // HACK: this should be unnecessary, but bevy_rapier has an awkward system ordering that
            // means we have at least one frame before transforms get synchronized
            translation: Translation(position.extend(0.0)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic().translation(position.x(), position.y()))
        .with(ColliderBuilder::capsule_x(size.x() / 2.0, size.y() / 2.0).friction(0.0));
}
