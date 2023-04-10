use std::{f32::consts::TAU, rc::Rc, time::Duration};

use glium::{Display, Frame, Program};
use itertools::intersperse;
use nalgebra::{zero, Matrix2, Vector2};

use crate::{
    get_display_ratio,
    renderdata::{RenderData, Transform},
};

pub struct Ball {
    renderdata: RenderData,
    transform: Transform,
    velocity: Vector2<f32>,
    bounds: Matrix2<f32>,
}

impl Ball {
    const N_SEGMENTS: usize = 128;
    const SCALE: f32 = 0.4;
    const INITIAL_VELOCITY: Vector2<f32> = Vector2::new(2., 1.4);

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
        let bounds = Matrix2::from_columns(&[
            Vector2::new(-ratio, -1.) + scale / 2.,
            Vector2::new(ratio, 1.) - scale / 2.,
        ]);

        Ball {
            renderdata: RenderData::new(display, program, positions),
            transform: Transform {
                translation: zero(),
                scale,
            },
            velocity: Self::INITIAL_VELOCITY,
            bounds,
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        let t_maybe = self.transform.translation + self.velocity * delta.as_secs_f32();

        let t_delta =
            t_maybe.zip_zip_map(&self.bounds.column(0), &self.bounds.column(1), |t, l, h| {
                f32::max(l - t, t - h).max(0.)
            });
        let t_clamp =
            t_maybe.zip_zip_map(&self.bounds.column(0), &self.bounds.column(1), |t, l, h| {
                nalgebra::clamp(t, l, h)
            });
        self.transform.translation = t_clamp - t_delta;

        let v_delta = t_delta.map(|x| if x > 0. { -1. } else { 1. });
        self.velocity.component_mul_assign(&v_delta);
    }

    pub fn render(&self, frame: &mut Frame) {
        self.renderdata.render(frame, self.transform)
    }
}
