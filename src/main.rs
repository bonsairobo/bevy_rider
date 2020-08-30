mod camera_dragging;
mod camera_zooming;
mod line_drawing;
mod sled_spawner;

use camera_dragging::camera_dragging_system;
use camera_zooming::camera_zooming_system;
use line_drawing::{line_drawing_system, LineMaterial};
use sled_spawner::{sled_spawner_system, SledMaterial};

use bevy::{prelude::*, render::pass::ClearColor};
use bevy_rapier2d::{
    na::Vector,
    physics::{Gravity, RapierPhysicsPlugin},
};

/// An implementation of the classic game "Line Rider"
fn main() {
    let mut window_desc = WindowDescriptor::default();
    window_desc.width = 1600;
    window_desc.height = 900;

    App::build()
        .add_resource(window_desc)
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_resource(Gravity(Vector::y() * -300.0))
        .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::FIRST, sled_spawner_system.system())
        .add_system_to_stage(stage::FIRST, line_drawing_system.system())
        .add_system(camera_dragging_system.system())
        .add_system(camera_zooming_system.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());

    commands.insert_resource(SledMaterial(
        materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
    ));
    commands.insert_resource(LineMaterial(
        materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
    ));
}
