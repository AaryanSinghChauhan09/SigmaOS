//! SigmaOS I/O Subsystem Management Engine
//! Enforces the 10-stage authoritative I/O lifecycle contract:
//! submit -> validate -> enqueue -> execute -> interrupt/DMA -> complete -> callback -> reclaim -> free -> error/recovery.
pub mod io_uring;

pub use io_uring::{
    FixedBuffer, IoUringCqe, IoUringOp, IoUringRing, IoUringSqe, IORING_CQE_F_BUFFER,
    IORING_CQE_F_MORE, IORING_MAX_ENTRIES, IOSQE_ASYNC, IOSQE_FIXED_FILE, IOSQE_IO_LINK,
};

// io_uring v2: Advanced Async I/O (multishot, zero-copy, NVMe passthrough)
pub mod io_uring_v2;
pub use io_uring_v2::{
    CqeV2, IoUringOpV2, IoUringRegisteredBuf, IoUringV2, MultishotState, NvmeUringCmd,
    RegisteredFileType,
};

// Core I/O Lifecycle Managers
pub mod handler;
pub mod interrupt;
pub use handler::{HandlerId, HandlerMetadata, HandlerMode, InterruptPriority};
pub use interrupt::{InterruptError, InterruptManager};

pub mod dma;
pub mod dma_queue;
pub use dma::{DmaError, DmaManager};
pub use dma_queue::{DmaDescriptor, DmaDirection, ScatterGatherElement};

pub mod scheduler;
pub mod scheduler_queue;
pub use scheduler::{IoScheduler, SchedulerError};
pub use scheduler_queue::{IoOperationKind, IoPriority, IoRequest, IoStage};

pub mod callback;
pub use callback::{CallbackHandle, CallbackManager, CallbackStatus};

pub mod sovereign_io_management_and_pr_suite;
pub use sovereign_io_management_and_pr_suite::{
    IoPrEntry, IoPrGatewayTracker, PrStatus, SovereignIoManagementAndPrSuite,
};
