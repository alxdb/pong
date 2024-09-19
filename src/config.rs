use bevy::prelude::*;

pub struct BallConfig {
    pub color: Color,
    pub radius: f32,
    pub init_velocity: Vec2,
}

#[derive(Resource)]
pub struct Config {
    pub ball: BallConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ball: BallConfig {
                color: Color::WHITE,
                radius: 50.0,
                init_velocity: Vec2::new(1.0, 0.0).normalize() * 400.0,
            },
        }
    }
}
