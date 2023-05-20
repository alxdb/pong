use std::f32::consts::{FRAC_PI_2, PI, TAU};

use nalgebra::{
    point, vector, Isometry2, Orthographic3, Point2, Rotation2, Transform2, Vector2, Vector3,
};

/// https://en.wikipedia.org/wiki/Line_coordinates
/// https://leimao.github.io/blog/2D-Line-Mathematics-Homogeneous-Coordinates/

#[derive(Debug)]
struct Line {
    a: Point2<f32>,
    b: Point2<f32>,
}

impl Line {
    fn new(v: Vector2<f32>, p: Point2<f32>) -> Self {
        Self { a: p, b: p + v }
    }

    fn coords(&self) -> Vector3<f32> {
        self.a.to_homogeneous().cross(&self.b.to_homogeneous())
    }

    fn infinite_intersection(&self, other: &Line) -> Option<Point2<f32>> {
        Point2::from_homogeneous(self.coords().cross(&other.coords()))
    }

    fn vector(&self) -> Vector2<f32> {
        self.b - self.a
    }

    fn midpoint(&self) -> Point2<f32> {
        self.a + (self.vector() / 2.)
    }

    fn start(&self) -> Point2<f32> {
        self.a
    }

    fn end(&self) -> Point2<f32> {
        self.b
    }

    fn radius(&self) -> f32 {
        (self.vector() / 2.).norm()
    }

    fn within_radius(&self, point: &Point2<f32>) -> bool {
        (point - self.midpoint()).norm() < self.radius()
    }

    fn intersection(&self, other: &Line) -> Option<Point2<f32>> {
        self.infinite_intersection(other)
            .filter(|x| self.within_radius(x))
            .filter(|x| other.within_radius(x))
    }

    fn rotate(&self, angle: f32) -> Line {
        Line::new(Rotation2::new(angle) * self.vector(), self.a)
    }

