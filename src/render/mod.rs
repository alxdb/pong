use super::geometry::Shape;
use glium as gl;
use nalgebra as na;
use std::sync::Arc;

mod shader;

struct RenderData {
    vertex_buffer: gl::VertexBuffer<shader::Vertex>,
    primitive_type: gl::index::PrimitiveType,
    renderable: Arc<dyn Renderable>,
}

impl RenderData {
    fn new(renderer: &Renderer, shape: &Shape, renderable: Arc<dyn Renderable>) -> Self {
        let vertices: Vec<shader::Vertex> = shape
            .positions()
            .into_iter()
            .map(shader::Vertex::new)
            .collect();
        Self {
            vertex_buffer: gl::VertexBuffer::new(&renderer.display, &vertices).unwrap(),
            primitive_type: shape.primitive_type(),
            renderable,
        }
    }
}

pub trait Renderable {
    fn transform(&self) -> [[f32; 3]; 3];
}

pub struct Renderer {
    display: gl::Display,
    program: gl::Program,
    render_data: Vec<RenderData>,
}

impl Renderer {
    pub fn new(display: gl::Display) -> Self {
        let program = shader::create_program(&display);
        Renderer {
            display,
            program,
            render_data: vec![],
        }
    }

    pub fn register_render_data(&mut self, shape: &Shape, renderable: Arc<dyn Renderable>) {
        self.render_data
            .push(RenderData::new(&self, shape, renderable));
    }

    pub fn render(&self) {
        use gl::Surface;

        let mut frame = self.display.draw();
        frame.clear_color(0., 0., 0., 1.);

        self.render_data.iter().for_each(|render_data| {
            let transform = render_data.renderable.transform();
            let projection = self.projection();
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

    fn projection(&self) -> [[f32; 4]; 4] {
        let (w, h) = self.display.get_framebuffer_dimensions();
        let zoom = 700.;
        let w = (w as f32) / zoom;
        let h = (h as f32) / zoom;
        na::geometry::Orthographic3::new(-w / 2., w / 2., -h / 2., h / 2., -1., 1.)
            .to_homogeneous()
            .into()
        // if h < w {
        //     na::geometry::Scale2::new(h as f32 / w as f32, 1.0)
        //         .to_homogeneous()
        //         .into()
        // } else {
        //     na::geometry::Scale2::new(1.0, w as f32 / h as f32)
        //         .to_homogeneous()
        //         .into()
        // }
    }
}
