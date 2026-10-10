pub mod mobile_variant;
pub mod zenith_config;
pub use mobile_variant::*;
pub mod pipewire_audio;
pub use pipewire_audio::*;
pub mod theme_picker;
pub use theme_picker::*;
pub mod pix_image_viewer;
pub use pix_image_viewer::*;
pub mod text_extraction;
pub use text_extraction::*;
pub mod hypnotix_iptv;
pub use hypnotix_iptv::*;
pub mod hyprland_tiling;
pub use hyprland_tiling::*;
pub mod xreader;
pub use xreader::*;
pub mod walker_launcher;
pub use walker_launcher::*;
pub mod photoflare_editor;
pub use photoflare_editor::*;
pub mod screen_recording;
pub use screen_recording::*;
pub mod software_manager;
pub use software_manager::*;
pub mod screenshot_capture;
pub use screenshot_capture::*;
pub mod tiling;
pub use tiling::{TilingLayout, TilingWindow, TilingWindowManager, WindowArea, Workspace};
pub mod applications;
pub use applications::{
    AppManager as PackageManager, AppStatistics, AppStatus, Application as DesktopApplication,
    DesktopAppCategory,
};
pub mod display_manager;
pub use display_manager::{
    DesktopSession, DisplayConfig, DisplayManager, DisplayStatistics, SessionType, UserSession,
};
pub mod desktop_file_manager;
pub use desktop_file_manager::{
    DesktopFileExplorer, DesktopFileExplorerConfig, DesktopFileExplorerStatistics,
    DesktopSortOrder, DesktopViewMode, FileInfo, FileType,
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
    DesktopLockOnSleep, DesktopScreenSaverManager, DesktopScreenSaverMode,
    DesktopScreenSaverStatistics,
};
pub mod sound_manager;
pub use sound_manager::{
    DesktopAudioDevice, DesktopAudioDeviceStatus, DesktopAudioDeviceType, DesktopSoundApplication,
    DesktopSoundManager, DesktopSoundProfile, DesktopSoundStatistics,
};
pub mod brightness_manager;
pub use brightness_manager::{
    AdaptiveBrightnessMode, BrightnessDevice, BrightnessManager, BrightnessStatistics,
    BrightnessType,
};
pub mod session_manager;
pub use session_manager::{
    DesktopSessionManager, DesktopSessionStatistics, DesktopSessionStatus, DesktopSessionType,
    DesktopUserSession,
};
pub mod wallpaper_manager;
pub use wallpaper_manager::{
    WallpaperManager, WallpaperMode, WallpaperProfile, WallpaperSource, WallpaperStatistics,
};
pub mod de_manager;
pub use de_manager::{DEConfig, DEManager, DEStatistics, DesktopDESessionType, DesktopEnvironment};
pub mod icon_manager;
pub use icon_manager::{
    DesktopDesktopIcon, DesktopIconManager, DesktopIconStatistics, DesktopIconType, GridAlignment,
};
pub mod taskbar_manager;
pub use taskbar_manager::{
    DesktopTaskbarItem, DesktopTaskbarItemType, DesktopTaskbarManager, DesktopTaskbarPosition,
    DesktopTaskbarStatistics,
};
pub mod menu_manager;
pub use menu_manager::{MenuEntry, MenuEntryType, MenuManager, MenuStatistics};
pub mod widget_manager;
pub use widget_manager::{
    DesktopWidget, DesktopWidgetManager, DesktopWidgetPosition, DesktopWidgetStatistics,
    DesktopWidgetType,
};
pub mod layout_manager;
pub use layout_manager::{
    DesktopLayoutManager, DesktopLayoutStatistics, DesktopLayoutType, WorkspaceLayout,
};
pub mod notification_area_manager;
pub use notification_area_manager::{
    NotificationAreaManager, NotificationAreaStatistics, TrayNotificationItem,
    TrayNotificationItemType,
};
pub mod workspace_manager;
pub use workspace_manager::{
    DesktopWorkspace, DesktopWorkspaceManager, DesktopWorkspaceStatistics, DesktopWorkspaceType,
};
pub mod quick_settings_manager;
pub use quick_settings_manager::{
    DesktopQuickSetting, DesktopQuickSettingType, DesktopQuickSettingsManager,
    DesktopQuickSettingsStatistics,
};
pub mod screen_lock_manager;
pub use screen_lock_manager::{
    LockScreenType, LockStatus, ScreenLockConfig, ScreenLockManager, ScreenLockStatistics,
};
pub mod screenshot_manager;
pub use screenshot_manager::{
    DesktopScreenshot, DesktopScreenshotFormat, DesktopScreenshotManager, DesktopScreenshotMode,
    DesktopScreenshotStatistics,
};
pub mod recent_files_manager;
pub use recent_files_manager::{
    RecentFileEntry, RecentFileType, RecentFilesManager, RecentFilesStatistics,
};
pub mod a11y_manager;
pub use a11y_manager::{
    A11yCursorSize, A11yManagerStatistics, A11yProfile, DesktopA11yManager, HighContrastMode,
    KeyboardRepeat, ScreenReaderMode, TextScaling,
};
pub mod notification_sound_manager;
pub use notification_sound_manager::{
    NotificationSoundManager, NotificationSoundStatistics, NotificationSoundType, SoundConfig,
    SoundEvent,
};
pub mod display_resolution_manager;
pub use display_resolution_manager::{
    Display, DisplayMode, DisplayResolutionManager, DisplayResolutionStatistics, RefreshRate,
    Resolution,
};
pub mod input_method_manager;
pub use input_method_manager::{
    DesktopInputMethodManager, InputMethodEngine, InputMethodManagerStatistics, InputMethodType,
};
pub mod screen_orientation_manager;
pub use screen_orientation_manager::{
    DesktopScreenOrientationManager, OrientationPolicy, ScreenOrientation, ScreenOrientationConfig,
    ScreenOrientationStatistics,
};
pub mod theme_manager;
pub use theme_manager::{
    DesktopTheme, DesktopThemeManager, DesktopThemeStatistics, DesktopThemeType,
};
pub mod color_scheme_manager;
pub use color_scheme_manager::{
    ColorScheme, ColorSchemeStatistics, ColorSchemeType, DesktopColorSchemeManager,
};
pub mod power_manager;
pub use power_manager::{
    BatteryDevice, BatteryStatus, DesktopPowerManager, PowerAction, PowerManagerStatistics,
    PowerProfile,
};
pub mod touchpad_manager;
pub use touchpad_manager::{
    DesktopTouchpadManager, EdgeScrolling, NaturalScrolling, PalmDetection, TapToClickMode,
    TouchpadConfiguration, TouchpadDevice, TouchpadManagerStatistics, TwoFingerScrolling,
};
pub mod keyboard_manager;
pub use keyboard_manager::{
    DesktopKeyboardManager, KeyboardConfiguration, KeyboardDevice, KeyboardLayout,
    KeyboardManagerStatistics, RepeatMode,
};
pub mod login_manager;
pub use login_manager::{
    DesktopLoginManager, LoginDesktopEnvironment, LoginManagerStatistics, LoginSessionType,
    LoginUserSession,
};
pub mod print_manager;
pub use print_manager::{
    DesktopPrintManager, PrintJob, PrintJobStatus, PrintManagerStatistics, Printer, PrinterStatus,
};
pub mod network_manager;
pub use network_manager::{
    DesktopNetworkManager, NetConnectionStatus, NetConnectionType, NetworkConnection,
    NetworkManagerStatistics, SecurityType,
};
pub mod desktop_bluetooth_manager;
pub use desktop_bluetooth_manager::{
    DesktopBluetoothDevice, DesktopBluetoothDeviceStatus, DesktopBluetoothDeviceType,
    DesktopBluetoothManager, DesktopBluetoothManagerStatistics,
};
pub mod time_manager;
pub use time_manager::{
    DateFormat, DesktopNTPServer, DesktopTimeManager, DesktopTimezone, TimeFormat,
    TimeManagerStatistics,
};
pub mod sound_theme_manager;
pub use sound_theme_manager::{
    DesktopSoundTheme, DesktopSoundThemeManager, DesktopSoundThemeManagerStatistics, SoundEventType,
};
pub mod cursor_manager;
pub use cursor_manager::{
    DesktopCursorManager, DesktopCursorManagerStatistics, DesktopCursorSize, DesktopCursorTheme,
    DesktopCursorType,
};
pub mod font_manager;
pub use font_manager::{
    DesktopFontManager, FontFamily, FontManagerStatistics, FontSlant, FontStyle, FontUsageType,
    FontWeight, FontWidth,
};
pub mod display_settings_manager;
pub use display_settings_manager::{
    DesktopDisplay, DesktopDisplayMode, DesktopDisplaySettingsManager,
    DesktopDisplaySettingsManagerStatistics, DesktopRefreshRate, DesktopResolution,
};

