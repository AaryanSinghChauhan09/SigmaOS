pub mod mobile_variant;
pub mod zenith_config;
pub use mobile_variant::*;
pub mod tiling;
pub use tiling::{TilingLayout, TilingWindow, TilingWindowManager, WindowArea, Workspace};
pub mod applications;
pub use applications::{
    AppManager as PackageManager, Application as DesktopApplication, DesktopAppCategory, AppStatus, AppStatistics,
};
pub mod display_manager;
pub use display_manager::{
    DesktopSession, DisplayConfig, DisplayManager, DisplayStatistics, SessionType, UserSession,
};
pub mod desktop_file_manager;
pub use desktop_file_manager::{
    FileInfo, DesktopFileExplorer, DesktopFileExplorerConfig, DesktopFileExplorerStatistics,
    FileType, DesktopSortOrder, DesktopViewMode,
};
pub mod notification_manager;
pub use notification_manager::{
    DesktopNotification, DesktopNotificationManager, DesktopNotificationStatistics,
    DesktopNotificationUrgency,
};
pub mod search_manager;
pub use search_manager::{
    SearchManager, SearchQuery, SearchResult, SearchResultType, SearchStatistics,
};
pub mod application_launcher;
pub use application_launcher::{
    ApplicationLauncher, LauncherApp, LauncherCategory, LauncherStatistics,
};
pub mod screen_saver_manager;
pub use screen_saver_manager::{
    DesktopLockOnSleep, DesktopScreenSaverManager, DesktopScreenSaverMode, DesktopScreenSaverStatistics,
};
pub mod sound_manager;
pub use sound_manager::{
    DesktopAudioDevice, DesktopAudioDeviceStatus, DesktopAudioDeviceType, DesktopSoundApplication, DesktopSoundManager, DesktopSoundProfile, DesktopSoundStatistics,
};
pub mod brightness_manager;
pub use brightness_manager::{
    AdaptiveBrightnessMode, BrightnessDevice, BrightnessManager, BrightnessStatistics, BrightnessType,
};
pub mod session_manager;
pub use session_manager::{
    DesktopSessionManager, DesktopSessionStatistics, DesktopSessionStatus, DesktopSessionType, DesktopUserSession,
};
pub mod wallpaper_manager;
pub use wallpaper_manager::{
    WallpaperManager, WallpaperMode, WallpaperProfile, WallpaperSource, WallpaperStatistics,
};
pub mod de_manager;
pub use de_manager::{
    DEConfig, DEManager, DEStatistics, DesktopDESessionType, DesktopEnvironment,
};
pub mod icon_manager;
pub use icon_manager::{
    DesktopDesktopIcon, DesktopIconManager, GridAlignment, DesktopIconStatistics, DesktopIconType,
};
pub mod taskbar_manager;
pub use taskbar_manager::{
    DesktopTaskbarItem, DesktopTaskbarItemType, DesktopTaskbarManager, DesktopTaskbarPosition, DesktopTaskbarStatistics,
};
pub mod menu_manager;
pub use menu_manager::{
    MenuEntry, MenuEntryType, MenuManager, MenuStatistics,
};
pub mod widget_manager;
pub use widget_manager::{
    DesktopWidget, DesktopWidgetManager, DesktopWidgetPosition, DesktopWidgetStatistics, DesktopWidgetType,
};
pub mod layout_manager;
pub use layout_manager::{
    DesktopLayoutManager, DesktopLayoutStatistics, DesktopLayoutType, WorkspaceLayout,
};
pub mod notification_area_manager;
pub use notification_area_manager::{
    NotificationAreaManager, NotificationAreaStatistics, TrayNotificationItem, TrayNotificationItemType,
};
pub mod workspace_manager;
pub use workspace_manager::{
    DesktopWorkspace, DesktopWorkspaceManager, DesktopWorkspaceStatistics, DesktopWorkspaceType,
};
pub mod quick_settings_manager;
pub use quick_settings_manager::{
    DesktopQuickSetting, DesktopQuickSettingsManager, DesktopQuickSettingsStatistics, DesktopQuickSettingType,
};
pub mod screen_lock_manager;
pub use screen_lock_manager::{
    LockStatus, ScreenLockConfig, ScreenLockManager, ScreenLockStatistics, LockScreenType,
};
pub mod screenshot_manager;
pub use screenshot_manager::{
    DesktopScreenshot, DesktopScreenshotFormat, DesktopScreenshotManager, DesktopScreenshotMode, DesktopScreenshotStatistics,
};
pub mod recent_files_manager;
pub use recent_files_manager::{
    RecentFileEntry, RecentFilesManager, RecentFilesStatistics, RecentFileType,
};
pub mod a11y_manager;
pub use a11y_manager::{
    A11yProfile, A11yCursorSize, A11yManagerStatistics, DesktopA11yManager,
    HighContrastMode, KeyboardRepeat, ScreenReaderMode, TextScaling,
};
pub mod notification_sound_manager;
pub use notification_sound_manager::{
    NotificationSoundManager, NotificationSoundStatistics, NotificationSoundType,
    SoundConfig, SoundEvent,
};
pub mod display_resolution_manager;
pub use display_resolution_manager::{
    Display, DisplayMode, DisplayResolutionManager, DisplayResolutionStatistics,
    RefreshRate, Resolution,
};

