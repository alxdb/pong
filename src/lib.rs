pub mod ball;
pub mod paddle;

pub(crate) mod shaders;

use std::rc::Rc;

use cgmath::{Matrix4, Vector3};
use glium::{
    index::{NoIndices, PrimitiveType},
    uniform, Display, Frame, Program, Surface, VertexBuffer,
};
use itertools::Itertools;
use shaders::{Uniforms, Vertex};

pub struct RenderData {
    vertex_buffer: VertexBuffer<Vertex>,
    projection: Matrix4<f32>,
    pub transform: Transform,
    program: Rc<Program>,
}

impl RenderData {
    pub fn new(
        display: &Display,
        program: Rc<Program>,
        positions: impl Iterator<Item = [f32; 2]>,
        transform: Transform,
    ) -> Self {
        let vertices = positions
            .map(|[a, b]| [a, b, 0., 1.])
            .map(Vertex::new)
            .collect_vec();

        Self {
            vertex_buffer: VertexBuffer::new(display, &vertices).unwrap(),
            projection: create_projection(display),
            transform,
            program,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let uniforms: Uniforms = uniform! {
            projection: self.projection.into(),
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

#[derive(Clone, Copy)]
pub struct Transform {
    translation: Vector3<f32>,
    scale: Vector3<f32>,
}

impl From<Transform> for Matrix4<f32> {
    fn from(transform: Transform) -> Self {
        let translation = Matrix4::from_translation(transform.translation);
        let scale =
            Matrix4::from_nonuniform_scale(transform.scale.x, transform.scale.y, transform.scale.z);
        translation * scale
    }
}

impl From<Transform> for [[f32; 4]; 4] {
    fn from(transform: Transform) -> Self {
        Into::<Matrix4<f32>>::into(transform).into()
    }
}

fn get_display_ratio(display: &Display) -> f32 {
    let (width, height) = display.get_framebuffer_dimensions();
    width as f32 / height as f32
}

fn create_projection(display: &Display) -> Matrix4<f32> {
    let ratio = get_display_ratio(display);
    cgmath::ortho(-ratio, ratio, -1., 1., 0., 1.)
}
