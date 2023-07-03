use std::time::Instant;

use speedy2d::{
    color::Color,
    dimen::Vec2,
    shape::Rectangle,
    window::{
        KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
        WindowPosition, WindowSize,
    },
    Graphics2D, Window,
};

fn clamp(min: Vec2, max: Vec2, value: Vec2) -> Vec2 {
    Vec2::new(value.x.clamp(min.x, max.x), value.y.clamp(min.y, max.y))
}

fn dot(a: Vec2, b: Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}

fn resize(value: Vec2, magnitude: f32) -> Vec2 {
    value * (magnitude / value.magnitude())
}

fn project(a: Vec2, b: Vec2) -> Vec2 {
    b * (dot(a, b) / dot(b, b))
}

fn main() {
    let window = Window::new_with_options(
        "Pong",
        WindowCreationOptions::new_windowed(
            WindowSize::PhysicalPixels(Pong::DIMENSIONS.into_u32()),
            Some(WindowPosition::Center),
        )
        .with_resizable(false),
    )
    .unwrap();
    window.run_loop(Pong::new());
}

struct Ball {
    position: Vec2,
    velocity: Vec2,
}

impl Ball {
    const VELOCITY: Vec2 = Vec2::new(-1000., 100.);
    const RADIUS: f32 = 75.;

    fn collide_with_paddle(&self, paddle: &Paddle) -> (Vec2, Vec2) {
        let closest_point = paddle.closest_point(self.position);
        let distance = self.position - closest_point;

        if distance.magnitude_squared() < Self::RADIUS.powi(2) {
            let position_d = resize(distance, Self::RADIUS) - distance;
            let velocity_d = project(self.velocity, distance) * -2.;
            (position_d, velocity_d)
        } else {
            (Vec2::ZERO, Vec2::ZERO)
        }
    }

    fn collide_with_walls(&self) -> (Vec2, Vec2) {
        let wall_low = Vec2::new(Self::RADIUS, Self::RADIUS);
        let wall_high = Pong::DIMENSIONS - wall_low;
        let wall_protrusion = self.position - clamp(wall_low, wall_high, self.position);

        if wall_protrusion != Vec2::ZERO {
            let position_d = wall_protrusion * -1.;
            let velocity_d = project(self.velocity, wall_protrusion * -1.) * -2.;
            (position_d, velocity_d)
        } else {
            (Vec2::ZERO, Vec2::ZERO)
        }
    }
}

#[derive(Clone)]
struct Paddle {
    position: Vec2,
    velocity: Vec2,
}

impl Paddle {
    const VELOCITY: Vec2 = Vec2::new(0., 1000.);
    const DIMENSIONS: Vec2 = Vec2::new(100., 400.);

    fn top_left(&self) -> Vec2 {
        self.position - (Self::DIMENSIONS / 2.)
    }

    fn bottom_right(&self) -> Vec2 {
        self.position + (Self::DIMENSIONS / 2.)
    }

    fn closest_point(&self, point: Vec2) -> Vec2 {
        clamp(self.top_left(), self.bottom_right(), point)
    }

    fn clamp_to_walls(&mut self) {
        let half_height = (Self::DIMENSIONS / 2.).y;
        let y = &mut self.position.y;
        *y = y.clamp(half_height, Pong::DIMENSIONS.y - half_height);
    }
}

struct Pong {
    last_update: Instant,
    ball: Ball,
    paddles: [Paddle; 2],
}

impl Pong {
    const DIMENSIONS: Vec2 = Vec2::new(1920., 1080.);

    fn new() -> Self {
        let half_screen_dims = Self::DIMENSIONS / 2.;
        let paddle_dims = Paddle::DIMENSIONS;
        let paddle_offset = Vec2::new(half_screen_dims.x - paddle_dims.x, 0.);
        Pong {
            last_update: Instant::now(),
            ball: Ball {
                position: half_screen_dims.into(),
                velocity: Ball::VELOCITY,
            },
            paddles: [
                Paddle {
                    position: half_screen_dims - paddle_offset,
                    velocity: Vec2::ZERO,
                },
                Paddle {
                    position: half_screen_dims + paddle_offset,
                    velocity: Vec2::ZERO,
                },
            ],
        }
    }

    fn update(&mut self) {
        let delta = self.last_update.elapsed();
        self.last_update = Instant::now();

        // move paddles if not colliding
        for paddle in &mut self.paddles {
            let mut proposed_paddle = paddle.clone();
            proposed_paddle.position += paddle.velocity * delta.as_secs_f32();
            if self.ball.collide_with_paddle(&proposed_paddle) == (Vec2::ZERO, Vec2::ZERO) {
                proposed_paddle.clamp_to_walls();
                *paddle = proposed_paddle;
            }
        }

        // move ball
        self.ball.position += self.ball.velocity * delta.as_secs_f32();

        // ball collision with paddles
        let closest_paddle = if self.ball.position.x < (Self::DIMENSIONS.x / 2.) {
            &self.paddles[0]
        } else {
            &self.paddles[1]
        };

        let (position_d, velocity_d) = self.ball.collide_with_paddle(closest_paddle);
        self.ball.position += position_d;
        self.ball.velocity += velocity_d;

        // ball collision with walls
        let (position_d, velocity_d) = self.ball.collide_with_walls();
        self.ball.position += position_d;
        self.ball.velocity += velocity_d;
    }
}

impl WindowHandler for Pong {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let foreground: Color = Color::from_hex_rgb(0xebdbb2);
        let background: Color = Color::from_hex_rgb(0x1d2021);

        graphics.clear_screen(background);
        graphics.draw_circle(self.ball.position, Ball::RADIUS, foreground);
        for paddle in &self.paddles {
            graphics.draw_rectangle(
                Rectangle::new(paddle.top_left(), paddle.bottom_right()),
                foreground,
            )
        }

        self.update();

        helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(key_code) = virtual_key_code {
            match key_code {
                VirtualKeyCode::W if self.paddles[0].velocity == Vec2::ZERO => {
                    self.paddles[0].velocity = Paddle::VELOCITY * -1.
                }
                VirtualKeyCode::S if self.paddles[0].velocity == Vec2::ZERO => {
                    self.paddles[0].velocity = Paddle::VELOCITY
                }
                _ => (),
            }
        }
    }

    fn on_key_up(
        &mut self,
        _helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        if let Some(key_code) = virtual_key_code {
            match key_code {
                VirtualKeyCode::W if self.paddles[0].velocity == Paddle::VELOCITY * -1. => {
                    self.paddles[0].velocity = Vec2::ZERO
                }
                VirtualKeyCode::S if self.paddles[0].velocity == Paddle::VELOCITY => {
                    self.paddles[0].velocity = Vec2::ZERO
                }
                _ => (),
            }
        }
    }
}
