use std::time::Instant;

use glium::{
    glutin::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::WindowBuilder,
        ContextBuilder,
    },
    Display, Surface,
};

use pong::render::RenderProgram;

mod ball;
mod paddle;

use ball::Ball;
use paddle::{Paddle, PaddleSide};

fn main() {
    let window_builder = WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(1280.0, 720.0))
        .with_title("Pong");
    let context_builder = ContextBuilder::new().with_vsync(true);

    let event_loop = EventLoop::new();
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();
    let program = RenderProgram::new(&display);

    let mut left_paddle = Paddle::new(&display, program.projection(), PaddleSide::Left);
    let right_paddle = Paddle::new(&display, program.projection(), PaddleSide::Right);
    let mut ball = Ball::new(&display);

    let mut last_updated: Instant = Instant::now();
    event_loop.run(move |event, _, flow| {
        flow.set_poll();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => flow.set_exit(),
                WindowEvent::KeyboardInput { input, .. } => left_paddle.handle_input(input),
                _ => (),
            },
            Event::MainEventsCleared => {
                // update
                let delta = last_updated.elapsed();
                left_paddle.update(&delta, program.projection());
                ball.update(&delta, program.projection());
                // do updates
                last_updated = Instant::now();

                // render
                let mut frame = display.draw();
                frame.clear_color(0., 0., 0., 1.);
                program.render(&mut frame, &left_paddle);
                program.render(&mut frame, &right_paddle);
                program.render(&mut frame, &ball);
                frame.finish().unwrap();
            }
            _ => (),
        }
    })
}
