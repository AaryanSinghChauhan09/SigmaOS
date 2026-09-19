// SPDX-License-Identifier: MIT
// SigmaOS Kernel Timer Subsystem
// High-resolution timer management inspired by Linux hrtimers and BSD callouts

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use std::time::{Duration, Instant};

/// Timer ID type
pub type TimerId = u64;

/// Timer type (Linux hrtimer modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerMode {
    /// One-shot timer (fires once)
    OneShot,
    /// Periodic timer (fires repeatedly)
    Periodic,
    /// Absolute deadline timer
    Absolute,
}

/// Timer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    /// Timer is idle
    Idle,
    /// Timer is armed (scheduled)
    Armed,
    /// Timer is executing callback
    Executing,
    /// Timer is expired
    Expired,
}

/// Timer error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerError {
    InvalidTimerId,
    TimerAlreadyArmed,
    TimerNotArmed,
    InvalidInterval,
    CallbackFailed,
}

/// Timer callback function type
pub type TimerCallback = fn(TimerId, u64) -> Result<(), TimerError>;

/// Timer descriptor
#[derive(Debug)]
pub struct TimerDescriptor {
    pub id: TimerId,
    pub mode: TimerMode,
    pub interval_ns: u64,
    pub expires_at: u64,
    pub state: AtomicU32, // TimerState as u32
    pub fire_count: AtomicU32,
    pub callback: Option<TimerCallback>,
    pub user_data: u64,
}

impl TimerDescriptor {
    pub fn new(id: TimerId, mode: TimerMode, interval_ns: u64, callback: TimerCallback, user_data: u64) -> Self {
        TimerDescriptor {
            id,
            mode,
            interval_ns,
            expires_at: 0,
            state: AtomicU32::new(TimerState::Idle as u32),
            fire_count: AtomicU32::new(0),
            callback: Some(callback),
            user_data,
        }
    }

    pub fn get_state(&self) -> TimerState {
        match self.state.load(Ordering::SeqCst) {
            0 => TimerState::Idle,
            1 => TimerState::Armed,
            2 => TimerState::Executing,
            3 => TimerState::Expired,
            _ => TimerState::Idle,
        }
    }

    pub fn set_state(&self, state: TimerState) {
        self.state.store(state as u32, Ordering::SeqCst);
    }
}

/// Kernel timer subsystem
#[derive(Debug)]
pub struct KernelTimerSubsystem {
    timers: BTreeMap<TimerId, TimerDescriptor>,
    next_id: AtomicU64,
    current_time_ns: AtomicU64,
}

impl KernelTimerSubsystem {
    pub fn new() -> Self {
        KernelTimerSubsystem {
            timers: BTreeMap::new(),
            next_id: AtomicU64::new(1),
            current_time_ns: AtomicU64::new(0),
        }
    }

    /// Create a new timer
    pub fn create_timer(&mut self, mode: TimerMode, interval_ns: u64, callback: TimerCallback, user_data: u64) -> TimerId {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let timer = TimerDescriptor::new(id, mode, interval_ns, callback, user_data);
        self.timers.insert(id, timer);
        id
    }

    /// Arm a timer
    pub fn arm_timer(&mut self, id: TimerId) -> Result<(), TimerError> {
        let timer = self.timers.get_mut(&id).ok_or(TimerError::InvalidTimerId)?;
        
        if timer.get_state() == TimerState::Armed {
            return Err(TimerError::TimerAlreadyArmed);
        }

        let now = self.current_time_ns.load(Ordering::SeqCst);
        timer.expires_at = now + timer.interval_ns;
        timer.set_state(TimerState::Armed);
        Ok(())
    }

    /// Disarm a timer
    pub fn disarm_timer(&mut self, id: TimerId) -> Result<(), TimerError> {
        let timer = self.timers.get_mut(&id).ok_or(TimerError::InvalidTimerId)?;
        
        if timer.get_state() != TimerState::Armed {
            return Err(TimerError::TimerNotArmed);
        }

        timer.set_state(TimerState::Idle);
        Ok(())
    }

