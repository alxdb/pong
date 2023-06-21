#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 2]) -> Self {
        Vertex { position }
    }
}

glium::implement_vertex!(Vertex, position);

pub fn create_program(facade: &dyn glium::backend::Facade) -> glium::Program {
    glium::Program::from_source(
        facade,
        include_str!("vert.glsl"),
        include_str!("frag.glsl"),
        None,
    )
    .unwrap()
}

pub type Uniforms = glium::uniforms::UniformsStorage<
    'static,
    [[f32; 4]; 4],
    glium::uniforms::UniformsStorage<'static, [[f32; 3]; 3], glium::uniforms::EmptyUniforms>,
>;

pub fn uniforms(transform: [[f32; 3]; 3], projection: [[f32; 4]; 4]) -> Uniforms {
    glium::uniform! {
        transform: transform,
        projection: projection
    }
}
