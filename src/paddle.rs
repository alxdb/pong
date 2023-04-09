use std::{rc::Rc, time::Duration};

use cgmath::vec3;
use glium::{Display, Frame, Program};
use itertools::iproduct;

use crate::{get_display_ratio, RenderData, Transform};

pub struct Paddle {
    renderdata: RenderData,
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

    pub fn new(display: &Display, program: Rc<Program>, side: PaddleSide) -> Self {
        let positions = iproduct!([0.5, -0.5], [0.5, -0.5]).map(|(a, b)| [a, b]);

        let ratio = get_display_ratio(display);
        let transform = Transform {
            translation: match side {
                PaddleSide::Left => vec3(-ratio + (Self::WIDTH / 2.) + Self::PADDING, 0., 0.),
                PaddleSide::Right => vec3(ratio - (Self::WIDTH / 2.) - Self::PADDING, 0., 0.),
            },
            scale: vec3(Self::WIDTH, Self::HEIGHT, 1.),
        };

        Paddle {
            renderdata: RenderData::new(display, program, positions, transform),
            state: PaddleState::DoNothing,
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        let velocity = match self.state {
            PaddleState::MoveUp => vec3(0., Self::VELOCITY * delta.as_secs_f32(), 0.),
            PaddleState::MoveDown => vec3(0., -Self::VELOCITY * delta.as_secs_f32(), 0.),
            PaddleState::DoNothing => vec3(0., 0., 0.),
        };
        self.renderdata.transform.translation += velocity;
    }

    pub fn render(&self, frame: &mut Frame) {
        self.renderdata.render(frame)
    }
}
