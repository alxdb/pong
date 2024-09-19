use bevy::prelude::*;

mod config;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::BasePlugin)
        .run();
}

mod plugins {
    pub use base::BasePlugin;

    pub(crate) mod base {
        use bevy::{
            prelude::*,
            sprite::{MaterialMesh2dBundle, Mesh2dHandle},
        };

        use crate::config::Config;

        #[derive(Component, Deref, DerefMut)]
        struct Velocity(Vec2);

        #[derive(Component)]
        struct Collider;

        #[derive(Component)]
        struct Ball;

        #[derive(Bundle)]
        struct BallBundle {
            ball: Ball,
            velocity: Velocity,
            material_mesh_2d: MaterialMesh2dBundle<ColorMaterial>,
        }

        impl BallBundle {
            fn new(
                config: &Config,
                meshes: &mut ResMut<Assets<Mesh>>,
                materials: &mut ResMut<Assets<ColorMaterial>>,
            ) -> Self {
                BallBundle {
                    ball: Ball,
                    velocity: Velocity(config.ball.init_velocity),
                    material_mesh_2d: MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Circle {
                            radius: config.ball.radius,
                        })),
                        material: materials.add(config.ball.color),
                        ..default()
                    },
                }
            }
        }

        #[derive(Component)]
        struct Paddle;

        #[derive(Bundle)]
        struct PaddleBundle {
            paddle: Paddle,
            sprite: SpriteBundle,
        }

        pub struct BasePlugin;

        impl Plugin for BasePlugin {
            fn build(&self, app: &mut App) {
                app.insert_resource(Config::default())
                    .add_systems(Startup, setup)
                    .add_systems(FixedUpdate, apply_velocity);
            }
        }

        fn setup(
            config: Res<Config>,
            mut commands: Commands,
            mut meshes: ResMut<Assets<Mesh>>,
            mut materials: ResMut<Assets<ColorMaterial>>,
        ) {
            commands.spawn(Camera2dBundle::default());
            commands.spawn(BallBundle::new(&config, &mut meshes, &mut materials));
        }

        fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
            for (mut transform, velocity) in &mut query {
                transform.translation.x += velocity.x * time.delta_seconds();
                transform.translation.y += velocity.y * time.delta_seconds();
            }
        }
    }
}
