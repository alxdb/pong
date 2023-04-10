use std::{f32::consts::TAU, rc::Rc, time::Duration};

use cgmath::{num_traits::zero, vec2, Matrix2, Vector2};
use glium::{Display, Frame, Program};
use itertools::intersperse;

use crate::{
    collider::Collider,
    get_display_ratio,
    renderdata::{RenderData, Transform},
};

pub struct Ball {
    renderdata: RenderData,
    collider: Collider,
}

impl Ball {
    const N_SEGMENTS: usize = 128;
    const SCALE: f32 = 0.4;
    const INITIAL_VELOCITY: Vector2<f32> = vec2(10., 10.);

    pub fn new(display: &Display, program: Rc<Program>) -> Self {
        let center = [0.0, 0.0];
        let radius = 0.5;
        let increment = TAU / Self::N_SEGMENTS as f32;
        let positions = (0..Self::N_SEGMENTS + 1)
            .map(|n| increment * n as f32)
            .map(|theta| [radius * f32::cos(theta), radius * f32::sin(theta)]);
        let positions = intersperse(positions, center);

        let scale = vec2(Self::SCALE, Self::SCALE);
        let ratio = get_display_ratio(display);

        Ball {
            renderdata: RenderData::new(display, program, positions),
            collider: Collider::new(
                Transform {
                    translation: zero(),
                    scale,
                },
                Self::INITIAL_VELOCITY,
                Matrix2::from_cols(vec2(-ratio, ratio), vec2(-1., 1.)),
            ),
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        self.collider.update(delta);
    }

    pub fn render(&self, frame: &mut Frame) {
        self.renderdata.render(frame, self.collider.transform())
    }
}
