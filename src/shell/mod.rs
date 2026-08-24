// SigmaOS Shell Module
pub mod alias_system;
pub mod busybox_applet;
pub mod command;
pub mod intelligent_terminal;
pub mod kimi_code_agent;
pub mod multicall;
pub mod repl;
pub mod sigma_sh;
pub mod terminal_emulator;

pub use command::{CommandError, ShellSession, SimpleShellSession};
pub use repl::{ShellCommand, ShellRepl};
pub use terminal_emulator::{
    AnsiColor, AutoSuggestionEngine, TerminalSession, UserDefinedFunction,
};
