// SPDX-License-Identifier: MIT
// ItsFOSS Inspired Open-Source Desktop & System Innovations for SigmaOS
// Zero-dependency, safe Rust, #![no_std] compliant architecture

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// =========================================================================
// 1. FLATPAK / SNAP SANDBOX RUNTIME LAYER (ItsFossFlatpakSnapLayer)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxAppFormat {
    Flatpak,
    Snap,
    AppImage,
}

#[derive(Debug, Clone)]
pub struct AppPortalPermission {
    pub allow_network: bool,
    pub allow_home_dir: bool,
    pub allow_x11_wayland: bool,
    pub allow_pulseaudio_pipewire: bool,
    pub custom_overrides: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SandboxContainerApp {
    pub app_id: String,
    pub format: SandboxAppFormat,
    pub runtime_branch: String,
    pub permissions: AppPortalPermission,
    pub is_running: bool,
}

pub struct ItsFossFlatpakSnapLayer {
    pub installed_apps: Vec<SandboxContainerApp>,
}

impl ItsFossFlatpakSnapLayer {
    pub fn new() -> Self {
        Self {
            installed_apps: Vec::new(),
        }
    }

    pub fn install_container_app(
        &mut self,
        app_id: &str,
        format: SandboxAppFormat,
        runtime_branch: &str,
    ) {
        let app = SandboxContainerApp {
            app_id: app_id.to_string(),
            format,
            runtime_branch: runtime_branch.to_string(),
            permissions: AppPortalPermission {
                allow_network: true,
                allow_home_dir: false,
                allow_x11_wayland: true,
                allow_pulseaudio_pipewire: true,
                custom_overrides: Vec::new(),
            },
            is_running: false,
        };
        self.installed_apps.push(app);
    }

    pub fn set_override_permission(&mut self, app_id: &str, allow_home: bool, extra: &str) -> bool {
        if let Some(app) = self.installed_apps.iter_mut().find(|a| a.app_id == app_id) {
            app.permissions.allow_home_dir = allow_home;
            if !extra.is_empty() {
                app.permissions.custom_overrides.push(extra.to_string());
            }
            true
        } else {
            false
        }
    }

    pub fn launch_app_sandbox(&mut self, app_id: &str) -> Result<String, &'static str> {
        let app = self
            .installed_apps
            .iter_mut()
            .find(|a| a.app_id == app_id)
            .ok_or("Application not found in sandbox layer")?;

        app.is_running = true;
        Ok(format!(
            "Launched {:?} app '{}' under runtime branch '{}' (HomeDir: {}, Net: {})",
            app.format, app.app_id, app.runtime_branch, app.permissions.allow_home_dir, app.permissions.allow_network
        ))
    }
}

impl Default for ItsFossFlatpakSnapLayer {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. BLEACHBIT / STACER SYSTEM CLEANER ENGINE (ItsFossSystemCleanerEngine)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanTargetCategory {
    PackageCache,
    ThumbnailCache,
    JournaldLogs,
    TempFiles,
    OldKernels,
}

#[derive(Debug, Clone)]
pub struct CleanableCacheItem {
    pub category: CleanTargetCategory,
    pub path: String,
    pub size_bytes: u64,
}

pub struct ItsFossSystemCleanerEngine {
    pub cache_items: Vec<CleanableCacheItem>,
    pub total_space_freed_bytes: u64,
}

impl ItsFossSystemCleanerEngine {
    pub fn new() -> Self {
        let items = vec![
            CleanableCacheItem {
                category: CleanTargetCategory::PackageCache,
                path: "/var/cache/sigma-pkg/archives".to_string(),
                size_bytes: 450 * 1024 * 1024,
            },
            CleanableCacheItem {
                category: CleanTargetCategory::ThumbnailCache,
                path: "/home/user/.cache/thumbnails".to_string(),
                size_bytes: 120 * 1024 * 1024,
            },
            CleanableCacheItem {
                category: CleanTargetCategory::JournaldLogs,
                path: "/var/log/journal".to_string(),
                size_bytes: 300 * 1024 * 1024,
            },
            CleanableCacheItem {
                category: CleanTargetCategory::TempFiles,
                path: "/tmp/sigma-temp".to_string(),
                size_bytes: 80 * 1024 * 1024,
            },
        ];

        Self {
            cache_items: items,
            total_space_freed_bytes: 0,
        }
    }