    /// Delete a timer
    pub fn delete_timer(&mut self, id: TimerId) -> Result<(), TimerError> {
        self.timers.remove(&id).ok_or(TimerError::InvalidTimerId)?;
        Ok(())
    }

    /// Get current time in nanoseconds
    pub fn get_time_ns(&self) -> u64 {
        self.current_time_ns.load(Ordering::SeqCst)
    }

    /// Advance time and process expired timers
    pub fn tick(&mut self, delta_ns: u64) -> Vec<TimerId> {
        self.current_time_ns.fetch_add(delta_ns, Ordering::SeqCst);
        let now = self.get_time_ns();
        let mut expired = Vec::new();

        for (id, timer) in self.timers.iter_mut() {
            if timer.get_state() == TimerState::Armed && timer.expires_at <= now {
                timer.set_state(TimerState::Executing);
                timer.fire_count.fetch_add(1, Ordering::SeqCst);
                
                if let Some(callback) = timer.callback {
                    if callback(*id, timer.user_data).is_ok() {
                        expired.push(*id);
                    }
                }

                // Handle periodic timers
                if timer.mode == TimerMode::Periodic {
                    timer.expires_at = now + timer.interval_ns;
                    timer.set_state(TimerState::Armed);
                } else {
                    timer.set_state(TimerState::Expired);
                }
            }
        }

        expired
    }

    /// Get timer fire count
    pub fn get_fire_count(&self, id: TimerId) -> Result<u32, TimerError> {
        let timer = self.timers.get(&id).ok_or(TimerError::InvalidTimerId)?;
        Ok(timer.fire_count.load(Ordering::SeqCst))
    }

    /// Get active timer count
    pub fn active_timer_count(&self) -> usize {
        self.timers.iter().filter(|(_, t)| t.get_state() == TimerState::Armed).count()
    }
}

impl Default for KernelTimerSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_creation() {
        let mut subsystem = KernelTimerSubsystem::new();
        
        let callback: TimerCallback = |_id, _data| Ok(());
        let id = subsystem.create_timer(TimerMode::OneShot, 1000, callback, 42);
        
        assert!(id > 0);
        assert_eq!(subsystem.active_timer_count(), 0);
    }

    #[test]
    fn test_timer_arm_disarm() {
        let mut subsystem = KernelTimerSubsystem::new();
        
        let callback: TimerCallback = |_id, _data| Ok(());
        let id = subsystem.create_timer(TimerMode::OneShot, 1000, callback, 42);
        
        assert!(subsystem.arm_timer(id).is_ok());
        assert_eq!(subsystem.active_timer_count(), 1);
        
        assert!(subsystem.disarm_timer(id).is_ok());
        assert_eq!(subsystem.active_timer_count(), 0);
    }

    #[test]
    fn test_timer_expiration() {
        let mut subsystem = KernelTimerSubsystem::new();
        
        let callback: TimerCallback = |_id, _data| Ok(());
        let id = subsystem.create_timer(TimerMode::OneShot, 100, callback, 42);
        
        subsystem.arm_timer(id).unwrap();
        let expired = subsystem.tick(150);
        
        assert!(expired.contains(&id));
        assert_eq!(subsystem.get_fire_count(id).unwrap(), 1);
    }

    #[test]
    fn test_periodic_timer() {
        let mut subsystem = KernelTimerSubsystem::new();
        
        let callback: TimerCallback = |_id, _data| Ok(());
        let id = subsystem.create_timer(TimerMode::Periodic, 100, callback, 42);
        
        subsystem.arm_timer(id).unwrap();
        subsystem.tick(150);
        subsystem.tick(100);
        
        assert_eq!(subsystem.get_fire_count(id).unwrap(), 2);
        assert_eq!(subsystem.active_timer_count(), 1);
    }

    #[test]
    fn test_timer_deletion() {
        let mut subsystem = KernelTimerSubsystem::new();
        
        let callback: TimerCallback = |_id, _data| Ok(());
        let id = subsystem.create_timer(TimerMode::OneShot, 1000, callback, 42);
        
        assert!(subsystem.delete_timer(id).is_ok());
        assert!(subsystem.arm_timer(id).is_err());
    }
}
