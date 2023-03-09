use std::{rc::Rc, time::Duration};

use cgmath::{vec3, Matrix4};
use glium::{
    index::{NoIndices, PrimitiveType},
    uniform,
    uniforms::EmptyUniforms,
    Display, Frame, Program, Surface, VertexBuffer,
};
use itertools::{iproduct, Itertools};

use crate::shaders::Vertex;

pub struct Paddle {
    vertex_buffer: VertexBuffer<Vertex>,
    transform: Matrix4<f32>,
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
        let positions = iproduct!([1., -1.], [1., -1.]).map(|(a, b)| [a, b, 0., 1.]);
        let vertices = positions.map(Vertex::new).collect_vec();
        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

        let transform = Matrix4::from_translation(match side {
            PaddleSide::Left => vec3(-1. + Self::WIDTH + Self::PADDING, 0., 0.),
            PaddleSide::Right => vec3(1. - Self::WIDTH - Self::PADDING, 0., 0.),
        }) * Matrix4::from_nonuniform_scale(Self::WIDTH, Self::HEIGHT, 1.);

        Paddle {
            vertex_buffer,
            transform,
            program,
            state: PaddleState::DoNothing,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let uniforms: glium::uniforms::UniformsStorage<[[f32; 4]; 4], EmptyUniforms> = uniform! {
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
            PaddleState::MoveUp => vec3(0., Self::VELOCITY * delta.as_secs_f32(), 0.),
            PaddleState::MoveDown => vec3(0., -Self::VELOCITY * delta.as_secs_f32(), 0.),
            PaddleState::DoNothing => vec3(0., 0., 0.),
        };
        self.transform = Matrix4::from_translation(vector) * self.transform;
    }
}
