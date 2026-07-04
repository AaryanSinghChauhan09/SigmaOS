// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: sigma-interrupt-coalesce - Adaptive interrupt batching
//! Hand-rolled zero-dependency implementation, no_std, no pre-defined libraries/functions
//! =========================================================================

#![no_std]

/// Interrupt coalescer state
pub struct InterruptCoalescer {
    pending_count: usize,
    max_threshold: usize,
    timeout_us: u64,
    last_trigger_time: u64,
}

impl InterruptCoalescer {
    pub const fn new(max_threshold: usize, timeout_us: u64) -> Self {
        Self {
            pending_count: 0,
            max_threshold,
            timeout_us,
            last_trigger_time: 0,
        }
    }

    /// Record an interrupt, returns true if we should trigger processing now
    pub fn record_interrupt(&mut self, current_time_us: u64) -> bool {
        self.pending_count += 1;
        
        if self.pending_count >= self.max_threshold {
            self.trigger(current_time_us);
            return true;
        }
        
        if self.last_trigger_time == 0 {
            self.last_trigger_time = current_time_us;
        } else if current_time_us - self.last_trigger_time >= self.timeout_us {
            self.trigger(current_time_us);
            return true;
        }
        
        false
    }

    fn trigger(&mut self, current_time_us: u64) {
        self.pending_count = 0;
        self.last_trigger_time = current_time_us;
    }

    /// Update max threshold dynamically
    pub fn set_max_threshold(&mut self, max: usize) {
        self.max_threshold = max;
    }
}

/// Registry for all interrupt coalescers
pub struct CoalescerRegistry {
    coalescers: [Option<&'static mut InterruptCoalescer>; 32],
}

impl CoalescerRegistry {
    pub const fn new() -> Self {
        Self { coalescers: [None; 32] }
    }

    pub fn register(&mut self, idx: usize, coalescer: &'static mut InterruptCoalescer) {
        if idx < 32 {
            self.coalescers[idx] = Some(coalescer);
        }
    }
}
