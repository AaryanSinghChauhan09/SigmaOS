// SPDX-License-Identifier: MIT
/// SigmaOS: Syscall Integration Layer
/// Connects syscall handlers to kernel subsystems (VFS, ProcessManager, SocketTable, Signal Handler)

use crate::filesystem::VirtualFileSystem;
use crate::process::ProcessManager;
use crate::network::{SocketTable, SocketAddr, SocketType, AddressFamily};
use alloc::sync::Arc;
use core::sync::atomic::Mutex;

/// Integrated Syscall Context
/// Holds references to all kernel subsystems needed for syscall processing
pub struct SyscallContext {
    /// Virtual Filesystem
    pub vfs: Arc<Mutex<VirtualFileSystem>>,
    
    /// Process Manager
    pub process_manager: Arc<Mutex<ProcessManager>>,
    
    /// Network Socket Table
    pub socket_table: Arc<Mutex<SocketTable>>,
    
    /// Signal Handler Table (future)
    pub signal_handlers: Arc<Mutex<SignalHandlerTable>>,
}

/// Signal Handler Table
#[derive(Debug, Clone)]
pub struct SignalHandlerTable {
    /// Handlers indexed by signal number (1-64)
    handlers: [Option<SignalHandler>; 64],
}

#[derive(Debug, Clone)]
pub struct SignalHandler {
    pub handler_fn: usize,      // Address of handler function
    pub sa_mask: u64,           // Signals to block during handler
    pub sa_flags: u32,          // Behavior flags
}

impl SignalHandlerTable {
    pub fn new() -> Self {
        Self {
            handlers: [None; 64],
        }
    }
    
    pub fn install_handler(&mut self, sig: u32, handler: SignalHandler) -> Result<(), &'static str> {
        if sig < 1 || sig > 64 {
            return Err("Invalid signal number");
        }
        self.handlers[(sig - 1) as usize] = Some(handler);
        Ok(())
    }
    
    pub fn get_handler(&self, sig: u32) -> Option<&SignalHandler> {
        if sig < 1 || sig > 64 {
            return None;
        }
        self.handlers[(sig - 1) as usize].as_ref()
    }
}

impl Default for SignalHandlerTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SyscallContext {
    /// Create new syscall context with all subsystems
    pub fn new() -> Self {
        Self {
            vfs: Arc::new(Mutex::new(VirtualFileSystem::new())),
            process_manager: Arc::new(Mutex::new(ProcessManager::new())),
            socket_table: Arc::new(Mutex::new(SocketTable::new())),
            signal_handlers: Arc::new(Mutex::new(SignalHandlerTable::new())),
        }
    }
    
    // File Syscall Integration
    
