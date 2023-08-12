use futures::executor::block_on;
use std::rc::Rc;

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use pong::graphics::Graphics;

struct App {
    event_loop: EventLoop<()>,
    window: Rc<Window>,
    graphics: Graphics,
}

impl App {
    async fn new() -> App {
        let event_loop = EventLoop::new();
        let window = Rc::new(
            WindowBuilder::new()
                .with_title("Pong")
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(1920, 1080))
                .build(&event_loop)
                .unwrap(),
        );
        let graphics = Graphics::new(window.clone()).await;
        App {
            event_loop,
            window,
            graphics,
        }
    }

    fn run(self) {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            log::info!("Exiting application");
                            control_flow.set_exit();
                        }
                        WindowEvent::Resized(_) => self.graphics.configure(),
                        _ => (),
                    }
                }
                Event::MainEventsCleared => {
                    self.graphics.draw();
                }
                _ => (),
            }
        })
    }
}

fn main() {
    env_logger::init();
    let app = block_on(App::new());
    app.run()
}
