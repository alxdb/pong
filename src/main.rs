use std::time::Instant;

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

struct Object {
    pub body: physics::Body,
    render_data: render::RenderData,
}

impl Object {
    fn new(renderer: &render::Renderer, body_builder: physics::BodyBuilder) -> Self {
        let body = body_builder.build();
        Self {
            render_data: render::RenderData::new(&renderer, &body.figure().shape),
            body,
        }
    }

    fn to_item(&self) -> (&render::RenderData, &dyn render::Renderable) {
        (&self.render_data, &self.body)
    }
}

fn main() {
    let event_loop = glu::event_loop::EventLoop::new();
    let window_builder = glu::window::WindowBuilder::new()
        .with_inner_size(glu::dpi::LogicalSize::new(1000., 1000.))
        .with_title("Pong");
    let context_builder = glu::ContextBuilder::new().with_vsync(true);

    let renderer = render::Renderer::new(
        gl::Display::new(window_builder, context_builder, &event_loop).unwrap(),
    );

    let rect = Object::new(&renderer, physics::BodyBuilder::rect(0.5, 0.5));
    let mut circle = Object::new(
        &renderer,
        physics::BodyBuilder::circle(0.1)
            .mass(1.0)
            .position(na::point![-0.75, 0.75])
            .velocity(na::vector![-0.2, 0.0]),
    );
    let walls = [
        na::point![-2., 0.],
        na::point![2., 0.],
        na::point![0., 2.],
        na::point![0., -2.],
    ]
    .map(|pos| Object::new(&renderer, physics::BodyBuilder::rect(2., 2.).position(pos)));

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
                let delta = last_updated.elapsed();
                circle.body.update(
                    delta.as_secs_f64(),
                    &[
                        &rect.body,
                        &walls[0].body,
                        &walls[1].body,
                        &walls[2].body,
                        &walls[3].body,
                    ],
                );
                last_updated = Instant::now();

                // render
                renderer.render(
                    &[&rect, &circle, &walls[0], &walls[1], &walls[2], &walls[3]]
                        .map(Object::to_item),
                );
            }
            _ => (),
        }
    })
}
