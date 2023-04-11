use std::rc::Rc;

use crate::{
    create_projection,
    shaders::{Uniforms, Vertex},
    transform::Transform,
};
use glium::{
    index::{NoIndices, PrimitiveType},
    uniform, Display, Frame, Program, Surface, VertexBuffer,
};
use itertools::Itertools;
use nalgebra::Matrix4;

pub struct RenderData {
    vertex_buffer: VertexBuffer<Vertex>,
    projection: Matrix4<f32>,
    program: Rc<Program>,
}

impl RenderData {
    pub fn new(
        display: &Display,
        program: Rc<Program>,
        positions: impl Iterator<Item = [f32; 2]>,
    ) -> Self {
        let vertices = positions
            .map(|[a, b]| [a, b, 0., 1.])
            .map(Vertex::new)
            .collect_vec();

        Self {
            vertex_buffer: VertexBuffer::new(display, &vertices).unwrap(),
            projection: create_projection(display),
            program,
        }
    }

    pub fn render(&self, frame: &mut Frame, transform: &Transform) {
        let uniforms: Uniforms = uniform! {
            projection: self.projection.into(),
            transform: transform.into(),
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
