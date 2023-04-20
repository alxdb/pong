use glium::{
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    uniform,
    uniforms::{EmptyUniforms, UniformsStorage},
    Display, Frame, Program, Surface, VertexBuffer,
};
use itertools::Itertools;
use nalgebra::{Orthographic3, Transform3};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 4],
}

impl Vertex {
    pub fn new(position: [f32; 4]) -> Self {
        Vertex { position }
    }
}

implement_vertex!(Vertex, position);

pub type Uniforms<'a, 'b> =
    UniformsStorage<'a, [[f32; 4]; 4], UniformsStorage<'b, [[f32; 4]; 4], EmptyUniforms>>;

pub struct RenderData {
    vertex_buffer: VertexBuffer<Vertex>,
}

impl RenderData {
    pub fn new(display: &Display, positions: impl Iterator<Item = [f32; 2]>) -> Self {
        let vertices = positions
            .map(|[a, b]| [a, b, 0., 1.])
            .map(Vertex::new)
            .collect_vec();

        Self {
            vertex_buffer: VertexBuffer::new(display, &vertices).unwrap(),
        }
    }
}

pub struct RenderProgram {
    program: Program,
    projection: Orthographic3<f32>,
}

impl RenderProgram {
    pub fn new(display: &Display) -> Self {
        Self {
            program: Program::from_source(
                display,
                include_str!("vert.glsl"),
                include_str!("frag.glsl"),
                None,
            )
            .unwrap(),
            projection: {
                let (width, height) = display.get_framebuffer_dimensions();
                let ratio = width as f32 / height as f32;
                Orthographic3::new(-ratio, ratio, -1., 1., 0., 1.)
            },
        }
    }

    pub fn render(&self, frame: &mut Frame, transform: &Transform3<f32>, renderdata: &RenderData) {
        let uniforms: Uniforms = uniform! {
            projection: self.projection.to_homogeneous().into(),
            transform: transform.to_homogeneous().into(),
        };

        frame
            .draw(
                &renderdata.vertex_buffer,
                NoIndices(PrimitiveType::TriangleStrip),
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap()
    }

    pub fn projection(&self) -> &Orthographic3<f32> {
        &self.projection
    }
}
