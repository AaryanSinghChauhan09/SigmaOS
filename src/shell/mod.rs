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
pub use self::sigma_sh::{
    ContextualCompleter, HistoryExpansionEngine, JobControlManager, ParameterExpansionEngine,
    PipelineExecutor, ShellPledgeUnveilGuard, ShellSyntaxHighlighter, ZshPromptFormatter,
};
pub use self::repl::ShellRepl;

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
