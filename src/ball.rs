use std::{f32::consts::TAU, rc::Rc};

use cgmath::{Matrix4, SquareMatrix};
use glium::{
    index::{NoIndices, PrimitiveType},
    uniform,
    uniforms::EmptyUniforms,
    Display, Frame, Program, Surface, VertexBuffer,
};
use itertools::{intersperse, Itertools};

use crate::shaders::Vertex;

pub struct Ball {
    vertex_buffer: VertexBuffer<Vertex>,
    transform: Matrix4<f32>,
    program: Rc<Program>,
}

impl Ball {
    const N_SEGMENTS: usize = 32;

    pub fn new(display: &Display, program: Rc<Program>) -> Self {
        let center = [0.0, 0.0];

        let circle_points = (0..Self::N_SEGMENTS)
            .map(|n| TAU / n as f32)
            .map(|theta| [f32::cos(theta), f32::sin(theta)]);

        let vertices = intersperse(circle_points, center)
            .map(|[a, b]| [a, b, 0., 1.])
            .map(Vertex::new)
            .collect_vec();

        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

        Ball {
            vertex_buffer,
            transform: Matrix4::identity(),
            program,
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
}
