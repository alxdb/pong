use bevy::prelude::*;

pub struct BallConfig {
    pub color: Color,
    pub radius: f32,
    pub init_velocity: Vec2,
}

pub struct PaddleConfig {
    pub color: Color,
    pub size: (f32, f32),
}

#[derive(Resource)]
pub struct Config {
    pub arena_size: (f32, f32),
    pub ball: BallConfig,
    pub paddle: PaddleConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            arena_size: (900., 600.),
            ball: BallConfig {
                color: Color::WHITE,
                radius: 50.0,
                init_velocity: Vec2::new(1.0, 0.0).normalize() * 400.0,
            },
            paddle: PaddleConfig {
                color: Color::WHITE,
                size: (50., 200.),
            },
        }
    }
}
