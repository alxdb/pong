use bevy::{prelude::*, window::WindowMode};

mod config;
use config::Config;

fn main() {
    let config = Config::default();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".into(),
                name: Some("Bevy Pong".into()),
                resolution: config.arena_size.into(),
                resizable: false,
                mode: WindowMode::Fullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(plugins::BasePlugin)
        .insert_resource(config)
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
                meshes: &mut Assets<Mesh>,
                materials: &mut Assets<ColorMaterial>,
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

        #[derive(Copy, Clone)]
        enum PaddleSide {
            Left,
            Right,
        }

        #[derive(Component)]
        struct Paddle(PaddleSide);

        #[derive(Bundle)]
        struct PaddleBundle {
            paddle: Paddle,
            collider: Collider,
            sprite: SpriteBundle,
        }

        impl PaddleBundle {
            fn new(config: &Config, side: PaddleSide) -> Self {
                let distance_from_center = (config.arena_size.0 / 2.) - config.paddle.size.0;

                PaddleBundle {
                    paddle: Paddle(side),
                    collider: Collider,
                    sprite: SpriteBundle {
                        sprite: Sprite {
                            color: config.paddle.color,
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                match side {
                                    PaddleSide::Left => -distance_from_center,
                                    PaddleSide::Right => distance_from_center,
                                },
                                0.,
                                0.,
                            ),
                            scale: Vec3::new(config.paddle.size.0, config.paddle.size.1, 0.),
                            ..default()
                        },
                        ..default()
                    },
                }
            }
        }

        pub struct BasePlugin;

        impl Plugin for BasePlugin {
            fn build(&self, app: &mut App) {
                app.add_systems(Startup, setup)
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
            for side in [PaddleSide::Left, PaddleSide::Right] {
                commands.spawn(PaddleBundle::new(&config, side));
            }
        }

        fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
            for (mut transform, velocity) in &mut query {
                transform.translation.x += velocity.x * time.delta_seconds();
                transform.translation.y += velocity.y * time.delta_seconds();
            }
        }
    }
}
