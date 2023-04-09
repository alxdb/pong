pub mod ball;
pub mod paddle;

pub(crate) mod shaders;

use std::rc::Rc;

use cgmath::Matrix4;
use glium::{
    index::{NoIndices, PrimitiveType},
    uniform,
    uniforms::{EmptyUniforms, UniformsStorage},
    Frame, Program, Surface, VertexBuffer,
};
use shaders::Vertex;

pub struct RenderData {
    vertex_buffer: VertexBuffer<Vertex>,
    transform: Matrix4<f32>,
    program: Rc<Program>,
}

impl RenderData {
    pub fn render(&self, frame: &mut Frame) {
        let uniforms: UniformsStorage<[[f32; 4]; 4], EmptyUniforms> = uniform! {
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
