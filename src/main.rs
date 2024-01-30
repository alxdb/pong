use std::rc::Rc;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use pong::graphics::{self, Graphics};

struct App {
    event_loop: EventLoop<()>,
    window: Rc<Window>,
    graphics: Graphics,
    graphics_objects: Vec<graphics::Object>,
}

impl App {
    pub fn new() -> App {
        let event_loop = EventLoop::new();
        let window = Rc::new(
            WindowBuilder::new()
                .with_title("Pong")
                .with_resizable(true)
                .with_inner_size(LogicalSize::new(1920, 1080))
                .build(&event_loop)
                .unwrap(),
        );
        let graphics = Graphics::new(window.clone());
        App {
            event_loop,
            window,
            graphics,
            graphics_objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, descriptor: graphics::ObjectDescriptor) {
        self.graphics_objects
            .push(graphics::Object::new(&self.graphics, descriptor));
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            log::info!("Exiting application");
                            control_flow.set_exit();
                        }
                        WindowEvent::Resized(_) => self.graphics.on_resize(),
                        _ => (),
                    }
                }
                Event::MainEventsCleared => {
                    self.graphics.draw(&self.graphics_objects);
                }
                _ => (),
            }
        })
    }
}

fn main() {
    env_logger::init();
    let mut app = App::new();
    app.add_object(graphics::ObjectDescriptor::circle(128, 0.5));
    app.run()
}
