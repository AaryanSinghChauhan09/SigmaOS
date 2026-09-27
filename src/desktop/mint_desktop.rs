//! Linux Mint Cinnamon-inspired Desktop Environment
//! 
//! This module implements desktop environment features inspired by Linux Mint's
//! Cinnamon desktop, including panels, desklets, themes, extensions, and XApp integration.

#![allow(dead_code)]



use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Cinnamon panel position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinnamonPanelPosition {
    /// Top of screen
    Top,
    /// Bottom of screen
    Bottom,
    /// Left of screen
    Left,
    /// Right of screen
    Right,
}

impl CinnamonPanelPosition {
    /// Get display name for the position
    pub fn display_name(&self) -> &'static str {
        match self {
            CinnamonPanelPosition::Top => "Top",
            CinnamonPanelPosition::Bottom => "Bottom",
            CinnamonPanelPosition::Left => "Left",
            CinnamonPanelPosition::Right => "Right",
        }
    }
}

/// Panel applet type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelAppletType {
    /// Application menu launcher
    Menu,
    /// Task switcher
    TaskSwitcher,
    /// System tray
    SystemTray,
    /// Clock/calendar
    Clock,
    /// Volume control
    Volume,
    /// Network status
    Network,
    /// Battery indicator
    Battery,
    /// Custom applet
    Custom(String),
}

/// Panel applet configuration
#[derive(Debug, Clone)]
pub struct PanelApplet {
    /// Applet type
    pub applet_type: PanelAppletType,
    /// Applet ID
    pub id: String,
    /// Whether applet is enabled
    pub enabled: bool,
    /// Applet position in panel (left-to-right index)
    pub position: u32,
    /// Applet-specific configuration
    pub config: BTreeMap<String, String>,
}

/// Cinnamon panel configuration
#[derive(Debug, Clone)]
pub struct CinnamonPanel {
    /// Panel ID
    pub id: String,
    /// Panel position
    pub position: CinnamonPanelPosition,
    /// Panel height in pixels
    pub height: u32,
    /// Whether panel is auto-hidden
    pub auto_hide: bool,
    /// Panel applets
    pub applets: Vec<PanelApplet>,
    /// Panel background color (hex)
    pub background_color: String,
    /// Panel opacity (0-255)
    pub opacity: u8,
}

impl CinnamonPanel {
    /// Create a new panel
    pub fn new(id: String, position: CinnamonPanelPosition) -> Self {
        Self {
            id,
            position,
            height: 40,
            auto_hide: false,
            applets: Vec::new(),
            background_color: "#2c2c2c".to_string(),
            opacity: 255,
        }
    }

    /// Add an applet to the panel
    pub fn add_applet(&mut self, applet: PanelApplet) {
        self.applets.push(applet);
    }

    /// Remove an applet by ID
    pub fn remove_applet(&mut self, applet_id: &str) {
        self.applets.retain(|a| a.id != applet_id);
    }

    /// Get applet by ID
    pub fn get_applet(&self, applet_id: &str) -> Option<&PanelApplet> {
        self.applets.iter().find(|a| a.id == applet_id)
    }

    /// Move applet to new position
    pub fn move_applet(&mut self, applet_id: &str, new_position: u32) {
        if let Some(idx) = self.applets.iter().position(|a| a.id == applet_id) {
            let mut applet = self.applets.remove(idx);
            applet.position = new_position;
            // Insert at correct position
            let insert_idx = self
                .applets
                .binary_search_by_key(&new_position, |a| a.position)
                .unwrap_or_else(|e| e);
            self.applets.insert(insert_idx, applet);
        }
    }
}

/// Desklet (desktop widget) configuration
#[derive(Debug, Clone)]
pub struct CinnamonDesklet {
    /// Desklet ID
    pub id: String,
    /// Desklet name
    pub name: String,
    /// Desklet UUID
    pub uuid: String,
    /// X position on desktop
    pub x: i32,
    /// Y position on desktop
    pub y: i32,
    /// Desklet width
    pub width: u32,
    /// Desklet height
    pub height: u32,
    /// Whether desklet is enabled
    pub enabled: bool,
    /// Desklet-specific configuration
    pub config: BTreeMap<String, String>,
}

impl CinnamonDesklet {
    /// Create a new desklet
    pub fn new(id: String, name: String, uuid: String) -> Self {
        Self {
            id,
            name,
            uuid,
            x: 100,
            y: 100,
            width: 200,
            height: 150,
            enabled: true,
            config: BTreeMap::new(),
        }
    }
}

/// Cinnamon theme configuration
#[derive(Debug, Clone)]
pub struct CinnamonTheme {
    /// Theme name
    pub name: String,
    /// Theme ID
    pub id: String,
    /// GTK theme name
    pub gtk_theme: String,
    /// Icon theme name
    pub icon_theme: String,
    /// Window theme name
    pub window_theme: String,
    /// Cursor theme name
    pub cursor_theme: String,
    /// Font name
    pub font_name: String,
    /// Font size
    pub font_size: u32,
    /// Whether theme is dark
    pub dark_mode: bool,
}

