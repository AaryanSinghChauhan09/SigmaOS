pub mod mobile_variant;
pub mod zenith_config;
pub use mobile_variant::*;
pub mod pipewire_audio;
pub use pipewire_audio::*;
pub mod theme_picker;
pub use theme_picker::*;
pub mod pix_image_viewer;
pub use pix_image_viewer::*;
pub mod slick_greeter;
pub use slick_greeter::*;
pub mod walker_engine;
pub use walker_engine::*;
pub mod text_extraction;
pub use text_extraction::*;
pub mod hypnotix_iptv;
pub use hypnotix_iptv::*;
pub mod hyprland_tiling;
pub use hyprland_tiling::*;
pub mod hyprland;
pub use hyprland::{
    get_hyprland, init_hyprland, HyprlandConfig, HyprlandInstance, HyprlandWindow,
    HyprlandWorkspace, LayoutRule, WindowState,
};
pub mod wayland;
pub use wayland::{
    get_wayland_compositor, init_wayland_compositor, WaylandArg, WaylandCompositor, WaylandDisplay,
    WaylandKeyboard, WaylandMessage, WaylandObject, WaylandObjectType, WaylandPointer, WaylandSeat,
    WaylandSurface, WaylandTouch,
};
pub mod layer_shell;
pub use layer_shell::{
    create_hud_overlay, create_status_bar, get_layer_shell_manager, init_layer_shell_manager,
    Anchor, ExclusiveZone, Layer, LayerShellManager, LayerSurface, LayerSurfaceState,
};
pub mod waybar;
pub use waybar::{
    get_waybar, init_waybar, BarPosition, WaybarConfig, WaybarInstance, WaybarModule,
    WaybarModuleType,
};
pub mod wofi;
pub use wofi::{get_wofi, hide_wofi, init_wofi, show_wofi, AppEntry, WofiInstance};
pub mod mako;
pub use mako::{
    get_mako_daemon, init_mako_daemon, send_notification, Layer as MakoLayer, MakoConfig,
    MakoDaemon, Notification, NotificationUrgency,
};
pub mod swaylock;
pub use swaylock::{
    get_swaylock, init_swaylock, lock_screen, unlock_screen, LockState as SwaylockLockState,
    SwaylockInstance,
};
pub mod direct_scanout;
pub use direct_scanout::{
    disable_direct_scanout, enable_gaming_direct_scanout, get_direct_scanout_manager,
    init_direct_scanout_manager, DirectScanoutConfig, DirectScanoutManager, ScanoutMode,
};
pub mod wlogout;
pub use wlogout::{
    get_wlogout, hide_wlogout, init_wlogout, show_wlogout, LogoutAction, WlogoutInstance,
};
pub mod waypaper;
pub use waypaper::{get_waypaper, init_waypaper, set_wallpaper, Wallpaper, WaypaperInstance};
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
pub mod display_settings_manager;
pub use display_settings_manager::{
    DesktopDisplay, DesktopDisplayMode, DesktopDisplaySettingsManager,
    DesktopDisplaySettingsManagerStatistics, DesktopRefreshRate, DesktopResolution,
};

// SigmaOS Desktop Module

pub mod mate_betsy;
pub mod mint_tools;
pub mod moksha;
pub mod omarchy_omakase;
pub mod omarchy_palette;
pub mod omarchy_theme;
pub mod unified_shell;
pub use unified_shell::{
    get_unified_shell, init_unified_shell, KeyBinding, KeyboardLayout as ShellKeyboardLayout,
    NotificationUrgency as ShellNotificationUrgency, OSDElement, Panel, Plugin, PluginState,
    PluginType, PolicyRule, Theme as ShellTheme, ThemeVariant, UnifiedNotification,
    UnifiedShellEngine, Widget, WidgetPosition,
};
pub mod pantheon;
pub mod screensaver;
pub mod sovereign_navigation_engine;
pub mod sovereign_ux_innovation_hub;
pub mod ultimate_distro_desktop;
pub mod wayland_protocol;
pub mod weather_panel;
pub mod web_wasm_bridge;
pub mod zenith_compositor;
pub mod zenith_quickshell_plugin;

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
    DpmsState, LockState as ScreensaverLockState, ScreenSaverConfig, ScreenSaverEngine,
    ScreenSaverFrame, ScreenSaverMode,
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
    WindowGeometry, WindowState as ZenithWindowState, ZenithCompositor, ZenithWindow,
};

