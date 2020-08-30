use bevy::prelude::*;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

#[derive(Default)]
pub struct SledMaterial(pub Handle<ColorMaterial>);

pub fn sled_spawner_system(
    mut commands: Commands,
    sled_material: Res<SledMaterial>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let sled_size = Vec2::new(50.0, 10.0);
        spawn_sled(sled_size, sled_material.0, &mut commands);
    }
}

fn spawn_sled(size: Vec2, material: Handle<ColorMaterial>, commands: &mut Commands) {
    commands
        .spawn(SpriteComponents {
            material,
            sprite: Sprite { size },
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic().translation(-200.0, 200.0))
        .with(ColliderBuilder::cuboid(size.x() / 2.0, size.y() / 2.0).friction(0.0));
}