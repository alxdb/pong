use cgmath::Matrix4;
use glium::Display;

pub mod ball;
pub mod paddle;

pub(crate) mod collider;
pub(crate) mod renderdata;
pub(crate) mod shaders;

fn get_display_ratio(display: &Display) -> f32 {
    let (width, height) = display.get_framebuffer_dimensions();
    width as f32 / height as f32
}

fn create_projection(display: &Display) -> Matrix4<f32> {
    let ratio = get_display_ratio(display);
    cgmath::ortho(-ratio, ratio, -1., 1., 0., 1.)
}
