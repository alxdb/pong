use crate::geometry::*;

use nalgebra as na;

#[derive(Debug, Clone)]
pub struct Body {
    figure: Figure,
    mass: Option<f64>,
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
        Self::new(Shape::Rectangle { w, h })
    }

    pub fn circle(r: f64) -> Self {
        Self::new(Shape::Circle { r })
    }

    pub fn position(mut self, position: na::Point2<f64>) -> Self {
        self.body.figure.center = position;
        self
    }

    pub fn mass(mut self, mass: f64) -> Self {
        self.body.mass = Some(mass);
        self
    }

    pub fn velocity(mut self, velocity: na::Vector2<f64>) -> Self {
        self.body.velocity = velocity;
        self
    }

    pub fn build(self) -> Body {
        self.body
    }
}

impl Body {
    fn collide(&mut self, other: &Body) {
        if self.figure.intersects(&other.figure) {
            // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional
            let distance = self.figure.center - other.figure.center;
            let velocity = self.velocity - other.velocity;
            let projection = (velocity.dot(&distance) / distance.dot(&distance)) * distance;

            let mass_ratio = if let (Some(m1), Some(m2)) = (self.mass, other.mass) {
                (2. * m2) / (m1 + m2)
            } else {
                2.
            };
            self.velocity = self.velocity - (mass_ratio * projection);
        }
    }

    pub fn figure(&self) -> &Figure {
        &self.figure
    }

    pub fn update(&mut self, delta: f64, collidables: &[&Body]) {
        for collidable in collidables {
            self.collide(&collidable);
        }
        self.figure.center += self.velocity * delta;
    }
}
