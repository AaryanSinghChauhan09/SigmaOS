//! SigmaOS Debugger Module
//!
//! This module provides debugging tools for the SigmaOS kernel and userland applications,
//! including breakpoints, watchpoints, stack tracing, and memory inspection.
//! Replicates Debian-style debug symbols packages (.dbgsym) and Build ID lookup systems.

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;

pub mod breakpoint;

pub use breakpoint::{Breakpoint, BreakpointID, BreakpointType, DebuggerError, SimpleBreakpoint};
<<<<<<< HEAD
=======

/// Represents a Debian-style debug symbol package containing Build ID mappings
#[derive(Debug, Clone)]
pub struct DebianDbgsymPackage {
    pub build_id: String,           // Unique ELF .note.gnu.build-id hash
    pub package_name: String,       // Target binary name, e.g. "nano"
    pub symbols_map: Vec<(u64, String, String, u32)>, // Address -> (Function, File, Line)
}

impl DebianDbgsymPackage {
    pub fn new(build_id: &str, name: &str) -> Self {
        Self {
            build_id: build_id.to_string(),
            package_name: name.to_string(),
            symbols_map: Vec::new(),
        }
    }

    pub fn register_symbol(&mut self, addr: u64, func: &str, file: &str, line: u32) {
        self.symbols_map.push((addr, func.to_string(), file.to_string(), line));
    }
}
>>>>>>> origin/jules-880081283500171861-1eb07604

/// Breakpoint representation for software debugger
#[derive(Debug, Clone, Copy)]
pub struct Breakpoint {
    pub address: u64,
    pub breakpoint_type: BreakpointType,
    pub enabled: bool,
    pub hit_count: u32,
}

impl Breakpoint {
    pub fn new(address: u64, breakpoint_type: BreakpointType) -> Self {
        Self {
            address,
            breakpoint_type,
            enabled: true,
            hit_count: 0,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn hit(&mut self) {
        self.hit_count += 1;
    }
}

/// Debugger state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebuggerState {
    Detached,
    Attached,
    Running,
    Paused,
    Stepping,
}

/// Stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub address: u64,
    pub function_name: Option<String>,
    pub file_name: Option<String>,
    pub line_number: Option<u32>,
}

impl StackFrame {
    pub fn new(address: u64) -> Self {
        Self {
            address,
            function_name: None,
            file_name: None,
            line_number: None,
        }
    }
}

/// Memory region
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub permissions: u8, // rwx
    pub name: Option<String>,
}

impl MemoryRegion {
    pub fn new(start: u64, end: u64, permissions: u8) -> Self {
        Self {
            start,
            end,
            permissions,
            name: None,
        }
    }

    pub fn contains(&self, address: u64) -> bool {
        address >= self.start && address < self.end
    }

    pub fn is_readable(&self) -> bool {
        self.permissions & 0x4 != 0
    }

    pub fn is_writable(&self) -> bool {
        self.permissions & 0x2 != 0
    }

    pub fn is_executable(&self) -> bool {
        self.permissions & 0x1 != 0
    }
}

