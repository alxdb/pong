use std::time::Duration;

use nalgebra::Vector2;

use crate::{paddle::PaddleState, transform::Transform};

struct Bounds {
    l: Vector2<f32>,
    h: Vector2<f32>,
}

struct BDelta {
    l: Vector2<f32>,
    h: Vector2<f32>,
}

impl Bounds {
    fn new(x_l: f32, x_h: f32, y_l: f32, y_h: f32) -> Self {
        Self {
            l: Vector2::new(x_l, y_l),
            h: Vector2::new(x_h, y_h),
        }
    }

    // fn from_display(display: &Display) -> Self {
    //     let ratio = get_display_ratio(display);
    //     Self::new(-ratio, ratio, -1., 1.)
    // }

    fn from_transform(transform: &Transform) -> Self {
        Self::new(
            transform.translation.x - transform.scale.x / 2.,
            transform.translation.x + transform.scale.x / 2.,
            transform.translation.y - transform.scale.y / 2.,
            transform.translation.y + transform.scale.y / 2.,
        )
    }

    fn delta_vec(&self, point: Vector2<f32>) -> BDelta {
        BDelta {
            l: point - self.l,
            h: self.h - point,
        }
    }

    fn delta_transform(&self, translation: &Vector2<f32>, scale: &Vector2<f32>) -> BDelta {
        let h_offset = translation + scale / 2.;
        let l_offset = translation - scale / 2.;
        BDelta {
            l: l_offset - self.l,
            h: self.h - h_offset,
        }
    }
}

impl BDelta {
    fn clamp_in(&self) -> Vector2<f32> {
        fn overshoot(l_d: f32, h_d: f32) -> f32 {
            match (l_d, h_d) {
                (l, h) if l >= 0. && h >= 0. => 0.,
                (l, h) if l > 0. && h < 0. => h,
                (l, h) if l < 0. && h > 0. => -l,
                (_, _) => panic!(),
            }
        }

        Vector2::new(overshoot(self.l.x, self.h.x), overshoot(self.l.y, self.h.y))
    }
}

pub struct PaddleCollider {
    transform: Transform,
    velocity: f32,
    bounds: Bounds,
}

impl PaddleCollider {
    pub fn new(velocity: f32, transform: Transform, ratio: f32) -> Self {
        Self {
            transform,
            velocity,
            bounds: Bounds::new(-ratio, ratio, -1., 1.),
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn update(&mut self, delta: &Duration, state: &PaddleState) {
        let velocity = match state {
            PaddleState::MoveUp => self.velocity,
            PaddleState::MoveDown => -self.velocity,
            PaddleState::DoNothing => 0.0,
        };

        let y_maybe = self.transform.translation.y + velocity * delta.as_secs_f32();
        let t_delta = self
            .bounds
            .delta_transform(&Vector2::new(0.0, y_maybe), &self.transform.scale)
            .clamp_in();
        self.transform.translation.y = y_maybe + t_delta.y;
    }
}

enum Hit {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct BallCollider {
    transform: Transform,
    velocity: Vector2<f32>,
    bounds: Bounds,
}

impl BallCollider {
    pub fn new(velocity: Vector2<f32>, transform: Transform, ratio: f32) -> Self {
        Self {
            transform,
            velocity,
            bounds: Bounds::new(-ratio, ratio, -1., 1.),
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn update(&mut self, delta: &Duration) {
        let t_maybe = self.transform.translation + self.velocity * delta.as_secs_f32();
        let b_delta = self.bounds.delta_transform(&t_maybe, &self.transform.scale);
        let t_delta = b_delta.clamp_in() * 2.;
        self.transform.translation = t_maybe + t_delta;

        let v_delta = t_delta.map(|x| if x.abs() > 0. { -1. } else { 1. });
        self.velocity.component_mul_assign(&v_delta);
    }
}
