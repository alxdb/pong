use winit::{
    dpi::PhysicalSize,
    event::{
        Event::{MainEventsCleared, RedrawRequested, WindowEvent},
        WindowEvent::CloseRequested,
    },
    event_loop::EventLoopBuilder,
    window::WindowBuilder,
};

use pong::render;

struct Triangle {
    render_data: render::RenderData,
    transform: nalgebra::Similarity2<f64>,
}

impl Triangle {
    fn new(renderer: &render::Renderer) -> Self {
        Self {
            render_data: render::RenderData::new(renderer, &render::Vertex::triangle()),
            transform: nalgebra::Similarity2::identity(),
        }
    }

    fn translate(&mut self, translation: nalgebra::Translation2<f64>) {
        self.transform.append_translation_mut(&translation);
    }
}

impl render::Renderable for Triangle {
    fn render_data(&self) -> &render::RenderData {
        &self.render_data
    }

    fn transform(&self) -> nalgebra::Matrix4<f32> {
        // cannot use mat3 in shaders...
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

#[async_std::main]
async fn main() {
    env_logger::init();
    let event_loop = EventLoopBuilder::new().build();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1920, 1080))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let renderer = render::Renderer::new(window).await;
    let mut triangle = Triangle::new(&renderer);
    triangle.translate(nalgebra::Translation2::new(-0.1, -0.1));

    event_loop.run(move |event, _, control| match event {
        WindowEvent {
            event: CloseRequested,
            ..
        } => control.set_exit(),
        MainEventsCleared => renderer.window().request_redraw(),
        RedrawRequested(_) => renderer.render(&[&triangle]).unwrap(),
        _ => (),
    })
}