impl CinnamonTheme {
    /// Create a new theme
    pub fn new(name: String, id: String) -> Self {
        Self {
            name,
            id,
            gtk_theme: "Adwaita".to_string(),
            icon_theme: "Adwaita".to_string(),
            window_theme: "Adwaita".to_string(),
            cursor_theme: "default".to_string(),
            font_name: "Sans".to_string(),
            font_size: 11,
            dark_mode: false,
        }
    }

    /// Apply dark mode
    pub fn set_dark_mode(&mut self, dark: bool) {
        self.dark_mode = dark;
        if dark {
            self.gtk_theme = "Adwaita-dark".to_string();
        } else {
            self.gtk_theme = "Adwaita".to_string();
        }
    }
}

/// Cinnamon extension configuration
#[derive(Debug, Clone)]
pub struct CinnamonExtension {
    /// Extension ID
    pub id: String,
    /// Extension name
    pub name: String,
    /// Extension UUID
    pub uuid: String,
    /// Extension description
    pub description: String,
    /// Extension version
    pub version: String,
    /// Whether extension is enabled
    pub enabled: bool,
    /// Extension-specific configuration
    pub config: BTreeMap<String, String>,
}

impl CinnamonExtension {
    /// Create a new extension
    pub fn new(id: String, name: String, uuid: String) -> Self {
        Self {
            id,
            name,
            uuid,
            description: String::new(),
            version: "1.0.0".to_string(),
            enabled: true,
            config: BTreeMap::new(),
        }
    }
}

/// Titlebar layout style for XApp applications across desktop environments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XAppTitlebarStyle {
    /// Traditional titlebar with separate menubar and toolbar
    Traditional,
    /// Headerbar / CSD (Client-Side Decoration) titlebar
    HeaderBar,
    /// Compact headerbar with integrated tabs
    CompactHeaderBar,
    /// Seamless borderless window with overlay controls
    SeamlessOverlay,
}

/// XApp preferences (cross-desktop integration & common resources)
#[derive(Debug, Clone)]
pub struct XAppPreferences {
    /// Application name
    pub app_name: String,
    /// Dark mode preference
    pub dark_mode: bool,
    /// Accent color (hex)
    pub accent_color: String,
    /// Enable animations
    pub enable_animations: bool,
    /// Animation speed (1-10)
    pub animation_speed: u8,
    /// Enable sound effects
    pub enable_sounds: bool,
    /// Default font
    pub default_font: String,
    /// Monospace font
    pub monospace_font: String,
    /// Locale
    pub locale: String,
    /// Time format (12h or 24h)
    pub time_format: String,
    /// Window button layout (e.g., "close,minimize,maximize" or "minimize,maximize:close")
    pub window_button_layout: String,
    /// Titlebar style (Traditional vs CSD HeaderBar)
    pub titlebar_style: XAppTitlebarStyle,
    /// Automatically synchronize GTK and Qt widget themes
    pub sync_gtk_qt_themes: bool,
    /// Enable overlay scrollbars
    pub overlay_scrollbars: bool,
    /// Compact mode for toolbars and sidebars
    pub compact_mode: bool,
    /// Enable symbolic icons in sidebar navigation
    pub symbolic_sidebar_icons: bool,
}

impl XAppPreferences {
    /// Create new XApp preferences
    pub fn new(app_name: String) -> Self {
        Self {
            app_name,
            dark_mode: false,
            accent_color: "#3daee9".to_string(),
            enable_animations: true,
            animation_speed: 5,
            enable_sounds: true,
            default_font: "Sans 11".to_string(),
            monospace_font: "Monospace 10".to_string(),
            locale: "en_US.UTF-8".to_string(),
            time_format: "24h".to_string(),
            window_button_layout: "close,minimize,maximize".to_string(),
            titlebar_style: XAppTitlebarStyle::Traditional,
            sync_gtk_qt_themes: true,
            overlay_scrollbars: true,
            compact_mode: false,
            symbolic_sidebar_icons: true,
        }
    }

    /// Set dark mode
    pub fn set_dark_mode(&mut self, dark: bool) {
        self.dark_mode = dark;
    }

    /// Set accent color
    pub fn set_accent_color(&mut self, color: String) {
        self.accent_color = color;
    }

    /// Set locale
    pub fn set_locale(&mut self, locale: String) {
        self.locale = locale;
    }

    /// Set window button layout
    pub fn set_window_button_layout(&mut self, layout: String) {
        self.window_button_layout = layout;
    }

    /// Set titlebar style
    pub fn set_titlebar_style(&mut self, style: XAppTitlebarStyle) {
        self.titlebar_style = style;
    }

    /// Generate unified GTK/Qt theme environment variables
    pub fn generate_desktop_theme_env(&self) -> Vec<(String, String)> {
        let mut envs = Vec::new();
        let gtk_theme = if self.dark_mode { "Adwaita-dark" } else { "Adwaita" };
        envs.push(("GTK_THEME".to_string(), gtk_theme.to_string()));
        envs.push(("QT_STYLE_OVERRIDE".to_string(), "kvantum".to_string()));
        envs.push(("XAPP_ACCENT_COLOR".to_string(), self.accent_color.clone()));
        envs.push(("XAPP_BUTTON_LAYOUT".to_string(), self.window_button_layout.clone()));
        envs
    }
}

