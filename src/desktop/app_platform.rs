//! Desktop app platform: the unified integration layer for SigmaOS.
//!
//! This is the "missing product layer" that turns individual component repos
//! into a coherent, trustworthy desktop ecosystem. It defines:
//!
//! - How apps declare their capabilities (.desktop entry contract)
//! - How MIME types are resolved to default apps (FileAssociationManager)
//! - How apps are granted permissions (AppPermissions)
//! - How the desktop registry discovers and launches apps (AppRegistry)
//! - How apps integrate with tray, notifications, clipboard, and file manager
//!
//! Inspired by: XDG Desktop Spec, Flatpak runtime model, D-Bus activation,
//! macOS CFBundleIdentifier, Android PackageManager.

#![allow(dead_code, clippy::new_without_default)]

use std::collections::HashMap;
use std::path::PathBuf;

// ─── App Category ────────────────────────────────────────────────────────────

/// XDG menu category for an application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppCategory {
    WebBrowser, FileManager, TerminalEmulator, TextEditor, Office, Spreadsheet,
    WordProcessor, Presentation, PdfViewer, ImageViewer, AudioVideo, Audio,
    Video, Game, Development, System, Settings, Utility, Accessibility,
    Network, Security, Science, Graphics,
}

impl AppCategory {
    pub fn xdg_name(self) -> &'static str {
        match self {
            Self::WebBrowser => "Network;WebBrowser",
            Self::FileManager => "System;FileManager",
            Self::TerminalEmulator => "System;TerminalEmulator",
            Self::TextEditor => "Utility;TextEditor",
            Self::Office => "Office",
            Self::Spreadsheet => "Office;Spreadsheet",
            Self::WordProcessor => "Office;WordProcessor",
            Self::Presentation => "Office;Presentation",
            Self::PdfViewer => "Office;Viewer",
            Self::ImageViewer => "Graphics;Viewer",
            Self::AudioVideo => "AudioVideo",
            Self::Audio => "AudioVideo;Audio",
            Self::Video => "AudioVideo;Video",
            Self::Game => "Game",
            Self::Development => "Development",
            Self::System => "System",
            Self::Settings => "Settings",
            Self::Utility => "Utility",
            Self::Accessibility => "Accessibility",
            Self::Network => "Network",
            Self::Security => "System;Security",
            Self::Science => "Science",
            Self::Graphics => "Graphics",
        }
    }
}

// ─── Desktop Entry ───────────────────────────────────────────────────────────

/// XDG `.desktop` file specification for a SigmaOS application.
#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub app_id: String,
    pub name: String,
    pub generic_name: String,
    pub comment: String,
    pub exec: String,
    pub icon: String,
    pub desktop_file_name: String,
    pub categories: Vec<AppCategory>,
    pub mime_types: Vec<String>,
    pub terminal: bool,
    pub startup_notify: bool,
    pub keywords: Vec<String>,
}

impl DesktopEntry {
    /// Render as an XDG `.desktop` file string.
    pub fn render(&self) -> String {
        let mut s = String::from("[Desktop Entry]\nType=Application\n");
        s += &format!("Name={}\n", self.name);
        if !self.generic_name.is_empty() { s += &format!("GenericName={}\n", self.generic_name); }
        if !self.comment.is_empty() { s += &format!("Comment={}\n", self.comment); }
        s += &format!("Exec={}\n", self.exec);
        s += &format!("Icon={}\n", self.icon);
        s += &format!("Terminal={}\n", self.terminal);
        if self.startup_notify { s += "StartupNotify=true\n"; }
        if !self.categories.is_empty() {
            let cats: Vec<&str> = self.categories.iter().map(|c| c.xdg_name()).collect();
            s += &format!("Categories={};\n", cats.join(";"));
        }
        if !self.mime_types.is_empty() {
            s += &format!("MimeType={};\n", self.mime_types.join(";"));
        }
        if !self.keywords.is_empty() {
            s += &format!("Keywords={};\n", self.keywords.join(";"));
        }
        s
    }

    /// Install path for this desktop entry.
    pub fn install_path(&self) -> PathBuf {
        PathBuf::from("/usr/share/applications").join(&self.desktop_file_name)
    }
}

// ─── Permissions ─────────────────────────────────────────────────────────────

