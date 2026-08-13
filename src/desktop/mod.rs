//! Desktop environment module for SigmaOS
//! 
//! Contains GUI components, window management, and desktop applications
//! including the enhanced terminal with SerenityOS-style tabs.

pub mod terminal;

pub use terminal::{
    Terminal, TerminalManager, TerminalTab, TabManager, TabColorScheme,
    SplitDirection, TabSplitConfig, TabGroup, TabStats, TerminalError,
};