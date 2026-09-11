// src/shell/mod.rs
// SigmaShell module - Desktop shell interface
// Pure Nim implementation (no QML/JS dependencies)

pub mod sigma_sh;
pub use sigma_sh::*;
pub mod command;
pub use command::*;

// FFI bindings to Nim SigmaShell
extern "C" {
    fn sigma_shell_create() -> *mut SigmaShellHandle;
    fn sigma_shell_run(shell: *mut SigmaShellHandle);
    fn sigma_shell_destroy(shell: *mut SigmaShellHandle);
}

#[repr(C)]
pub struct SigmaShellHandle {
    _opaque: [u8; 0],
}

pub struct SigmaShell {
    handle: *mut SigmaShellHandle,
}

impl SigmaShell {
    pub fn new() -> Option<Self> {
        unsafe {
            let handle = sigma_shell_create();
            if handle.is_null() {
                None
            } else {
                Some(Self { handle })
            }
        }
    }
    
    pub fn run(&mut self) {
        unsafe {
            sigma_shell_run(self.handle);
        }
    }
}

impl Drop for SigmaShell {
    fn drop(&mut self) {
        unsafe {
            sigma_shell_destroy(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shell_creation() {
        // Note: Requires Nim library to be compiled
        // let shell = SigmaShell::new();
        // assert!(shell.is_some());
    }
}