    pub fn scan_total_cleanable_bytes(&self) -> u64 {
        self.cache_items.iter().map(|item| item.size_bytes).sum()
    }

    pub fn clean_category(&mut self, category: CleanTargetCategory) -> u64 {
        let mut freed = 0u64;
        let mut retain_items = Vec::new();

        for item in self.cache_items.drain(..) {
            if item.category == category {
                freed += item.size_bytes;
            } else {
                retain_items.push(item);
            }
        }

        self.cache_items = retain_items;
        self.total_space_freed_bytes += freed;
        freed
    }
}

impl Default for ItsFossSystemCleanerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. GNOME SOFTWARE / DISCOVER GUI APP CENTER ENGINE (ItsFossGuiSoftwareCenterEngine)
// =========================================================================

#[derive(Debug, Clone)]
pub struct SoftwareCatalogEntry {
    pub name: String,
    pub app_id: String,
    pub description: String,
    pub rating_stars: f32,
    pub download_size_mb: u32,
    pub backend_format: SandboxAppFormat,
    pub is_installed: bool,
}

pub struct ItsFossGuiSoftwareCenterEngine {
    pub catalog: Vec<SoftwareCatalogEntry>,
}

impl ItsFossGuiSoftwareCenterEngine {
    pub fn new() -> Self {
        let catalog = vec![
            SoftwareCatalogEntry {
                name: "GIMP Image Editor".to_string(),
                app_id: "org.gimp.GIMP".to_string(),
                description: "GNU Image Manipulation Program".to_string(),
                rating_stars: 4.8,
                download_size_mb: 250,
                backend_format: SandboxAppFormat::Flatpak,
                is_installed: false,
            },
            SoftwareCatalogEntry {
                name: "VS Code".to_string(),
                app_id: "com.visualstudio.code".to_string(),
                description: "Code editing redefined".to_string(),
                rating_stars: 4.9,
                download_size_mb: 120,
                backend_format: SandboxAppFormat::Flatpak,
                is_installed: true,
            },
            SoftwareCatalogEntry {
                name: "VLC Media Player".to_string(),
                app_id: "org.videolan.VLC".to_string(),
                description: "Universal media player".to_string(),
                rating_stars: 4.7,
                download_size_mb: 85,
                backend_format: SandboxAppFormat::Snap,
                is_installed: false,
            },
        ];

        Self { catalog }
    }

    pub fn search_catalog(&self, query: &str) -> Vec<SoftwareCatalogEntry> {
        let q = query.to_lowercase();
        self.catalog
            .iter()
            .filter(|e| e.name.to_lowercase().contains(&q) || e.description.to_lowercase().contains(&q))
            .cloned()
            .collect()
    }

    pub fn one_click_install(&mut self, app_id: &str) -> Result<String, &'static str> {
        let entry = self
            .catalog
            .iter_mut()
            .find(|e| e.app_id == app_id)
            .ok_or("Application not found in software center catalog")?;

        entry.is_installed = true;
        Ok(format!("Successfully installed '{}' ({}) via {:?}", entry.name, entry.app_id, entry.backend_format))
    }
}

impl Default for ItsFossGuiSoftwareCenterEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. GAMEMODE & MANGOHUD GAMING BOOSTER ENGINE (ItsFossGamingBoosterEngine)
// =========================================================================

#[derive(Debug, Clone)]
pub struct MangoHudMetrics {
    pub fps: u32,
    pub frametime_ms: f32,
    pub cpu_temp_celsius: u32,
    pub gpu_temp_celsius: u32,
    pub vram_used_mb: u32,
}

