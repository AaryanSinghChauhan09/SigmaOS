//! SigmaOS I/O subsystem
pub mod io_uring;

pub use io_uring::{
    FixedBuffer, IoUringCqe, IoUringOp, IoUringRing, IoUringSqe, IORING_CQE_F_BUFFER,
    IORING_CQE_F_MORE, IORING_MAX_ENTRIES, IOSQE_ASYNC, IOSQE_FIXED_FILE, IOSQE_IO_LINK,
};

// io_uring v2: Advanced Async I/O (multishot, zero-copy, NVMe passthrough)
pub mod io_uring_v2;
pub use io_uring_v2::{
    IoUringV2, IoUringOpV2, IoUringRegisteredBuf, CqeV2,
    MultishotState, NvmeUringCmd, RegisteredFileType,
};
