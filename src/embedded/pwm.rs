#![no_std]
#![no_main]

/// OOP-based PWM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1106
/// Implements pulse-width modulation

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PWMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PWMError { Success = 0, NotFound = 1 }

pub trait PWMChannel {
    fn id(&self) -> PWMID;
    fn frequency(&self) -> u32;
    fn duty_cycle(&self) -> f32;
}

#[repr(C)]
pub struct SimplePWMChannel {
    pub id: PWMID,
    pub frequency: AtomicUsize,
    pub duty_cycle: AtomicUsize,
}

impl SimplePWMChannel {
    pub fn new(id: PWMID, frequency: u32, duty_cycle: f32) -> Self {
        SimplePWMChannel {
            id,
            frequency: AtomicUsize::new(frequency as usize),
            duty_cycle: AtomicUsize::new((duty_cycle * 1000.0) as usize),
        }
    }
}

impl PWMChannel for SimplePWMChannel {
    fn id(&self) -> PWMID { self.id }
    fn frequency(&self) -> u32 { self.frequency.load(Ordering::SeqCst) as u32 }
    fn duty_cycle(&self) -> f32 { (self.duty_cycle.load(Ordering::SeqCst) as f32) / 1000.0 }
}

pub trait PWMController {
    fn configure(&mut self, pwm_id: PWMID, frequency: u32, duty_cycle: f32) -> Result<(), PWMError>;
    fn set_duty(&mut self, pwm_id: PWMID, duty_cycle: f32) -> Result<(), PWMError>;
    fn enable(&mut self, pwm_id: PWMID) -> Result<(), PWMError>;
    fn disable(&mut self, pwm_id: PWMID) -> Result<(), PWMError>;
}

#[repr(C)]
pub struct SimplePWMController {
    pub channels: Vec<Option<Box<dyn PWMChannel>>>,
    pub enabled: Vec<(PWMID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimplePWMController {
    pub fn new() -> Self {
        SimplePWMController {
            channels: Vec::new(),
            enabled: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PWMController for SimplePWMController {
    fn configure(&mut self, pwm_id: PWMID, frequency: u32, duty_cycle: f32) -> Result<(), PWMError> {
        let channel = SimplePWMChannel::new(pwm_id, frequency, duty_cycle);
        self.channels.push(Some(Box::new(channel)));
        Ok(())
    }
    
    fn set_duty(&mut self, pwm_id: PWMID, duty_cycle: f32) -> Result<(), PWMError> {
        for channel_option in &mut self.channels {
            if let Some(ref mut channel) = *channel_option {
                if channel.id() == pwm_id {
                    if let SimplePWMChannel { ref mut duty, .. } = **channel {
                        duty.store((duty_cycle * 1000.0) as usize, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(PWMError::NotFound)
    }
    
    fn enable(&mut self, pwm_id: PWMID) -> Result<(), PWMError> {
        self.enabled.push((pwm_id, AtomicUsize::new(1)));
        Ok(())
    }
    
    fn disable(&mut self, pwm_id: PWMID) -> Result<(), PWMError> {
        for i in 0..self.enabled.len() {
            if self.enabled[i].0 == pwm_id {
                self.enabled[i].1.store(0, Ordering::SeqCst);
                return Ok(());
            }
        }
        Err(PWMError::NotFound)
    }
}

pub trait ServoControl {
    def set_angle(&mut self, pwm_id: PWMID, angle: f32) -> Result<(), PWMError>;
    def get_angle(&self, pwm_id: PWMID) -> Result<f32, PWMError>;
}

#[repr(C)]
pub struct SimpleServoControl {
    pub controller: SimplePWMController,
}

impl SimpleServoControl {
    pub fn new(controller: SimplePWMController) -> Self {
        SimpleServoControl { controller }
    }
}

impl ServoControl for SimpleServoControl {
    fn set_angle(&mut self, pwm_id: PWMID, angle: f32) -> Result<(), PWMError> {
        let duty = (angle / 180.0) * 0.1 + 0.05;
        self.controller.set_duty(pwm_id, duty)
    }
    
    fn get_angle(&self, pwm_id: PWMID) -> Result<f32, PWMError> {
        for channel_option in &self.controller.channels {
            if let Some(ref channel) = *channel_option {
                if channel.id() == pwm_id {
                    let duty = channel.duty_cycle();
                    let angle = ((duty - 0.05) / 0.1) * 180.0;
                    return Ok(angle.max(0.0).min(180.0));
                }
            }
        }
        Err(PWMError::NotFound)
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
