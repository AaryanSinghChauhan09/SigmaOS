// SigmaOS Productivity Module
pub mod advanced_app_absorber;
pub mod gamification;
pub mod media;

pub use advanced_app_absorber::{
    AudacityEditor, BraveBrowserEngine, EarTrumpetAudioRouter, EverythingSearchEngine,
    NotepadPlusWorkspace, ObsStudioMixer, OneCommanderDualPane, PotPlayerVlcEngine,
    SevenZipCompressor, ShareXFlameshotEngine,
};
pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use media::{AudioChannel, SigmaMediaEngine, GLOBAL_MEDIA_ENGINE};
