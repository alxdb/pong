use std::sync::Arc;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use pong::graphics::{self, Graphics};

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Pong")
            .with_resizable(true)
            .with_inner_size(LogicalSize::new(1920, 1080))
            .build(&event_loop)
            .unwrap(),
    );
    let graphics = Graphics::new(&window);

    let graphics_objects = vec![graphics::Object::new(
        &graphics,
        graphics::ObjectDescriptor::circle(128, 0.5),
    )];

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    log::info!("Exiting application");
                    target.exit();
                }
                WindowEvent::Resized(_) => graphics.on_resize(),
                _ => (),
            },
            Event::AboutToWait => {
                graphics.draw(&graphics_objects);
            }
            _ => (),
        })
        .unwrap()
}
