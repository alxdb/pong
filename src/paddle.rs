use glium::Display;
use itertools::iproduct;
use nalgebra::{Orthographic3, Scale2, Scale3, Transform3, Translation2, Translation3};

use pong::render::RenderData;

pub struct Paddle {
    renderdata: RenderData,
    translation: Translation2<f32>,
    scale: Scale2<f32>,
    pub state: PaddleState,
}

pub enum PaddleSide {
    Left,
    Right,
}

#[derive(PartialEq, Copy, Clone)]
pub enum PaddleState {
    MoveUp,
    MoveDown,
    DoNothing,
}

impl Paddle {
    const VELOCITY: f32 = 1.0;
    const WIDTH: f32 = 0.15;
    const PADDING: f32 = 0.05;
    const HEIGHT: f32 = 1.0;

    pub fn new(display: &Display, projection: &Orthographic3<f32>, side: PaddleSide) -> Self {
        let positions = iproduct!([0.5, -0.5], [0.5, -0.5]).map(|(a, b)| [a, b]);

        let translation = match side {
            PaddleSide::Left => {
                Translation2::new(projection.left() + Self::WIDTH / 2. + Self::PADDING, 0.)
            }
            PaddleSide::Right => {
                Translation2::new(projection.right() - Self::WIDTH / 2. - Self::PADDING, 0.)
            }
        };

        Paddle {
            renderdata: RenderData::new(display, positions),
            translation,
            scale: Scale2::new(Self::WIDTH, Self::HEIGHT),
            state: PaddleState::DoNothing,
        }
    }

    pub fn renderdata(&self) -> &RenderData {
        &self.renderdata
    }

    pub fn transform(&self) -> Transform3<f32> {
        Transform3::from_matrix_unchecked(
            Translation3::new(self.translation.x, self.translation.y, 0.).to_homogeneous()
                * Scale3::new(self.scale.x, self.scale.y, 1.).to_homogeneous(),
        )
    }
}
