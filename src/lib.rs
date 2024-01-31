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