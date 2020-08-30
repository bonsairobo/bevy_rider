use crate::screen_to_world::screen_to_world;

use bevy::prelude::*;
use bevy_rapier2d::na;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Default)]
pub struct LineMaterial(pub Handle<ColorMaterial>);

pub struct LineDrawingState {
    cursor_event_reader: EventReader<CursorMoved>,
    cursor_curve: VecDeque<Vec2>,
    camera_entity: Entity,
}

const SEGMENT_LENGTH: f32 = 15.0;

impl LineDrawingState {
    pub fn new(camera_entity: Entity) -> Self {
        LineDrawingState {
            cursor_event_reader: Default::default(),
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
    line_material: Res<LineMaterial>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    transforms: Query<&Transform>,
) {
    let camera_transform = transforms.get::<Transform>(state.camera_entity).unwrap();

    if mouse_button_input.pressed(MouseButton::Left) {
        for event in state.cursor_event_reader.iter(&cursor_moved_events) {
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
        spawn_line_segment(p1, p2, line_material.0, &mut commands);
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
    let x = midpoint.x();
    let y = midpoint.y();

    let local_p1 = na::Point2::new(-length / 2.0, 0.0);
    let local_p2 = na::Point2::new(length / 2.0, 0.0);

    commands
        .spawn(SpriteComponents {
            material,
            sprite: Sprite {
                size: Vec2::new(length, LINE_THICKNESS),
            },
            // HACK: this should be unnecessary, but bevy_rapier has an awkward system ordering that
            // means we have at least one frame before transforms get synchronized
            translation: Translation(Vec3::new(x, y, 0.0)),
            rotation: Rotation::from_rotation_z(angle),
            ..Default::default()
        })
        .with(
            RigidBodyBuilder::new_static()
                .translation(x, y)
                .rotation(angle),
        )
        .with(ColliderBuilder::segment(local_p1, local_p2).friction(0.0));
}
