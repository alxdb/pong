use nalgebra as na;

#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rectangle { w: f64, h: f64 },
    Circle { r: f64 },
    Wall { o: Orientation, s: f64 },
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
            Shape::Wall { o, s } => {
                let s = s as f32;
                match o {
                    Orientation::Left => {
                        vec![[0., s], [-s, s], [0., -s], [-s, -s]]
                    }
                    Orientation::Right => {
                        vec![[s, s], [0., s], [s, -s], [0., -s]]
                    }
                    Orientation::Top => {
                        vec![[s, s], [-s, s], [s, 0.], [-s, 0.]]
                    }
                    Orientation::Bottom => {
                        vec![[s, 0.], [-s, 0.], [s, -s], [-s, -s]]
                    }
                }
            }
        }
    }

    pub fn primitive_type(&self) -> glium::index::PrimitiveType {
        match self {
            Shape::Rectangle { .. } => glium::index::PrimitiveType::TriangleStrip,
            Shape::Circle { .. } => glium::index::PrimitiveType::TriangleFan,
            Shape::Wall { .. } => glium::index::PrimitiveType::TriangleStrip,
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
        let rect_edges = |w: &f64, h: &f64, c: &na::Point2<f64>| -> [f64; 4] {
            [c.x - w / 2., c.x + w / 2., c.y + h / 2., c.y - h / 2.]
        };
        let circle_edges =
            |r: &f64, c: &na::Point2<f64>| -> [f64; 4] { [c.x - r, c.x + r, c.y + r, c.y - r] };
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
                let [r1_l, r1_r, r1_t, r1_b] = rect_edges(w1, h1, center_1);
                let [r2_l, r2_r, r2_t, r2_b] = rect_edges(w2, h2, center_2);
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
                let [rl, rr, rt, rb] = rect_edges(w, h, rect_center);
                let closest = na::point![
                    na::clamp(circle_center.x, rl, rr),
                    na::clamp(circle_center.y, rb, rt)
                ];
                na::distance(circle_center, &closest) <= *r
            }
            (
                Self {
                    shape: Shape::Wall { .. },
                    ..
                },
                Self {
                    shape: Shape::Wall { .. },
                    ..
                },
            ) => unimplemented!(),
            (
                Self {
                    shape: Shape::Wall { o, s },
                    center: wall_center,
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
                    shape: Shape::Wall { o, s },
                    center: wall_center,
                },
            ) => {
                let [rl, rr, rt, rb] = rect_edges(w, h, rect_center);
                match o {
                    Orientation::Left => rl <= wall_center.x,
                    Orientation::Right => rr >= wall_center.x,
                    Orientation::Top => rt <= wall_center.y,
                    Orientation::Bottom => rb >= wall_center.y,
                }
            }
            (
                Self {
                    shape: Shape::Wall { o, s },
                    center: wall_center,
                },
                Self {
                    shape: Shape::Circle { r },
                    center: circle_center,
                },
            )
            | (
                Self {
                    shape: Shape::Circle { r },
                    center: circle_center,
                },
                Self {
                    shape: Shape::Wall { o, s },
                    center: wall_center,
                },
            ) => {
                let [cl, cr, ct, cb] = circle_edges(r, circle_center);
                match o {
                    Orientation::Left => cl <= wall_center.x,
                    Orientation::Right => cr >= wall_center.x,
                    Orientation::Top => ct >= wall_center.y,
                    Orientation::Bottom => cb <= wall_center.y,
                }
            }
        }
    }
}