/// Category of favorite item managed by XApp
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XAppFavoriteKind {
    /// Document file (PDF, text, office)
    Document,
    /// Image or media file
    Media,
    /// Desktop application shortcut (.desktop)
    Application,
    /// Folder or directory
    Folder,
    /// Web URL / bookmark
    WebBookmark,
}

/// Favorite item entry managed across desktop environments
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XAppFavoriteItem {
    /// Unique identifier or path
    pub uri: String,
    /// Display title
    pub display_name: String,
    /// Category kind
    pub kind: XAppFavoriteKind,
    /// Associated icon name
    pub icon_name: String,
    /// Associated MIME type
    pub mime_type: String,
    /// Timestamp when pinned (seconds since Epoch)
    pub pinned_at_secs: u64,
    /// Position index for custom ordering
    pub order_index: u32,
}

/// Cross-desktop favorites manager inspired by Linux Mint xapp-favorites
#[derive(Debug, Clone)]
pub struct XAppFavoritesManager {
    /// List of pinned favorite items
    pub items: Vec<XAppFavoriteItem>,
    /// Maximum number of allowed favorite items
    pub max_items: usize,
}

impl XAppFavoritesManager {
    /// Create a new XApp favorites manager
    pub fn new(max_items: usize) -> Self {
        Self {
            items: Vec::new(),
            max_items,
        }
    }

    /// Pin a new item to favorites
    pub fn pin_item(
        &mut self,
        uri: &str,
        display_name: &str,
        kind: XAppFavoriteKind,
        icon_name: &str,
        mime_type: &str,
        timestamp: u64,
    ) -> Result<bool, &'static str> {
        if self.items.iter().any(|i| i.uri == uri) {
            return Ok(false); // Already pinned
        }
        if self.items.len() >= self.max_items {
            return Err("Maximum favorites limit reached");
        }

        let order = self.items.len() as u32;
        self.items.push(XAppFavoriteItem {
            uri: uri.to_string(),
            display_name: display_name.to_string(),
            kind,
            icon_name: icon_name.to_string(),
            mime_type: mime_type.to_string(),
            pinned_at_secs: timestamp,
            order_index: order,
        });
        Ok(true)
    }

    /// Unpin an item from favorites by URI
    pub fn unpin_item(&mut self, uri: &str) -> bool {
        let original_len = self.items.len();
        self.items.retain(|i| i.uri != uri);
        if self.items.len() < original_len {
            self.reindex_order();
            true
        } else {
            false
        }
    }

    /// Check if a URI is pinned
    pub fn is_pinned(&self, uri: &str) -> bool {
        self.items.iter().any(|i| i.uri == uri)
    }

    /// Get favorites filtered by category kind
    pub fn get_by_kind(&self, kind: XAppFavoriteKind) -> Vec<&XAppFavoriteItem> {
        self.items.iter().filter(|i| i.kind == kind).collect()
    }

    /// Search favorite items by query string
    pub fn search(&self, query: &str) -> Vec<&XAppFavoriteItem> {
        let q = query.to_lowercase();
        self.items
            .iter()
            .filter(|i| {
                i.display_name.to_lowercase().contains(&q)
                    || i.uri.to_lowercase().contains(&q)
                    || i.mime_type.to_lowercase().contains(&q)
            })
            .collect()
    }

    /// Move item to a new order index
    pub fn reorder_item(&mut self, uri: &str, new_index: usize) -> bool {
        if let Some(pos) = self.items.iter().position(|i| i.uri == uri) {
            let item = self.items.remove(pos);
            let target = new_index.min(self.items.len());
            self.items.insert(target, item);
            self.reindex_order();
            true
        } else {
            false
        }
    }

    fn reindex_order(&mut self) {
        for (idx, item) in self.items.iter_mut().enumerate() {
            item.order_index = idx as u32;
        }
    }
}

impl Default for XAppFavoritesManager {
    fn default() -> Self {
        Self::new(100)
    }
}

/// Action item for XApp status icon context menus
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XAppStatusAction {
    pub id: String,
    pub label: String,
    pub enabled: bool,
}

/// StatusNotifierItem / XApp StatusIcon representation for cross-desktop system tray icons
#[derive(Debug, Clone)]
pub struct XAppStatusIcon {
    pub id: String,
    pub title: String,
    pub icon_name: String,
    pub badge_count: u32,
    pub tooltip: String,
    pub visible: bool,
    pub actions: Vec<XAppStatusAction>,
}

/// Status notifier manager providing cross-desktop system tray icon & badge support
#[derive(Debug, Clone)]
pub struct XAppStatusNotifier {
    pub icons: Vec<XAppStatusIcon>,
}

impl XAppStatusNotifier {
    pub fn new() -> Self {
        Self { icons: Vec::new() }
    }

    pub fn register_icon(&mut self, id: &str, title: &str, icon_name: &str) {
        if !self.icons.iter().any(|i| i.id == id) {
            self.icons.push(XAppStatusIcon {
                id: id.to_string(),
                title: title.to_string(),
                icon_name: icon_name.to_string(),
                badge_count: 0,
                tooltip: title.to_string(),
                visible: true,
                actions: Vec::new(),
            });
        }
    }

