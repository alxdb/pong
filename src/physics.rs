use crate::geometry::*;

use nalgebra as na;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Body {
    figure: Figure,
    mass: f64,
    velocity: na::Vector2<f64>,
}

#[derive(Debug, Clone)]
pub struct BodyBuilder {
    body: Body,
}

impl BodyBuilder {
    pub fn new(shape: Shape) -> Self {
        BodyBuilder {
            body: Body {
                figure: Figure {
                    shape,
                    center: Default::default(),
                },
                mass: Default::default(),
                velocity: Default::default(),
            },
        }
    }

    pub fn rect(w: f64, h: f64) -> Self {
        Self::new(Shape::Rectangle(Rectangle { w, h }))
    }

    pub fn circle(r: f64) -> Self {
        Self::new(Shape::Circle(Circle { r }))
    }

    pub fn center(mut self, center: na::Point2<f64>) -> Self {
        self.body.figure.center = center;
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
    pub fn collide(&mut self, other: &Body) {
        if self.figure.intersects(&other.figure) {
            // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional
            let distance = self.figure.center - other.figure.center;

            let mass_ratio = (2. * other.mass) / (self.mass + other.mass);
            let velocity_delta = self.velocity - other.velocity;
            let magnitude = velocity_delta.dot(&distance) / distance.norm_squared();

            self.velocity -= mass_ratio * magnitude * distance;
        }
    }

    pub fn figure(&self) -> &Figure {
        &self.figure
    }

    pub fn set_position(&mut self, position: na::Point2<f64>) {
        self.figure.center = position;
    }
}
