use std::time::Duration;

use cgmath::{vec2, ElementWise, Matrix, Matrix2, Vector2};

use crate::renderdata::Transform;

pub struct Collider {
    transform: Transform,
    velocity: Vector2<f32>,
    bounds: Matrix2<f32>,
}

impl Collider {
    pub fn new(transform: Transform, velocity: Vector2<f32>, bounds: Matrix2<f32>) -> Self {
        Self {
            transform,
            velocity,
            bounds: Matrix2::from_cols(
                vec2(
                    bounds.x.x + transform.scale.x / 2.,
                    bounds.x.y - transform.scale.x / 2.,
                ),
                vec2(
                    bounds.y.x + transform.scale.y / 2.,
                    bounds.y.y - transform.scale.y / 2.,
                ),
            ),
        }
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }

    pub fn update(&mut self, delta: &Duration) {
        let t_maybe = self.transform.translation + self.velocity * delta.as_secs_f32();

        let l_delta = self.bounds.row(0) - t_maybe;
        let h_delta = t_maybe - self.bounds.row(1);
        let t_delta = Vector2::zip(l_delta, h_delta, f32::max).map(|x| x.max(0.));

        let v_delta = t_delta.map(|x| if x > 0. { -1. } else { 1. });

        let clamp_t = vec2(
            t_maybe.x.clamp(self.bounds.x.x, self.bounds.x.y),
            t_maybe.y.clamp(self.bounds.y.x, self.bounds.y.y),
        );
        self.transform.translation = clamp_t - t_delta;
        self.velocity = self.velocity.mul_element_wise(v_delta);
    }
}
