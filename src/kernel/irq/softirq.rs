/// SigmaOS Softirq and tasklets deferred execution engine
/// Handles lower-priority interrupt bottom-half processing

use std::collections::VecDeque;
use core::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftirqType {
    Hi = 0,
    Timer = 1,
    NetTx = 2,
    NetRx = 3,
    Block = 4,
    Tasklet = 5,
}

pub struct SoftirqAction {
    pub kind: SoftirqType,
    pub action: fn(),
}

pub struct SoftirqEngine {
    pending_flags: AtomicU32,
    actions: Vec<Option<fn()>>,
    tasklet_queue: VecDeque<fn()>,
}

impl SoftirqEngine {
    pub fn new() -> Self {
        SoftirqEngine {
            pending_flags: AtomicU32::new(0),
            actions: vec![None; 6],
            tasklet_queue: VecDeque::new(),
        }
    }

    pub fn register_action(&mut self, kind: SoftirqType, action: fn()) {
        let idx = kind as usize;
        if idx < self.actions.len() {
            self.actions[idx] = Some(action);
        }
    }

    pub fn raise_softirq(&self, kind: SoftirqType) {
        let bit = 1 << (kind as u32);
        self.pending_flags.fetch_or(bit, Ordering::SeqCst);
    }

    pub fn queue_tasklet(&mut self, action: fn()) {
        self.tasklet_queue.push_back(action);
        self.raise_softirq(SoftirqType::Tasklet);
    }

    pub fn execute_pending(&mut self) -> usize {
        let pending = self.pending_flags.swap(0, Ordering::SeqCst);
        let mut count = 0;

        for i in 0..6 {
            if pending & (1 << i) != 0 {
                if i == SoftirqType::Tasklet as usize {
                    while let Some(tasklet) = self.tasklet_queue.pop_front() {
                        tasklet();
                        count += 1;
                    }
                } else if let Some(action) = self.actions[i] {
                    action();
                    count += 1;
                }
            }
        }
        count
    }
}

impl Default for SoftirqEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    static CALLED_COUNT: AtomicUsize = AtomicUsize::new(0);

    fn mock_action() {
        CALLED_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_softirq_engine() {
        let mut engine = SoftirqEngine::new();
        engine.register_action(SoftirqType::Timer, mock_action);
        
        engine.raise_softirq(SoftirqType::Timer);
        assert_eq!(engine.execute_pending(), 1);
        assert_eq!(CALLED_COUNT.load(Ordering::SeqCst), 1);
        
        engine.queue_tasklet(mock_action);
        assert_eq!(engine.execute_pending(), 1);
        assert_eq!(CALLED_COUNT.load(Ordering::SeqCst), 2);
    }
}
