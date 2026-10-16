// SPDX-License-Identifier: MIT
// SigmaOS - Timer Application
// Inspired by GNOME Clocks and Gnome Timer
// Provides timer, stopwatch, and alarm functionality

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Timer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
    Completed,
}

/// Stopwatch state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopwatchState {
    Idle,
    Running,
    Paused,
}

/// Alarm state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlarmState {
    Active,
    Triggered,
    Snoozed,
    Disabled,
}

/// Timer
#[derive(Debug, Clone)]
pub struct Timer {
    pub name: String,
    pub duration: u32,
    pub remaining: u32,
    pub state: TimerState,
    pub sound_enabled: bool,
}

/// Stopwatch lap
#[derive(Debug, Clone)]
pub struct Lap {
    pub number: u32,
    pub time: u32,
    pub total_time: u32,
}

/// Stopwatch
#[derive(Debug, Clone)]
pub struct Stopwatch {
    pub elapsed: u32,
    pub state: StopwatchState,
    pub laps: Vec<Lap>,
}

/// Alarm
#[derive(Debug, Clone)]
pub struct Alarm {
    pub name: String,
    pub hour: u32,
    pub minute: u32,
    pub days: Vec<bool>,
    pub state: AlarmState,
    pub sound_enabled: bool,
    pub snooze_duration: u32,
}

/// Timer application
pub struct TimerApp {
    pub timers: Vec<Timer>,
    pub stopwatch: Stopwatch,
    pub alarms: Vec<Alarm>,
    pub current_timer: Option<usize>,
}

impl TimerApp {
    pub fn new() -> Self {
        Self {
            timers: Vec::new(),
            stopwatch: Stopwatch {
                elapsed: 0,
                state: StopwatchState::Idle,
                laps: Vec::new(),
            },
            alarms: Vec::new(),
            current_timer: None,
        }
    }

    /// Create timer
    pub fn create_timer(&mut self, name: String, duration: u32) -> usize {
        let timer = Timer {
            name,
            duration,
            remaining: duration,
            state: TimerState::Idle,
            sound_enabled: true,
        };
        self.timers.push(timer);
        self.timers.len() - 1
    }

    /// Start timer
    pub fn start_timer(&mut self, index: usize) -> Result<(), &'static str> {
        let timer = self.timers.get_mut(index).ok_or("Timer not found")?;
        timer.state = TimerState::Running;
        self.current_timer = Some(index);
        Ok(())
    }

    /// Pause timer
    pub fn pause_timer(&mut self, index: usize) -> Result<(), &'static str> {
        let timer = self.timers.get_mut(index).ok_or("Timer not found")?;
        timer.state = TimerState::Paused;
        Ok(())
    }

    /// Resume timer
    pub fn resume_timer(&mut self, index: usize) -> Result<(), &'static str> {
        let timer = self.timers.get_mut(index).ok_or("Timer not found")?;
        timer.state = TimerState::Running;
        Ok(())
    }

    /// Reset timer
    pub fn reset_timer(&mut self, index: usize) -> Result<(), &'static str> {
        let timer = self.timers.get_mut(index).ok_or("Timer not found")?;
        timer.remaining = timer.duration;
        timer.state = TimerState::Idle;
        Ok(())
    }

    /// Delete timer
    pub fn delete_timer(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.timers.len() {
            return Err("Timer index out of bounds");
        }
        self.timers.remove(index);
        Ok(())
    }

    /// Start stopwatch
    pub fn start_stopwatch(&mut self) {
        self.stopwatch.state = StopwatchState::Running;
    }

    /// Pause stopwatch
    pub fn pause_stopwatch(&mut self) {
        self.stopwatch.state = StopwatchState::Paused;
    }

    /// Reset stopwatch
    pub fn reset_stopwatch(&mut self) {
        self.stopwatch.elapsed = 0;
        self.stopwatch.state = StopwatchState::Idle;
        self.stopwatch.laps.clear();
    }

    /// Lap stopwatch
    pub fn lap_stopwatch(&mut self) -> Result<(), &'static str> {
        if self.stopwatch.state != StopwatchState::Running {
            return Err("Stopwatch not running");
        }
        let lap_number = self.stopwatch.laps.len() as u32 + 1;
        let lap_time = self.stopwatch.elapsed;
        let total_time = self.stopwatch.elapsed;
        self.stopwatch.laps.push(Lap {
            number: lap_number,
            time: lap_time,
            total_time,
        });
        Ok(())
    }

    /// Create alarm
    pub fn create_alarm(&mut self, name: String, hour: u32, minute: u32) -> usize {
        let alarm = Alarm {
            name,
            hour,
            minute,
            days: vec![true; 7],
            state: AlarmState::Active,
            sound_enabled: true,
            snooze_duration: 5,
        };
        self.alarms.push(alarm);
        self.alarms.len() - 1
    }

    /// Enable alarm
    pub fn enable_alarm(&mut self, index: usize) -> Result<(), &'static str> {
        let alarm = self.alarms.get_mut(index).ok_or("Alarm not found")?;
        alarm.state = AlarmState::Active;
        Ok(())
    }

    /// Disable alarm
    pub fn disable_alarm(&mut self, index: usize) -> Result<(), &'static str> {
        let alarm = self.alarms.get_mut(index).ok_or("Alarm not found")?;
        alarm.state = AlarmState::Disabled;
        Ok(())
    }

    /// Snooze alarm
    pub fn snooze_alarm(&mut self, index: usize) -> Result<(), &'static str> {
        let alarm = self.alarms.get_mut(index).ok_or("Alarm not found")?;
        alarm.state = AlarmState::Snoozed;
        Ok(())
    }

    /// Delete alarm
    pub fn delete_alarm(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.alarms.len() {
            return Err("Alarm index out of bounds");
        }
        self.alarms.remove(index);
        Ok(())
    }

    /// Get timers
    pub fn get_timers(&self) -> &[Timer] {
        &self.timers
    }

    /// Get alarms
    pub fn get_alarms(&self) -> &[Alarm] {
        &self.alarms
    }

    /// Get stopwatch
    pub fn get_stopwatch(&self) -> &Stopwatch {
        &self.stopwatch
    }
}

/// Global timer app
static mut TIMER_APP: Option<TimerApp> = None;

/// Initialize timer app
pub fn init_timer_app() {
    unsafe {
        TIMER_APP = Some(TimerApp::new());
    }
}

/// Get timer app
pub fn get_timer_app() -> &'static mut TimerApp {
    unsafe { TIMER_APP.as_mut().expect("Timer app not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_app_creation() {
        let ta = TimerApp::new();
        assert_eq!(ta.timers.len(), 0);
    }

    #[test]
    fn test_create_timer() {
        let mut ta = TimerApp::new();
        ta.create_timer("Test Timer".to_string(), 60);
        assert_eq!(ta.timers.len(), 1);
    }

    #[test]
    fn test_start_stopwatch() {
        let mut ta = TimerApp::new();
        ta.start_stopwatch();
        assert_eq!(ta.stopwatch.state, StopwatchState::Running);
    }
}
