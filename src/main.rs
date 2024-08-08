use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::ShapesPlugin)
        .run();
}

mod plugins {
    pub use shapes::ShapesPlugin;

    pub(crate) mod shapes {
        use bevy::{
            prelude::*,
            sprite::{MaterialMesh2dBundle, Mesh2dHandle},
        };

        pub struct ShapesPlugin;

        impl Plugin for ShapesPlugin {
            fn build(&self, app: &mut App) {
                app.add_systems(Startup, add_shapes);
            }
        }

        fn add_shapes(
            mut commands: Commands,
            mut meshes: ResMut<Assets<Mesh>>,
            mut materials: ResMut<Assets<ColorMaterial>>,
        ) {
            commands.spawn(Camera2dBundle::default());
            commands.spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
                material: materials.add(Color::WHITE),
                ..default()
            });
        }
    }
}
