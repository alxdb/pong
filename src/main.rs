use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event::{
        Event::{AboutToWait, NewEvents, WindowEvent},
        StartCause,
        WindowEvent::{CloseRequested, RedrawRequested},
    },
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

use pong::render::{self, PushConstants};

struct Triangle {
    render_data: render::VertexData,
    transform: nalgebra::Similarity2<f64>,
}

impl Triangle {
    fn new(renderer: &render::RenderContext) -> Self {
        Self {
            render_data: render::VertexData::new(
                renderer,
                &[
                    render::Vertex {
                        position: [0.0, 0.5],
                    },
                    render::Vertex {
                        position: [-0.5, -0.5],
                    },
                    render::Vertex {
                        position: [0.5, -0.5],
                    },
                ],
            ),
            transform: nalgebra::Similarity2::identity(),
        }
    }

    fn translate(&mut self, translation: nalgebra::Translation2<f64>) {
        self.transform.append_translation_mut(&translation);
    }

    fn transform(&self) -> nalgebra::Matrix4<f32> {
        // cannot use mat3 in shaders... (alignment issues)
        let m2d: nalgebra::Matrix3<f32> = self.transform.to_homogeneous().cast();
        let mut m3d: nalgebra::Matrix4<f32> = nalgebra::Matrix4::identity();
        // copied from chatGPT (it works!)
        m3d[(0, 0)] = m2d[(0, 0)];
        m3d[(0, 1)] = m2d[(0, 1)];
        m3d[(0, 3)] = m2d[(0, 2)];
        m3d[(1, 0)] = m2d[(1, 0)];
        m3d[(1, 1)] = m2d[(1, 1)];
        m3d[(1, 3)] = m2d[(1, 2)];
        m3d
    }
}

impl render::Renderable for Triangle {
    fn vertex_data(&self) -> &render::VertexData {
        &self.render_data
    }
    fn push_constants(&self) -> render::PushConstants {
        PushConstants {
            transform: self.transform().into(),
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), EventLoopError> {
    env_logger::init();

    let event_loop = EventLoopBuilder::new().build().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1920, 1080))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let render_context = render::RenderContext::new(window).await;
    let mut triangle = Triangle::new(&render_context);
    triangle.translate(nalgebra::Translation2::new(0.5, 0.0));

    event_loop.run(move |event, window_target| match event {
        NewEvents(cause) => match cause {
            StartCause::Init => window_target.set_control_flow(ControlFlow::Poll),
            _ => (),
        },
        AboutToWait => render_context.window().request_redraw(),
        WindowEvent { event, .. } => match event {
            CloseRequested => window_target.exit(),
            RedrawRequested => render_context.render(&[&triangle]).unwrap(),
            _ => (),
        },
        _ => (),
    })
}
