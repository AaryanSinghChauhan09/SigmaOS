//! Linux & BSD-inspired Debugger, Process, and Thread Control Engine (ptrace)
//! Implements trace event queues, exception monitors, singlestep traps, and exception continue-state routing.

use crate::klib::HashMap;
use std::vec::Vec;
use std::string::{String, ToString};

/// ptrace system call requests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtraceRequest {
    TraceMe,
    PeekText,
    PokeText,
    Cont,
    SingleStep,
    Kill,
}

/// Tracee debug events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbgEvent {
    Breakpoint,
    SingleStepTrap,
    SysEnter,
    SysExit,
    ForkTrace,
    ExitTrace,
}

/// Trace state of a specific thread/process
#[derive(Debug, Clone)]
pub struct ThreadDebugContext {
    pub thread_id: u64,
    pub is_traced: bool,
    pub breakpoints: Vec<usize>,      // Virtual addresses of set breakpoints
    pub last_event: Option<DbgEvent>,
    pub single_step_active: bool,
}

impl ThreadDebugContext {
    pub fn new(thread_id: u64) -> Self {
        Self {
            thread_id,
            is_traced: false,
            breakpoints: Vec::new(),
            last_event: None,
            single_step_active: false,
        }
    }
}

/// Dynamic Exception resolution statuses (Handled vs NotHandled)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionStatus {
    Handled,
    NotHandled,
}

/// Exceptions dispatched by the CPU or software traps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionType {
    DivisionByZero,
    Breakpoint,
    AccessViolation,
    IllegalInstruction,
}

pub struct ExceptionMonitor {
    pub debug_contexts: HashMap<u64, ThreadDebugContext>,
    pub debug_event_queue: Vec<(u64, DbgEvent)>, // (ThreadID, Event)
}

impl ExceptionMonitor {
    pub fn new() -> Self {
        Self {
            debug_contexts: HashMap::new(),
            debug_event_queue: Vec::new(),
        }
    }

    /// Registers a thread for active ptrace monitoring
    pub fn attach_tracee(&mut self, thread_id: u64) {
        let mut ctx = ThreadDebugContext::new(thread_id);
        ctx.is_traced = true;
        self.debug_contexts.insert(thread_id, ctx);
    }

    /// Triggers set/poke breakpoint at a memory location
    pub fn set_breakpoint(&mut self, thread_id: u64, address: usize) -> Result<(), &'static str> {
        let ctx = self.debug_contexts.get_mut(&thread_id).ok_or("Thread not traced")?;
        if !ctx.breakpoints.contains(&address) {
            ctx.breakpoints.push(address);
        }
        Ok(())
    }

    /// Processes a debug command request (ptrace style)
    pub fn process_ptrace_request(
        &mut self,
        request: PtraceRequest,
        thread_id: u64,
    ) -> Result<Option<String>, &'static str> {
        let ctx = self.debug_contexts.get_mut(&thread_id).ok_or("Thread not traced")?;

        match request {
            PtraceRequest::TraceMe => {
                ctx.is_traced = true;
                Ok(Some("Tracer attached".to_string()))
            }
            PtraceRequest::PeekText => {
                Ok(Some("PEEKTEXT: [0x90909090]".to_string())) // mock nop code
            }
            PtraceRequest::PokeText => {
                Ok(Some("POKETEXT: Succeeded".to_string()))
            }
            PtraceRequest::Cont => {
                ctx.single_step_active = false;
                ctx.last_event = None;
                Ok(Some("CONTINUE: Process resumed".to_string()))
            }
            PtraceRequest::SingleStep => {
                ctx.single_step_active = true;
                ctx.last_event = Some(DbgEvent::SingleStepTrap);
                self.debug_event_queue.push((thread_id, DbgEvent::SingleStepTrap));
                Ok(Some("SINGLESTEP: Stepped one instruction".to_string()))
            }
            PtraceRequest::Kill => {
                ctx.is_traced = false;
                Ok(Some("KILL: Tracee force-killed".to_string()))
            }
        }
    }

    /// Evaluates hardware/software trap exceptions, routing them through debuggers
    pub fn dispatch_exception(
        &mut self,
        thread_id: u64,
        exception: ExceptionType,
        instruction_pointer: usize,
    ) -> ExceptionStatus {
        if let Some(ctx) = self.debug_contexts.get_mut(&thread_id) {
            if ctx.is_traced {
                match exception {
                    ExceptionType::Breakpoint => {
                        if ctx.breakpoints.contains(&instruction_pointer) {
                            ctx.last_event = Some(DbgEvent::Breakpoint);
                            self.debug_event_queue.push((thread_id, DbgEvent::Breakpoint));
                            return ExceptionStatus::Handled; // Caught and Handled by debugger
                        }
                    }
                    _ => {
                        // Traced exceptions can be intercepted and handled by ptracers
                        return ExceptionStatus::Handled;
                    }
                }
            }
        }
        ExceptionStatus::NotHandled // Will result in kernel crash dump or signal escalation
    }
}

impl Default for ExceptionMonitor {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 5. Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptrace_lifecycle() {
        let mut monitor = ExceptionMonitor::new();
        monitor.attach_tracee(500);

        // Peek text check
        let res = monitor.process_ptrace_request(PtraceRequest::PeekText, 500).unwrap();
        assert!(res.unwrap().contains("PEEKTEXT"));

        // Single step check
        let res = monitor.process_ptrace_request(PtraceRequest::SingleStep, 500).unwrap();
        assert!(res.unwrap().contains("SINGLESTEP"));

        let ctx = monitor.debug_contexts.get(&500).unwrap();
        assert!(ctx.single_step_active);
        assert_eq!(ctx.last_event, Some(DbgEvent::SingleStepTrap));
    }

    #[test]
    fn test_exception_dispatch_and_resolution() {
        let mut monitor = ExceptionMonitor::new();

        // Unregistered thread -> NotHandled (crashes process)
        let status = monitor.dispatch_exception(999, ExceptionType::AccessViolation, 0x1000);
        assert_eq!(status, ExceptionStatus::NotHandled);

        // Attach tracee
        monitor.attach_tracee(500);

        // Breakpoint exception without active set breakpoint -> NotHandled
        let status = monitor.dispatch_exception(500, ExceptionType::Breakpoint, 0x2000);
        assert_eq!(status, ExceptionStatus::NotHandled);

        // Set breakpoint at instruction pointer
        assert!(monitor.set_breakpoint(500, 0x2000).is_ok());

        // Exception at breakpoint address -> Handled! (debugger catches the trap)
        let status = monitor.dispatch_exception(500, ExceptionType::Breakpoint, 0x2000);
        assert_eq!(status, ExceptionStatus::Handled);

        let ctx = monitor.debug_contexts.get(&500).unwrap();
        assert_eq!(ctx.last_event, Some(DbgEvent::Breakpoint));
        assert_eq!(monitor.debug_event_queue.len(), 1);
    }
}
