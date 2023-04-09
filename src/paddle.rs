use std::{rc::Rc, time::Duration};

use cgmath::{vec3, Matrix4};
use glium::{Display, Frame, Program, VertexBuffer};
use itertools::{iproduct, Itertools};

use crate::{shaders::Vertex, RenderData};

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
    const WIDTH: f32 = 0.05;
    const PADDING: f32 = 0.025;
    const HEIGHT: f32 = 0.4;

    pub fn new(display: &Display, program: Rc<Program>, side: PaddleSide) -> Self {
        let positions = iproduct!([1., -1.], [1., -1.]);

        let vertices = positions
            .map(|(a, b)| [a, b, 0., 1.])
            .map(Vertex::new)
            .collect_vec();
        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

        let transform = Matrix4::from_translation(match side {
            PaddleSide::Left => vec3(-1. + Self::WIDTH + Self::PADDING, 0., 0.),
            PaddleSide::Right => vec3(1. - Self::WIDTH - Self::PADDING, 0., 0.),
        }) * Matrix4::from_nonuniform_scale(Self::WIDTH, Self::HEIGHT, 1.);

        Paddle {
            renderdata: RenderData {
                vertex_buffer,
                transform,
                program,
            },
            state: PaddleState::DoNothing,
        }
    }

    pub fn update(&mut self, delta: &Duration) {
        let vector = match self.state {
            PaddleState::MoveUp => vec3(0., Self::VELOCITY * delta.as_secs_f32(), 0.),
            PaddleState::MoveDown => vec3(0., -Self::VELOCITY * delta.as_secs_f32(), 0.),
            PaddleState::DoNothing => vec3(0., 0., 0.),
        };
        self.renderdata.transform = Matrix4::from_translation(vector) * self.renderdata.transform;
    }

    pub fn render(&self, frame: &mut Frame) {
        self.renderdata.render(frame)
    }
}
