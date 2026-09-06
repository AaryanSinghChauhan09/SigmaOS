#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::boxed::Box;

#[cfg(not(target_os = "none"))]
#[cfg(not(target_os = "none"))]
use std::vec::Vec;

use core::mem;
/// OOP-based Debugger for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 171
/// Implements breakpoints and debugging interface
use core::sync::atomic::{AtomicUsize, Ordering};

pub type BreakpointID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum BreakpointType {
    Software = 0,
    Hardware = 1,
    Watchpoint = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DebuggerError {
    Success = 0,
    NotFound = 1,
    InvalidAddress = 2,
}

pub trait Breakpoint {
    fn id(&self) -> BreakpointID;
    fn address(&self) -> usize;
    fn breakpoint_type(&self) -> BreakpointType;
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
}

#[repr(C)]
pub struct SimpleBreakpoint {
    pub id: BreakpointID,
    pub address: AtomicUsize,
    pub breakpoint_type: AtomicUsize,
    pub enabled: AtomicUsize,
}

impl SimpleBreakpoint {
    pub fn new(id: BreakpointID, address: usize, breakpoint_type: BreakpointType) -> Self {
        SimpleBreakpoint {
            id,
            address: AtomicUsize::new(address),
            breakpoint_type: AtomicUsize::new(breakpoint_type as usize),
            enabled: AtomicUsize::new(1),
        }
    }
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
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

impl Breakpoint for SimpleBreakpoint {
    fn id(&self) -> BreakpointID {
        self.id
    }
    fn address(&self) -> usize {
        self.address.load(Ordering::SeqCst)
    }
    fn breakpoint_type(&self) -> BreakpointType {
        {
            let raw = self.breakpoint_type.load(Ordering::SeqCst) as u32;
            match raw {
                1 => BreakpointType::Hardware,
                2 => BreakpointType::Watchpoint,
                _ => BreakpointType::Software,
            }
        }
    }
    fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) == 1
    }

    fn enable(&mut self) {
        self.enabled.store(1, Ordering::SeqCst);
    }

    fn disable(&mut self) {
        self.enabled.store(0, Ordering::SeqCst);
    }
}

pub trait Debugger {
    fn set_breakpoint(
        &mut self,
        address: usize,
        breakpoint_type: BreakpointType,
    ) -> Result<BreakpointID, DebuggerError>;
    fn remove_breakpoint(&mut self, id: BreakpointID) -> Result<(), DebuggerError>;
    fn get_breakpoint(&self, id: BreakpointID) -> Option<&dyn Breakpoint>;
    fn hit_breakpoint(&self, address: usize) -> Option<BreakpointID>;
    fn step(&mut self) -> Result<(), DebuggerError>;
    fn continue_execution(&mut self) -> Result<(), DebuggerError>;
}

#[repr(C)]
pub struct SimpleDebugger {
    pub breakpoints: Vec<Option<Box<dyn Breakpoint>>>,
    pub next_id: AtomicUsize,
    pub stopped: AtomicUsize,
}

impl SimpleDebugger {
    pub fn new() -> Self {
        SimpleDebugger {
            breakpoints: Vec::new(),
            next_id: AtomicUsize::new(1),
            stopped: AtomicUsize::new(0),
        }
    }
}

impl Debugger for SimpleDebugger {
    fn set_breakpoint(
        &mut self,
        address: usize,
        breakpoint_type: BreakpointType,
    ) -> Result<BreakpointID, DebuggerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let breakpoint = SimpleBreakpoint::new(id, address, breakpoint_type);
        self.breakpoints.push(Some(Box::new(breakpoint)));
        Ok(id)
    }

    fn remove_breakpoint(&mut self, id: BreakpointID) -> Result<(), DebuggerError> {
        for breakpoint_option in &mut self.breakpoints {
            if let Some(ref breakpoint) = *breakpoint_option {
                if breakpoint.id() == id {
                    return Ok(());
                }
            }
        }
        Err(DebuggerError::NotFound)
    }

    fn get_breakpoint(&self, id: BreakpointID) -> Option<&dyn Breakpoint> {
        for breakpoint_option in &self.breakpoints {
            if let Some(ref breakpoint) = *breakpoint_option {
                if breakpoint.id() == id {
                    return Some(breakpoint.as_ref());
                }
            }
        }
        None
    }

    fn hit_breakpoint(&self, address: usize) -> Option<BreakpointID> {
        for breakpoint_option in &self.breakpoints {
            if let Some(ref breakpoint) = *breakpoint_option {
                if breakpoint.address() == address && breakpoint.is_enabled() {
                    return Some(breakpoint.id());
                }
            }
        }
        None
    }

    fn step(&mut self) -> Result<(), DebuggerError> {
        self.stopped.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn continue_execution(&mut self) -> Result<(), DebuggerError> {
        self.stopped.store(0, Ordering::SeqCst);
        Ok(())
    }
}

pub trait RegisterViewer {
    fn read_register(&self, register_id: usize) -> Result<u64, DebuggerError>;
    fn write_register(&mut self, register_id: usize, value: u64) -> Result<(), DebuggerError>;
    fn list_registers(&self) -> Vec<usize>;
}

#[repr(C)]
pub struct SimpleRegisterViewer {
    pub registers: Vec<u64>,
}

impl SimpleRegisterViewer {
    pub fn new() -> Self {
        let mut registers = Vec::new();
        for i in 0..16 {
            registers.push(0u64);
        }
        SimpleRegisterViewer { registers }
    }
}

impl RegisterViewer for SimpleRegisterViewer {
    fn read_register(&self, register_id: usize) -> Result<u64, DebuggerError> {
        if register_id < self.registers.len() {
            Ok(self.registers[register_id])
        } else {
            Err(DebuggerError::NotFound)
        }
    }

    fn write_register(&mut self, register_id: usize, value: u64) -> Result<(), DebuggerError> {
        if register_id < self.registers.len() {
            self.registers[register_id] = value;
            Ok(())
        } else {
            Err(DebuggerError::NotFound)
        }
    }

    fn list_registers(&self) -> Vec<usize> {
        let mut ids = Vec::new();
        for i in 0..self.registers.len() {
            ids.push(i);
        }
        ids
    }
}

#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