pub use zenith_config::{
    AppearanceConfig, CompositorBackend, CompositorConfig, InputConfig, MouseAcceleration,
    OutputScale, Theme as ZenithTheme, ZenithConfig,
};

pub use crate::desktop::sovereign_navigation_engine::{
    AppCategory, GnomePopLauncherNav, HudActionResult, KrunnerRofiCommandHud, LauncherAppItem,
    NavDirection, RangerDolphinSpatialFileNav, SovereignUniversalNavigationEngine,
    SystemControlNode, TilingWindowManagerNav, WindowNode, YastBsdConfigControlTreeNav,
};

pub use weather_panel::{WeatherCondition, WeatherData, WeatherForecast, WeatherPanel};
pub mod compositor;
pub use compositor::{
    get_desktop_environment, init_desktop_environment, Compositor,
    DesktopEnvironment as CompositorDesktopEnvironment, InputEvent as CompositorInputEvent,
    InputEventType as CompositorInputEventType, Surface as CompositorSurface, SurfaceFormat,
    Window, WindowAttributes, WindowManager, WindowState as CompositorWindowState,
};
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
pub mod sigmanotify;
pub use sigmanotify::{
    get_sigmanotify, init_sigmanotify, show_notification, Notification as SystemNotification,
    NotificationCategory, NotificationUrgency as SigmaNotifyUrgency, SigmaNotify,
};
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
    Anchors, AppEntry as QuickshellAppEntry, HyprlandEvent, LauncherContent,
    LayerSurface as QuickshellLayerSurface, NotificationContent,
    NotificationUrgency as QuickshellNotificationUrgency, PanelContent, PanelItem, ReactiveCell,
    ShellWidget, SigmaQuickshell, StatusBarContent, StatusBarModule, WidgetContent,
};
pub mod sigma_ghostty;
pub use archive_manager::{
    get_archive_manager, init_archive_manager, Archive, ArchiveFormat, ArchiveManager,
    CompressionLevel,
};
pub use audio_editor::{
    get_audio_editor, init_audio_editor, AudioEditor, AudioEffect, AudioFormat, AudioRegion,
    AudioTool, AudioTrack, BitDepth, ChannelCount, SampleRate,
};
pub use audio_recorder::{
    get_audio_recorder, init_audio_recorder, AudioFormat as AudioRecorderFormat, AudioRecorder,
    BitDepth as AudioRecorderBitDepth, Recording, RecordingState,
    SampleRate as AudioRecorderSampleRate,
};
pub use bluetooth_manager::{
    get_bluetooth_manager, init_bluetooth_manager, BluetoothAdapter, BluetoothDevice,
    BluetoothManager, DeviceStatus, DeviceType,
};
pub use calculator::{
    get_calculator, init_calculator, AngleUnit, Calculator, CalculatorError, CalculatorMode,
    MemoryOperation,
};
pub use calendar::{
    get_calendar, init_calendar, Calendar, CalendarManager, Date, Event, EventColor, Recurrence,
    Reminder,
};
pub use clipboard_manager::{
    get_clipboard_manager, init_clipboard_manager, ClipboardItem, ClipboardManager,
};
pub use color_picker::{
    get_color_picker, init_color_picker, Color, ColorFormat, ColorPicker, Palette,
};
pub use contact_manager::{
    get_contact_manager, init_contact_manager, Address, AddressType, Contact, ContactGroup,
    ContactManager, EmailAddress, EmailType, PhoneNumber, PhoneType,
};
pub use dictionary::{
    get_dictionary, init_dictionary, Definition, Dictionary, DictionaryType, Synonym, Word,
};
pub use disk_manager::{
    get_disk_manager, init_disk_manager, Disk, DiskManager, DiskOperation, DiskOperationTask,
    FileSystemType, OperationStatus, Partition, PartitionType,
};
pub use disk_usage_analyzer::{
    get_disk_usage_analyzer, init_disk_usage_analyzer, DiskUsageAnalyzer, FileSystemEntry,
    ScanResult,
};
pub use email_client::{
    get_email_client, init_email_client, AccountType, Email, EmailAccount, EmailAttachment,
    EmailClient, EmailFolder, EmailPriority, EncryptionType,
};
pub use file_manager::{
    get_file_manager, init_file_manager, File, FileManager, FileMetadata, FilePermission,
    FileType as ManagerFileType, SelectionMode, SortOrder, ViewMode,
};
pub use font_manager::{
    get_font_manager, init_font_manager, Font, FontFamily, FontManager, FontStyle,
};
pub use image_editor::{
    get_image_editor, init_image_editor, BlendMode, ColorMode, ImageEditor, ImageFormat,
    Layer as ImageLayer, Selection, ToolType,
};
pub use image_viewer::{
    get_image_viewer, init_image_viewer, FlipDirection, Image, ImageMetadata, ImageViewer,
    Rotation, SlideshowMode, ZoomMode,
};
pub use keyboard_shortcuts::{
    get_keyboard_shortcuts_manager, init_keyboard_shortcuts_manager, KeyModifier as SysKeyModifier,
    KeyboardShortcut as SysKeyboardShortcut,
    KeyboardShortcutsManager as SysKeyboardShortcutsManager, ShortcutAction,
    ShortcutCategory as SysShortcutCategory,
};
pub use media_player::{
    get_media_player, init_media_player, AudioTrack as PlayerAudioTrack, EqualizerBand, MediaItem,
    MediaPlayer, PlaybackState, Playlist, RepeatMode as PlayerRepeatMode, ShuffleMode,
    SubtitleTrack, VideoTrack,
};
pub use news_reader::{get_news_reader, init_news_reader, Feed, FeedItem, FeedType, NewsReader};
pub use notes_app::{get_notes_app, init_notes_app, Note, NoteColor, NoteFolder, NotesApp};
pub use password_manager::{
    get_password_manager, init_password_manager, Folder, GeneratorOptions, PasswordEntry,
    PasswordManager, PasswordStrength,
};
pub use pdf_viewer::{
    get_pdf_viewer, init_pdf_viewer, PageMode, PdfDocument, PdfViewer, ZoomLevel,
};
pub use printer_manager::{
    get_printer_manager, init_printer_manager, ConnectionType, PaperSize, PrintJob as SysPrintJob,
    PrintQuality, Printer as SysPrinter, PrinterManager, PrinterStatus as SysPrinterStatus,
};
pub use screen_recorder::{
    get_screen_recorder, init_screen_recorder, AudioCodec as RecorderAudioCodec,
    ContainerFormat as RecorderContainerFormat, RecordingArea, RecordingConfig, RecordingQuality,
    RecordingState as ScreenRecordingState, ScreenRecorder, VideoCodec as RecorderVideoCodec,
};
pub use screenshot_tool::{
    get_screenshot_tool, init_screenshot_tool, CaptureDelay, CaptureMode,
    ImageFormat as ScreenshotImageFormat, IncludeCursor, Screenshot, ScreenshotOptions,
    ScreenshotTool,
};
pub use search_app::{
    get_search_app, init_search_app, FileTypeFilter, SearchApp, SearchIndex,
    SearchResult as AppSearchResult, SearchType, SortOrder as SearchSortOrder,
};
pub use sigma_ghostty::{
    CellAttrs, CursorStyle, GhosttyConfig, SigmaGhostty, TermCell, TermColor, TermGrid, VtAction,
    VtParser,
};
pub use system_cleaner::{
    get_system_cleaner, init_system_cleaner, CleanResult, CleanerCategory, CleanerOption,
    SystemCleaner,
};
pub use system_info::{
    get_system_info, init_system_info, CpuInfo, DiskInfo, GpuInfo, MemoryInfo, NetworkInfo,
    SystemInfo,
};
pub use task_manager::{get_task_manager, init_task_manager, Process, ProcessState, TaskManager};
pub use terminal::{
    get_terminal, init_terminal, BellMode, ColorScheme as TerminalColorScheme, CursorShape,
    ScrollbarPosition, TerminalEmulator, TerminalProfile, TerminalSession, TerminalTab,
};
pub use text_editor::{
    get_text_editor, init_text_editor, CursorPosition, Document, EditorState, Encoding, Language,
    LineEnding, Selection as TextEditorSelection, TabSize, TextEditor, Theme as TextEditorTheme,
};
pub use themes_manager::{
    get_themes_manager, init_themes_manager, AccentColor, AppearanceSettings,
    ColorScheme as ThemesColorScheme, Theme as ThemesTheme, ThemeType, ThemesManager,
};
pub use timer_app::{
    get_timer_app, init_timer_app, Alarm, AlarmState, Lap, Stopwatch, StopwatchState, Timer,
    TimerApp, TimerState,
};
pub use video_editor::{
    get_video_editor, init_video_editor, AudioCodec as EditorAudioCodec, Clip, ClipType,
    Effect as VideoEffect, FrameRate as VideoFrameRate, Resolution as VideoResolution, Track,
    Transition as VideoTransition, VideoCodec as EditorVideoCodec, VideoEditor,
};
pub use weather_app::{
    get_weather_app, init_weather_app, CurrentWeather, ForecastDay, TemperatureUnit, WeatherApp,
    WeatherCondition as AppWeatherCondition,
};
pub use weather_map::{
    get_weather_map, init_weather_map, MapLayer, MapLocation, MapZoom, WeatherMap,
};
pub mod webapp_manager;
pub use webapp_manager::*;
pub mod appstore;
pub mod clipboard;
pub mod clipboard_advanced;
pub mod desktop_portal;
pub mod elementary_os_github_inspired_components;
pub use elementary_os_github_inspired_components::*;
pub mod archive_manager;
pub mod audio_editor;
pub mod audio_recorder;
pub mod bluetooth_manager;
pub mod calculator;
pub mod calendar;
pub mod clipboard_manager;
pub mod color_picker;
pub mod contact_manager;
pub mod dictionary;
pub mod disk_manager;
pub mod disk_usage_analyzer;
pub mod elementary_pantheon_innovations;
pub mod email_client;
pub mod file_manager;
pub mod filemanager;
pub mod font_manager;
pub mod gaming_engine;
pub mod granite_contractor;
pub mod image_editor;
pub mod image_viewer;
pub mod keyboard_shortcuts;
pub mod mate_packages;
pub mod media_inspired_desktop;
pub mod media_player;
pub mod menu;
pub mod network_panel;
pub mod news_reader;
pub mod notes_app;
pub mod notifications;
pub mod onboarding;
pub mod panel;
pub mod password_manager;
pub mod pdf_viewer;
pub mod printer_manager;
pub mod screen_recorder;
pub mod screenshot;
pub mod screenshot_tool;
pub mod search_app;
pub mod settings;
pub mod sovereign_gui_development_and_pr_suite;
pub mod system_cleaner;
pub mod system_info;
pub mod task_manager;
pub mod terminal;
pub mod text_editor;
pub mod text_scaling;
pub mod themes_manager;
pub mod timer_app;
pub mod universal_desktop_framework;
pub mod video_editor;
pub mod volume_manager;
pub mod weather_app;
pub mod weather_map;
pub mod workspace;
pub mod x11_desktop_environment;
pub mod xfce_engine;
pub mod zenith;
pub use sovereign_gui_development_and_pr_suite::*;
pub mod sovereign_desktop_toolkits_and_services;
pub use sovereign_desktop_toolkits_and_services::*;
pub mod sovereign_desktop_toolkits_and_services_v72_pr;
pub use sovereign_desktop_toolkits_and_services_v72_pr::*;
pub mod sovereign_desktop_toolkits_and_services_v75_pr;
pub use sovereign_desktop_toolkits_and_services_v75_pr::*;
pub mod sovereign_de_wm_innovation_engine;
pub use sovereign_de_wm_innovation_engine::*;
pub mod elementary_os_missing_components_pr;
pub mod zenith_advanced_features;
pub use elementary_os_missing_components_pr::*;

pub mod omarchy_hypr_ecosystem;
pub use omarchy_hypr_ecosystem::*;

pub mod omarchy_media_tools;
pub use omarchy_media_tools::*;

pub mod omarchy_terminal_gamemode;
pub use omarchy_terminal_gamemode::*;

pub mod elementary_os_ecosystem_expansion_v64;
pub use elementary_os_ecosystem_expansion_v64::*;

pub mod elementary_os_missing_components_v66;
pub use elementary_os_missing_components_v66::*;

pub mod elementary_os_missing_components_v72;
pub use elementary_os_missing_components_v72::*;

pub mod sovereign_de_wm_advancements_v68_pr;
pub use sovereign_de_wm_advancements_v68_pr::*;
pub mod sovereign_de_wm_advancements_v74_pr;
pub use sovereign_de_wm_advancements_v74_pr::*;
