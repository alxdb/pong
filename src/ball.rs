use std::f32::consts::TAU;

use glium::Display;
use itertools::intersperse;
use nalgebra::{zero, Similarity3, Transform3, Vector2};
use pong::render::{RenderData, Renderable};

pub struct Ball {
    renderdata: RenderData,
    transform: Similarity3<f32>,
}

impl Ball {
    const N_SEGMENTS: usize = 128;
    const SCALE: f32 = 0.4;
    const INITIAL_VELOCITY: Vector2<f32> = Vector2::new(-52.4, 57.3);

    pub fn new(display: &Display) -> Self {
        let center = [0.0, 0.0];
        let radius = 0.5;
        let increment = TAU / Self::N_SEGMENTS as f32;
        let positions = (0..Self::N_SEGMENTS + 1)
            .map(|n| increment * n as f32)
            .map(|theta| [radius * f32::cos(theta), radius * f32::sin(theta)]);
        let positions = intersperse(positions, center);

        Ball {
            renderdata: RenderData::new(display, positions),
            transform: Similarity3::new(zero(), zero(), Self::SCALE),
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
