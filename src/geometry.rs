#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub r: f64,
}

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
}

impl Shape {
    pub fn positions(&self) -> Vec<[f32; 2]> {
        match *self {
            Shape::Rectangle(Rectangle { w, h }) => {
                let w = w as f32;
                let h = h as f32;
                #[cfg_attr(rustfmt, rustfmt_skip)]
                vec![
                    [ w / 2.,  h / 2.],
                    [-w / 2.,  h / 2.],
                    [ w / 2., -h / 2.],
                    [-w / 2., -h / 2.],
                ]
            }
            Shape::Circle(Circle { r }) => {
                const SECTIONS: usize = 128;
                const ANGLE_DELTA: f64 = std::f64::consts::TAU / SECTIONS as f64;
                let mut points = vec![[0., 0.]]; // start with center
                for i in 0..=SECTIONS {
                    let angle = ANGLE_DELTA * i as f64;
                    points.push([(r * angle.cos()) as f32, (r * angle.sin()) as f32]);
                }
                points
            }
        }
    }

    pub fn primitive_type(&self) -> glium::index::PrimitiveType {
        match self {
            Shape::Rectangle { .. } => glium::index::PrimitiveType::TriangleStrip,
            Shape::Circle { .. } => glium::index::PrimitiveType::TriangleFan,
        }
    }
}
