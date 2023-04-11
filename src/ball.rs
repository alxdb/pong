use std::{f32::consts::TAU, rc::Rc, time::Duration};

use glium::{Display, Frame, Program};
use itertools::intersperse;
use nalgebra::{zero, Vector2};

use crate::{
    collider::BallCollider, get_display_ratio, paddle::Paddle, renderdata::RenderData,
    transform::Transform,
};

pub struct Ball {
    renderdata: RenderData,
    collider: BallCollider,
}

impl Ball {
    const N_SEGMENTS: usize = 128;
    const SCALE: f32 = 0.4;
    const INITIAL_VELOCITY: Vector2<f32> = Vector2::new(-52.4, 57.3);

    pub fn new(display: &Display, program: Rc<Program>) -> Self {
        let center = [0.0, 0.0];
        let radius = 0.5;
        let increment = TAU / Self::N_SEGMENTS as f32;
        let positions = (0..Self::N_SEGMENTS + 1)
            .map(|n| increment * n as f32)
            .map(|theta| [radius * f32::cos(theta), radius * f32::sin(theta)]);
        let positions = intersperse(positions, center);

        let scale = Vector2::from_element(Self::SCALE);
        let ratio = get_display_ratio(display);
        let transform = Transform {
            translation: zero(),
            scale,
        };

        Ball {
            renderdata: RenderData::new(display, program, positions),
            collider: BallCollider::new(Self::INITIAL_VELOCITY, transform, ratio),
        }
    }

    pub fn update(&mut self, delta: &Duration, paddles: &[&Paddle]) {
        self.collider.update(delta);
    }

    pub fn render(&self, frame: &mut Frame) {
        self.renderdata.render(frame, self.collider.transform())
    }
}
