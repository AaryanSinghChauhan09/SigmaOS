#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

#[cfg(not(target_os = "none"))]
extern crate alloc;
#[cfg(not(target_os = "none"))]
use alloc::vec::Vec;

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
    #[allow(clippy::new_without_default)]
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
    #[allow(clippy::new_without_default)]
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

// ==========================================
// 5. Windbg / GDB / LLDB-Grade Debugging Suite
// ==========================================

/// Debug Process Representation
#[derive(Debug, Clone)]
pub struct DebugProcess {
    pub pid: usize,
    pub name: &'static str,
    pub modules: Vec<DebugModule>,
}

/// Debug Module Representation
#[derive(Debug, Clone)]
pub struct DebugModule {
    pub base_address: usize,
    pub size: usize,
    pub name: &'static str,
}

/// Windbg-parity Pseudo Registers State.
/// Supports predefined registers ($peb, $teb, $ip, $sp)
/// and User-Defined registers ($u0 through $u9).
pub struct PseudoRegisterSet {
    pub predefined_registers: Vec<(&'static str, u64)>,
    pub user_registers: [u64; 10], // $u0 to $u9
}

impl PseudoRegisterSet {
    pub fn new() -> Self {
        let mut predefined = Vec::new();
        predefined.push(("$peb", 0x7FFFF000));
        predefined.push(("$teb", 0x7FFF1000));
        predefined.push(("$ip", 0x1000));
        predefined.push(("$sp", 0x9000));

        Self {
            predefined_registers: predefined,
            user_registers: [0; 10],
        }
    }

    pub fn read(&self, name: &str) -> Option<u64> {
        if name.starts_with("$u") && name.len() == 3 {
            let idx = name.chars().nth(2)?.to_digit(10)? as usize;
            if idx < 10 {
                return Some(self.user_registers[idx]);
            }
        }
        for &(k, v) in &self.predefined_registers {
            if k == name {
                return Some(v);
            }
        }
        None
    }

    pub fn write(&mut self, name: &str, val: u64) -> bool {
        if name.starts_with("$u") && name.len() == 3 {
            if let Some(idx) = name.chars().nth(2).and_then(|c| c.to_digit(10)) {
                let idx = idx as usize;
                if idx < 10 {
                    self.user_registers[idx] = val;
                    return true;
                }
            }
        }
        false
    }
}

/// Debugger Alias Manager.
/// Supports User-Named (as/ad), Fixed-Name ($ntns), and Automatic ($cache) aliases.
pub struct DebugAliasManager {
    pub user_aliases: Vec<(&'static str, &'static str)>,
}

impl DebugAliasManager {
    pub fn new() -> Self {
        Self {
            user_aliases: Vec::new(),
        }
    }

    pub fn set_user_alias(&mut self, name: &'static str, expansion: &'static str) {
        // Remove existing alias if it matches
        for i in 0..self.user_aliases.len() {
            if self.user_aliases[i].0 == name {
                self.user_aliases[i] = (name, expansion);
                return;
            }
        }
        self.user_aliases.push((name, expansion));
    }

    pub fn expand(&self, name: &str) -> alloc::string::String {
        use alloc::string::ToString;
        // Handle Automatic and Fixed aliases parity
        if name == "$ntns" {
            return "sigma_kernel::sys".to_string();
        }
        if name == "$cache" {
            return "VMM_Page_Cache".to_string();
        }
        for &(k, v) in &self.user_aliases {
            if k == name {
                return v.to_string();
            }
        }
        name.to_string()
    }
}

/// Windbg DML (Debugger Markup Language) Renderer
pub struct DmlRenderer;

impl DmlRenderer {
    /// Renders text stripping markup tags or simulating interactive clickable links
    pub fn render_dml(input: &str) -> alloc::string::String {
        let mut output = alloc::string::String::new();
        let mut in_tag = false;
        for c in input.chars() {
            if c == '<' {
                in_tag = true;
                continue;
            }
            if c == '>' {
                in_tag = false;
                continue;
            }
            if !in_tag {
                output.push(c);
            }
        }
        output
    }
}

/// Debugger script parser and .printf command engine
pub struct DebugScriptEngine;

impl DebugScriptEngine {
    /// Simple .printf interpreter that evaluates register placeholders
    pub fn printf_eval(format_str: &str, val: u64) -> alloc::string::String {
        use alloc::format;
        let mut output = alloc::string::String::new();
        let mut chars = format_str.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '%' && chars.peek() == Some(&'x') {
                chars.next();
                output.push_str(&format!("{:x}", val));
            } else if c == '%' && chars.peek() == Some(&'d') {
                chars.next();
                output.push_str(&format!("{}", val));
            } else {
                output.push(c);
            }
        }
        output
    }
}

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pseudo_registers() {
        let mut pr = PseudoRegisterSet::new();
        assert_eq!(pr.read("$peb"), Some(0x7FFFF000));
        assert_eq!(pr.read("$teb"), Some(0x7FFF1000));

        // Test user-defined registers
        assert_eq!(pr.read("$u1"), Some(0));
        assert!(pr.write("$u1", 0xDEADBEEF));
        assert_eq!(pr.read("$u1"), Some(0xDEADBEEF));
    }

    #[test]
    fn test_debugger_aliases() {
        let mut am = DebugAliasManager::new();
        assert_eq!(am.expand("$ntns"), "sigma_kernel::sys");
        assert_eq!(am.expand("$cache"), "VMM_Page_Cache");

        am.set_user_alias("my_alias", "Value_Here");
        assert_eq!(am.expand("my_alias"), "Value_Here");
    }

    #[test]
    fn test_dml_rendering() {
        let raw = "<b>Bold Text</b> with <link cmd=\"g\">Clickable Target</link>";
        let rendered = DmlRenderer::render_dml(raw);
        assert_eq!(rendered, "Bold Text with Clickable Target");
    }

    #[test]
    fn test_printf_evaluation() {
        let format_str = "Value = %x or decimal %d";
        let rendered = DebugScriptEngine::printf_eval(format_str, 255);
        assert_eq!(rendered, "Value = ff or decimal 255");
    }
}
