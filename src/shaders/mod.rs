use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 2]) -> Self {
        Vertex { position }
    }
}

implement_vertex!(Vertex, position);
