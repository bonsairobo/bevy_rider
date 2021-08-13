mod camera_dragging;
mod camera_zooming;
mod line_drawing;
mod screen_to_world;
mod sled_spawner;

use camera_dragging::{camera_dragging_system, CameraDraggingState};
use camera_zooming::camera_zooming_system;
use line_drawing::{line_drawing_system, LineDrawingState, LineMaterial};
use sled_spawner::{sled_spawner_system, SledMaterial, SledSpawnerState};

use bevy::{prelude::*, render::pass::ClearColor};
use bevy_rapier2d::{na::Vector, physics::{NoUserData, RapierConfiguration, RapierPhysicsPlugin}};

/// An implementation of the classic game "Line Rider"
fn main() {
    let mut window_desc = WindowDescriptor::default();
    window_desc.width = 1600.0;
    window_desc.height = 900.0;
    window_desc.title = "Bevy Rider".to_string();

    App::build()
        .insert_resource(window_desc)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // TODO: apparently I shouldn't just scale the gravity to compensate for using pixel
        // coordinates, as it can cause inaccuracies in the physics simulation
        .insert_resource(RapierConfiguration {
            gravity: Vector::y() * -300.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_startup_system(setup.system())
        .add_system(sled_spawner_system.system())
        .add_system(line_drawing_system.system())
        .add_system(camera_dragging_system.system())
        .add_system(camera_zooming_system.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let camera_entity = commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .id();

    commands.insert_resource(CameraDraggingState::new(camera_entity));
    commands.insert_resource(LineDrawingState::new(camera_entity));
    commands.insert_resource(SledSpawnerState::new(camera_entity));

    commands.insert_resource(SledMaterial(
        materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
    ));
    commands.insert_resource(LineMaterial(
        materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
    ));
}
