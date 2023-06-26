use super::geometry::Shape;
use glium as gl;
use glium::glutin as glu;
use nalgebra as na;

mod shader;

pub struct RenderData {
    vertex_buffer: gl::VertexBuffer<shader::Vertex>,
    primitive_type: gl::index::PrimitiveType,
}

impl RenderData {
    pub fn new(renderer: &Renderer, shape: &Shape) -> Self {
        let vertices: Vec<shader::Vertex> = shape
            .positions()
            .into_iter()
            .map(shader::Vertex::new)
            .collect();
        Self {
            vertex_buffer: gl::VertexBuffer::new(&renderer.display, &vertices).unwrap(),
            primitive_type: shape.primitive_type(),
        }
    }
}

pub trait Renderable {
    fn transform(&self) -> [[f32; 3]; 3];
}

pub struct Renderer {
    display: gl::Display,
    program: gl::Program,
}

impl Renderer {
    pub fn new(display: gl::Display) -> Self {
        let program = shader::create_program(&display);
        Renderer { display, program }
    }

    pub fn render(&self, items: &[(&RenderData, &dyn Renderable)]) {
        use gl::Surface;

        let mut frame = self.display.draw();
        frame.clear_color(0., 0., 0., 1.);

        items.iter().for_each(|(render_data, renderable)| {
            let transform = renderable.transform();
            let projection = self.projection().to_homogeneous().cast().into();
            frame
                .draw(
                    &render_data.vertex_buffer,
                    &gl::index::NoIndices(render_data.primitive_type),
                    &self.program,
                    &shader::uniforms(transform, projection),
                    &Default::default(),
                )
                .unwrap()
        });

        frame.finish().unwrap();
    }

    pub fn to_world_coords(&self, position: glu::dpi::PhysicalPosition<f64>) -> na::Point2<f64> {
        let (w, h) = self.screen_size();
        let screen_point = na::point![(position.x / w) - 0.5, 0.5 - (position.y / h), 0.0] * 2.;

        self.projection()
            .inverse()
            .transform_point(&screen_point)
            .xy()
    }

    fn screen_size(&self) -> (f64, f64) {
        let (w, h) = self.display.get_framebuffer_dimensions();
        (w as f64, h as f64)
    }

    fn projection(&self) -> na::geometry::Orthographic3<f64> {
        let (w, h) = self.screen_size();
        let zoom = 700.;
        let w = w / zoom;
        let h = h / zoom;
        na::geometry::Orthographic3::new(-w / 2., w / 2., -h / 2., h / 2., -1., 1.)
    }
}
