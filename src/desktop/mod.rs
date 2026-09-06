#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::new_without_default)]
pub mod mobile_variant;
pub use mobile_variant::*;

// SigmaOS Desktop Module
pub mod mate_betsy;
pub mod mint_tools;
pub mod moksha;
pub mod omarchy_omakase;
pub mod pantheon;
pub mod screensaver;
pub mod web_wasm_bridge;
pub mod zenith_compositor;
pub mod ultimate_distro_desktop;

pub use ultimate_distro_desktop::{
    ContainerSplitDirection, Gnome46MutterEngine, KRunnerQueryResult, KdePlasma6Engine,
    LuminaBsdDesktopEngine, SwayRegolithWmEngine, SwayWorkspaceContainerNode, ThunarCustomAction,
    Xfce418Engine,
};

// web_wasm_bridge items accessed via pub mod web_wasm_bridge above

pub use mate_betsy::{
    AtrilDocumentViewer, CajaFileManager, EyeOfMateImageViewer, MarcoWindowManager,
    MateBetsyDesktopEnvironment, PlumaTextEditor,
};

pub use mint_tools::{
    AppMetadata, MintSoftwareManager, MintTimeshiftEngine, MintUpdateManager, SnapshotType,
    TimeshiftSnapshot, UpdateLevel, UpdatePackage,
};

pub use screensaver::{
    DpmsState, LockState, ScreenSaverConfig, ScreenSaverEngine, ScreenSaverFrame, ScreenSaverMode,
};

pub use pantheon::{
    AppCenter, AppCenterProduct, GalaTransitionStyle, GalaWindowManager, PantheonGreeter,
    PlankDock, PlankDockItem, SlingshotApp, SlingshotCategory, SlingshotLauncher, Wingpanel,
    WingpanelIndicator,
};

pub use moksha::{
    BodhiAppCenterInstaller, EphotoViewer, EvasCanvasManager, EvasObject, MokshaProfile,
    MokshaWindowManager, MokshaWindowType, ShelfOrientation, TerminologyBackend,
    WallpaperTransition,
};

pub use zenith_compositor::{
    DamageRegion, InputEvent, InputEventData, InputEventType, Output, Surface, SurfaceType,
    WindowGeometry, WindowState, ZenithCompositor, ZenithWindow,
};

pub use omarchy_omakase::{
    AgenticWorkstationLayout, AgenticWorkstationOrchestrator, OmakasePresetConfig,
    OmarchySystemEngine, WorkstationPane, WorkstationPaneRole,
};
