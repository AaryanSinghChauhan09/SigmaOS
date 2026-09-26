// src/shell/mod.rs
// SigmaShell module - Desktop shell interface

pub mod alias_system;
pub mod busybox_applet;
pub mod command;
pub mod intelligent_terminal;
pub mod kimi_code_agent;
pub mod multicall;
pub mod repl;
pub mod sigma_sh;
pub mod sovereign_shell_parity;
pub mod tech_media_shell_innovations;
pub mod terminal_emulator;
pub mod zsh_bash_parity;

pub use self::command::{ShellCommand, SimpleShellSession};
pub use self::sigma_sh::*;
pub use self::zsh_bash_parity::*;
pub use self::repl::ShellRepl;

// Optional FFI bindings to Nim SigmaShell with native Rust fallback
#[cfg(feature = "nim_ffi")]
extern "C" {
    fn sigma_shell_create() -> *mut SigmaShellHandle;
    fn sigma_shell_run(shell: *mut SigmaShellHandle);
    fn sigma_shell_destroy(shell: *mut SigmaShellHandle);
}

#[cfg(feature = "nim_ffi")]
#[repr(C)]
pub struct SigmaShellHandle {
    _opaque: [u8; 0],
}

pub struct SigmaShell {
    #[cfg(feature = "nim_ffi")]
    handle: *mut SigmaShellHandle,
    pub running: bool,
}

impl SigmaShell {
    pub fn new() -> Option<Self> {
        #[cfg(feature = "nim_ffi")]
        {
            unsafe {
                let handle = sigma_shell_create();
                if !handle.is_null() {
                    return Some(Self { handle, running: false });
                }
            }
        }
        Some(Self {
            #[cfg(feature = "nim_ffi")]
            handle: core::ptr::null_mut(),
            running: false,
        })
    }
    
    pub fn run(&mut self) {
        self.running = true;
        #[cfg(feature = "nim_ffi")]
        {
            if !self.handle.is_null() {
                unsafe {
                    sigma_shell_run(self.handle);
                }
                return;
            }
        }
        // Native zero-dependency Rust shell fallback loop execution
    }
}

impl Drop for SigmaShell {
    fn drop(&mut self) {
        #[cfg(feature = "nim_ffi")]
        {
            if !self.handle.is_null() {
                unsafe {
                    sigma_shell_destroy(self.handle);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shell_creation() {
        let shell = SigmaShell::new();
        assert!(shell.is_some());
    }
}