// SigmaOS Desktop Module
pub mod elementary_pantheon_innovations;
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
    ShortcutConfig,
};
pub mod mint_backup_tool;
pub mod mint_desktop;
pub mod mint_software_store;
pub mod mint_update_manager;
pub mod notification;
pub mod omarchy_dynamic_workspace_suite;
pub use omarchy_dynamic_workspace_suite::*;
pub mod dev_workspace;
pub mod localization;
pub mod network_sharing;
pub mod onboarding_wizard;
pub mod permission_portal;

pub use onboarding_wizard::*;
pub use permission_portal::*;
pub mod cinnamon_xapp_libgui;
pub mod file_manager_extensions;
pub mod sovereign_bulky_batch_renamer;
pub mod system_tray;
pub use sovereign_bulky_batch_renamer::*;
pub mod omarchy_disktree_inspector;
pub use omarchy_disktree_inspector::*;
pub mod omarchy_chord_rebind_engine;
pub use omarchy_chord_rebind_engine::*;
pub mod sovereign_xed_code_editor;
pub use sovereign_xed_code_editor::*;
pub mod omarchy_autosave_capture_engine;
pub use omarchy_autosave_capture_engine::*;
pub mod omarchy_browser_theme_sync;
pub use omarchy_browser_theme_sync::*;
pub mod omarchy_hidpi_scale_engine;
pub use omarchy_hidpi_scale_engine::*;
pub mod sigma_quickshell;
pub use sigma_quickshell::{
    Anchors, AppEntry, HyprlandEvent, LauncherContent, LayerSurface, NotificationContent,
    NotificationUrgency, PanelContent, PanelItem, ReactiveCell, ShellWidget, SigmaQuickshell,
    StatusBarContent, StatusBarModule, WidgetContent,
};
pub mod sigma_ghostty;
pub use sigma_ghostty::{
    CellAttrs, CursorStyle, GhosttyConfig, SigmaGhostty, TermCell, TermColor, TermGrid, VtAction,
    VtParser,
};
pub mod webapp_manager;
pub use elementary_pantheon_innovations::*;
pub use webapp_manager::*;
