//! SigmaOS Interrupt Handler Definition & Management

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Identifier for an interrupt handler registration.
pub type HandlerId = u64;

/// Priority level for top-half or bottom-half interrupt processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterruptPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Execution mode for interrupt handlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerMode {
    /// Top-half: direct execution in hard IRQ context (must be ultra-fast).
    TopHalf,
    /// Bottom-half: deferred execution via tasklet / worker thread.
    BottomHalf,
}

/// Metadata associated with an interrupt handler.
#[derive(Debug, Clone)]
pub struct HandlerMetadata {
    pub id: HandlerId,
    pub name: String,
    pub irq: u32,
    pub priority: InterruptPriority,
    pub mode: HandlerMode,
    pub invocations: Arc<AtomicU64>,
}

impl HandlerMetadata {
    pub fn new(id: HandlerId, name: impl Into<String>, irq: u32, priority: InterruptPriority, mode: HandlerMode) -> Self {
        Self {
            id,
            name: name.into(),
            irq,
            priority,
            mode,
            invocations: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_invocation(&self) -> u64 {
        self.invocations.fetch_add(1, Ordering::Relaxed) + 1
    }
}
