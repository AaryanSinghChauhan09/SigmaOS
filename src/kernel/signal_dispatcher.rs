// S-SIGNAL DISPATCHER (Capability-Gated Async Signals)
// Asynchronous signal handling with capability-based access control

#![no_std]

extern crate alloc;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_dispatcher_initialization() {
        let dispatcher = SignalDispatcher::new();
        assert_eq!(dispatcher.pending_signals.len(), 16);
    }

    #[test]
    fn test_raise_signal_with_capability() {
        let mut dispatcher = SignalDispatcher::new();
        
        let result = dispatcher.raise_signal(100, SovereignSignal::Terminate, true);
        assert!(result.is_ok());
        assert!(dispatcher.pending_signals[0].is_some());
    }

    #[test]
    fn test_raise_signal_without_capability() {
        let mut dispatcher = SignalDispatcher::new();
        
        let result = dispatcher.raise_signal(100, SovereignSignal::Terminate, false);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sender process lacks capability to raise signal to target");
    }

    #[test]
    fn test_poll_signal() {
        let mut dispatcher = SignalDispatcher::new();
        
        dispatcher.raise_signal(100, SovereignSignal::Interrupt, true).unwrap();
        
        let signal = dispatcher.poll_signal(100);
        assert!(signal.is_some());
        assert_eq!(signal.unwrap(), SovereignSignal::Interrupt);
    }

    #[test]
    fn test_poll_signal_wrong_pid() {
        let mut dispatcher = SignalDispatcher::new();
        
        dispatcher.raise_signal(100, SovereignSignal::Interrupt, true).unwrap();
        
        // Try to poll with wrong PID
        let signal = dispatcher.poll_signal(999);
        assert!(signal.is_none());
    }

    #[test]
    fn test_signal_queue_full() {
        let mut dispatcher = SignalDispatcher::new();
        
        // Fill all 16 signal slots
        for i in 0..16 {
            dispatcher.raise_signal(i as u32, SovereignSignal::Terminate, true).unwrap();
        }
        
        // Try to raise one more signal (should fail)
        let result = dispatcher.raise_signal(100, SovereignSignal::Terminate, true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Signal queue is full");
    }

    #[test]
    fn test_signal_consumption() {
        let mut dispatcher = SignalDispatcher::new();
        
        dispatcher.raise_signal(100, SovereignSignal::Terminate, true).unwrap();
        assert!(dispatcher.pending_signals[0].is_some());
        
        dispatcher.poll_signal(100);
        assert!(dispatcher.pending_signals[0].is_none());
    }

    #[test]
    fn test_multiple_signals_same_pid() {
        let mut dispatcher = SignalDispatcher::new();
        
        dispatcher.raise_signal(100, SovereignSignal::Terminate, true).unwrap();
        dispatcher.raise_signal(100, SovereignSignal::Interrupt, true).unwrap();
        
        let signal1 = dispatcher.poll_signal(100);
        assert!(signal1.is_some());
        
        let signal2 = dispatcher.poll_signal(100);
        assert!(signal2.is_some());
        
        // Should be no more signals
        let signal3 = dispatcher.poll_signal(100);
        assert!(signal3.is_none());
    }

    #[test]
    fn test_signal_types() {
        let mut dispatcher = SignalDispatcher::new();
        
        dispatcher.raise_signal(100, SovereignSignal::PageFault, true).unwrap();
        dispatcher.raise_signal(200, SovereignSignal::PowerStateTransition, true).unwrap();
        
        let signal1 = dispatcher.poll_signal(100);
        assert_eq!(signal1.unwrap(), SovereignSignal::PageFault);
        
        let signal2 = dispatcher.poll_signal(200);
        assert_eq!(signal2.unwrap(), SovereignSignal::PowerStateTransition);
    }

    #[test]
    fn test_sovereign_signal_variants() {
        let terminate = SovereignSignal::Terminate;
        let interrupt = SovereignSignal::Interrupt;
        let page_fault = SovereignSignal::PageFault;
        let power_transition = SovereignSignal::PowerStateTransition;
        
        assert_eq!(terminate, SovereignSignal::Terminate);
        assert_eq!(interrupt, SovereignSignal::Interrupt);
        assert_eq!(page_fault, SovereignSignal::PageFault);
        assert_eq!(power_transition, SovereignSignal::PowerStateTransition);
        
        assert_ne!(terminate, interrupt);
        assert_ne!(interrupt, page_fault);
    }
}