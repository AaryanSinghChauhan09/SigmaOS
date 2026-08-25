// SigmaOS Shell Module
pub mod command;
pub mod repl;
pub mod terminal_emulator;
pub mod alias_system;

// pub use repl::{ShellCommand, ShellRepl};
pub use command::{CommandError, CommandParser, CommandRegistry, ShellCommand, ShellSession, SimpleCommandRegistry, SimpleShellSession};
pub use terminal_emulator::{
    AnsiColor, AutoSuggestionEngine, BracketedPasteBuffer, BsdConsoleColorPalette,
    BsdConsoleTheme, TermiosInputMode, TermiosLineDiscipline, TerminalSession,
    UserDefinedFunction,
};
pub use alias_system::{AliasManager, SigmaAlias, AliasType};
