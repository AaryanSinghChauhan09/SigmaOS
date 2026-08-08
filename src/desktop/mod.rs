// SigmaOS Desktop and User Experience Subsystem Mod

pub mod terminal;

pub use terminal::{
    ShellIntegration, SimpleShellIntegration, SimpleTerminal, SimpleTerminalManager, Terminal,
    TerminalError, TerminalID, TerminalManager,
};
