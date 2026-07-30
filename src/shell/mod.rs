// SigmaOS Shell Module
pub mod command;
pub mod repl;
<<<<<<< HEAD
=======
#[cfg(any())]
pub mod command;
>>>>>>> origin/jules-6565657164915217370-c04e8c01

pub use command::{CommandError, ShellSession, SimpleShellSession};
pub use repl::{ShellCommand, ShellRepl};