pub struct ItsFossGamingBoosterEngine {
    pub game_mode_active: bool,
    pub active_game_title: Option<String>,
    pub hud_metrics: MangoHudMetrics,
}

impl ItsFossGamingBoosterEngine {
    pub fn new() -> Self {
        Self {
            game_mode_active: false,
            active_game_title: None,
            hud_metrics: MangoHudMetrics {
                fps: 144,
                frametime_ms: 6.94,
                cpu_temp_celsius: 58,
                gpu_temp_celsius: 62,
                vram_used_mb: 4096,
            },
        }
    }

    pub fn enable_game_mode(&mut self, game_title: &str) {
        self.game_mode_active = true;
        self.active_game_title = Some(game_title.to_string());
    }

    pub fn disable_game_mode(&mut self) {
        self.game_mode_active = false;
        self.active_game_title = None;
    }

    pub fn render_mangohud_overlay(&self) -> String {
        if !self.game_mode_active {
            return String::from("[MangoHud: Inactive]");
        }
        format!(
            "FPS: {} | Frametime: {:.2}ms | CPU: {}°C | GPU: {}°C | VRAM: {}MB [{}]",
            self.hud_metrics.fps,
            self.hud_metrics.frametime_ms,
            self.hud_metrics.cpu_temp_celsius,
            self.hud_metrics.gpu_temp_celsius,
            self.hud_metrics.vram_used_mb,
            self.active_game_title.as_deref().unwrap_or("Game")
        )
    }
}

impl Default for ItsFossGamingBoosterEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. TERMINAL PRODUCTIVITY SUITE (ItsFossTerminalProductivitySuite)
// =========================================================================

/// Starship prompt parity
pub struct StarshipPromptThemeEngine {
    pub username: String,
    pub hostname: String,
    pub current_dir: String,
    pub git_branch: Option<String>,
}

impl StarshipPromptThemeEngine {
    pub fn new(username: &str, hostname: &str, current_dir: &str) -> Self {
        Self {
            username: username.to_string(),
            hostname: hostname.to_string(),
            current_dir: current_dir.to_string(),
            git_branch: Some("main".to_string()),
        }
    }

    pub fn render_prompt(&self) -> String {
        let branch_str = self
            .git_branch
            .as_ref()
            .map(|b| format!(" on  {}", b))
            .unwrap_or_default();
        format!(
            "┌─[{}@{}] - [{}]\n└─{} ❯ ",
            self.username, self.hostname, self.current_dir, branch_str
        )
    }
}

/// Micro text editor parity
pub struct MicroTextEditorEngine {
    pub filename: String,
    pub lines: Vec<String>,
    pub cursor_line: usize,
    pub cursor_col: usize,
}

impl MicroTextEditorEngine {
    pub fn new(filename: &str, content: &str) -> Self {
        let lines = content.lines().map(|s| s.to_string()).collect();
        Self {
            filename: filename.to_string(),
            lines,
            cursor_line: 0,
            cursor_col: 0,
        }
    }

    pub fn insert_text(&mut self, text: &str) {
        if self.lines.is_empty() {
            self.lines.push(text.to_string());
        } else if self.cursor_line < self.lines.len() {
            self.lines[self.cursor_line].push_str(text);
        }
    }

    pub fn save_buffer(&self) -> String {
        self.lines.join("\n")
    }
}

/// Ncdu / Dust interactive disk usage TUI analyzer
#[derive(Debug, Clone)]
pub struct DiskNodeUsage {
    pub path: String,
    pub size_mb: u64,
}

pub struct BduDiskUsageAnalyzer {
    pub nodes: Vec<DiskNodeUsage>,
}

impl BduDiskUsageAnalyzer {
    pub fn new() -> Self {
        let nodes = vec![
            DiskNodeUsage {
                path: "/var/log".to_string(),
                size_mb: 1200,
            },
            DiskNodeUsage {
                path: "/usr/lib".to_string(),
                size_mb: 8500,
            },
            DiskNodeUsage {
                path: "/home/user/Downloads".to_string(),
                size_mb: 4200,
            },
        ];
        Self { nodes }
    }