    pub fn update_badge(&mut self, id: &str, count: u32, tooltip: &str) -> bool {
        if let Some(icon) = self.icons.iter_mut().find(|i| i.id == id) {
            icon.badge_count = count;
            icon.tooltip = tooltip.to_string();
            true
        } else {
            false
        }
    }

    pub fn add_context_action(&mut self, icon_id: &str, action_id: &str, label: &str) -> bool {
        if let Some(icon) = self.icons.iter_mut().find(|i| i.id == icon_id) {
            if !icon.actions.iter().any(|a| a.id == action_id) {
                icon.actions.push(XAppStatusAction {
                    id: action_id.to_string(),
                    label: label.to_string(),
                    enabled: true,
                });
            }
            true
        } else {
            false
        }
    }

    pub fn remove_icon(&mut self, id: &str) -> bool {
        let len = self.icons.len();
        self.icons.retain(|i| i.id != id);
        self.icons.len() < len
    }

    pub fn get_icon(&self, id: &str) -> Option<&XAppStatusIcon> {
        self.icons.iter().find(|i| i.id == id)
    }
}

impl Default for XAppStatusNotifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Thumbnail size classification for XApp thumbnail service
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XAppThumbnailSize {
    Normal128,
    Large256,
    XLarge512,
}

/// Thumbnail generation result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XAppThumbnailResult {
    pub original_uri: String,
    pub thumbnail_path: String,
    pub size: XAppThumbnailSize,
    pub success: bool,
}

/// Cross-desktop file preview and thumbnail generator service
#[derive(Debug, Clone)]
pub struct XAppThumbnailerService {
    pub cache_dir: String,
    pub supported_mime_prefixes: Vec<String>,
}

impl XAppThumbnailerService {
    pub fn new(cache_dir: &str) -> Self {
        Self {
            cache_dir: cache_dir.to_string(),
            supported_mime_prefixes: vec![
                "image/".to_string(),
                "video/".to_string(),
                "application/pdf".to_string(),
                "text/".to_string(),
            ],
        }
    }

    pub fn can_thumbnail(&self, mime_type: &str) -> bool {
        self.supported_mime_prefixes
            .iter()
            .any(|prefix| mime_type.starts_with(prefix))
    }

    pub fn generate_thumbnail(
        &self,
        uri: &str,
        mime_type: &str,
        size: XAppThumbnailSize,
    ) -> XAppThumbnailResult {
        if !self.can_thumbnail(mime_type) {
            return XAppThumbnailResult {
                original_uri: uri.to_string(),
                thumbnail_path: String::new(),
                size,
                success: false,
            };
        }

        let hash_str = uri.len();
        let thumb_path = match size {
            XAppThumbnailSize::Normal128 => format!("{}/normal/{}.png", self.cache_dir, hash_str),
            XAppThumbnailSize::Large256 => format!("{}/large/{}.png", self.cache_dir, hash_str),
            XAppThumbnailSize::XLarge512 => format!("{}/xlarge/{}.png", self.cache_dir, hash_str),
        };

        XAppThumbnailResult {
            original_uri: uri.to_string(),
            thumbnail_path: thumb_path,
            size,
            success: true,
        }
    }
}

impl Default for XAppThumbnailerService {
    fn default() -> Self {
        Self::new("/tmp/xapp-thumbnail-cache")
    }
}

/// Hardware media key actions for cross-desktop playback control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XAppMediaKeyAction {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    MuteToggle,
    VolumeUp,
    VolumeDown,
}

/// Media key subscriber registration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaKeySubscriber {
    pub app_id: String,
    pub priority: u32,
}

/// Cross-desktop hardware media key dispatcher
#[derive(Debug, Clone)]
pub struct XAppMediaKeysDispatcher {
    pub subscribers: Vec<MediaKeySubscriber>,
    pub active_app_id: Option<String>,
}

impl XAppMediaKeysDispatcher {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
            active_app_id: None,
        }
    }

    pub fn register_subscriber(&mut self, app_id: &str, priority: u32) {
        if !self.subscribers.iter().any(|s| s.app_id == app_id) {
            self.subscribers.push(MediaKeySubscriber {
                app_id: app_id.to_string(),
                priority,
            });
            self.subscribers.sort_by(|a, b| b.priority.cmp(&a.priority));
        }
    }

    pub fn set_active_app(&mut self, app_id: &str) {
        if self.subscribers.iter().any(|s| s.app_id == app_id) {
            self.active_app_id = Some(app_id.to_string());
        }
    }

    pub fn dispatch_action(&self, action: XAppMediaKeyAction) -> Option<String> {
        if let Some(ref active) = self.active_app_id {
            Some(format!("Dispatched {:?} to active app {}", action, active))
        } else if let Some(top) = self.subscribers.first() {
            Some(format!("Dispatched {:?} to top priority app {}", action, top.app_id))
        } else {
            None
        }
    }
}

impl Default for XAppMediaKeysDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Master suite unifying all Linux Mint XApp-inspired common resources & libraries
#[derive(Debug, Clone)]
pub struct SovereignXAppCrossDesktopSuite {
    pub preferences: XAppPreferences,
    pub favorites: XAppFavoritesManager,
    pub status_notifier: XAppStatusNotifier,
    pub thumbnailer: XAppThumbnailerService,
    pub media_keys: XAppMediaKeysDispatcher,
}

