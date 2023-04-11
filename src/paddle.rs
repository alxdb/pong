use std::{rc::Rc, time::Duration};

use glium::{Display, Frame, Program};
use itertools::iproduct;
use nalgebra::Vector2;

use crate::{
    collider::PaddleCollider,
    get_display_ratio,
    renderdata::{RenderData, Transform},
};

pub struct Paddle {
    renderdata: RenderData,
    collider: PaddleCollider,
    // transform: Transform,
    pub state: PaddleState,
}

pub enum PaddleSide {
    Left,
    Right,
}

#[derive(PartialEq, Copy, Clone)]
pub enum PaddleState {
    MoveUp,
    MoveDown,
    DoNothing,
}

impl Paddle {
    const VELOCITY: f32 = 1.0;
    const WIDTH: f32 = 0.15;
    const PADDING: f32 = 0.05;
    const HEIGHT: f32 = 1.0;
    const BOUNDS: f32 = Self::HEIGHT / 2.;

    pub fn new(display: &Display, program: Rc<Program>, side: PaddleSide) -> Self {
        let positions = iproduct!([0.5, -0.5], [0.5, -0.5]).map(|(a, b)| [a, b]);

        let ratio = get_display_ratio(display);
        let transform = Transform {
            translation: match side {
                PaddleSide::Left => Vector2::new(-ratio + Self::WIDTH / 2. + Self::PADDING, 0.),
                PaddleSide::Right => Vector2::new(ratio - Self::WIDTH / 2. - Self::PADDING, 0.),
            },
            scale: Vector2::new(Self::WIDTH, Self::HEIGHT),
        };

        Paddle {
            renderdata: RenderData::new(display, program, positions),
            collider: PaddleCollider::new(Self::VELOCITY, transform, display),
            state: PaddleState::DoNothing,
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        self.collider.update(delta, &self.state)
    }

    pub fn render(&self, frame: &mut Frame) {
        self.renderdata.render(frame, self.collider.transform())
    }
}
