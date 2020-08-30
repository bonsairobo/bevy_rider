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
    let x = -200.0;
    let y = 200.0;

    commands
        .spawn(SpriteComponents {
            material,
            sprite: Sprite { size },
            // HACK: this should be unnecessary, but bevy_rapier has an awkward system ordering that
            // means we have at least one frame before transforms get synchronized
            translation: Translation(Vec3::new(x, y, 0.0)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic().translation(x, y))
        .with(ColliderBuilder::cuboid(size.x() / 2.0, size.y() / 2.0).friction(0.0));
}
