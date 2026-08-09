// SigmaOS Shell Module
pub mod command;
pub mod repl;
<<<<<<< HEAD
||||||| 0ddf2eac7
pub mod command;
=======
pub mod terminal_emulator;
>>>>>>> origin/jules-523778995335499834-002b2189

pub use repl::{ShellCommand, ShellRepl};
pub use terminal_emulator::{TerminalSession, UserDefinedFunction, AutoSuggestionEngine, AnsiColor};
