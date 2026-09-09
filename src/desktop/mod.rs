pub mod mobile_variant;
pub use mobile_variant::*;

// SigmaOS Desktop Module
pub mod mate_betsy;
pub mod mint_desktop;
pub mod mint_tools;
pub mod moksha;
pub mod omarchy_theme;
pub mod omarchy_omakase;
pub mod pantheon;
pub mod screensaver;
pub mod sovereign_navigation_engine;
pub mod zenith_compositor;
pub mod ultimate_distro_desktop;
pub mod wayland_protocol;

pub use wayland_protocol::*;

pub use sovereign_navigation_engine::*;

pub use ultimate_distro_desktop::{
    ContainerSplitDirection, Gnome46MutterEngine, KRunnerQueryResult, KdePlasma6Engine,
    LuminaBsdDesktopEngine, SwayRegolithWmEngine, SwayWorkspaceContainerNode, ThunarCustomAction,
    Xfce418Engine,
};

pub mod web_wasm_bridge;
pub use web_wasm_bridge::*;

pub use mate_betsy::{
    AtrilDocumentViewer, CajaFileManager, EyeOfMateImageViewer, MarcoWindowManager,
    MateBetsyDesktopEnvironment, PlumaTextEditor,
};

pub use mint_desktop::{
    CinnamonDesklet, CinnamonDesktopManager, CinnamonExtension, CinnamonPanel, CinnamonPanelPosition,
    CinnamonTheme, PanelApplet, PanelAppletType, XAppPreferences,
};
pub use omarchy_theme::{
    Color, OmarchyThemeManager, SemanticColor, Theme, ThemeComponent,
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

pub use sovereign_navigation_engine::{
    AppCategory, GnomePopLauncherNav, HudActionResult, KrunnerRofiCommandHud, LauncherAppItem,
    NavDirection, RangerDolphinSpatialFileNav, SovereignUniversalNavigationEngine,
    SystemControlNode, TilingWindowManagerNav, WindowNode, YastBsdConfigControlTreeNav,
};
