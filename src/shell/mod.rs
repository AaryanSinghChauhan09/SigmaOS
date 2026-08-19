pub mod alias_system;
pub mod command;
pub mod intelligent_terminal;
pub mod kimi_code_agent;
pub mod multicall;
pub mod repl;
pub mod sigma_sh;
pub mod terminal_emulator;

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub use command::ShellCommand;
pub use repl::ShellRepl;
