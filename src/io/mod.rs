//! SigmaOS I/O subsystem
pub mod io_uring;

pub use io_uring::{
    IoUringRing, IoUringSqe, IoUringCqe, IoUringOp, FixedBuffer,
    IORING_MAX_ENTRIES, IOSQE_IO_LINK, IOSQE_FIXED_FILE, IOSQE_ASYNC,
    IORING_CQE_F_BUFFER, IORING_CQE_F_MORE,
};
