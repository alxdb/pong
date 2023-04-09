use std::{f32::consts::TAU, rc::Rc};

use cgmath::{num_traits::zero, vec3};
use glium::{Display, Frame, Program};
use itertools::intersperse;

use crate::{RenderData, Transform};

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

        let scale = vec3(Self::SCALE, Self::SCALE, Self::SCALE);

        Ball {
            renderdata: RenderData::new(
                display,
                program,
                positions,
                Transform {
                    translation: zero(),
                    scale,
                },
            ),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        self.renderdata.render(frame)
    }
}