// SigmaOS Desktop Module
pub mod mate_betsy;
pub mod mint_tools;
pub mod moksha;
pub mod omarchy_omakase;
pub mod pantheon;
pub mod screensaver;
pub mod sovereign_navigation_engine;
pub mod sovereign_ux_innovation_hub;
pub mod ultimate_distro_desktop;
pub mod wayland_protocol;
pub mod weather_panel;
pub mod web_wasm_bridge;
pub mod zenith_compositor;

pub use sovereign_ux_innovation_hub::*;
pub use wayland_protocol::*;

pub use crate::desktop::sovereign_navigation_engine::*;

pub use sovereign_navigation_engine::*;

pub use sovereign_navigation_engine::*;

pub use ultimate_distro_desktop::{
    ContainerSplitDirection, Gnome46MutterEngine, KRunnerQueryResult, KdePlasma6Engine,
    LuminaBsdDesktopEngine, SwayRegolithWmEngine, SwayWorkspaceContainerNode, ThunarCustomAction,
    Xfce418Engine,
};

pub use web_wasm_bridge::*;

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

pub use zenith_config::{
    AppearanceConfig, CompositorBackend, CompositorConfig, InputConfig, MouseAcceleration,
    OutputScale, Theme, ZenithConfig,
};

pub use crate::desktop::sovereign_navigation_engine::{
    AppCategory, GnomePopLauncherNav, HudActionResult, KrunnerRofiCommandHud, LauncherAppItem,
    NavDirection, RangerDolphinSpatialFileNav, SovereignUniversalNavigationEngine,
    SystemControlNode, TilingWindowManagerNav, WindowNode, YastBsdConfigControlTreeNav,
};

pub use weather_panel::{WeatherCondition, WeatherData, WeatherForecast, WeatherPanel};
pub mod compositor;
pub mod launcher;
pub mod shortcuts;
pub use shortcuts::{
    KeyAction, KeyModifier, KeyboardShortcut, KeyboardShortcutsManager, ShortcutCategory,
    ShortcutConfig, ShortcutRegistrationError,
};
pub mod mint_backup_tool;
pub mod mint_software_store;
pub mod mint_update_manager;
pub mod mint_desktop;
pub mod notification;
pub mod omarchy_dynamic_workspace_suite;
pub use omarchy_dynamic_workspace_suite::*;
pub mod dev_workspace;
pub mod font_manager;
pub mod localization;
pub mod network_sharing;
pub mod onboarding_wizard;
pub mod permission_portal;

pub use font_manager::*;
pub use onboarding_wizard::*;
pub use permission_portal::*;
pub mod cinnamon_xapp_libgui;
pub mod file_manager_extensions;
pub mod system_tray;
pub mod sovereign_bulky_batch_renamer; pub use sovereign_bulky_batch_renamer::*;
pub mod omarchy_disktree_inspector; pub use omarchy_disktree_inspector::*;
pub mod omarchy_chord_rebind_engine; pub use omarchy_chord_rebind_engine::*;
pub mod sovereign_xed_code_editor; pub use sovereign_xed_code_editor::*;
pub mod omarchy_autosave_capture_engine; pub use omarchy_autosave_capture_engine::*;
pub mod omarchy_browser_theme_sync; pub use omarchy_browser_theme_sync::*;
pub mod omarchy_hidpi_scale_engine; pub use omarchy_hidpi_scale_engine::*;
pub mod sigma_quickshell;
pub use sigma_quickshell::{
    SigmaQuickshell, ShellWidget, LayerSurface, Anchors, WidgetContent,
    PanelContent, PanelItem, StatusBarModule, StatusBarContent,
    LauncherContent, AppEntry, HyprlandEvent, ReactiveCell,
    NotificationContent, NotificationUrgency,
};
pub mod sigma_ghostty; pub use sigma_ghostty::{SigmaGhostty, GhosttyConfig, TermGrid, TermCell, TermColor, CellAttrs, VtParser, VtAction, CursorStyle};
pub mod webapp_manager; pub use webapp_manager::*;
