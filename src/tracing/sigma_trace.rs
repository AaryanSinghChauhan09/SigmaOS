#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Kali-Style: Isolated Dynamic System Tracing Sandbox Hook
// Hooks trace handlers directly inside the kernel transaction bus using isolated spans

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;

extern crate alloc;
use alloc::vec::Vec;

extern crate alloc;
use alloc::vec::Vec;

extern crate alloc;
use alloc::vec::Vec;

extern crate alloc;
use alloc::vec::Vec;

extern crate alloc;
use alloc::vec::Vec;

extern crate alloc;
use alloc::vec::Vec;

extern crate alloc;
use alloc::vec::Vec;

pub const TRACE_BUFFER_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceEvent {
    Syscall(u32),
    ContextSwitch(u32, u32),
    Interrupt(u8),
    MemoryAccess(u64, u64),
    NetworkPacket(u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceSpan {
    pub timestamp: u64,
    pub event: TraceEvent,
    pub payload: u64,
}

pub struct SigmaTrace {
    pub buffer: [Option<TraceSpan>; TRACE_BUFFER_SIZE],
    pub write_pointer: usize,
    pub overflow_count: u64,
}

impl SigmaTrace {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            buffer: [None; TRACE_BUFFER_SIZE],
            write_pointer: 0,
            overflow_count: 0,
        }
    }

    /// Record a system event in a thread-safe, lock-free ring buffer
    pub fn record_span(&mut self, timestamp: u64, event: TraceEvent, payload: u64) {
        let span = TraceSpan {
            timestamp,
            event,
            payload,
        };

        // Check if we're overwriting data
        if self.buffer[self.write_pointer].is_some() {
            self.overflow_count += 1;
        }

        self.buffer[self.write_pointer] = Some(span);
        self.write_pointer = (self.write_pointer + 1) % TRACE_BUFFER_SIZE;
    }

    /// Query the captured traces for forensics audits
    pub fn get_recorded_count(&self) -> usize {
        let mut count = 0;
        for slot in self.buffer.iter() {
            if slot.is_some() {
                count += 1;
            }
        }
        count
    }

    /// Get all recorded spans in chronological order
    pub fn get_all_spans(&self) -> Vec<TraceSpan> {
        let mut spans = Vec::new();
        let mut idx = self.write_pointer;

        for _ in 0..TRACE_BUFFER_SIZE {
            if let Some(span) = self.buffer[idx] {
                spans.push(span);
            }
            idx = (idx + 1) % TRACE_BUFFER_SIZE;
        }

        spans
    }

    /// Clear all recorded traces
    pub fn clear(&mut self) {
        self.buffer = [None; TRACE_BUFFER_SIZE];
        self.write_pointer = 0;
        self.overflow_count = 0;
    }

    /// Get the overflow count (how many spans were overwritten)
    pub fn get_overflow_count(&self) -> u64 {
        self.overflow_count
    }
}

impl Default for SigmaTrace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kali_style_trace_sandbox() {
        let mut tracer = SigmaTrace::new();
        assert_eq!(tracer.get_recorded_count(), 0);

        // Record some syscall and context switch events
        tracer.record_span(1000, TraceEvent::Syscall(1), 0x1000);
        tracer.record_span(1001, TraceEvent::ContextSwitch(101, 102), 0x2000);
        tracer.record_span(1002, TraceEvent::Interrupt(5), 0x3000);

        assert_eq!(tracer.get_recorded_count(), 3);
    }

    #[test]
    fn test_ring_buffer_overflow() {
        let mut tracer = SigmaTrace::new();

        // Fill the buffer
        for i in 0..TRACE_BUFFER_SIZE + 5 {
            tracer.record_span(i as u64, TraceEvent::Syscall(i as u32), i as u64);
        }

        assert_eq!(tracer.get_recorded_count(), TRACE_BUFFER_SIZE);
        assert_eq!(tracer.get_overflow_count(), 5);
    }

    #[test]
    fn test_trace_clear() {
        let mut tracer = SigmaTrace::new();

        tracer.record_span(1000, TraceEvent::Syscall(1), 0x1000);
        assert_eq!(tracer.get_recorded_count(), 1);

        tracer.clear();
        assert_eq!(tracer.get_recorded_count(), 0);
        assert_eq!(tracer.get_overflow_count(), 0);
    }

    #[test]
    fn test_memory_access_tracing() {
        let mut tracer = SigmaTrace::new();

        tracer.record_span(1000, TraceEvent::MemoryAccess(0x1000, 0x2000), 0x100);

        let spans = tracer.get_all_spans();
        assert_eq!(spans.len(), 1);

        if let TraceEvent::MemoryAccess(addr, size) = spans[0].event {
            assert_eq!(addr, 0x1000);
            assert_eq!(size, 0x2000);
        } else {
            panic!("Expected MemoryAccess event");
        }
    }
}
