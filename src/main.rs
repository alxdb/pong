use glium::{
    glutin::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::WindowBuilder,
        ContextBuilder,
    },
    index::{NoIndices, PrimitiveType},
    uniforms::EmptyUniforms,
    Display, Program, Surface, VertexBuffer,
};

mod shaders;

use shaders::Vertex;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1920.0, 1080.0))
        .with_title("Pong");
    let context_builder = ContextBuilder::new().with_vsync(true);

    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();
    let program = Program::from_source(
        &display,
        include_str!("shaders/vert.glsl"),
        include_str!("shaders/frag.glsl"),
        None,
    )
    .unwrap();

    let tri_buff = VertexBuffer::new(
        &display,
        &[
            Vertex {
                position: [1., -1., 0., 1.],
            },
            Vertex {
                position: [-1., -1., 0., 1.],
            },
            Vertex {
                position: [0., 1.0, 0., 1.],
            },
        ],
    )
    .unwrap();

    event_loop.run(move |event, _, flow| {
        flow.set_poll();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => flow.set_exit(),
                _ => (),
            },
            Event::MainEventsCleared => {
                let mut frame = display.draw();
                frame.clear_color(0., 0., 0., 1.);
                frame
                    .draw(
                        &tri_buff,
                        &NoIndices(PrimitiveType::TrianglesList),
                        &program,
                        &EmptyUniforms,
                        &Default::default(),
                    )
                    .unwrap();
                frame.finish().unwrap();
            }
            _ => (),
        }
    })
}