    fn orientation(&self, point: &Point2<f32>) -> Orientation {
        // https://stackoverflow.com/questions/14066933/direct-way-of-computing-the-clockwise-angle-between-two-vectors
        // let cang = self.vector().dot(&Vector2::y()) / self.vector().norm();
        // let sang = self
        //     .vector()
        //     .resize_vertically(3, 0.)
        //     .cross(&Vector3::y())
        //     .norm()
        //     / self.vector().norm();
        // let translated = point - self.a.coords;
        // let oriented = if self.vector().x != 0. {
        //     let cang = self.vector().dot(&Vector2::y()) / self.vector().norm();
        //     // println!("angle={angle}, cang={cang}");
        //     Rotation2::new(f32::acos(cang)) * translated
        // } else {
        //     translated
        // };
        // let angle = if self.vector().x == 0. {
        //     if self.vector().y < 0. {
        //         PI
        //     } else {
        //         0.
        //     }
        // } else {
        //     self.vector().angle(&Vector2::y_axis())
        // };

        // Solve transformation given in https://en.wikipedia.org/wiki/Rotation_of_axes for theta when x' = 0, y' = [x, y].norm()
        let angle = match (self.vector().x, self.vector().y) {
            (x, y) if x > 0. && y == 0. => -FRAC_PI_2,
            _ => self.vector().angle(&vector![0., 1.]),
        };
        if angle != self.vector().angle(&vector![0., 1.]) {
            println!(
                "angle={angle:?}, nalgebra::angle{:?}",
                self.vector().angle(&vector![0., 1.])
            );
        }

        // let angle =
        let oriented = Rotation2::new(-angle) * (point - self.a.coords);
        if oriented.x >= 0. {
            Orientation::Right
        } else {
            Orientation::Left
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Orientation {
    Left,
    Right,
}

#[cfg(test)]
mod line_tests {
    use proptest::prelude::*;
    use rstest::rstest;

    use super::{Line, Orientation};
    use nalgebra::{point, vector, Point2};

    proptest! {
        #[test]
        fn lower_y(x_len in 0.1f32..10.0, p_x in -10.0f32..10.0, p_y in -10.0f32..0.0) {
            prop_assert_eq!(Line::new(vector![x_len, 0.0], point![0.0, 0.0]).orientation(&point![p_x, p_y]), Orientation::Right, "positive X");
            prop_assert_eq!(Line::new(vector![-x_len, 0.0], point![0.0, 0.0]).orientation(&point![p_x, p_y]), Orientation::Left, "negative X");
        }

        #[test]
        fn upper_y(x_len in 0.1f32..10.0, p_x in -1.0f32..1.0, p_y in 0.0f32..1.0) {
            prop_assert_eq!(Line::new(vector![x_len, 0.0], point![0.0, 0.0]).orientation(&point![p_x, p_y]), Orientation::Left, "positive X");
            prop_assert_eq!(Line::new(vector![-x_len, 0.0], point![0.0, 0.0]).orientation(&point![p_x, p_y]), Orientation::Right, "negative X");
        }
    }

    #[rstest]
    #[case::y_pos_r(Line::new(vector![0.0, 2.0], point![1.0, 1.0]), point![2.0, 0.5], Orientation::Right)]
    #[case::y_pos_l(Line::new(vector![0.0, 2.0], point![1.0, 1.0]), point![0.5, 0.5], Orientation::Left)]
    #[case::y_neg_l(Line::new(vector![0.0, -2.], point![1.0, 1.0]), point![2.0, 0.5], Orientation::Left)]
    #[case::y_neg_r(Line::new(vector![0.0, -2.], point![1.0, 1.0]), point![0.5, 0.5], Orientation::Right)]
    #[case::x_pos_l(Line::new(vector![2.0, 0.0], point![1.0, 1.0]), point![0.5, 2.0], Orientation::Left)]
    #[case::x_pos_r(Line::new(vector![2.0, 0.0], point![1.0, 1.0]), point![0.5, 0.5], Orientation::Right)]
    #[case::x_neg_r(Line::new(vector![-2., 0.0], point![1.0, 1.0]), point![0.5, 2.0], Orientation::Right)]
    #[case::x_neg_l(Line::new(vector![-2., 0.0], point![1.0, 1.0]), point![0.5, 0.5], Orientation::Left)]
    #[case::c_pos_l(Line::new(vector![1.0, 1.0], point![1.0, 1.0]), point![1.0, 2.0], Orientation::Left)]
    #[case::c_pos_r(Line::new(vector![1.0, 1.0], point![1.0, 1.0]), point![1.0, 0.5], Orientation::Right)]
    #[case::c_neg_l(Line::new(vector![-1., -1.], point![1.0, 1.0]), point![1.5, 0.5], Orientation::Left)]
    #[case::c_neg_r(Line::new(vector![-1., -1.], point![1.0, 1.0]), point![1.5, 2.5], Orientation::Right)]
    #[case::a_pos_r(Line::new(vector![-1., 1.0], point![1.0, 1.0]), point![1.5, 1.5], Orientation::Right)]
    #[case::a_pos_l(Line::new(vector![-1., 1.0], point![1.0, 1.0]), point![0.5, 0.5], Orientation::Left)]
    #[case::a_neg_l(Line::new(vector![1.0, -1.], point![1.0, 1.0]), point![1.5, 1.5], Orientation::Left)]
    #[case::a_neg_r(Line::new(vector![1.0, -1.], point![1.0, 1.0]), point![0.5, 0.5], Orientation::Right)]
    #[case::real_case(Line::new(vector![-(1.7777 * 2.) , 0.], point![1.777, 1.0]), point![0.0, 0.0031], Orientation::Left)]
    fn orientation_axes(
        #[case] line: Line,
        #[case] point: Point2<f32>,
        #[case] orientation: Orientation,
    ) {
        assert_eq!(line.orientation(&point), orientation);
    }
}

pub struct Walls {
    left: Line,
    right: Line,
    top: Line,
    bottom: Line,
}

impl From<&Orthographic3<f32>> for Walls {
    fn from(proj: &Orthographic3<f32>) -> Self {
        Self {
            left: Line {
                a: point![proj.left(), proj.bottom()],
                b: point![proj.left(), proj.top()],
            },
            right: Line {
                a: point![proj.right(), proj.top()],
                b: point![proj.right(), proj.bottom()],
            },
            top: Line {
                a: point![proj.left(), proj.top()],
                b: point![proj.right(), proj.top()],
            },
            bottom: Line {
                a: point![proj.right(), proj.bottom()],
                b: point![proj.left(), proj.bottom()],
            },
        }
    }
}

impl Walls {
    fn as_slice(&self) -> [&Line; 4] {
        [&self.left, &self.right, &self.top, &self.bottom]
    }

    pub fn collide(&self, circle: &Circle) -> Vector2<f32> {
        self.as_slice()
            .iter()
            .filter_map(|wall| {
                let x = circle.collide(wall);
                println!("center={:?}, collision={x:?}, wall={wall:?}", circle.center);
                x
            })
            .sum()
    }
}

pub struct Circle {
    center: Point2<f32>,
    radius: f32,
}

impl Circle {
    pub fn new(center: Point2<f32>, radius: f32) -> Self {
        Circle { center, radius }
    }

    /// returns a line of length radius from the center to the closest point on the line
    fn tangent(&self, line: &Line) -> Line {
        let line_vn = line.vector().normalize();
        match line.orientation(&self.center) {
            Orientation::Right => Line::new(line_vn * self.radius, self.center).rotate(TAU / 4.),
            Orientation::Left => Line::new(line_vn * self.radius, self.center).rotate(-TAU / 4.),
        }
    }

    /// returns the point of intersection with the tangent radius and the line, and the endpoint of the tangent radius
    fn intersection(&self, line: &Line) -> Option<Line> {
        let tangent = self.tangent(line);
        tangent.intersection(line).map(|i| Line {
            a: i,
            b: tangent.end(),
        })
    }

    /// Computes the delta required to push the circle away from the line
    /// Assumes the initial circle position is on the right of the line, and is not colliding
    fn collide(&self, line: &Line) -> Option<Vector2<f32>> {
        // https://www.geogebra.org/calculator/eesqkde7
        match line.orientation(&self.center) {
            Orientation::Left => {
                let tangent = self.tangent(line);
                let infinite_intersection = tangent.infinite_intersection(line).unwrap(); // line is always tangent
                Some(((infinite_intersection - self.center) + tangent.vector()) * 2.)
            }
            Orientation::Right => self.intersection(line).map(|i| -i.vector() * 2.),
        }
    }
}
