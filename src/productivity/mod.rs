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

pub use media::{AudioChannel, GLOBAL_MEDIA_ENGINE, SigmaMediaEngine};
pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use sovereign_apps::{
    ProductivityTask, SigmaOfficeDocument, SigmaTasksBoard, SigmaVaultContainer, TaskPriority,
    TextNode,
};

pub use subtitle_editor::{AegisubEngine, SubtitleEditEngine, SubtitleEntry, SubtitleFormat};

pub use mint_competitor::{
    CinnamonApplet, CinnamonAppletEngine, NvidiaPowerState, NvidiaPrimeApplet,
    NvidiaPrimeProfile, NvidiaPrimeTelemetry, OffloadCommand, SovereignDriverManager,
    SovereignNvidiaPrimeEngine, SovereignSoftwareStore, SovereignUpdateManager,
};
