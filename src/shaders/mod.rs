use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 4],
}

implement_vertex!(Vertex, position);
