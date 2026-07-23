#![no_std]
#![no_main]

use core::mem;
/// OOP-based Debugger for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 171
/// Implements breakpoints and debugging interface
use core::sync::atomic::{AtomicUsize, Ordering};

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub type BreakpointID = usize;

#[repr(C)]
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
        unsafe { core::mem::transmute(self.breakpoint_type.load(Ordering::SeqCst)) }
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


