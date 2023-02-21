use std::rc::Rc;

use glium::{
    glutin::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::WindowBuilder,
        ContextBuilder,
    },
    Display, Frame, Program, Surface,
};
use paddle::{Paddle, PaddleSide};

mod paddle;
mod shaders;

trait Renderable {
    fn render(&self, frame: &mut Frame);
}

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(1280.0, 720.0))
        .with_title("Pong");
    let context_builder = ContextBuilder::new().with_vsync(true);

    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();
    let program = Rc::new(
        Program::from_source(
            &display,
            include_str!("shaders/vert.glsl"),
            include_str!("shaders/frag.glsl"),
            None,
        )
        .unwrap(),
    );

    let left_paddle = Paddle::new(&display, program.clone(), PaddleSide::Left);
    let right_paddle = Paddle::new(&display, program, PaddleSide::Right);

    event_loop.run(move |event, _, flow| {
        flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => flow.set_exit(),
            Event::MainEventsCleared => {
                let mut frame = display.draw();
                frame.clear_color(0., 0., 0., 1.);
                left_paddle.render(&mut frame);
                right_paddle.render(&mut frame);
                frame.finish().unwrap();
            }
            _ => (),
        }
    })
}