    /// open syscall integration
    pub fn syscall_open(&self, path: &str, flags: i32, mode: u32) -> Result<i32, &'static str> {
        let mut vfs = self.vfs.lock().unwrap();
        vfs.open(path, flags, mode)
            .map_err(|_| "Failed to open file")
            .map(|fd| fd as i32)
    }
    
    /// read syscall integration
    pub fn syscall_read(&self, fd: i32, buf: &mut [u8]) -> Result<usize, &'static str> {
        let mut vfs = self.vfs.lock().unwrap();
        vfs.read(fd as u32, buf)
            .map_err(|_| "Failed to read file")
    }
    
    /// write syscall integration
    pub fn syscall_write(&self, fd: i32, buf: &[u8]) -> Result<usize, &'static str> {
        let mut vfs = self.vfs.lock().unwrap();
        vfs.write(fd as u32, buf)
            .map_err(|_| "Failed to write file")
    }
    
    /// close syscall integration
    pub fn syscall_close(&self, fd: i32) -> Result<(), &'static str> {
        let mut vfs = self.vfs.lock().unwrap();
        vfs.close(fd as u32)
            .map_err(|_| "Failed to close file")
    }
    
    // Process Syscall Integration
    
    /// fork syscall integration
    pub fn syscall_fork(&self) -> Result<i32, &'static str> {
        let mut pm = self.process_manager.lock().unwrap();
        pm.fork()
            .map(|pid| pid as i32)
            .map_err(|_| "Failed to fork process")
    }
    
    /// exec syscall integration
    pub fn syscall_exec(&self, pid: i32, path: &str, args: &[&str]) -> Result<(), &'static str> {
        let mut pm = self.process_manager.lock().unwrap();
        pm.exec(pid as u32, path, args)
            .map_err(|_| "Failed to exec process")
    }
    
    /// exit syscall integration
    pub fn syscall_exit(&self, code: i32) -> ! {
        // In real implementation, would clean up and exit
        // For now, just panic
        panic!("Process exited with code {}", code);
    }
    
    /// wait syscall integration
    pub fn syscall_wait(&self, pid: i32) -> Result<i32, &'static str> {
        let mut pm = self.process_manager.lock().unwrap();
        pm.wait(pid as u32)
            .map(|exit_code| exit_code as i32)
            .map_err(|_| "Failed to wait for process")
    }
    
    // Network Syscall Integration
    
    /// socket syscall integration
    pub fn syscall_socket(&self, family: u32, sock_type: u32) -> Result<i32, &'static str> {
        let family = match family {
            2 => AddressFamily::Ipv4,
            10 => AddressFamily::Ipv6,
            _ => return Err("Unsupported address family"),
        };
        
        let sock_type = match sock_type {
            1 => SocketType::Stream,
            2 => SocketType::Datagram,
            3 => SocketType::Raw,
            _ => return Err("Unsupported socket type"),
        };
        
        let mut table = self.socket_table.lock().unwrap();
        table.socket(family, sock_type, 0)
            .map_err(|_| "Failed to create socket")
    }
    
    /// bind syscall integration
    pub fn syscall_bind(&self, fd: i32, addr: SocketAddr) -> Result<(), &'static str> {
        let mut table = self.socket_table.lock().unwrap();
        let socket = table.get_socket(fd)
            .map_err(|_| "Invalid socket")?;
        socket.bind(addr)
            .map_err(|_| "Failed to bind socket")
    }
    
    /// connect syscall integration
    pub fn syscall_connect(&self, fd: i32, addr: SocketAddr) -> Result<(), &'static str> {
        let mut table = self.socket_table.lock().unwrap();
        let socket = table.get_socket(fd)
            .map_err(|_| "Invalid socket")?;
        socket.connect(addr)
            .map_err(|_| "Failed to connect")
    }
    
    /// listen syscall integration
    pub fn syscall_listen(&self, fd: i32, backlog: u32) -> Result<(), &'static str> {
        let mut table = self.socket_table.lock().unwrap();
        let socket = table.get_socket(fd)
            .map_err(|_| "Invalid socket")?;
        socket.listen(backlog)
            .map_err(|_| "Failed to listen")
    }
    
    /// send syscall integration
    pub fn syscall_send(&self, fd: i32, buf: &[u8]) -> Result<usize, &'static str> {
        let mut table = self.socket_table.lock().unwrap();
        let socket = table.get_socket(fd)
            .map_err(|_| "Invalid socket")?;
        socket.send(buf)
            .map_err(|_| "Failed to send")
            .map(|_| buf.len())
    }
    
    /// recv syscall integration
    pub fn syscall_recv(&self, fd: i32, buf: &mut [u8]) -> Result<usize, &'static str> {
        let mut table = self.socket_table.lock().unwrap();
        let socket = table.get_socket(fd)
            .map_err(|_| "Invalid socket")?;
        socket.recv(buf)
            .map_err(|_| "Failed to receive")
    }
    
    // Signal Syscall Integration
    
    /// rt_sigaction syscall integration
    pub fn syscall_sigaction(&self, sig: u32, handler_fn: usize, sa_flags: u32) -> Result<(), &'static str> {
        let mut handlers = self.signal_handlers.lock().unwrap();
        let handler = SignalHandler {
            handler_fn,
            sa_mask: 0,
            sa_flags,
        };
        handlers.install_handler(sig, handler)
    }
    
    /// kill syscall integration
    pub fn syscall_kill(&self, pid: i32, sig: i32) -> Result<(), &'static str> {
        if sig < 0 || sig > 64 {
            return Err("Invalid signal");
        }
        
        // In real implementation, would:
        // 1. Find process with pid
        // 2. Add signal to pending signal set
        // 3. Wake process if blocked
        // For now, just verify it's valid
        Ok(())
    }
}

impl Default for SyscallContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syscall_context_creation() {
        let ctx = SyscallContext::new();
        assert!(!ctx.vfs.lock().unwrap().is_empty());
    }
    
    #[test]
    fn test_signal_handler_table() {
        let mut handlers = SignalHandlerTable::new();
        let sig_handler = SignalHandler {
            handler_fn: 0x1000,
            sa_mask: 0,
            sa_flags: 0,
        };
        
        assert!(handlers.install_handler(1, sig_handler).is_ok());
        assert!(handlers.get_handler(1).is_some());
        assert!(handlers.get_handler(2).is_none());
    }
    
    #[test]
    fn test_invalid_signal() {
        let mut handlers = SignalHandlerTable::new();
        let sig_handler = SignalHandler {
            handler_fn: 0x1000,
            sa_mask: 0,
            sa_flags: 0,
        };
        
        assert!(handlers.install_handler(0, sig_handler).is_err());
        assert!(handlers.install_handler(65, sig_handler).is_err());
    }
}