impl SovereignXAppCrossDesktopSuite {
    pub fn new(app_name: &str) -> Self {
        Self {
            preferences: XAppPreferences::new(app_name.to_string()),
            favorites: XAppFavoritesManager::default(),
            status_notifier: XAppStatusNotifier::default(),
            thumbnailer: XAppThumbnailerService::default(),
            media_keys: XAppMediaKeysDispatcher::default(),
        }
    }

    pub fn export_environment_variables(&self) -> Vec<(String, String)> {
        self.preferences.generate_desktop_theme_env()
    }
}

impl Default for SovereignXAppCrossDesktopSuite {
    fn default() -> Self {
        Self::new("sigma-xapp-core")
    }
}

/// Target selection requirements for Cinnamon Spices Context Menu Actions (.nemo_action)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinnamonActionTarget {
    /// Action applies to exactly one file
    SingleFile,
    /// Action applies to one or more files
    MultipleFiles,
    /// Action applies to directory target
    Directory,
    /// Action applies anywhere in file manager background or selection
    Any,
}

/// Dynamic condition required for a Cinnamon Spices Action to be active
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CinnamonActionCondition {
    /// MIME type match list (e.g., ["image/png", "text/*"])
    MimeTypesMatch(Vec<String>),
    /// File extension match list (e.g., ["jpg", "pdf", "zip"])
    FileExtensionsMatch(Vec<String>),
    /// Target must be a directory
    IsDirectory,
    /// Executable binary must exist in PATH
    ExecExists(String),
    /// Target file/folder path must exist
    PathExists(String),
}

/// Context menu action inspired by Linux Mint `cinnamon-spices-actions` (.nemo_action)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CinnamonAction {
    /// Action identifier / filename
    pub id: String,
    /// Human readable name displayed in context menu
    pub name: String,
    /// Tooltip description
    pub comment: String,
    /// Associated icon name
    pub icon_name: String,
    /// Execution command pattern (supports %F, %f, %U, %d)
    pub exec_pattern: String,
    /// Target selection type
    pub target: CinnamonActionTarget,
    /// Conditions required for action activation
    pub conditions: Vec<CinnamonActionCondition>,
    /// Whether action is currently enabled
    pub enabled: bool,
    /// Stock location or user installed spice priority
    pub is_user_spice: bool,
}

impl CinnamonAction {
    /// Create a new Cinnamon action
    pub fn new(id: &str, name: &str, exec_pattern: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            comment: String::new(),
            icon_name: "system-run".to_string(),
            exec_pattern: exec_pattern.to_string(),
            target: CinnamonActionTarget::Any,
            conditions: Vec::new(),
            enabled: true,
            is_user_spice: false,
        }
    }

    /// Expand execution command pattern given selected file targets and directory
    pub fn expand_command(&self, selected_paths: &[&str], current_dir: &str) -> String {
        let first = selected_paths.first().copied().unwrap_or("");
        let all_files = selected_paths.join(" ");

        let mut cmd = self.exec_pattern.clone();
        cmd = cmd.replace("%f", first);
        cmd = cmd.replace("%F", &all_files);
        cmd = cmd.replace("%d", current_dir);
        cmd = cmd.replace("%U", &format!("file://{}", first));
        cmd
    }

    /// Evaluate whether this action is applicable to a given selection and MIME list
    pub fn is_applicable(&self, selected_paths: &[&str], mime_types: &[&str]) -> bool {
        if !self.enabled {
            return false;
        }

        // Check target count match
        match self.target {
            CinnamonActionTarget::SingleFile => {
                if selected_paths.len() != 1 {
                    return false;
                }
            }
            CinnamonActionTarget::MultipleFiles => {
                if selected_paths.is_empty() {
                    return false;
                }
            }
            CinnamonActionTarget::Directory => {
                if selected_paths.len() != 1 || !mime_types.iter().any(|m| *m == "inode/directory") {
                    return false;
                }
            }
            CinnamonActionTarget::Any => {}
        }

        // Check conditions
        for cond in &self.conditions {
            match cond {
                CinnamonActionCondition::MimeTypesMatch(expected_mimes) => {
                    let matches = mime_types.iter().any(|m| {
                        expected_mimes.iter().any(|expected| {
                            if expected.ends_with("/*") {
                                let prefix = &expected[..expected.len() - 2];
                                m.starts_with(prefix)
                            } else {
                                m == expected
                            }
                        })
                    });
                    if !matches {
                        return false;
                    }
                }
                CinnamonActionCondition::FileExtensionsMatch(expected_exts) => {
                    let matches = selected_paths.iter().any(|p| {
                        p.split('.').last().map(|ext| {
                            expected_exts.iter().any(|e| e.eq_ignore_ascii_case(ext))
                        }).unwrap_or(false)
                    });
                    if !matches {
                        return false;
                    }
                }
                CinnamonActionCondition::IsDirectory => {
                    if !mime_types.iter().any(|m| *m == "inode/directory") {
                        return false;
                    }
                }
                CinnamonActionCondition::ExecExists(_binary) => {
                    // In simulation / no_std environment, treat as matched
                }
                CinnamonActionCondition::PathExists(_path) => {
                    // In simulation / no_std environment, treat as matched
                }
            }
        }

        true
    }
}

