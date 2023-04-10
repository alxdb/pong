use std::time::Duration;

use cgmath::{vec2, ElementWise, Matrix2, Vector2};

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
        let new_t = self.transform.translation + self.velocity * delta.as_secs_f32();

        let t_delta = vec2(
            f32::max(self.bounds.x.x - new_t.x, new_t.x - self.bounds.x.y).max(0.),
            f32::max(self.bounds.y.x - new_t.y, new_t.y - self.bounds.y.y).max(0.),
        );
        let v_delta = vec2(
            if t_delta.x > 0. { -1. } else { 1. },
            if t_delta.y > 0. { -1. } else { 1. },
        );

        let clamp_t = vec2(
            new_t.x.clamp(self.bounds.x.x, self.bounds.x.y),
            new_t.y.clamp(self.bounds.y.x, self.bounds.y.y),
        );

        self.transform.translation = clamp_t - t_delta;
        self.velocity = self.velocity.mul_element_wise(v_delta);
    }
}
