use super::geometry::Shape;
use nalgebra as na;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Body {
    center: na::Point2<f64>,
    mass: f64,
    velocity: na::Vector2<f64>,
    shape: Shape,
}

#[derive(Debug, Clone)]
pub struct BodyBuilder {
    body: Body,
}

impl BodyBuilder {
    pub fn new(shape: Shape) -> Self {
        BodyBuilder {
            body: Body {
                center: Default::default(),
                mass: Default::default(),
                velocity: Default::default(),
                shape,
            },
        }
    }

    pub fn rect(w: f64, h: f64) -> Self {
        Self::new(Shape::Rectangle { w, h })
    }

    pub fn circle(r: f64) -> Self {
        Self::new(Shape::Circle { r })
    }

    pub fn center(mut self, center: na::Point2<f64>) -> Self {
        self.body.center = center;
        self
    }

    pub fn build(self) -> Body {
        self.body
    }

    pub fn build_arc(self) -> Arc<Body> {
        Arc::new(self.build())
    }
}

impl Body {
    pub fn new(
        center: na::Point2<f64>,
        mass: f64,
        velocity: na::Vector2<f64>,
        shape: Shape,
    ) -> Self {
        Body {
            center,
            mass,
            velocity,
            shape,
        }
    }

    pub fn collide(&mut self, other: &Body) {
        if self.intersects(other) {
            // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional
            let distance = self.center - other.center;

            self.velocity -= ((2. * other.mass) / (self.mass + other.mass))
                * ((self.velocity - other.velocity).dot(&distance) / distance.norm_squared())
                * distance;
        }
    }

    pub fn intersects(&self, other: &Body) -> bool {
        todo!()
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn translation(&self) -> na::Translation2<f64> {
        self.center.into()
    }
}
