#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Servo Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1396
/// Implements servo motor control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ServoID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ServoError { Success = 0, NotFound = 1 }

pub trait ServoMotor {
    fn id(&self) -> ServoID;
    fn get_angle(&self) -> f32;
}

#[repr(C)]
pub struct SimpleServoMotor {
    pub id: ServoID,
    pub angle: AtomicUsize,
}

impl SimpleServoMotor {
    pub fn new(id: ServoID) -> Self {
        SimpleServoMotor {
            id,
            angle: AtomicUsize::new(900),
        }
    }
}

impl ServoMotor for SimpleServoMotor {
    fn id(&self) -> ServoID { self.id }
    fn get_angle(&self) -> f32 { (self.angle.load(Ordering::SeqCst) as f32) / 10.0 }
}

pub trait ServoController {
    fn set_angle(&mut self, servo_id: ServoID, angle: f32) -> Result<(), ServoError>;
    fn get_angle(&self, servo_id: ServoID) -> Result<f32, ServoError>;
    def detach(&mut self, servo_id: ServoID) -> Result<(), ServoError>;
}

#[repr(C)]
pub struct SimpleServoController {
    pub servos: Vec<Option<Box<dyn ServoMotor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleServoController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleServoController {
            servos: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ServoController for SimpleServoController {
    fn set_angle(&mut self, servo_id: ServoID, angle: f32) -> Result<(), ServoError> {
        let clamped_angle = angle.max(0.0).min(180.0);
        for servo_option in &mut self.servos {
            if let Some(ref mut servo) = *servo_option {
                if servo.id() == servo_id {
                    servo.angle.store((clamped_angle * 10.0) as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ServoError::NotFound)
    }
    
    fn get_angle(&self, servo_id: ServoID) -> Result<f32, ServoError> {
        for servo_option in &self.servos {
            if let Some(ref servo) = *servo_option {
                if servo.id() == servo_id {
                    return Ok(servo.get_angle());
                }
            }
        }
        Err(ServoError::NotFound)
    }
    
    fn detach(&mut self, servo_id: ServoID) -> Result<(), ServoError> {
        for servo_option in &mut self.servos {
            if let Some(ref servo) = *servo_option {
                if servo.id() == servo_id {
                    return Ok(());
                }
            }
        }
        Err(ServoError::NotFound)
    }
}

pub trait ContinuousServo {
    def set_speed(&mut self, servo_id: ServoID, speed: f32) -> Result<(), ServoError>;
    def stop(&mut self, servo_id: ServoID) -> Result<(), ServoError>;
}

#[repr(C)]
pub struct SimpleContinuousServo {
    pub controller: SimpleServoController,
    pub speeds: Vec<(ServoID, AtomicUsize)>,
}

impl SimpleContinuousServo {
    pub fn new(controller: SimpleServoController) -> Self {
        SimpleContinuousServo {
            controller,
            speeds: Vec::new(),
        }
    }
}

impl ContinuousServo for SimpleContinuousServo {
    fn set_speed(&mut self, servo_id: ServoID, speed: f32) -> Result<(), ServoError> {
        let clamped_speed = speed.max(-1.0).min(1.0);
        self.speeds.push((servo_id, AtomicUsize::new((clamped_speed * 1000.0) as usize)));
        Ok(())
    }
    
    fn stop(&mut self, servo_id: ServoID) -> Result<(), ServoError> {
        self.controller.set_angle(servo_id, 90.0)
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


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
