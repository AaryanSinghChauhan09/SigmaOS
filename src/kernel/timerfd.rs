// Linux-inspired timerfd for timer-based file descriptor notifications
// Timer management through file descriptor interface for SigmaOS

use std::time::{Duration, Instant};

/// timerfd clock types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockId {
    Realtime = 0,
    Monotonic = 1,
    Boottime = 2,
    RealtimeAlarm = 3,
    BoottimeAlarm = 4,
}

/// timerfd configuration flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerFlags {
    pub non_blocking: bool,
    pub close_on_exec: bool,
}

impl TimerFlags {
    pub fn new() -> Self {
        TimerFlags {
            non_blocking: false,
            close_on_exec: false,
        }
    }
}

impl Default for TimerFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// timerfd setting
#[derive(Debug, Clone)]
pub struct TimerSetting {
    pub interval: Duration,
    pub initial: Duration,
}

impl TimerSetting {
    pub fn new(interval: Duration, initial: Duration) -> Self {
        TimerSetting { interval, initial }
    }

    pub fn single_shot(duration: Duration) -> Self {
        TimerSetting {
            interval: Duration::ZERO,
            initial: duration,
        }
    }

    pub fn periodic(interval: Duration) -> Self {
        TimerSetting {
            interval,
            initial: interval,
        }
    }
}

/// timerfd instance
pub struct TimerFd {
    clock_id: ClockId,
    flags: TimerFlags,
    setting: TimerSetting,
    expirations: u64,
    armed: bool,
    deadline: Option<Instant>,
}

impl TimerFd {
    pub fn new(clock_id: ClockId, flags: TimerFlags) -> Self {
        TimerFd {
            clock_id,
            flags,
            setting: TimerSetting::new(Duration::ZERO, Duration::ZERO),
            expirations: 0,
            armed: false,
            deadline: None,
        }
    }

    /// Set timer configuration
    pub fn set_time(&mut self, setting: TimerSetting) {
        self.setting = setting;
        self.armed = true;
        
        // Set deadline based on initial delay
        if self.setting.initial > Duration::ZERO {
            self.deadline = Some(Instant::now() + self.setting.initial);
        } else {
            self.deadline = None;
        }
        
        self.expirations = 0;
    }

    /// Get timer configuration
    pub fn get_time(&self) -> TimerSetting {
        self.setting.clone()
    }

    /// Read expirations (returns number of expirations since last read)
    pub fn read(&mut self) -> Result<u64, String> {
        if !self.armed {
            return Err("Timer is not armed".to_string());
        }

        // Check if timer has expired
        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                self.expirations += 1;
                
                // Reset deadline for periodic timers
                if self.setting.interval > Duration::ZERO {
                    self.deadline = Some(Instant::now() + self.setting.interval);
                } else {
                    // Single-shot timer, disarm
                    self.armed = false;
                    self.deadline = None;
                }
            }
        }

        let expirations = self.expirations;
        self.expirations = 0;
        Ok(expirations)
    }

    /// Check if timer is armed
    pub fn is_armed(&self) -> bool {
        self.armed
    }

    /// Get remaining time until next expiration
    pub fn get_remaining(&self) -> Option<Duration> {
        self.deadline.map(|deadline| {
            let now = Instant::now();
            if deadline > now {
                deadline - now
            } else {
                Duration::ZERO
            }
        })
    }

    /// Disarm the timer
    pub fn disarm(&mut self) {
        self.armed = false;
        self.deadline = None;
        self.expirations = 0;
    }
}

impl Default for TimerFd {
    fn default() -> Self {
        Self::new(ClockId::Monotonic, TimerFlags::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_timer_fd_creation() {
        let timer = TimerFd::new(ClockId::Monotonic, TimerFlags::new());
        assert_eq!(timer.clock_id, ClockId::Monotonic);
        assert!(!timer.is_armed());
    }

    #[test]
    fn test_timer_setting_single_shot() {
        let setting = TimerSetting::single_shot(Duration::from_millis(100));
        assert_eq!(setting.interval, Duration::ZERO);
        assert_eq!(setting.initial, Duration::from_millis(100));
    }

    #[test]
    fn test_timer_setting_periodic() {
        let setting = TimerSetting::periodic(Duration::from_millis(50));
        assert_eq!(setting.interval, Duration::from_millis(50));
        assert_eq!(setting.initial, Duration::from_millis(50));
    }

    #[test]
    fn test_timer_fd_set_time() {
        let mut timer = TimerFd::new(ClockId::Monotonic, TimerFlags::new());
        let setting = TimerSetting::single_shot(Duration::from_millis(100));
        
        timer.set_time(setting);
        assert!(timer.is_armed());
        assert!(timer.get_remaining().is_some());
    }

    #[test]
    fn test_timer_fd_disarm() {
        let mut timer = TimerFd::new(ClockId::Monotonic, TimerFlags::new());
        let setting = TimerSetting::single_shot(Duration::from_millis(100));
        
        timer.set_time(setting);
        timer.disarm();
        
        assert!(!timer.is_armed());
        assert!(timer.get_remaining().is_none());
    }

    #[test]
    fn test_timer_fd_read_not_armed() {
        let mut timer = TimerFd::new(ClockId::Monotonic, TimerFlags::new());
        let result = timer.read();
        
        assert!(result.is_err());
    }

    #[test]
    fn test_timer_fd_expiration() {
        let mut timer = TimerFd::new(ClockId::Monotonic, TimerFlags::new());
        let setting = TimerSetting::single_shot(Duration::from_millis(10));
        
        timer.set_time(setting);
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(20));
        
        let expirations = timer.read().unwrap();
        assert!(expirations >= 1);
    }

    #[test]
    fn test_timer_fd_get_remaining() {
        let mut timer = TimerFd::new(ClockId::Monotonic, TimerFlags::new());
        let setting = TimerSetting::single_shot(Duration::from_millis(100));
        
        timer.set_time(setting);
        
        let remaining = timer.get_remaining();
        assert!(remaining.is_some());
        assert!(remaining.unwrap() > Duration::ZERO);
    }
}
