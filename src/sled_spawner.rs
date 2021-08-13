use crate::screen_to_world::screen_to_world;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct SledSpawnerState {
    camera_entity: Entity,
    last_cursor_pos: Option<Vec2>,
}

impl SledSpawnerState {
    pub fn new(camera_entity: Entity) -> Self {
        SledSpawnerState {
            camera_entity,
            last_cursor_pos: None,
        }
    }
}

#[derive(Default)]
pub struct SledMaterial(pub Handle<ColorMaterial>);

pub fn sled_spawner_system(
    commands: Commands,
    mut state: ResMut<SledSpawnerState>,
    mut cursor_event_reader: EventReader<CursorMoved>,
    sled_material: Res<SledMaterial>,
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    transforms: Query<&Transform>,
) {
    let camera_transform = transforms.get(state.camera_entity).unwrap();

    state.last_cursor_pos = cursor_event_reader
        .iter()
        .nth(0)
        .map(|event| screen_to_world(event.position, &camera_transform, &windows))
        .or(state.last_cursor_pos);
    if let Some(cursor_pos) = state.last_cursor_pos {
        if keyboard_input.just_pressed(KeyCode::Space) {
            let sled_size = Vec2::new(50.0, 10.0);
            spawn_sled(cursor_pos, sled_size, sled_material.0.clone(), commands);
        }
    }
}

fn spawn_sled(position: Vec2, size: Vec2, material: Handle<ColorMaterial>, mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            material,
            sprite: Sprite {
                size,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: position.into(),
            forces: RigidBodyForces {
                gravity_scale: 50.0,
                ..Default::default()
            },
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert_bundle(ColliderBundle {
            shape: SharedShape::new(Capsule::new_x(size.x / 2.0, size.y / 2.0)),
            material: ColliderMaterial::new(0.0, 0.0),
            ..Default::default()
        });
}
