// SigmaOS Productivity Module
pub mod advanced_app_absorber;
pub mod calendar;
pub mod clipboard_manager;
pub mod document_engine;
pub mod editor;
pub mod email;
pub mod finance;
pub mod flint_chart;
pub mod gamification;
pub mod linux_bsd_tools;
pub mod media;
pub mod mind_map;
pub mod mint_competitor;
pub mod notes;
pub mod pdf;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
pub mod sigmadev;
pub mod subtitle_editor;
pub mod system_settings;
pub mod tasks;
pub mod terminal;
pub mod tmux;
pub mod utility_suite;

pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use subtitle_editor::{AegisubEngine, SubtitleEditEngine, SubtitleEntry, SubtitleFormat};
pub use tmux::{
    LayoutPreset, SplitDirection, TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};
