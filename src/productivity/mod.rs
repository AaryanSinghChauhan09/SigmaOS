#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
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
pub mod sovereign_apps;
pub mod subtitle_editor;
pub mod tmux;

pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use media::{AudioChannel, SigmaMediaEngine, GLOBAL_MEDIA_ENGINE};
pub use sovereign_apps::{
    ProductivityTask, SigmaOfficeDocument, SigmaTasksBoard, SigmaVaultContainer, TaskPriority,
    TextNode,
};
pub use tmux::{
    LayoutPreset, SplitDirection, TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};
pub use subtitle_editor::{AegisubEngine, SubtitleEditEngine, SubtitleEntry, SubtitleFormat};