/// Permissions an app has been granted by the user or system policy.
#[derive(Debug, Clone)]
pub struct AppPermissions {
    pub notifications: bool,
    pub clipboard_read: bool,
    pub clipboard_write: bool,
    pub camera: bool,
    pub microphone: bool,
    pub location: bool,
    pub bluetooth: bool,
    pub printing: bool,
    pub background: bool,
    pub autostart: bool,
    pub network: NetworkPermission,
    pub filesystem: FilesystemPermission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkPermission { Full, LocalOnly, None }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemPermission { Full, HomeOnly, DocumentsOnly, None }

impl AppPermissions {
    pub fn trusted() -> Self {
        Self {
            notifications: true, clipboard_read: true, clipboard_write: true,
            camera: false, microphone: false, location: false, bluetooth: false,
            printing: true, background: true, autostart: true,
            network: NetworkPermission::Full,
            filesystem: FilesystemPermission::HomeOnly,
        }
    }

    pub fn sandboxed() -> Self {
        Self {
            notifications: true, clipboard_read: false, clipboard_write: false,
            camera: false, microphone: false, location: false, bluetooth: false,
            printing: false, background: false, autostart: false,
            network: NetworkPermission::None,
            filesystem: FilesystemPermission::None,
        }
    }

    pub fn has_dangerous_permissions(&self) -> bool {
        self.camera || self.microphone || self.location
            || self.network == NetworkPermission::Full
            || self.filesystem == FilesystemPermission::Full
    }
}

// ─── App Runtime ─────────────────────────────────────────────────────────────

/// Resource limits applied to an app process.
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_mb: Option<u64>,
    pub max_cpu_percent: Option<u8>,
    pub max_fds: Option<u32>,
    pub network_access: NetworkPermission,
    pub fs_access: FilesystemPermission,
}

impl ResourceLimits {
    pub fn unlimited() -> Self {
        Self {
            max_memory_mb: None, max_cpu_percent: None, max_fds: None,
            network_access: NetworkPermission::Full,
            fs_access: FilesystemPermission::HomeOnly,
        }
    }

    pub fn sandboxed() -> Self {
        Self {
            max_memory_mb: Some(512), max_cpu_percent: Some(80), max_fds: Some(256),
            network_access: NetworkPermission::None,
            fs_access: FilesystemPermission::None,
        }
    }
}

/// How an application is launched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationMethod {
    DirectExec,
    DBusActivation,
    SystemdUserUnit,
}

// ─── Registered App ──────────────────────────────────────────────────────────

/// A fully-registered application in the SigmaOS desktop platform.
#[derive(Debug, Clone)]
pub struct RegisteredApp {
    pub app_id: String,
    pub entry: DesktopEntry,
    pub permissions: AppPermissions,
    pub limits: ResourceLimits,
    pub activation: ActivationMethod,
    pub installed: bool,
    pub version: String,
    pub trust_level: AppTrustLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppTrustLevel { System, Official, Community, Unknown }

// ─── File Association Manager ────────────────────────────────────────────────

/// Maps MIME types to their default handler application.
#[derive(Debug, Clone)]
pub struct FileAssociationManager {
    defaults: HashMap<String, String>,
    added: HashMap<String, Vec<String>>,
}

impl FileAssociationManager {
    pub fn new() -> Self {
        Self { defaults: HashMap::new(), added: HashMap::new() }
    }

    /// Set the default app for a MIME type.
    pub fn set_default(&mut self, mime: &str, desktop_file: &str) {
        self.defaults.insert(mime.to_string(), desktop_file.to_string());
        self.added.entry(mime.to_string()).or_default().push(desktop_file.to_string());
    }

    pub fn get_default(&self, mime: &str) -> Option<&str> {
        self.defaults.get(mime).map(|s| s.as_str())
    }

    pub fn remove(&mut self, mime: &str) { self.defaults.remove(mime); }

    pub fn len(&self) -> usize { self.defaults.len() }

    /// Generate ~/.config/mimeapps.list content.
    pub fn render_mimeapps_list(&self) -> String {
        let mut s = String::from("[Default Applications]\n");
        let mut entries: Vec<_> = self.defaults.iter().collect();
        entries.sort_by_key(|(k, _)| k.as_str());
        for (mime, file) in &entries { s += &format!("{mime}={file}\n"); }
        s += "\n[Added Associations]\n";
        for (mime, files) in &self.added {
            s += &format!("{mime}={}\n", files.join(";"));
        }
        s
    }

