// SigmaOS Customization Module
pub mod routines;
pub mod theme;

pub use routines::{
    Action, Condition, CustomizationEngine, CustomizationError, Routine, Theme as RoutineTheme, TriggerType,
};
pub use theme::{
    AnimationSettings, BorderRadiusSettings, BuiltInThemeProvider, ColorPalette, CustomThemeProvider,
    ShadowSettings, SpacingSettings, Theme, ThemeEngine, ThemeError, ThemeMode, ThemeProvider,
    TypographySettings,
};
