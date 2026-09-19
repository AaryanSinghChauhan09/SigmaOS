// SPDX-License-Identifier: MIT
// SigmaOS RCU (Read-Copy-Update) Synchronization
// Scalable read-mostly data structure synchronization inspired by Linux RCU

#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use std::collections::VecDeque;

/// RCU epoch type
pub type RcuEpoch = u64;

/// RCU grace period ID
pub type GracePeriodId = u64;

/// RCU callback function type
pub type RcuCallback = fn(u64) -> Result<(), &'static str>;

/// RCU callback descriptor
#[derive(Debug)]
pub struct RcuCallbackDescriptor {
    pub id: u64,
    pub callback: RcuCallback,
    pub user_data: u64,
    pub registered_at: RcuEpoch,
}

impl RcuCallbackDescriptor {
    pub fn new(id: u64, callback: RcuCallback, user_data: u64, registered_at: RcuEpoch) -> Self {
        RcuCallbackDescriptor {
            id,
            callback,
            user_data,
            registered_at,
        }
    }
}

/// RCU state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RcuState {
    /// RCU is idle
    Idle,
    /// RCU is in grace period
    InGracePeriod,
    /// RCU is processing callbacks
    Processing,
}

/// RCU subsystem
#[derive(Debug)]
pub struct RcuSubsystem {
    pub current_epoch: AtomicU64,
    pub grace_period_start: AtomicU64,
    pub state: AtomicU32, // RcuState as u32
    pub pending_callbacks: VecDeque<RcuCallbackDescriptor>,
    pub next_callback_id: AtomicU64,
    pub readers_count: AtomicU32,
    pub grace_period_id: AtomicU64,
}

impl RcuSubsystem {
    pub fn new() -> Self {
        RcuSubsystem {
            current_epoch: AtomicU64::new(0),
            grace_period_start: AtomicU64::new(0),
            state: AtomicU32::new(RcuState::Idle as u32),
            pending_callbacks: VecDeque::new(),
            next_callback_id: AtomicU64::new(1),
            readers_count: AtomicU32::new(0),
            grace_period_id: AtomicU64::new(1),
        }
    }

    /// Enter read-side critical section
    pub fn read_lock(&self) -> RcuEpoch {
        self.readers_count.fetch_add(1, Ordering::SeqCst);
        self.current_epoch.load(Ordering::SeqCst)
    }

    /// Exit read-side critical section
    pub fn read_unlock(&self, _epoch: RcuEpoch) {
        self.readers_count.fetch_sub(1, Ordering::SeqCst);
    }

    /// Register a callback to be called after grace period
    pub fn register_callback(&mut self, callback: RcuCallback, user_data: u64) -> u64 {
        let id = self.next_callback_id.fetch_add(1, Ordering::SeqCst);
        let epoch = self.current_epoch.load(Ordering::SeqCst);
        
        let desc = RcuCallbackDescriptor::new(id, callback, user_data, epoch);
        self.pending_callbacks.push_back(desc);
        
        id
    }

    /// Begin a grace period
    pub fn synchronize_rcu(&mut self) -> GracePeriodId {
        let gp_id = self.grace_period_id.fetch_add(1, Ordering::SeqCst);
        let epoch = self.current_epoch.load(Ordering::SeqCst);
        
        self.grace_period_start.store(epoch, Ordering::SeqCst);
        self.state.store(RcuState::InGracePeriod as u32, Ordering::SeqCst);
        
        gp_id
    }

    /// Check if grace period has ended
    pub fn grace_period_ended(&self) -> bool {
        let readers = self.readers_count.load(Ordering::SeqCst);
        let gp_start = self.grace_period_start.load(Ordering::SeqCst);
        let current = self.current_epoch.load(Ordering::SeqCst);
        
        readers == 0 && (current > gp_start)
    }

    /// Advance to next epoch
    pub fn advance_epoch(&self) {
        self.current_epoch.fetch_add(1, Ordering::SeqCst);
    }

    /// Process pending callbacks after grace period
    pub fn process_callbacks(&mut self) -> Vec<u64> {
        let mut processed = Vec::new();
        
        if self.grace_period_ended() {
            self.state.store(RcuState::Processing as u32, Ordering::SeqCst);
            
            while let Some(callback) = self.pending_callbacks.pop_front() {
                let id = callback.id;
                let user_data = callback.user_data;
                
                if let Err(_) = (callback.callback)(user_data) {
                    // Callback failed, but continue processing
                }
                
                processed.push(id);
            }
            
            self.state.store(RcuState::Idle as u32, Ordering::SeqCst);
        }
        
        processed
    }

    /// Get current state
    pub fn get_state(&self) -> RcuState {
        match self.state.load(Ordering::SeqCst) {
            0 => RcuState::Idle,
            1 => RcuState::InGracePeriod,
            2 => RcuState::Processing,
            _ => RcuState::Idle,
        }
    }

    /// Get current epoch
    pub fn get_epoch(&self) -> RcuEpoch {
        self.current_epoch.load(Ordering::SeqCst)
    }

    /// Get pending callback count
    pub fn pending_callback_count(&self) -> usize {
        self.pending_callbacks.len()
    }

    /// Get active reader count
    pub fn active_reader_count(&self) -> u32 {
        self.readers_count.load(Ordering::SeqCst)
    }
}

impl Default for RcuSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rcu_read_lock_unlock() {
        let rcu = RcuSubsystem::new();
        
        let epoch = rcu.read_lock();
        assert_eq!(rcu.active_reader_count(), 1);
        
        rcu.read_unlock(epoch);
        assert_eq!(rcu.active_reader_count(), 0);
    }

    #[test]
    fn test_rcu_callback_registration() {
        let mut rcu = RcuSubsystem::new();
        
        let callback: RcuCallback = |_data| Ok(());
        let id = rcu.register_callback(callback, 42);
        
        assert!(id > 0);
        assert_eq!(rcu.pending_callback_count(), 1);
    }

    #[test]
    fn test_rcu_synchronize() {
        let mut rcu = RcuSubsystem::new();
        
        let gp_id = rcu.synchronize_rcu();
        assert!(gp_id > 0);
        assert_eq!(rcu.get_state(), RcuState::InGracePeriod);
    }

    #[test]
    fn test_rcu_grace_period_end() {
        let mut rcu = RcuSubsystem::new();
        
        rcu.synchronize_rcu();
        // No readers, so grace period should end immediately
        assert!(rcu.grace_period_ended());
    }

    #[test]
    fn test_rcu_callback_processing() {
        let mut rcu = RcuSubsystem::new();
        
        let callback: RcuCallback = |_data| Ok(());
        rcu.register_callback(callback, 42);
        
        rcu.synchronize_rcu();
        let processed = rcu.process_callbacks();
        
        assert_eq!(processed.len(), 1);
        assert_eq!(rcu.pending_callback_count(), 0);
    }

    #[test]
    fn test_rcu_epoch_advancement() {
        let rcu = RcuSubsystem::new();
        
        let epoch1 = rcu.get_epoch();
        rcu.advance_epoch();
        let epoch2 = rcu.get_epoch();
        
        assert!(epoch2 > epoch1);
    }
}
