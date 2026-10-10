// SigmaOS Swap Subsystem Module
// Core swap-out, swap-in, slot allocation, and swap request queueing.

pub mod core;
pub mod enqueue;

pub use core::{SwapEntry, SwapError, SwapManager, SwapResult};
pub use enqueue::{SwapPriority, SwapQueueManager, SwapRequest};
