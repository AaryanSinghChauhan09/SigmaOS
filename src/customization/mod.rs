// SigmaOS Customization Module
pub mod routines;
pub mod theme;
pub mod profile;

pub use routines::{
    Action, AutoThemeScheduler, Condition, CustomizationEngine, CustomizationError, Routine,
    SituationalPersonalizer, Theme, TriggerType, WindowGridLayout, WorkspaceLayoutCustomizer,
};

pub use profile::{
    ProfileSwitcher, PerformanceProfile, ZenithProfile, GamifiedProductivity as CustomGamifiedProductivity,
    GameDifficultyBalancer,
};
