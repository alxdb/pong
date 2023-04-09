use std::{rc::Rc, time::Instant};

use glium::{
    glutin::{
        dpi::LogicalSize,
        event::{ElementState, Event, VirtualKeyCode, WindowEvent},
        event_loop::EventLoop,
        window::WindowBuilder,
        ContextBuilder,
    },
    Display, Program, Surface,
};

use pong::{
    ball::Ball,
    paddle::{Paddle, PaddleSide, PaddleState},
};

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

    let mut left_paddle = Paddle::new(&display, program.clone(), PaddleSide::Left);
    let right_paddle = Paddle::new(&display, program.clone(), PaddleSide::Right);
    let ball = Ball::new(&display, program);

    let mut last_updated: Instant = Instant::now();
    event_loop.run(move |event, _, flow| {
        flow.set_poll();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => flow.set_exit(),
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match (key, input.state, left_paddle.state) {
                            (VirtualKeyCode::W, ElementState::Pressed, PaddleState::DoNothing) => {
                                left_paddle.state = PaddleState::MoveUp
                            }
                            (VirtualKeyCode::W, ElementState::Released, PaddleState::MoveUp) => {
                                left_paddle.state = PaddleState::DoNothing
                            }
                            (VirtualKeyCode::S, ElementState::Pressed, PaddleState::DoNothing) => {
                                left_paddle.state = PaddleState::MoveDown
                            }
                            (VirtualKeyCode::S, ElementState::Released, PaddleState::MoveDown) => {
                                left_paddle.state = PaddleState::DoNothing
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                // update
                let delta = last_updated.elapsed();
                left_paddle.update(&delta);
                // do updates
                last_updated = Instant::now();

                // render
                let mut frame = display.draw();
                frame.clear_color(0., 0., 0., 1.);
                left_paddle.render(&mut frame);
                right_paddle.render(&mut frame);
                ball.render(&mut frame);
                frame.finish().unwrap();
            }
            _ => (),
        }
    })
}
