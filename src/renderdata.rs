use std::rc::Rc;

use crate::{
    create_projection,
    shaders::{Uniforms, Vertex},
};
use cgmath::{vec3, Matrix4, Vector2};
use glium::{
    index::{NoIndices, PrimitiveType},
    uniform, Display, Frame, Program, Surface, VertexBuffer,
};
use itertools::Itertools;

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

    pub fn render(&self, frame: &mut Frame, transform: Transform) {
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

#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Vector2<f32>,
    pub scale: Vector2<f32>,
}

impl From<Transform> for Matrix4<f32> {
    fn from(transform: Transform) -> Self {
        let translation =
            Matrix4::from_translation(vec3(transform.translation.x, transform.translation.y, 0.0));
        let scale = Matrix4::from_nonuniform_scale(transform.scale.x, transform.scale.y, 1.0);
        translation * scale
    }
}

impl From<Transform> for [[f32; 4]; 4] {
    fn from(transform: Transform) -> Self {
        Into::<Matrix4<f32>>::into(transform).into()
    }
}
