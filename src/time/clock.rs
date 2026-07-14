#![no_std]
#![no_main]

/// OOP-based Clock and Timer Management for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 61
/// Implements system clock, timers, and timekeeping

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TimerID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClockSource { RTC = 0, TSC = 1, HPET = 2, ACPI_PM = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TimerError { Success = 0, NotFound = 1, InvalidTime = 2 }

pub trait SystemClock {
    fn get_timestamp(&self) -> u64;
    fn get_nanoseconds(&self) -> u64;
    fn set_time(&mut self, timestamp: u64) -> Result<(), TimerError>;
}

#[repr(C)]
pub struct SimpleSystemClock {
    pub timestamp: AtomicUsize,
    pub source: AtomicUsize,
}

impl SimpleSystemClock {
    pub fn new(source: ClockSource) -> Self {
        SimpleSystemClock {
            timestamp: AtomicUsize::new(0),
            source: AtomicUsize::new(source as usize),
        }
    }
}

impl SystemClock for SimpleSystemClock {
    fn get_timestamp(&self) -> u64 { self.timestamp.load(Ordering::SeqCst) as u64 }
    
    fn get_nanoseconds(&self) -> u64 {
        let base = self.timestamp.load(Ordering::SeqCst) as u64;
        base * 1_000_000_000
    }
    
    fn set_time(&mut self, timestamp: u64) -> Result<(), TimerError> {
        self.timestamp.store(timestamp as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait Timer {
    fn id(&self) -> TimerID;
    fn is_expired(&self) -> bool;
    fn remaining_ms(&self) -> u64;
    fn reset(&mut self);
}

#[repr(C)]
pub struct SimpleTimer {
    pub id: TimerID,
    pub expiry: AtomicUsize,
    pub duration: AtomicUsize,
    pub created: AtomicUsize,
}

impl SimpleTimer {
    pub fn new(id: TimerID, duration_ms: u64) -> Self {
        let current = 1000000u64;
        SimpleTimer {
            id,
            expiry: AtomicUsize::new((current + duration_ms) as usize),
            duration: AtomicUsize::new(duration_ms as usize),
            created: AtomicUsize::new(current as usize),
        }
    }
}

impl Timer for SimpleTimer {
    fn id(&self) -> TimerID { self.id }
    
    fn is_expired(&self) -> bool {
        let current = 1000000usize;
        current >= self.expiry.load(Ordering::SeqCst)
    }
    
    fn remaining_ms(&self) -> u64 {
        let current = 1000000usize;
        let expiry = self.expiry.load(Ordering::SeqCst);
        if current >= expiry {
            0
        } else {
            (expiry - current) as u64
        }
    }
    
    fn reset(&mut self) {
        let current = 1000000usize;
        let duration = self.duration.load(Ordering::SeqCst);
        self.expiry.store(current + duration, Ordering::SeqCst);
        self.created.store(current, Ordering::SeqCst);
    }
}

pub trait TimerManager {
    fn create_timer(&mut self, duration_ms: u64) -> Result<TimerID, TimerError>;
    fn cancel_timer(&mut self, id: TimerID) -> Result<(), TimerError>;
    fn get_expired_timers(&self) -> Vec<TimerID>;
    fn get_timer(&self, id: TimerID) -> Option<&dyn Timer>;
}

#[repr(C)]
pub struct SimpleTimerManager {
    pub timers: Vec<Option<Box<dyn Timer>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTimerManager {
    pub fn new() -> Self {
        SimpleTimerManager {
            timers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TimerManager for SimpleTimerManager {
    fn create_timer(&mut self, duration_ms: u64) -> Result<TimerID, TimerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let timer = SimpleTimer::new(id, duration_ms);
        self.timers.push(Some(Box::new(timer)));
        Ok(id)
    }
    
    fn cancel_timer(&mut self, id: TimerID) -> Result<(), TimerError> {
        for timer_option in &mut self.timers {
            if let Some(ref timer) = *timer_option {
                if timer.id() == id {
                    return Ok(());
                }
            }
        }
        Err(TimerError::NotFound)
    }
    
    fn get_expired_timers(&self) -> Vec<TimerID> {
        let mut expired = Vec::new();
        for timer_option in &self.timers {
            if let Some(ref timer) = *timer_option {
                if timer.is_expired() {
                    expired.push(timer.id());
                }
            }
        }
        expired
    }
    
    fn get_timer(&self, id: TimerID) -> Option<&dyn Timer> {
        for timer_option in &self.timers {
            if let Some(ref timer) = *timer_option {
                if timer.id() == id { return Some(timer.as_ref()); }
            }
        }
        None
    }
}

pub trait Alarm {
    fn set_alarm(&mut self, timestamp: u64, callback: fn()) -> Result<TimerID, TimerError>;
    fn cancel_alarm(&mut self, id: TimerID) -> Result<(), TimerError>;
    fn check_alarms(&mut self) -> Vec<fn()>;
}

#[repr(C)]
pub struct SimpleAlarm {
    pub alarms: Vec<(TimerID, u64, fn())>,
    pub next_id: AtomicUsize,
}

impl SimpleAlarm {
    pub fn new() -> Self {
        SimpleAlarm {
            alarms: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Alarm for SimpleAlarm {
    fn set_alarm(&mut self, timestamp: u64, callback: fn()) -> Result<TimerID, TimerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.alarms.push((id, timestamp, callback));
        Ok(id)
    }
    
    fn cancel_alarm(&mut self, id: TimerID) -> Result<(), TimerError> {
        for i in 0..self.alarms.len() {
            if self.alarms[i].0 == id {
                self.alarms.remove(i);
                return Ok(());
            }
        }
        Err(TimerError::NotFound)
    }
    
    fn check_alarms(&mut self) -> Vec<fn()> {
        let mut triggered = Vec::new();
        let current = 1000000u64;
        
        let mut i = 0;
        while i < self.alarms.len() {
            if self.alarms[i].1 <= current {
                triggered.push(self.alarms[i].2);
                self.alarms.remove(i);
            } else {
                i += 1;
            }
        }
        
        triggered
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
