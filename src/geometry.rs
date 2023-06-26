use nalgebra as na;

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

#[derive(Debug, Copy, Clone)]
pub struct Figure {
    pub shape: Shape,
    pub center: na::Point2<f64>,
}

impl Figure {
    pub fn intersects(&self, other: &Self) -> bool {
        let edges = |r: &Rectangle, c: &na::Point2<f64>| -> [f64; 4] {
            [
                c.x - r.w / 2.,
                c.x + r.w / 2.,
                c.y + r.h / 2.,
                c.y - r.h / 2.,
            ]
        };
        match (self, other) {
            (
                Self {
                    shape: Shape::Rectangle(rect_1),
                    center: center_1,
                },
                Self {
                    shape: Shape::Rectangle(rect_2),
                    center: center_2,
                },
            ) => {
                let [r1_l, r1_r, r1_t, r1_b] = edges(rect_1, center_1);
                let [r2_l, r2_r, r2_t, r2_b] = edges(rect_2, center_2);
                !(r1_r < r2_l || r1_l > r2_r || r1_t < r2_b || r1_b > r2_t)
            }
            (
                Self {
                    shape: Shape::Circle(circle_1),
                    center: center_1,
                },
                Self {
                    shape: Shape::Circle(circle_2),
                    center: center_2,
                },
            ) => na::distance(&center_1, &center_2) <= (circle_1.r + circle_2.r),
            (
                Self {
                    shape: Shape::Circle(circle),
                    center: circle_center,
                },
                Self {
                    shape: Shape::Rectangle(rect),
                    center: rect_center,
                },
            )
            | (
                Self {
                    shape: Shape::Rectangle(rect),
                    center: rect_center,
                },
                Self {
                    shape: Shape::Circle(circle),
                    center: circle_center,
                },
            ) => {
                let [rl, rr, rt, rb] = edges(rect, rect_center);
                let closest = na::clamp(*circle_center, na::point![rl, rb], na::point![rr, rt]);
                na::distance(circle_center, &closest) <= circle.r
            }
        }
    }
}
