#![no_std]
#![no_main]

/// OOP-based Real-Time Clock for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1076
/// Implements RTC and timekeeping

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ClockID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClockError { Success = 0, NotFound = 1 }

pub trait RTCClock {
    fn id(&self) -> ClockID;
    fn get_time(&self) -> (u32, u32, u32);
    fn get_date(&self) -> (u32, u32, u32);
    def set_time(&mut self, hour: u32, minute: u32, second: u32);
    def set_date(&mut self, year: u32, month: u32, day: u32);
}

#[repr(C)]
pub struct SimpleRTCClock {
    pub id: ClockID,
    pub hour: AtomicUsize,
    pub minute: AtomicUsize,
    pub second: AtomicUsize,
    pub year: AtomicUsize,
    pub month: AtomicUsize,
    pub day: AtomicUsize,
}

impl SimpleRTCClock {
    pub fn new(id: ClockID) -> Self {
        SimpleRTCClock {
            id,
            hour: AtomicUsize::new(0),
            minute: AtomicUsize::new(0),
            second: AtomicUsize::new(0),
            year: AtomicUsize::new(2024),
            month: AtomicUsize::new(1),
            day: AtomicUsize::new(1),
        }
    }
}

impl RTCClock for SimpleRTCClock {
    fn id(&self) -> ClockID { self.id }
    
    fn get_time(&self) -> (u32, u32, u32) {
        (
            self.hour.load(Ordering::SeqCst) as u32,
            self.minute.load(Ordering::SeqCst) as u32,
            self.second.load(Ordering::SeqCst) as u32,
        )
    }
    
    fn get_date(&self) -> (u32, u32, u32) {
        (
            self.year.load(Ordering::SeqCst) as u32,
            self.month.load(Ordering::SeqCst) as u32,
            self.day.load(Ordering::SeqCst) as u32,
        )
    }
    
    fn set_time(&mut self, hour: u32, minute: u32, second: u32) {
        self.hour.store(hour as usize, Ordering::SeqCst);
        self.minute.store(minute as usize, Ordering::SeqCst);
        self.second.store(second as usize, Ordering::SeqCst);
    }
    
    fn set_date(&mut self, year: u32, month: u32, day: u32) {
        self.year.store(year as usize, Ordering::SeqCst);
        self.month.store(month as usize, Ordering::SeqCst);
        self.day.store(day as usize, Ordering::SeqCst);
    }
}

pub trait Alarm {
    fn set_alarm(&mut self, hour: u32, minute: u32, second: u32) -> Result<(), ClockError>;
    fn clear_alarm(&mut self);
    fn is_triggered(&self) -> bool;
}

#[repr(C)]
pub struct SimpleAlarm {
    pub alarm_hour: AtomicUsize,
    pub alarm_minute: AtomicUsize,
    pub alarm_second: AtomicUsize,
    pub enabled: AtomicUsize,
    pub triggered: AtomicUsize,
}

impl SimpleAlarm {
    pub fn new() -> Self {
        SimpleAlarm {
            alarm_hour: AtomicUsize::new(0),
            alarm_minute: AtomicUsize::new(0),
            alarm_second: AtomicUsize::new(0),
            enabled: AtomicUsize::new(0),
            triggered: AtomicUsize::new(0),
        }
    }
}

impl Alarm for SimpleAlarm {
    fn set_alarm(&mut self, hour: u32, minute: u32, second: u32) -> Result<(), ClockError> {
        self.alarm_hour.store(hour as usize, Ordering::SeqCst);
        self.alarm_minute.store(minute as usize, Ordering::SeqCst);
        self.alarm_second.store(second as usize, Ordering::SeqCst);
        self.enabled.store(1, Ordering::SeqCst);
        self.triggered.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn clear_alarm(&mut self) {
        self.enabled.store(0, Ordering::SeqCst);
        self.triggered.store(0, Ordering::SeqCst);
    }
    
    fn is_triggered(&self) -> bool { self.triggered.load(Ordering::SeqCst) == 1 }
}

pub trait Timer {
    fn start_timer(&mut self, duration: u32);
    fn stop_timer(&mut self);
    fn is_expired(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTimer {
    pub remaining: AtomicUsize,
    pub running: AtomicUsize,
}

impl SimpleTimer {
    pub fn new() -> Self {
        SimpleTimer {
            remaining: AtomicUsize::new(0),
            running: AtomicUsize::new(0),
        }
    }
}

impl Timer for SimpleTimer {
    fn start_timer(&mut self, duration: u32) {
        self.remaining.store(duration as usize, Ordering::SeqCst);
        self.running.store(1, Ordering::SeqCst);
    }
    
    fn stop_timer(&mut self) {
        self.running.store(0, Ordering::SeqCst);
    }
    
    fn is_expired(&self) -> bool {
        self.running.load(Ordering::SeqCst) == 1 && self.remaining.load(Ordering::SeqCst) == 0
    }
}