    /// Load the V1 default associations.
    pub fn load_v1_defaults(&mut self) {
        let defs = [
            ("text/html",                 "sigma-browser.desktop"),
            ("x-scheme-handler/http",     "sigma-browser.desktop"),
            ("x-scheme-handler/https",    "sigma-browser.desktop"),
            ("application/pdf",           "sigma-pdfview.desktop"),
            ("inode/directory",           "sigma-files.desktop"),
            ("text/plain",               "sigma-text.desktop"),
            ("image/jpeg",               "sigma-pix.desktop"),
            ("image/png",                "sigma-pix.desktop"),
            ("image/webp",               "sigma-pix.desktop"),
            ("image/gif",                "sigma-pix.desktop"),
            ("audio/mpeg",               "sigma-music.desktop"),
            ("audio/flac",               "sigma-music.desktop"),
            ("audio/ogg",                "sigma-music.desktop"),
            ("video/mp4",                "sigma-player.desktop"),
            ("video/x-matroska",         "sigma-player.desktop"),
            ("video/webm",               "sigma-player.desktop"),
            ("application/vnd.openxmlformats-officedocument.wordprocessingml.document",
             "libreoffice-writer.desktop"),
            ("application/vnd.oasis.opendocument.text", "libreoffice-writer.desktop"),
            ("application/vnd.ms-excel", "libreoffice-calc.desktop"),
            ("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
             "libreoffice-calc.desktop"),
            ("application/vnd.ms-powerpoint", "libreoffice-impress.desktop"),
            ("application/vnd.debian.binary-package", "sigma-packages.desktop"),
            ("text/x-python",            "sigma-text.desktop"),
            ("text/x-rust",              "sigma-text.desktop"),
            ("text/x-shellscript",       "sigma-text.desktop"),
        ];
        for (mime, file) in &defs { self.set_default(mime, file); }
    }
}

// ─── App Registry ────────────────────────────────────────────────────────────

/// Central registry of all desktop applications on SigmaOS.
pub struct AppRegistry {
    apps: HashMap<String, RegisteredApp>,
}

impl AppRegistry {
    pub fn new() -> Self { Self { apps: HashMap::new() } }

    pub fn register(&mut self, app: RegisteredApp) {
        self.apps.insert(app.app_id.clone(), app);
    }

    pub fn get(&self, id: &str) -> Option<&RegisteredApp> { self.apps.get(id) }
    pub fn len(&self) -> usize { self.apps.len() }
    pub fn is_empty(&self) -> bool { self.apps.is_empty() }

    pub fn installed_apps(&self) -> Vec<&RegisteredApp> {
        self.apps.values().filter(|a| a.installed).collect()
    }

    pub fn by_category(&self, cat: AppCategory) -> Vec<&RegisteredApp> {
        self.apps.values()
            .filter(|a| a.installed && a.entry.categories.contains(&cat))
            .collect()
    }

    pub fn default_for_mime<'a>(
        &'a self, mime: &str, assoc: &FileAssociationManager,
    ) -> Option<&'a RegisteredApp> {
        let desktop_file = assoc.get_default(mime)?;
        self.apps.values().find(|a| a.installed && a.entry.desktop_file_name == desktop_file)
    }

    /// Render all .desktop files as a map of path → content.
    pub fn render_desktop_files(&self) -> HashMap<PathBuf, String> {
        self.apps.values()
            .filter(|a| a.installed)
            .map(|a| (a.entry.install_path(), a.entry.render()))
            .collect()
    }
}

// ─── Desktop Integration Hub ─────────────────────────────────────────────────

/// Top-level coordinator: connects all desktop platform services.
pub struct DesktopPlatform {
    pub registry: AppRegistry,
    pub mime: FileAssociationManager,
}

impl DesktopPlatform {
    /// Create with V1 MIME defaults pre-loaded.
    pub fn new() -> Self {
        let mut mime = FileAssociationManager::new();
        mime.load_v1_defaults();
        Self { registry: AppRegistry::new(), mime }
    }

    /// Register an app and auto-wire its MIME types.
    pub fn install_app(&mut self, app: RegisteredApp) {
        for m in &app.entry.mime_types.clone() {
            self.mime.set_default(m, &app.entry.desktop_file_name.clone());
        }
        self.registry.register(app);
    }

    /// Open a file: resolve MIME → app → return the exec command.
    pub fn open_file(&self, mime: &str) -> Option<String> {
        let app = self.registry.default_for_mime(mime, &self.mime)?;
        Some(app.entry.exec.clone())
    }

    pub fn stats(&self) -> PlatformStats {
        PlatformStats {
            total_apps: self.registry.len(),
            installed_apps: self.registry.installed_apps().len(),
            mime_associations: self.mime.len(),
        }
    }
}