/// Main debugger interface
pub struct Debugger {
    state: DebuggerState,
    breakpoints: Vec<SimpleBreakpoint>,
    current_frame: Option<StackFrame>,
    call_stack: Vec<StackFrame>,
    memory_regions: Vec<MemoryRegion>,
    /// Registered Debian dbgsym packages, matching /usr/lib/debug/.build-id/ structures
    pub dbgsym_packages: Vec<DebianDbgsymPackage>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            state: DebuggerState::Detached,
            breakpoints: Vec::new(),
            current_frame: None,
            call_stack: Vec::new(),
            memory_regions: Vec::new(),
            dbgsym_packages: Vec::new(),
        }
    }

    pub fn attach(&mut self) {
        self.state = DebuggerState::Attached;
    }

    pub fn detach(&mut self) {
        self.state = DebuggerState::Detached;
        self.breakpoints.clear();
        self.call_stack.clear();
        self.current_frame = None;
    }

    pub fn pause(&mut self) {
        self.state = DebuggerState::Paused;
    }

    pub fn resume(&mut self) {
        self.state = DebuggerState::Running;
    }

    pub fn step(&mut self) {
        self.state = DebuggerState::Stepping;
    }

    pub fn state(&self) -> DebuggerState {
        self.state
    }

    /// Add a breakpoint
    pub fn add_breakpoint(
        &mut self,
        address: u64,
        breakpoint_type: BreakpointType,
    ) -> Result<(), &'static str> {
        // Check if breakpoint already exists
        if self.breakpoints.iter().any(|bp| bp.address() == address as usize) {
            return Err("Breakpoint already exists at this address");
        }

        let breakpoint = SimpleBreakpoint::new(self.breakpoints.len() + 1, address as usize, breakpoint_type);
        self.breakpoints.push(breakpoint);
        Ok(())
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, address: u64) -> Result<(), &'static str> {
        let original_len = self.breakpoints.len();
        self.breakpoints.retain(|bp| bp.address() != address as usize);

        if self.breakpoints.len() == original_len {
            return Err("Breakpoint not found at this address");
        }

        Ok(())
    }

    /// Enable a breakpoint
    pub fn enable_breakpoint(&mut self, address: u64) -> Result<(), &'static str> {
        let breakpoint = self
            .breakpoints
            .iter_mut()
            .find(|bp| bp.address() == address as usize)
            .ok_or("Breakpoint not found")?;

        breakpoint.enable();
        Ok(())
    }

    /// Disable a breakpoint
    pub fn disable_breakpoint(&mut self, address: u64) -> Result<(), &'static str> {
        let breakpoint = self
            .breakpoints
            .iter_mut()
            .find(|bp| bp.address() == address as usize)
            .ok_or("Breakpoint not found")?;

        breakpoint.disable();
        Ok(())
    }

    /// Check if a breakpoint is hit
    pub fn check_breakpoint(&self, address: u64) -> Option<&SimpleBreakpoint> {
        self.breakpoints
            .iter()
            .find(|bp| bp.address() == address as usize && bp.is_enabled())
    }

    /// Get all breakpoints
    pub fn get_breakpoints(&self) -> &[SimpleBreakpoint] {
        &self.breakpoints
    }

    /// Update current stack frame
    pub fn set_current_frame(&mut self, frame: StackFrame) {
        self.current_frame = Some(frame);
    }

    /// Get current stack frame
    pub fn get_current_frame(&self) -> Option<&StackFrame> {
        self.current_frame.as_ref()
    }

    /// Add a frame to the call stack
    pub fn push_frame(&mut self, frame: StackFrame) {
        self.call_stack.push(frame);
    }

    /// Remove a frame from the call stack
    pub fn pop_frame(&mut self) -> Option<StackFrame> {
        self.call_stack.pop()
    }

    /// Get the call stack
    pub fn get_call_stack(&self) -> &[StackFrame] {
        &self.call_stack
    }

    /// Add a memory region
    pub fn add_memory_region(&mut self, region: MemoryRegion) {
        self.memory_regions.push(region);
    }

    /// Find memory region containing an address
    pub fn find_memory_region(&self, address: u64) -> Option<&MemoryRegion> {
        self.memory_regions
            .iter()
            .find(|region| region.contains(address))
    }

    /// Get all memory regions
    pub fn get_memory_regions(&self) -> &[MemoryRegion] {
        &self.memory_regions
    }

    /// Read memory from a specific address
    pub fn read_memory(&self, address: u64, size: usize) -> Result<Vec<u8>, String> {
        let region = self
            .find_memory_region(address)
            .ok_or("Address not in any known memory region")?;

        if !region.is_readable() {
            return Err("Memory region is not readable".to_string());
        }

        // In a real implementation, this would read from actual memory
        // For now, return placeholder data
        Ok(vec![0; size])
    }

    /// Write memory to a specific address
    pub fn write_memory(&self, address: u64, data: &[u8]) -> Result<(), String> {
        let region = self
            .find_memory_region(address)
            .ok_or("Address not in any known memory region")?;

        if !region.is_writable() {
            return Err("Memory region is not writable".to_string());
        }

        // In a real implementation, this would write to actual memory
        Ok(())
    }

    /// Get register value
    pub fn get_register(&self, register_name: &str) -> Result<u64, String> {
        // In a real implementation, this would read from actual registers
        match register_name {
            "rip" | "pc" => Ok(0),
            "rsp" | "sp" => Ok(0),
            "rbp" | "fp" => Ok(0),
            _ => Err(format!("Unknown register: {}", register_name)),
        }
    }

    /// Set register value.
    pub fn set_register(&self, register_name: &str, _value: u64) -> Result<(), String> {
        // In a real implementation, this would write to actual registers
        match register_name {
            "rip" | "pc" | "rsp" | "sp" | "rbp" | "fp" => Ok(()),
            _ => Err(format!("Unknown register: {}", register_name)),
        }
    }

    /// Registers a Debian-style debug symbol package into the debugger
    pub fn register_dbgsym_package(&mut self, dbgsym: DebianDbgsymPackage) {
        self.dbgsym_packages.push(dbgsym);
    }

    /// Validates if a dbgsym package conforms to standard Build ID formatting (minimum 16-character hex hash)
    pub fn is_debian_dbgsym_compliant(&self, dbgsym: &DebianDbgsymPackage) -> bool {
        if dbgsym.build_id.len() < 16 {
            return false;
        }
        dbgsym.build_id.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Dynamically resolves an instruction address using registered Debian dbgsym packages.
    /// Replicates the build-id symbol lookup of standard Debian debuggers.
    pub fn resolve_address_to_symbol(&self, build_id: &str, address: u64) -> Option<StackFrame> {
        for dbgsym in &self.dbgsym_packages {
            if dbgsym.build_id == build_id {
                for &(symbol_addr, ref func, ref file, line) in &dbgsym.symbols_map {
                    if symbol_addr == address {
                        let mut frame = StackFrame::new(address);
                        frame.function_name = Some(func.clone());
                        frame.file_name = Some(file.clone());
                        frame.line_number = Some(line);
                        return Some(frame);
                    }
                }
            }
        }
        None
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_creation() {
        let bp = SimpleBreakpoint::new(1, 0x1000, BreakpointType::Software);
        assert_eq!(bp.address(), 0x1000);
        assert!(bp.is_enabled());
    }

    #[test]
    fn test_breakpoint_enable_disable() {
        let mut bp = SimpleBreakpoint::new(1, 0x1000, BreakpointType::Software);
        bp.disable();
        assert!(!bp.is_enabled());
        bp.enable();
        assert!(bp.is_enabled());
    }

    #[test]
    fn test_breakpoint_hit() {
        let bp = SimpleBreakpoint::new(1, 0x1000, BreakpointType::Software);
        assert!(bp.is_enabled());
    }

    #[test]
    fn test_debugger_attach_detach() {
        let mut debugger = Debugger::new();
        assert_eq!(debugger.state(), DebuggerState::Detached);

        debugger.attach();
        assert_eq!(debugger.state(), DebuggerState::Attached);

        debugger.detach();
        assert_eq!(debugger.state(), DebuggerState::Detached);
    }

    #[test]
    fn test_debugger_breakpoint_management() {
        let mut debugger = Debugger::new();
        debugger.attach();

        // Add breakpoint
        assert!(debugger
            .add_breakpoint(0x1000, BreakpointType::Software)
            .is_ok());
        assert_eq!(debugger.get_breakpoints().len(), 1);

        // Try to add duplicate
        assert!(debugger
            .add_breakpoint(0x1000, BreakpointType::Software)
            .is_err());

        // Remove breakpoint
        assert!(debugger.remove_breakpoint(0x1000).is_ok());
        assert_eq!(debugger.get_breakpoints().len(), 0);

        // Try to remove non-existent
        assert!(debugger.remove_breakpoint(0x1000).is_err());
    }

    #[test]
    fn test_memory_region() {
        let region = MemoryRegion::new(0x1000, 0x2000, 0x7); // rwx
        assert!(region.contains(0x1000));
        assert!(region.contains(0x1FFF));
        assert!(!region.contains(0x2000));
        assert!(region.is_readable());
        assert!(region.is_writable());
        assert!(region.is_executable());
    }

    #[test]
    fn test_memory_region_permissions() {
        let region = MemoryRegion::new(0x1000, 0x2000, 0x5); // r-x
        assert!(region.is_readable());
        assert!(!region.is_writable());
        assert!(region.is_executable());
    }

    #[test]
    fn test_call_stack() {
        let mut debugger = Debugger::new();
        debugger.attach();

        let frame1 = StackFrame::new(0x1000);
        let frame2 = StackFrame::new(0x2000);

        debugger.push_frame(frame1);
        debugger.push_frame(frame2);

        assert_eq!(debugger.get_call_stack().len(), 2);

        let popped = debugger.pop_frame();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().address, 0x2000);
        assert_eq!(debugger.get_call_stack().len(), 1);
    }

    #[test]
    fn test_debian_dbgsym_symbol_resolver() {
        let mut debugger = Debugger::new();
        debugger.attach();

        let build_id = "a1b2c3d4e5f67890abcdef1234567890".to_string();
        let mut dbgsym = DebianDbgsymPackage::new(&build_id, "nano");
        dbgsym.register_symbol(0x1040, "main", "nano.c", 42);

        assert!(debugger.is_debian_dbgsym_compliant(&dbgsym));
        debugger.register_dbgsym_package(dbgsym);

        // Non-compliant package test (short build id)
        let invalid_dbgsym = DebianDbgsymPackage::new("short_id", "nano");
        assert!(!debugger.is_debian_dbgsym_compliant(&invalid_dbgsym));

        // Resolve instruction address to debug symbol StackFrame
        let frame = debugger.resolve_address_to_symbol(&build_id, 0x1040).unwrap();
        assert_eq!(frame.function_name.unwrap(), "main");
        assert_eq!(frame.file_name.unwrap(), "nano.c");
        assert_eq!(frame.line_number.unwrap(), 42);

        // Fail to resolve address with invalid build id or wrong address
        assert!(debugger.resolve_address_to_symbol("wrong_id", 0x1040).is_none());
        assert!(debugger.resolve_address_to_symbol(&build_id, 0x9999).is_none());
    }
}
