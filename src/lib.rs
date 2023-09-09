#![feature(iter_intersperse, slice_flatten)]

pub mod utils {
    use core::future::Future;

    use futures::executor::block_on;

    pub trait BlockFuture: Future {
        fn block(self) -> Self::Output;
    }

    impl<T: Future> BlockFuture for T {
        fn block(self) -> Self::Output {
            block_on(self)
        }
    }
}

pub mod graphics;
pub mod geometry {
    pub mod point {
        use std::ops;

        #[derive(Debug, Copy, Clone, PartialEq, Default)]
        pub struct Point {
            pub x: f64,
            pub y: f64,
        }

        impl From<[f64; 2]> for Point {
            fn from(value: [f64; 2]) -> Self {
                Point {
                    x: value[0],
                    y: value[1],
                }
            }
        }

        impl ops::Add for Point {
            type Output = Self;

            fn add(self, rhs: Point) -> Point {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl ops::Sub for Point {
            type Output = Self;

            fn sub(self, rhs: Point) -> Point {
                Point {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }
    }
}
