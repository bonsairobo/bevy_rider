use crate::screen_to_world::screen_to_world;

use bevy::prelude::*;
use bevy_rapier2d::{
    na,
    physics::{ColliderBundle, RigidBodyBundle},
    prelude::*,
};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Default)]
pub struct LineMaterial(pub Handle<ColorMaterial>);

pub struct LineDrawingState {
    cursor_curve: VecDeque<Vec2>,
    camera_entity: Entity,
}

const SEGMENT_LENGTH: f32 = 15.0;

impl LineDrawingState {
    pub fn new(camera_entity: Entity) -> Self {
        LineDrawingState {
            cursor_curve: Default::default(),
            camera_entity,
        }
    }

    fn pop_line_segments(&mut self) -> Vec<(Vec2, Vec2)> {
        // Downsample the cursor curve by length.
        let mut line_segments = Vec::new();
        let mut segment_start = if let Some(back) = self.cursor_curve.back() {
            *back
        } else {
            return line_segments;
        };
        let mut curve_length = 0.0;
        let mut segment_points = 0;
        let mut confirmed_segment_points = 0;
        for (p1, p2) in self.cursor_curve.iter().rev().tuple_windows() {
            segment_points += 1;

            let diff = *p2 - *p1;
            curve_length += diff.length();
            if curve_length >= SEGMENT_LENGTH {
                if segment_start != *p2 {
                    line_segments.push((segment_start, *p2));
                }
                segment_start = *p2;
                confirmed_segment_points += segment_points;
                curve_length = 0.0;
                segment_points = 0;
            }
        }

        // Remove the points belonging to the segments we've gathered.
        self.cursor_curve
            .truncate(self.cursor_curve.len() - confirmed_segment_points);

        line_segments
    }
}

pub fn line_drawing_system(
    mut commands: Commands,
    mut state: ResMut<LineDrawingState>,
    mut cursor_event_reader: EventReader<CursorMoved>,
    line_material: Res<LineMaterial>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    transforms: Query<&Transform>,
) {
    let camera_transform = transforms.get(state.camera_entity).unwrap();

    if mouse_button_input.pressed(MouseButton::Left) {
        for event in cursor_event_reader.iter() {
            state.cursor_curve.push_front(screen_to_world(
                event.position,
                &camera_transform,
                &windows,
            ));
        }
    } else {
        state.cursor_curve.clear();
    }

    let new_line_segments = state.pop_line_segments();
    for (p1, p2) in new_line_segments.into_iter() {
        spawn_line_segment(p1, p2, line_material.0.clone(), &mut commands);
    }
}

const LINE_THICKNESS: f32 = 3.0;

fn spawn_line_segment(
    p1: Vec2,
    p2: Vec2,
    material: Handle<ColorMaterial>,
    commands: &mut Commands,
) {
    let midpoint = (p1 + p2) / 2.0;
    let diff = p2 - p1;
    let length = diff.length();
    let angle = Vec2::new(1.0, 0.0).angle_between(diff);
    let x = midpoint.x;
    let y = midpoint.y;

    let local_p1 = na::Point2::new(-length / 2.0, 0.0f32);
    let local_p2 = na::Point2::new(length / 2.0, 0.0f32);

    commands
        .spawn_bundle(SpriteBundle {
            material,
            sprite: Sprite {
                size: Vec2::new(length, LINE_THICKNESS),
                flip_x: false,
                flip_y: false,
                resize_mode: SpriteResizeMode::Manual,
            },
            // HACK: this should be unnecessary, but bevy_rapier has an awkward system ordering that
            // means we have at least one frame before transforms get synchronized
            transform: Transform {
                translation: Vec3::new(x, y, 0.0),
                rotation: Quat::from_axis_angle(Vec3::new(1.0, 1.0, 1.0), angle),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: (Vec2::new(x, y), angle).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: SharedShape::new(Capsule::new(local_p1, local_p2, 0.0)),
            material: ColliderMaterial {
                friction: 0.0,
                ..Default::default()
            },
            ..Default::default()
        });
}
