use std::rc::Rc;

use cgmath::{vec3, Matrix4};
use glium::{
    index::{NoIndices, PrimitiveType},
    uniform,
    uniforms::EmptyUniforms,
    Display, Frame, Program, Surface, VertexBuffer,
};
use itertools::Itertools;

use crate::{shaders::Vertex, Renderable};

pub struct Paddle {
    vertex_buffer: VertexBuffer<Vertex>,
    transform: Matrix4<f32>,
    program: Rc<Program>,
}

pub enum PaddleSide {
    Left,
    Right,
}

impl Paddle {
    pub fn new(display: &Display, program: Rc<Program>, side: PaddleSide) -> Self {
        const PADDLE_WIDTH: f32 = 0.05;
        const PADDLE_BUFFER: f32 = 0.025;
        const PADDLE_HEIGHT: f32 = 0.4;

        let positions = [1., -1.]
            .into_iter()
            .cartesian_product([1., -1.])
            .map(|(a, b)| [a, b, 0., 1.]);
        let vertices = positions.map(Vertex::new).collect_vec();
        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

        let transform = Matrix4::from_translation(match side {
            PaddleSide::Left => vec3(-1. + PADDLE_WIDTH + PADDLE_BUFFER, 0., 0.),
            PaddleSide::Right => vec3(1. - PADDLE_WIDTH - PADDLE_BUFFER, 0., 0.),
        }) * Matrix4::from_nonuniform_scale(PADDLE_WIDTH, PADDLE_HEIGHT, 1.);

        Paddle {
            vertex_buffer,
            transform,
            program,
        }
    }
}

impl Renderable for Paddle {
    fn render(&self, frame: &mut Frame) {
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
}