    pub fn sort_largest(&mut self) {
        self.nodes.sort_by(|a, b| b.size_mb.cmp(&a.size_mb));
    }
}

impl Default for BduDiskUsageAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Neowritable / Obsidian / zk terminal note-taking engine
#[derive(Debug, Clone)]
pub struct TerminalNote {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
}

pub struct NeowritableNotetakerEngine {
    pub notes: Vec<TerminalNote>,
}

impl NeowritableNotetakerEngine {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    pub fn create_note(&mut self, title: &str, tags: &[&str], content: &str) -> String {
        let id = format!("note_{}", self.notes.len() + 1);
        let note = TerminalNote {
            id: id.clone(),
            title: title.to_string(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            content: content.to_string(),
        };
        self.notes.push(note);
        id
    }

    pub fn find_notes_by_tag(&self, tag: &str) -> Vec<&TerminalNote> {
        self.notes
            .iter()
            .filter(|n| n.tags.iter().any(|t| t == tag))
            .collect()
    }
}

impl Default for NeowritableNotetakerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatpak_snap_sandbox_layer() {
        let mut layer = ItsFossFlatpakSnapLayer::new();
        layer.install_container_app("org.mozilla.firefox", SandboxAppFormat::Flatpak, "stable");
        assert_eq!(layer.installed_apps.len(), 1);

        assert!(layer.set_override_permission("org.mozilla.firefox", true, "--filesystem=/home"));
        let launch_res = layer.launch_app_sandbox("org.mozilla.firefox");
        assert!(launch_res.is_ok());
        assert!(launch_res.unwrap().contains("Launched Flatpak app 'org.mozilla.firefox'"));
    }

    #[test]
    fn test_system_cleaner_engine() {
        let mut cleaner = ItsFossSystemCleanerEngine::new();
        let total_bytes = cleaner.scan_total_cleanable_bytes();
        assert!(total_bytes > 0);

        let freed = cleaner.clean_category(CleanTargetCategory::PackageCache);
        assert_eq!(freed, 450 * 1024 * 1024);
        assert_eq!(cleaner.total_space_freed_bytes, freed);
    }

    #[test]
    fn test_gui_software_center_engine() {
        let mut center = ItsFossGuiSoftwareCenterEngine::new();
        let results = center.search_catalog("GIMP");
        assert_eq!(results.len(), 1);

        let install_res = center.one_click_install("org.gimp.GIMP");
        assert!(install_res.is_ok());
        assert!(center.catalog[0].is_installed);
    }

    #[test]
    fn test_gaming_booster_engine() {
        let mut booster = ItsFossGamingBoosterEngine::new();
        assert_eq!(booster.render_mangohud_overlay(), "[MangoHud: Inactive]");

        booster.enable_game_mode("Cyberpunk 2077");
        let overlay = booster.render_mangohud_overlay();
        assert!(overlay.contains("FPS: 144"));
        assert!(overlay.contains("Cyberpunk 2077"));

        booster.disable_game_mode();
        assert!(!booster.game_mode_active);
    }

    #[test]
    fn test_terminal_productivity_suite() {
        let starship = StarshipPromptThemeEngine::new("user", "sigma-pc", "~/projects");
        let prompt = starship.render_prompt();
        assert!(prompt.contains("user@sigma-pc"));
        assert!(prompt.contains("~/projects"));

        let mut micro = MicroTextEditorEngine::new("test.txt", "line1");
        micro.insert_text("\nline2");
        assert!(micro.save_buffer().contains("line2"));

        let mut bdu = BduDiskUsageAnalyzer::new();
        bdu.sort_largest();
        assert_eq!(bdu.nodes[0].path, "/usr/lib");

        let mut notes = NeowritableNotetakerEngine::new();
        notes.create_note("Kernel Notes", &["kernel", "rust"], "no_std architecture details");
        let found = notes.find_notes_by_tag("kernel");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].title, "Kernel Notes");
    }
}
