use std::time::Instant;

use glium as gl;
use glium::glutin as glu;
use nalgebra as na;

mod geometry;
mod physics;
mod render;

impl render::Renderable for physics::Body {
    fn transform(&self) -> [[f32; 3]; 3] {
        self.translation().to_homogeneous().cast().into()
    }
}

fn main() {
    let rect_builder = physics::BodyBuilder::rect(0.5, 0.5);
    let rects = &[
        na::point![-0.5, -0.5],
        na::point![-0.5, 0.5],
        na::point![0.5, 0.5],
        na::point![0.5, -0.5],
    ]
    .map(|center| rect_builder.clone().center(center).build_arc());
    let circle = physics::BodyBuilder::circle(0.25).build_arc();

    let event_loop = glu::event_loop::EventLoop::new();
    let window_builder = glu::window::WindowBuilder::new()
        .with_inner_size(glu::dpi::LogicalSize::new(1920., 1080.))
        .with_title("Pong");
    let context_builder = glu::ContextBuilder::new().with_vsync(true);

    let renderer = {
        let mut renderer = render::Renderer::new(
            gl::Display::new(window_builder, context_builder, &event_loop).unwrap(),
        );
        for rect in rects {
            renderer.register_render_data(rect.shape(), rect.clone());
        }
        renderer.register_render_data(circle.shape(), circle.clone());
        renderer
    };

    let mut last_updated: Instant = Instant::now();
    event_loop.run(move |event, _, flow| {
        use glu::event::*;

        flow.set_poll();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => flow.set_exit(),
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match (key, input.state) {
                            (VirtualKeyCode::Escape, ElementState::Pressed) => flow.set_exit(),
                            _ => (),
                        }
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let _delta = last_updated.elapsed();
                // do updates
                last_updated = Instant::now();

                // render
                renderer.render();
            }
            _ => (),
        }
    })
}