/// Manager for Cinnamon Spices Actions (`cinnamon-spices-actions` / Nemo actions)
#[derive(Debug, Clone)]
pub struct CinnamonSpicesActionManager {
    /// Collection of registered actions
    pub actions: Vec<CinnamonAction>,
    /// System actions path (~/.local/share/nemo/actions or /usr/share/nemo/actions)
    pub actions_directory: String,
}

impl CinnamonSpicesActionManager {
    /// Create a new Cinnamon Spices Action Manager
    pub fn new(actions_directory: &str) -> Self {
        Self {
            actions: Vec::new(),
            actions_directory: actions_directory.to_string(),
        }
    }

    /// Register a new action
    pub fn register_action(&mut self, action: CinnamonAction) {
        self.actions.retain(|a| a.id != action.id);
        self.actions.push(action);
    }

    /// Parse a simulated `.nemo_action` configuration block into a `CinnamonAction`
    pub fn parse_nemo_action_spec(&mut self, spec_lines: &[&str]) -> Result<&CinnamonAction, &'static str> {
        let mut id = String::new();
        let mut name = String::new();
        let mut comment = String::new();
        let mut icon = "system-run".to_string();
        let mut exec = String::new();
        let mut conditions = Vec::new();
        let mut target = CinnamonActionTarget::Any;

        for line in spec_lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((key, val)) = line.split_once('=') {
                let k = key.trim();
                let v = val.trim();
                match k {
                    "Id" | "Name" => {
                        if k == "Id" {
                            id = v.to_string();
                        } else {
                            name = v.to_string();
                        }
                    }
                    "Comment" => comment = v.to_string(),
                    "Icon-Name" => icon = v.to_string(),
                    "Exec" => exec = v.to_string(),
                    "Selection" => match v {
                        "s" | "1" => target = CinnamonActionTarget::SingleFile,
                        "m" | "multiple" => target = CinnamonActionTarget::MultipleFiles,
                        "dir" | "directory" => target = CinnamonActionTarget::Directory,
                        _ => target = CinnamonActionTarget::Any,
                    },
                    "Mimetypes" => {
                        let mimes: Vec<String> = v
                            .split(';')
                            .map(|m| m.trim().to_string())
                            .filter(|m| !m.is_empty() && m != "all")
                            .collect();
                        if !mimes.is_empty() {
                            conditions.push(CinnamonActionCondition::MimeTypesMatch(mimes));
                        }
                    }
                    "Extensions" => {
                        let exts: Vec<String> = v
                            .split(';')
                            .map(|e| e.trim().to_string())
                            .filter(|e| !e.is_empty())
                            .collect();
                        if !exts.is_empty() {
                            conditions.push(CinnamonActionCondition::FileExtensionsMatch(exts));
                        }
                    }
                    _ => {}
                }
            }
        }

        if id.is_empty() || exec.is_empty() {
            return Err("Missing required Id or Exec key in .nemo_action spec");
        }

        let mut action = CinnamonAction::new(&id, if name.is_empty() { &id } else { &name }, &exec);
        action.comment = comment;
        action.icon_name = icon;
        action.target = target;
        action.conditions = conditions;

        self.register_action(action);
        Ok(self.actions.last().unwrap())
    }

    /// Query actions applicable for selected file paths and their MIME types
    pub fn get_applicable_actions(
        &self,
        selected_paths: &[&str],
        mime_types: &[&str],
    ) -> Vec<&CinnamonAction> {
        self.actions
            .iter()
            .filter(|a| a.is_applicable(selected_paths, mime_types))
            .collect()
    }

    /// Execute action and return expanded command string
    pub fn execute_action(
        &self,
        action_id: &str,
        selected_paths: &[&str],
        current_dir: &str,
    ) -> Result<String, &'static str> {
        if let Some(action) = self.actions.iter().find(|a| a.id == action_id) {
            Ok(action.expand_command(selected_paths, current_dir))
        } else {
            Err("Action ID not found")
        }
    }
}

impl Default for CinnamonSpicesActionManager {
    fn default() -> Self {
        Self::new("~/.local/share/nemo/actions")
    }
}

/// Cinnamon Desktop Manager - manages Cinnamon desktop environment
#[derive(Debug)]
pub struct CinnamonDesktopManager {
    /// Configured panels
    pub panels: Vec<CinnamonPanel>,
    /// Configured desklets
    pub desklets: Vec<CinnamonDesklet>,
    /// Active theme
    pub theme: CinnamonTheme,
    /// Installed extensions
    pub extensions: Vec<CinnamonExtension>,
    /// XApp preferences
    pub xapp_prefs: XAppPreferences,
}

impl CinnamonDesktopManager {
    /// Create a new Cinnamon Desktop Manager
    pub fn new() -> Self {
        Self {
            panels: Vec::new(),
            desklets: Vec::new(),
            theme: CinnamonTheme::new("Mint-X".to_string(), "mint-x".to_string()),
            extensions: Vec::new(),
            xapp_prefs: XAppPreferences::new("cinnamon".to_string()),
        }
    }

    /// Add a panel
    pub fn add_panel(&mut self, panel: CinnamonPanel) {
        self.panels.push(panel);
    }

