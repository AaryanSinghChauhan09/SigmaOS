//! SigmaOS Interrupt Manager Subsystem

use super::handler::{HandlerId, HandlerMetadata, HandlerMode, InterruptPriority};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

/// Typed error for interrupt management operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterruptError {
    AlreadyRegistered(u32),
    NoSuchHandler(HandlerId),
    IrqNotAssigned(u32),
    TableFull,
    InvalidIrq(u32),
    LostInterruptThresholdExceeded(u32),
}

/// The Interrupt Manager governs IRQ routing, handler dispatching, double-handler guards, and lost interrupt detection.
pub struct InterruptManager {
    max_handlers: usize,
    next_handler_id: AtomicU64,
    handlers: BTreeMap<HandlerId, HandlerMetadata>,
    irq_map: BTreeMap<u32, HandlerId>,
    lost_interrupt_counters: BTreeMap<u32, ArcAtomicU32>,
    spurious_interrupts: AtomicU64,
}

#[derive(Clone)]
struct ArcAtomicU32(Arc<AtomicU32>);

impl ArcAtomicU32 {
    fn new(val: u32) -> Self {
        Self(Arc::new(AtomicU32::new(val)))
    }
    fn get(&self) -> u32 {
        self.0.load(Ordering::Relaxed)
    }
    fn inc(&self) -> u32 {
        self.0.fetch_add(1, Ordering::Relaxed) + 1
    }
}

impl InterruptManager {
    /// Create a new InterruptManager with a bounded maximum capacity.
    pub fn new(max_handlers: usize) -> Self {
        Self {
            max_handlers,
            next_handler_id: AtomicU64::new(1),
            handlers: BTreeMap::new(),
            irq_map: BTreeMap::new(),
            lost_interrupt_counters: BTreeMap::new(),
            spurious_interrupts: AtomicU64::new(0),
        }
    }

    /// Register a new handler for a given IRQ line.
    /// Prevents double-handler registration unless shared IRQs are enabled.
    pub fn register_handler(
        &mut self,
        irq: u32,
        name: impl Into<String>,
        priority: InterruptPriority,
        mode: HandlerMode,
    ) -> Result<HandlerId, InterruptError> {
        if irq >= 1024 {
            return Err(InterruptError::InvalidIrq(irq));
        }

        if self.handlers.len() >= self.max_handlers {
            return Err(InterruptError::TableFull);
        }

        if self.irq_map.contains_key(&irq) {
            return Err(InterruptError::AlreadyRegistered(irq));
        }

        let id = self.next_handler_id.fetch_add(1, Ordering::SeqCst);
        let metadata = HandlerMetadata::new(id, name, irq, priority, mode);

        self.handlers.insert(id, metadata);
        self.irq_map.insert(irq, id);
        self.lost_interrupt_counters
            .entry(irq)
            .or_insert_with(|| ArcAtomicU32::new(0));

        Ok(id)
    }

    /// Unregister a handler by Handler ID.
    pub fn unregister_handler(&mut self, id: HandlerId) -> Result<(), InterruptError> {
        if let Some(meta) = self.handlers.remove(&id) {
            self.irq_map.remove(&meta.irq);
            Ok(())
        } else {
            Err(InterruptError::NoSuchHandler(id))
        }
    }

    /// Dispatch an IRQ signal.
    /// Records invocation, handles top-half or bottom-half offload, and detects lost interrupts.
    pub fn dispatch_irq(&self, irq: u32) -> Result<HandlerId, InterruptError> {
        if let Some(&handler_id) = self.irq_map.get(&irq) {
            if let Some(meta) = self.handlers.get(&handler_id) {
                let _ = meta.record_invocation();
                Ok(handler_id)
            } else {
                Err(InterruptError::IrqNotAssigned(irq))
            }
        } else {
            self.spurious_interrupts.fetch_add(1, Ordering::Relaxed);
            if let Some(counter) = self.lost_interrupt_counters.get(&irq) {
                let _lost = counter.inc();
                if _lost > 100 {
                    return Err(InterruptError::LostInterruptThresholdExceeded(irq));
                }
            }
            Err(InterruptError::IrqNotAssigned(irq))
        }
    }

    pub fn get_handler(&self, id: HandlerId) -> Option<&HandlerMetadata> {
        self.handlers.get(&id)
    }

    pub fn spurious_count(&self) -> u64 {
        self.spurious_interrupts.load(Ordering::Relaxed)
    }
}
