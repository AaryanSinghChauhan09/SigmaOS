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

// S-Signal Dispatcher (Capability-Gated Async Signals)
// Sovereign AI-Native zero-dependency implementation

extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignSignal {
    Terminate,
    Interrupt,
    PageFault,
    PowerStateTransition,
}

pub struct SignalDispatcher {
    pub pending_signals: [Option<(u32, SovereignSignal)>; 16],
}

impl SignalDispatcher {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            pending_signals: [None; 16],
        }
    }

    pub fn raise_signal(
        &mut self,
        target_pid: u32,
        signal: SovereignSignal,
        is_sender_allowed: bool,
    ) -> Result<(), &'static str> {
        if !is_sender_allowed {
            return Err("Sender process lacks capability to raise signal to target");
        }
        for slot in self.pending_signals.iter_mut() {
            if slot.is_none() {
                *slot = Some((target_pid, signal));
                return Ok(());
            }
        }
        Err("Signal queue is full")
    }

    pub fn poll_signal(&mut self, target_pid: u32) -> Option<SovereignSignal> {
        for slot in self.pending_signals.iter_mut() {
            if let Some((pid, sig)) = slot {
                if *pid == target_pid {
                    let sig_to_return = *sig;
                    *slot = None;
                    return Some(sig_to_return);
                }
            }
        }
        None
    }

    pub fn pending_count(&self) -> usize {
        self.pending_signals.iter().filter(|s| s.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.pending_count() == 0
    }

    pub fn is_full(&self) -> bool {
        self.pending_count() >= 16
    }

    pub fn clear(&mut self) {
        self.pending_signals = [None; 16];
    }

    pub fn has_pending_for_pid(&self, target_pid: u32) -> bool {
        self.pending_signals.iter().any(|s| {
            if let Some((pid, _)) = s {
                *pid == target_pid
            } else {
                false
            }
        })
    }
}

impl Default for SignalDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_dispatcher_creation() {
        let dispatcher = SignalDispatcher::new();
        assert!(dispatcher.is_empty());
        assert!(!dispatcher.is_full());
        assert_eq!(dispatcher.pending_count(), 0);
    }

    #[test]
    fn test_raise_and_poll_signal() {
        let mut dispatcher = SignalDispatcher::new();
        
        assert!(dispatcher.raise_signal(1, SovereignSignal::Terminate, true).is_ok());
        assert_eq!(dispatcher.pending_count(), 1);
        
        let signal = dispatcher.poll_signal(1);
        assert!(signal.is_some());
        assert_eq!(signal.unwrap(), SovereignSignal::Terminate);
        assert!(dispatcher.is_empty());
    }

    #[test]
    fn test_capability_check() {
        let mut dispatcher = SignalDispatcher::new();
        
        // Should fail without capability
        assert!(dispatcher.raise_signal(1, SovereignSignal::Terminate, false).is_err());
        
        // Should succeed with capability
        assert!(dispatcher.raise_signal(1, SovereignSignal::Terminate, true).is_ok());
    }

    #[test]
    fn test_signal_queue_full() {
        let mut dispatcher = SignalDispatcher::new();
        
        // Fill the queue
        for i in 0..16 {
            assert!(dispatcher.raise_signal(i, SovereignSignal::Interrupt, true).is_ok());
        }
        
        assert!(dispatcher.is_full());
        
        // Should fail when queue is full
        assert!(dispatcher.raise_signal(100, SovereignSignal::Terminate, true).is_err());
    }

    #[test]
    fn test_poll_wrong_pid() {
        let mut dispatcher = SignalDispatcher::new();
        
        dispatcher.raise_signal(1, SovereignSignal::Terminate, true).unwrap();
        
        // Process 2 should not receive signal meant for process 1
        assert!(dispatcher.poll_signal(2).is_none());
        
        // Process 1 should receive the signal
        assert!(dispatcher.poll_signal(1).is_some());
    }

    #[test]
    fn test_multiple_signals_same_pid() {
        let mut dispatcher = SignalDispatcher::new();
        
        dispatcher.raise_signal(1, SovereignSignal::Terminate, true).unwrap();
        dispatcher.raise_signal(1, SovereignSignal::Interrupt, true).unwrap();
        
        assert_eq!(dispatcher.pending_count(), 2);
        assert!(dispatcher.has_pending_for_pid(1));
    }

    #[test]
    fn test_clear_dispatcher() {
        let mut dispatcher = SignalDispatcher::new();
        
        dispatcher.raise_signal(1, SovereignSignal::Terminate, true).unwrap();
        dispatcher.raise_signal(2, SovereignSignal::Interrupt, true).unwrap();
        
        assert_eq!(dispatcher.pending_count(), 2);
        
        dispatcher.clear();
        assert!(dispatcher.is_empty());
        assert_eq!(dispatcher.pending_count(), 0);
    }

    #[test]
    fn test_all_signal_types() {
        let mut dispatcher = SignalDispatcher::new();
        
        let signals = [
            SovereignSignal::Terminate,
            SovereignSignal::Interrupt,
            SovereignSignal::PageFault,
            SovereignSignal::PowerStateTransition,
        ];
        
        for (i, signal) in signals.iter().enumerate() {
            dispatcher.raise_signal(i as u32, *signal, true).unwrap();
        }
        
        for (i, signal) in signals.iter().enumerate() {
            let received = dispatcher.poll_signal(i as u32);
            assert!(received.is_some());
            assert_eq!(received.unwrap(), *signal);
        }
    }
}
