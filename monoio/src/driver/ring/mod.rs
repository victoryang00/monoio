//! Monoio Uring Driver.

use std::{
    cell::UnsafeCell,
    io,
    mem::ManuallyDrop,
    os::unix::prelude::{AsRawFd, RawFd},
    rc::Rc,
    task::{Context, Poll},
    time::Duration,
};

use ioring_rs::IoRing;

pub(crate) struct RingInner {
    /// In-flight operations
    ops: Ops,

    /// IoUring bindings
    uring: ManuallyDrop<IoRing>,

    /// Shared waker
    #[cfg(feature = "sync")]
    shared_waker: std::sync::Arc<waker::EventWaker>,

    // Mark if eventfd is in the ring
    #[cfg(feature = "sync")]
    eventfd_installed: bool,

    // Waker receiver
    #[cfg(feature = "sync")]
    waker_receiver: flume::Receiver<std::task::Waker>,
}

pub mod op {
    pub struct CompletionMeta {}
    pub struct Op<T> {
        pub data: T,
    }
    pub trait OpAble {}
}
