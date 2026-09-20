//! SigmaOS Coreutils
//!
//! Essential command-line utilities for SigmaOS.

pub mod echo;
pub mod false_module;
pub mod pwd;
pub mod true_module;

pub use echo::run as echo_run;
pub use false_module::main as false_main;
pub use pwd::run as pwd_run;
pub use true_module::main as true_main;
