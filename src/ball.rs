use std::{f32::consts::TAU, rc::Rc};

use cgmath::Matrix4;
use glium::{Display, Frame, Program, VertexBuffer};
use itertools::{intersperse, Itertools};

use crate::{shaders::Vertex, RenderData};

pub struct Ball {
    renderdata: RenderData,
}

impl Ball {
    const N_SEGMENTS: usize = 128;
    const SCALE: f32 = 0.15;

    pub fn new(display: &Display, program: Rc<Program>) -> Self {
        let center = [0.0, 0.0];
        let increment = TAU / Self::N_SEGMENTS as f32;
        let positions = (0..Self::N_SEGMENTS + 1)
            .map(|n| increment * n as f32)
            .map(|theta| [f32::cos(theta), f32::sin(theta)]);
        let positions = intersperse(positions, center);

        let vertices = positions
            .map(|[a, b]| [a, b, 0., 1.])
            .map(Vertex::new)
            .collect_vec();
        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();

        let (width, height): (f32, f32) = display.gl_window().window().inner_size().into();
        let ratio = width / height;
        let transform =
            cgmath::ortho(-ratio, ratio, 1., -1., 0., 1.) * Matrix4::from_scale(Self::SCALE);

        Ball {
            renderdata: RenderData {
                vertex_buffer,
                transform,
                program,
            },
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        self.renderdata.render(frame)
    }
}
