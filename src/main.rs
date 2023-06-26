use std::time::Instant;

use gl::glutin::dpi::LogicalPosition;
use glium as gl;
use glium::glutin as glu;
use nalgebra as na;

mod geometry;
mod physics;
mod render;

impl render::Renderable for physics::Body {
    fn transform(&self) -> [[f32; 3]; 3] {
        na::Translation2::from(self.figure().center)
            .to_homogeneous()
            .cast()
            .into()
    }
}

fn main() {
    let rect = physics::BodyBuilder::rect(0.5, 0.5).build_arc();
    let mut circle = physics::BodyBuilder::circle(0.25).build_arc();

    let event_loop = glu::event_loop::EventLoop::new();
    let window_builder = glu::window::WindowBuilder::new()
        .with_inner_size(glu::dpi::LogicalSize::new(1920., 1080.))
        .with_title("Pong");
    let context_builder = glu::ContextBuilder::new().with_vsync(true);

    let renderer = {
        let mut renderer = render::Renderer::new(
            gl::Display::new(window_builder, context_builder, &event_loop).unwrap(),
        );
        renderer.register_render_data(&rect.figure().shape, rect.clone());
        renderer.register_render_data(&circle.figure().shape, circle.clone());
        renderer
    };

    let mut last_updated: Instant = Instant::now();
    let mut current_scale_factor: f64 = renderer.current_scale_factor();
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
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    current_scale_factor = scale_factor
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let logical = position.to_logical::<f64>(current_scale_factor);
                    let screen_point = na::point![logical.x, logical.y, 0.];
                    let inv_proj = renderer.projection().inverse();
                    let space_point = inv_proj.transform_point(&screen_point);

                    println!("{}", space_point)
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
