use glium::{
    implement_vertex,
    uniforms::{EmptyUniforms, UniformsStorage},
};

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
