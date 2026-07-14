#![no_std]

/// Debugger Suite for SigmaOS
/// Based on 100-Improvement-Ideas.md #74: Debugger suite (kernel + userland)
/// Implements comprehensive debugging for both kernel and userland applications

use core::sync::atomic::{AtomicU64, Ordering};

/// Breakpoint ID type
pub type BreakpointID = u64;

/// Breakpoint types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakpointType {
    Software = 0,
    Hardware = 1,
    Watchpoint = 2,
}

/// Debug session state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugState {
    Running = 0,
    Paused = 1,
    Stopped = 2,
    Error = 3,
}

/// Breakpoint
#[repr(C)]
pub struct Breakpoint {
    pub id: BreakpointID,
    pub address: u64,
    pub breakpoint_type: BreakpointType,
    pub enabled: bool,
    pub hit_count: AtomicU64,
}

impl Breakpoint {
    pub fn new(id: BreakpointID, address: u64, breakpoint_type: BreakpointType) -> Self {
        Breakpoint {
            id,
            address,
            breakpoint_type,
            enabled: true,
            hit_count: AtomicU64::new(0),
        }
    }
    
    pub fn hit(&self) {
        self.hit_count.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn hit_count(&self) -> u64 {
        self.hit_count.load(Ordering::SeqCst)
    }
}

/// Debug session
pub struct DebugSession {
    pub process_id: u64,
    pub state: DebugState,
    pub breakpoints: Vec<Option<Breakpoint>>,
    pub current_address: u64,
    pub next_breakpoint_id: AtomicU64,
}

impl DebugSession {
    pub fn new(process_id: u64) -> Self {
        DebugSession {
            process_id,
            state: DebugState::Running,
            breakpoints: Vec::new(),
            current_address: 0,
            next_breakpoint_id: AtomicU64::new(1),
        }
    }
    
    pub fn set_breakpoint(&mut self, address: u64, breakpoint_type: BreakpointType) -> BreakpointID {
        let id = self.next_breakpoint_id.fetch_add(1, Ordering::SeqCst);
        let breakpoint = Breakpoint::new(id, address, breakpoint_type);
        self.breakpoints.push(Some(breakpoint));
        id
    }
    
    pub fn remove_breakpoint(&mut self, breakpoint_id: BreakpointID) -> bool {
        for bp_option in &mut self.breakpoints {
            if let Some(ref bp) = *bp_option {
                if bp.id == breakpoint_id {
                    *bp_option = None;
                    return true;
                }
            }
        }
        false
    }
    
    pub fn enable_breakpoint(&mut self, breakpoint_id: BreakpointID) -> bool {
        for bp_option in &mut self.breakpoints {
            if let Some(ref mut bp) = *bp_option {
                if bp.id == breakpoint_id {
                    bp.enabled = true;
                    return true;
                }
            }
        }
        false
    }
    
    pub fn disable_breakpoint(&mut self, breakpoint_id: BreakpointID) -> bool {
        for bp_option in &mut self.breakpoints {
            if let Some(ref mut bp) = *bp_option {
                if bp.id == breakpoint_id {
                    bp.enabled = false;
                    return true;
                }
            }
        }
        false
    }
    
    pub fn check_breakpoint(&mut self, address: u64) -> Option<BreakpointID> {
        for bp_option in &mut self.breakpoints {
            if let Some(ref mut bp) = *bp_option {
                if bp.enabled && bp.address == address {
                    bp.hit();
                    return Some(bp.id);
                }
            }
        }
        None
    }
    
    pub fn pause(&mut self) {
        self.state = DebugState::Paused;
    }
    
    pub fn resume(&mut self) {
        self.state = DebugState::Running;
    }
    
    pub fn step(&mut self) {
        self.current_address += 1;
    }
    
    pub fn set_address(&mut self, address: u64) {
        self.current_address = address;
    }
    
    pub fn get_address(&self) -> u64 {
        self.current_address
    }
}

/// Debugger
pub struct Debugger {
    sessions: Vec<Option<DebugSession>>,
    next_session_id: AtomicU64,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            sessions: Vec::new(),
            next_session_id: AtomicU64::new(1),
        }
    }
    
    pub fn attach(&mut self, process_id: u64) -> u64 {
        let session_id = self.next_session_id.fetch_add(1, Ordering::SeqCst);
        let session = DebugSession::new(process_id);
        self.sessions.push(Some(session));
        session_id
    }
    
    pub fn detach(&mut self, session_id: u64) -> bool {
        for session_option in &mut self.sessions {
            if let Some(ref session) = *session_option {
                if session.process_id == session_id {
                    *session_option = None;
                    return true;
                }
            }
        }
        false
    }
    
    pub fn get_session(&mut self, session_id: u64) -> Option<&mut DebugSession> {
        for session_option in &mut self.sessions {
            if let Some(ref mut session) = *session_option {
                if session.process_id == session_id {
                    return Some(session);
                }
            }
        }
        None
    }
    
    pub fn list_sessions(&self) -> Vec<u64> {
        let mut ids = Vec::new();
        for session_option in &self.sessions {
            if let Some(ref session) = *session_option {
                ids.push(session.process_id);
            }
        }
        ids
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

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

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

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

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
