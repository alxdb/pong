use nalgebra as na;

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rectangle { w: f64, h: f64 },
    Circle { r: f64 },
}

impl Shape {
    pub fn positions(&self) -> Vec<[f32; 2]> {
        match *self {
            Shape::Rectangle { w, h } => {
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
            Shape::Circle { r } => {
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
        let edges = |w: &f64, h: &f64, c: &na::Point2<f64>| -> [f64; 4] {
            [c.x - w / 2., c.x + w / 2., c.y + h / 2., c.y - h / 2.]
        };
        match (self, other) {
            (
                Self {
                    shape: Shape::Rectangle { w: w1, h: h1 },
                    center: center_1,
                },
                Self {
                    shape: Shape::Rectangle { w: w2, h: h2 },
                    center: center_2,
                },
            ) => {
                let [r1_l, r1_r, r1_t, r1_b] = edges(w1, h1, center_1);
                let [r2_l, r2_r, r2_t, r2_b] = edges(w2, h2, center_2);
                !(r1_r < r2_l || r1_l > r2_r || r1_t < r2_b || r1_b > r2_t)
            }
            (
                Self {
                    shape: Shape::Circle { r: r1 },
                    center: center_1,
                },
                Self {
                    shape: Shape::Circle { r: r2 },
                    center: center_2,
                },
            ) => na::distance(&center_1, &center_2) <= (r1 + r2),
            (
                Self {
                    shape: Shape::Circle { r },
                    center: circle_center,
                },
                Self {
                    shape: Shape::Rectangle { w, h },
                    center: rect_center,
                },
            )
            | (
                Self {
                    shape: Shape::Rectangle { w, h },
                    center: rect_center,
                },
                Self {
                    shape: Shape::Circle { r },
                    center: circle_center,
                },
            ) => {
                let [rl, rr, rt, rb] = edges(w, h, rect_center);
                let closest = na::point![
                    na::clamp(circle_center.x, rl, rr),
                    na::clamp(circle_center.y, rb, rt)
                ];
                na::distance(circle_center, &closest) <= *r
            }
        }
    }
}
