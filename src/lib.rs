use glium::Display;
use nalgebra::Matrix4;

pub mod ball;
pub mod paddle;

pub(crate) mod collider;
pub(crate) mod renderdata;
pub(crate) mod shaders;
pub(crate) mod transform;

fn get_display_ratio(display: &Display) -> f32 {
    let (width, height) = display.get_framebuffer_dimensions();
    width as f32 / height as f32
}

fn create_projection(display: &Display) -> Matrix4<f32> {
    let ratio = get_display_ratio(display);
    Matrix4::new_orthographic(-ratio, ratio, -1., 1., 0., 1.)
}
