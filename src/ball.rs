use std::{f32::consts::TAU, time::Duration};

use glium::Display;
use itertools::intersperse;
use nalgebra::{point, vector, zero, Orthographic3, Similarity3, Transform3, Vector2};
use pong::{
    collide,
    render::{RenderData, Renderable},
};

pub struct Ball {
    renderdata: RenderData,
    transform: Similarity3<f32>,
    velocity: Vector2<f32>,
}

impl Ball {
    const N_SEGMENTS: usize = 64;
    const SCALE: f32 = 0.2;
    const INITIAL_VELOCITY: Vector2<f32> = vector![0., 2.];

    pub fn new(display: &Display) -> Self {
        let center = [0.0, 0.0];
        let radius = 1.0;
        let increment = TAU / Self::N_SEGMENTS as f32;
        let positions = (0..Self::N_SEGMENTS + 1)
            .map(|n| increment * n as f32)
            .map(|theta| [radius * f32::cos(theta), radius * f32::sin(theta)]);
        let positions = intersperse(positions, center);

        Ball {
            renderdata: RenderData::new(display, positions),
            transform: Similarity3::new(zero(), zero(), Self::SCALE),
            velocity: Self::INITIAL_VELOCITY,
        }
    }

    pub fn update(&mut self, delta: &Duration, projection: &Orthographic3<f32>) {
        let position = point![
            self.transform.isometry.translation.vector.x,
            self.transform.isometry.translation.vector.y
        ];

        let proposed = position + self.velocity * delta.as_secs_f32();
        let adjustment =
            collide::Walls::from(projection).collide(&collide::Circle::new(proposed, Self::SCALE));

        // stupid floating point math
        if adjustment.norm() > 1.0e-6 {
            let adjusted = proposed + adjustment;
            self.transform.isometry.translation.vector.x = adjusted.x;
            self.transform.isometry.translation.vector.y = adjusted.y;
            // This doesn't work
            self.velocity = adjustment.normalize() * Self::INITIAL_VELOCITY.norm();
        } else {
            println!("No-op");
            self.transform.isometry.translation.vector.x = proposed.x;
            self.transform.isometry.translation.vector.y = proposed.y;
        }
    }
}

impl Renderable for Ball {
    fn transform(&self) -> Transform3<f32> {
        nalgebra::convert(self.transform)
    }

    fn renderdata(&self) -> &RenderData {
        &self.renderdata
    }
}
