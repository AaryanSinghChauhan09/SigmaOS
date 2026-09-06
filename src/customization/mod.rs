#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Customization Module
pub mod cursor;
pub mod profile;
pub mod routines;
pub mod theme;

pub use routines::{
    Action, AutoThemeScheduler, Condition, CustomizationEngine, CustomizationError, Routine,
    SituationalPersonalizer, Theme, TriggerType, WindowGridLayout, WorkspaceLayoutCustomizer,
};

pub use profile::{
    GameDifficultyBalancer, GamifiedProductivity as CustomGamifiedProductivity, PerformanceProfile,
    ProfileSwitcher, ZenithProfile,
};

pub use cursor::{CursorImageFrame, CursorShape, CursorTheme, CursorThemeEngine};
pub use theme::{
    IconThemeEngine, SigmaSoundscape, SovereignCssColorEngine, ThemeEngine, ThemeProvider,
    ZenithBackdropFilter,
};
