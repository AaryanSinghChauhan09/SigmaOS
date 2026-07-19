// SigmaOS Productivity Module
pub mod gamification;
pub mod terminal;

pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use terminal::{
    BashShell, ColorScheme, CommandResult, CursorStyle, IntegratedTerminal, SigmaShell, ShellImpl,
    ShellType, TerminalConfig, TerminalError, TerminalSession, ZshShell,
};
