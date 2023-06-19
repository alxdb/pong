use std::{rc::Rc, time::Duration};

use cgmath::{vec2, Matrix3};
use glium::{
    index::{NoIndices, PrimitiveType},
    uniform,
    uniforms::EmptyUniforms,
    Display, Frame, Program, Surface, VertexBuffer,
};
use itertools::Itertools;

use crate::shaders::Vertex;

pub struct Paddle {
    vertex_buffer: VertexBuffer<Vertex>,
    transform: Matrix3<f32>,
    program: Rc<Program>,
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
        let positions = [1., -1.]
            .into_iter()
            .cartesian_product([1., -1.])
            .map(|(a, b)| [a, b]);
        let vertices = positions.map(Vertex::new).collect_vec();
        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

        let transform = Matrix3::from_translation(match side {
            PaddleSide::Left => vec2(-1. + Paddle::WIDTH + Paddle::PADDING, 0.),
            PaddleSide::Right => vec2(1. - Paddle::WIDTH - Paddle::PADDING, 0.),
        }) * Matrix3::from_nonuniform_scale(Paddle::WIDTH, Paddle::HEIGHT);

        Paddle {
            vertex_buffer,
            transform,
            program,
            state: PaddleState::DoNothing,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let uniforms: glium::uniforms::UniformsStorage<[[f32; 3]; 3], EmptyUniforms> = uniform! {
            transform: self.transform.into(),
        };

        frame
            .draw(
                &self.vertex_buffer,
                NoIndices(PrimitiveType::TriangleStrip),
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap()
    }

    pub fn update(&mut self, delta: &Duration) {
        let vector = match self.state {
            PaddleState::MoveUp => vec2(0., Self::VELOCITY * delta.as_secs_f32()),
            PaddleState::MoveDown => vec2(0., -Self::VELOCITY * delta.as_secs_f32()),
            PaddleState::DoNothing => vec2(0., 0.),
        };
        self.transform = Matrix3::from_translation(vector) * self.transform;
    }
}