    /// Remove a panel by ID
    pub fn remove_panel(&mut self, panel_id: &str) {
        self.panels.retain(|p| p.id != panel_id);
    }

    /// Get panel by ID
    pub fn get_panel(&self, panel_id: &str) -> Option<&CinnamonPanel> {
        self.panels.iter().find(|p| p.id == panel_id)
    }

    /// Add a desklet
    pub fn add_desklet(&mut self, desklet: CinnamonDesklet) {
        self.desklets.push(desklet);
    }

    /// Remove a desklet by ID
    pub fn remove_desklet(&mut self, desklet_id: &str) {
        self.desklets.retain(|d| d.id != desklet_id);
    }

    /// Set the active theme
    pub fn set_theme(&mut self, theme: CinnamonTheme) {
        self.theme = theme;
    }

    /// Add an extension
    pub fn add_extension(&mut self, extension: CinnamonExtension) {
        self.extensions.push(extension);
    }

    /// Enable/disable an extension
    pub fn set_extension_enabled(&mut self, extension_id: &str, enabled: bool) {
        if let Some(ext) = self.extensions.iter_mut().find(|e| e.id == extension_id) {
            ext.enabled = enabled;
        }
    }

    /// Get enabled extensions
    pub fn get_enabled_extensions(&self) -> Vec<&CinnamonExtension> {
        self.extensions.iter().filter(|e| e.enabled).collect()
    }

    /// Apply XApp preferences
    pub fn apply_xapp_preferences(&mut self, prefs: XAppPreferences) {
        self.xapp_prefs = prefs;
        // Sync dark mode with theme
        self.theme.set_dark_mode(self.xapp_prefs.dark_mode);
    }

    /// Initialize default desktop layout
    pub fn initialize_default_layout(&mut self) {
        // Create bottom panel with default applets
        let mut bottom_panel = CinnamonPanel::new("panel-1".to_string(), CinnamonPanelPosition::Bottom);
        
        bottom_panel.add_applet(PanelApplet {
            applet_type: PanelAppletType::Menu,
            id: "menu-applet".to_string(),
            enabled: true,
            position: 0,
            config: BTreeMap::new(),
        });

        bottom_panel.add_applet(PanelApplet {
            applet_type: PanelAppletType::TaskSwitcher,
            id: "task-applet".to_string(),
            enabled: true,
            position: 1,
            config: BTreeMap::new(),
        });

        bottom_panel.add_applet(PanelApplet {
            applet_type: PanelAppletType::SystemTray,
            id: "tray-applet".to_string(),
            enabled: true,
            position: 2,
            config: BTreeMap::new(),
        });

        bottom_panel.add_applet(PanelApplet {
            applet_type: PanelAppletType::Clock,
            id: "clock-applet".to_string(),
            enabled: true,
            position: 3,
            config: BTreeMap::new(),
        });

        self.add_panel(bottom_panel);
    }
}

