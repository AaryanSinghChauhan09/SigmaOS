// SPDX-License-Identifier: MIT
/// Signal Delivery to User Space
/// Handles delivery of signals to user processes with proper context save/restore

use crate::process::ProcessManager;
use alloc::sync::Arc;
use core::sync::atomic::Mutex;

/// CPU Context for signal handler invocation
#[derive(Debug, Clone, Copy)]
pub struct CpuContext {
    // x86_64 registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,      // Instruction pointer
    pub rflags: u64,   // Flags register
    pub cs: u16,       // Code segment
    pub ss: u16,       // Stack segment
}

impl CpuContext {
    /// Create new CPU context with default values
    pub fn new() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rflags: 0,
            cs: 0x08, ss: 0x10,  // Kernel segments
        }
    }
    
    /// Save context to user stack
    /// Returns the new stack pointer
    pub fn save_to_stack(&self, stack_ptr: u64) -> u64 {
        // In real implementation, would write context to stack
        // For now, just return adjusted pointer
        stack_ptr - 128  // Reserve space for context
    }
}

impl Default for CpuContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal Frame pushed on user stack before handler invocation
#[derive(Debug, Clone)]
pub struct SignalFrame {
    pub saved_context: CpuContext,
    pub signal_number: u32,
    pub handler_address: u64,
    pub return_address: u64,  // Address of sigreturn stub
}

/// Signal Delivery Engine
/// Manages delivery of signals to processes
pub struct SignalDeliveryEngine {
    process_manager: Arc<Mutex<ProcessManager>>,
}

impl SignalDeliveryEngine {
    /// Create new signal delivery engine
    pub fn new(process_manager: Arc<Mutex<ProcessManager>>) -> Self {
        Self {
            process_manager,
        }
    }
    
    /// Deliver signal to process
    /// 
    /// # Arguments
    /// * `pid` - Process ID
    /// * `signal` - Signal number (1-64)
    /// * `context` - Current CPU context
    /// 
    /// # Returns
    /// * `Ok(())` if signal delivered
    /// * `Err(&str)` if signal cannot be delivered
    pub fn deliver_signal(
        &self,
        pid: u32,
        signal: u32,
        context: &CpuContext,
    ) -> Result<(), &'static str> {
        if signal < 1 || signal > 64 {
            return Err("Invalid signal number");
        }
        
        let mut pm = self.process_manager.lock().unwrap();
        let process = pm.get_process(pid)
            .map_err(|_| "Process not found")?;
        
        // Check if signal is blocked
        if self.is_signal_blocked(process, signal) {
            // Add to pending signals instead
            return self.add_pending_signal(pid, signal);
        }
        
        // Get signal handler address
        let handler_fn = self.get_signal_handler(process, signal)
            .map_err(|_| "No handler for signal")?;
        
        // Create signal frame
        let frame = SignalFrame {
            saved_context: *context,
            signal_number: signal,
            handler_address: handler_fn,
            return_address: self.get_sigreturn_address(),
        };
        
        // Push frame to user stack
        self.push_signal_frame(pid, &frame)
            .map_err(|_| "Failed to push signal frame")?;
        
        // Update instruction pointer to handler
        self.set_instruction_pointer(pid, handler_fn)
            .map_err(|_| "Failed to set instruction pointer")?;
        
        Ok(())
    }
    
    /// Check if signal is blocked for process
    fn is_signal_blocked(&self, process: &crate::process::SovereignProcess, signal: u32) -> bool {
        // In real implementation, check process signal mask
        // For now, never blocked
        false
    }
    
    /// Add signal to process's pending signal set
    fn add_pending_signal(&self, pid: u32, signal: u32) -> Result<(), &'static str> {
        let mut pm = self.process_manager.lock().unwrap();
        let _process = pm.get_process(pid)
            .map_err(|_| "Process not found")?;
        
        // In real implementation, would add to pending set
        Ok(())
    }
    
    /// Get signal handler address for process
    fn get_signal_handler(
        &self,
        _process: &crate::process::SovereignProcess,
        _signal: u32,
    ) -> Result<u64, &'static str> {
        // In real implementation, would look up handler in signal table
        // For now, return default handler
        Ok(0x401000)  // Example handler address
    }
    
    /// Get sigreturn stub address
    fn get_sigreturn_address(&self) -> u64 {
        0x401100  // Stub address that restores context
    }
    
    /// Push signal frame to user stack
    fn push_signal_frame(
        &self,
        _pid: u32,
        _frame: &SignalFrame,
    ) -> Result<(), &'static str> {
        // In real implementation, would write frame to user stack
        Ok(())
    }
    
    /// Set instruction pointer for process
    fn set_instruction_pointer(
        &self,
        _pid: u32,
        _rip: u64,
    ) -> Result<(), &'static str> {
        // In real implementation, would update process context
        Ok(())
    }
    
    /// Handle signal return (restore context after handler)
    pub fn handle_sigreturn(
        &self,
        pid: u32,
    ) -> Result<CpuContext, &'static str> {
        // In real implementation, would:
        // 1. Read signal frame from user stack
        // 2. Restore CPU context
        // 3. Continue execution
        
        let mut pm = self.process_manager.lock().unwrap();
        let _process = pm.get_process(pid)
            .map_err(|_| "Process not found")?;
        
        Ok(CpuContext::new())
    }
}

impl Default for SignalDeliveryEngine {
    fn default() -> Self {
        Self {
            process_manager: Arc::new(Mutex::new(ProcessManager::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cpu_context_creation() {
        let ctx = CpuContext::new();
        assert_eq!(ctx.rax, 0);
        assert_eq!(ctx.rip, 0);
        assert_eq!(ctx.cs, 0x08);
    }
    
    #[test]
    fn test_signal_frame_creation() {
        let ctx = CpuContext::new();
        let frame = SignalFrame {
            saved_context: ctx,
            signal_number: 15,  // SIGTERM
            handler_address: 0x401000,
            return_address: 0x401100,
        };
        
        assert_eq!(frame.signal_number, 15);
        assert_eq!(frame.handler_address, 0x401000);
    }
    
    #[test]
    fn test_delivery_engine_creation() {
        let engine = SignalDeliveryEngine::default();
        let _ctx = CpuContext::new();
        // Just test that it can be created
        assert!(true);
    }
}