/// Summary statistics for the desktop platform.
#[derive(Debug, Clone)]
pub struct PlatformStats {
    pub total_apps: usize,
    pub installed_apps: usize,
    pub mime_associations: usize,
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_browser() -> RegisteredApp {
        RegisteredApp {
            app_id: "org.sigmaos.browser".into(),
            entry: DesktopEntry {
                app_id: "org.sigmaos.browser".into(),
                name: "Sigma Browser".into(),
                generic_name: "Web Browser".into(),
                comment: "Browse the web".into(),
                exec: "sigma-browser %U".into(),
                icon: "sigma-browser".into(),
                desktop_file_name: "sigma-browser.desktop".into(),
                categories: vec![AppCategory::WebBrowser, AppCategory::Network],
                mime_types: vec!["text/html".into(), "x-scheme-handler/http".into()],
                terminal: false,
                startup_notify: true,
                keywords: vec!["web".into(), "browser".into()],
            },
            permissions: AppPermissions::trusted(),
            limits: ResourceLimits::unlimited(),
            activation: ActivationMethod::DirectExec,
            installed: true,
            version: "2026.11.0".into(),
            trust_level: AppTrustLevel::Official,
        }
    }

    #[test]
    fn test_desktop_entry_renders_correctly() {
        let app = sample_browser();
        let rendered = app.entry.render();
        assert!(rendered.contains("[Desktop Entry]"));
        assert!(rendered.contains("Name=Sigma Browser"));
        assert!(rendered.contains("MimeType="));
        assert!(rendered.contains("Terminal=false"));
    }

    #[test]
    fn test_file_association_v1_defaults_loaded() {
        let mut mgr = FileAssociationManager::new();
        mgr.load_v1_defaults();
        assert!(mgr.len() >= 24);
        assert_eq!(mgr.get_default("text/html"), Some("sigma-browser.desktop"));
        assert_eq!(mgr.get_default("application/pdf"), Some("sigma-pdfview.desktop"));
        assert_eq!(mgr.get_default("inode/directory"), Some("sigma-files.desktop"));
    }

    #[test]
    fn test_mimeapps_list_render_contains_header() {
        let mut mgr = FileAssociationManager::new();
        mgr.load_v1_defaults();
        let s = mgr.render_mimeapps_list();
        assert!(s.starts_with("[Default Applications]"));
        assert!(s.contains("sigma-browser.desktop"));
    }

    #[test]
    fn test_registry_install_and_lookup() {
        let mut reg = AppRegistry::new();
        reg.register(sample_browser());
        assert_eq!(reg.len(), 1);
        assert!(reg.get("org.sigmaos.browser").is_some());
    }

    #[test]
    fn test_registry_by_category() {
        let mut reg = AppRegistry::new();
        reg.register(sample_browser());
        let browsers = reg.by_category(AppCategory::WebBrowser);
        assert_eq!(browsers.len(), 1);
    }

    #[test]
    fn test_platform_open_file_resolves_mime() {
        let mut platform = DesktopPlatform::new();
        platform.install_app(sample_browser());
        let exec = platform.open_file("text/html");
        assert!(exec.is_some());
        assert!(exec.unwrap().contains("sigma-browser"));
    }

    #[test]
    fn test_platform_stats() {
        let mut platform = DesktopPlatform::new();
        platform.install_app(sample_browser());
        let stats = platform.stats();
        assert_eq!(stats.total_apps, 1);
        assert!(stats.mime_associations >= 24);
    }

    #[test]
    fn test_sandboxed_permissions_safe() {
        let p = AppPermissions::sandboxed();
        assert!(!p.camera);
        assert!(!p.microphone);
        assert!(!p.background);
        assert_eq!(p.network, NetworkPermission::None);
        assert!(!p.has_dangerous_permissions());
    }

    #[test]
    fn test_trusted_permissions_allows_notifications() {
        let p = AppPermissions::trusted();
        assert!(p.notifications);
        assert!(p.printing);
        assert!(p.autostart);
    }

    #[test]
    fn test_render_desktop_files_map() {
        let mut platform = DesktopPlatform::new();
        platform.install_app(sample_browser());
        let files = platform.registry.render_desktop_files();
        assert_eq!(files.len(), 1);
        let path = files.keys().next().unwrap();
        assert!(path.to_string_lossy().contains("sigma-browser.desktop"));
    }
}