impl Default for CinnamonDesktopManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let panel = CinnamonPanel::new("test-panel".to_string(), CinnamonPanelPosition::Bottom);
        assert_eq!(panel.id, "test-panel");
        assert_eq!(panel.height, 40);
        assert!(!panel.auto_hide);
    }

    #[test]
    fn test_panel_applet_management() {
        let mut panel = CinnamonPanel::new("test-panel".to_string(), CinnamonPanelPosition::Bottom);
        
        let applet = PanelApplet {
            applet_type: PanelAppletType::Menu,
            id: "menu-applet".to_string(),
            enabled: true,
            position: 0,
            config: BTreeMap::new(),
        };
        
        panel.add_applet(applet);
        assert_eq!(panel.applets.len(), 1);
        
        panel.remove_applet("menu-applet");
        assert_eq!(panel.applets.len(), 0);
    }

    #[test]
    fn test_desktop_manager_default_layout() {
        let mut manager = CinnamonDesktopManager::new();
        manager.initialize_default_layout();
        
        assert_eq!(manager.panels.len(), 1);
        assert_eq!(manager.panels[0].applets.len(), 4);
    }

    #[test]
    fn test_theme_dark_mode() {
        let mut theme = CinnamonTheme::new("test".to_string(), "test".to_string());
        assert!(!theme.dark_mode);
        
        theme.set_dark_mode(true);
        assert!(theme.dark_mode);
        assert_eq!(theme.gtk_theme, "Adwaita-dark");
    }

    #[test]
    fn test_xapp_preferences() {
        let mut prefs = XAppPreferences::new("test-app".to_string());
        assert!(!prefs.dark_mode);
        
        prefs.set_dark_mode(true);
        assert!(prefs.dark_mode);
        
        prefs.set_accent_color("#ff0000".to_string());
        assert_eq!(prefs.accent_color, "#ff0000");
    }

    #[test]
    fn test_xapp_preferences_enhanced() {
        let mut prefs = XAppPreferences::new("xed".to_string());
        prefs.set_dark_mode(true);
        prefs.set_window_button_layout("close,minimize,maximize".to_string());
        prefs.set_titlebar_style(XAppTitlebarStyle::HeaderBar);

        let envs = prefs.generate_desktop_theme_env();
        assert!(envs.iter().any(|(k, v)| k == "GTK_THEME" && v == "Adwaita-dark"));
        assert!(envs.iter().any(|(k, v)| k == "QT_STYLE_OVERRIDE" && v == "kvantum"));
        assert!(envs.iter().any(|(k, v)| k == "XAPP_BUTTON_LAYOUT" && v == "close,minimize,maximize"));
    }

    #[test]
    fn test_xapp_favorites_manager() {
        let mut favs = XAppFavoritesManager::new(10);
        let pinned = favs.pin_item(
            "file:///home/user/document.pdf",
            "Annual Report",
            XAppFavoriteKind::Document,
            "application-pdf",
            "application/pdf",
            1700000000,
        );
        assert!(pinned.unwrap());
        assert!(favs.is_pinned("file:///home/user/document.pdf"));

        let docs = favs.get_by_kind(XAppFavoriteKind::Document);
        assert_eq!(docs.len(), 1);

        let results = favs.search("Annual");
        assert_eq!(results.len(), 1);

        assert!(favs.unpin_item("file:///home/user/document.pdf"));
        assert!(!favs.is_pinned("file:///home/user/document.pdf"));
    }

    #[test]
    fn test_xapp_status_notifier() {
        let mut notifier = XAppStatusNotifier::new();
        notifier.register_icon("xreader-app", "XReader PDF", "xreader");
        assert!(notifier.update_badge("xreader-app", 2, "2 unread documents"));
        assert!(notifier.add_context_action("xreader-app", "open", "Open File"));

        let icon = notifier.get_icon("xreader-app").unwrap();
        assert_eq!(icon.badge_count, 2);
        assert_eq!(icon.actions.len(), 1);
        assert!(notifier.remove_icon("xreader-app"));
    }

    #[test]
    fn test_xapp_thumbnailer_service() {
        let thumb_service = XAppThumbnailerService::new("/tmp/test-thumb-cache");
        assert!(thumb_service.can_thumbnail("image/png"));
        assert!(thumb_service.can_thumbnail("video/mp4"));

        let res = thumb_service.generate_thumbnail(
            "file:///home/user/photo.jpg",
            "image/jpeg",
            XAppThumbnailSize::Large256,
        );
        assert!(res.success);
        assert!(res.thumbnail_path.contains("large"));
    }

    #[test]
    fn test_xapp_media_keys_dispatcher() {
        let mut media = XAppMediaKeysDispatcher::new();
        media.register_subscriber("celluloid", 100);
        media.register_subscriber("hypnotix", 50);

        let msg = media.dispatch_action(XAppMediaKeyAction::PlayPause).unwrap();
        assert!(msg.contains("celluloid"));

        media.set_active_app("hypnotix");
        let active_msg = media.dispatch_action(XAppMediaKeyAction::VolumeUp).unwrap();
        assert!(active_msg.contains("hypnotix"));
    }

    #[test]
    fn test_sovereign_xapp_cross_desktop_suite() {
        let suite = SovereignXAppCrossDesktopSuite::new("xplayer");
        let envs = suite.export_environment_variables();
        assert!(!envs.is_empty());
    }

    #[test]
    fn test_cinnamon_spices_action_registration_and_evaluation() {
        let mut mgr = CinnamonSpicesActionManager::default();

        let spec = &[
            "[Nemo Action]",
            "Id=set-as-wallpaper",
            "Name=Set as Wallpaper",
            "Comment=Set image as desktop background",
            "Exec=cinnamon-wallpaper-set %f",
            "Selection=s",
            "Mimetypes=image/*;",
        ];

        let action = mgr.parse_nemo_action_spec(spec).unwrap();
        assert_eq!(action.id, "set-as-wallpaper");
        assert_eq!(action.target, CinnamonActionTarget::SingleFile);

        let active = mgr.get_applicable_actions(&["/home/user/photo.png"], &["image/png"]);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id, "set-as-wallpaper");

        let inactive = mgr.get_applicable_actions(&["/home/user/doc.pdf"], &["application/pdf"]);
        assert_eq!(inactive.len(), 0);
    }

    #[test]
    fn test_cinnamon_action_command_expansion() {
        let action = CinnamonAction::new(
            "open-terminal-here",
            "Open Terminal Here",
            "gnome-terminal --working-directory=%d -e %F",
        );

        let expanded = action.expand_command(
            &["/tmp/file1.txt", "/tmp/file2.txt"],
            "/tmp",
        );
        assert_eq!(expanded, "gnome-terminal --working-directory=/tmp -e /tmp/file1.txt /tmp/file2.txt");
    }

    #[test]
    fn test_cinnamon_action_conditions() {
        let mut mgr = CinnamonSpicesActionManager::default();

        let spec = &[
            "Id=extract-archive",
            "Name=Extract Archive Here",
            "Exec=file-roller --extract-here %F",
            "Selection=m",
            "Extensions=zip;tar.gz;7z;",
        ];

        let _action = mgr.parse_nemo_action_spec(spec).unwrap();
        let applicable = mgr.get_applicable_actions(
            &["/tmp/archive.zip", "/tmp/data.tar.gz"],
            &["application/zip", "application/gzip"],
        );
        assert_eq!(applicable.len(), 1);
        assert_eq!(applicable[0].id, "extract-archive");
    }
}
