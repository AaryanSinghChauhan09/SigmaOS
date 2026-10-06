// SigmaOS Omarchy Linux Gap Closure Components
// Clean-room, zero-dependency safe Rust implementations inspired by Omarchy 1.1.0:
// - Terminal Fastfetch ASCII Art Banner & Diagnostic Summary Generator
// - Interactive TUI Menu Applet for Terminal Tasks & Themes
// - Power Profile Manager for Dynamic CPU Governor Toggling
// - Dotfiles Snapshot Backup & Version Control Manager

use std::collections::HashMap;
use std::format;
use std::vec::Vec;

/// Omarchy Terminal Fastfetch ASCII Art Banner & Hardware Info Summary
pub struct OmarchyFastfetchEngine {
    pub os_name: String,
    pub kernel_version: String,
    pub active_theme: String,
    pub memory_used_mb: u32,
    pub memory_total_mb: u32,
}

impl OmarchyFastfetchEngine {
    pub fn new(os: &str, kernel: &str, theme: &str, used_mem: u32, total_mem: u32) -> Self {
        Self {
            os_name: os.to_string(),
            kernel_version: kernel.to_string(),
            active_theme: theme.to_string(),
            memory_used_mb: used_mem,
            memory_total_mb: total_mem,
        }
    }

    pub fn generate_summary_banner(&self) -> String {
        let ascii_art = r#"
   ____                         _   _
  / __ \                       | | | |
 | |  | |_ __ ___   __ _ _ __  | |_| |
 | |  | | '_ ` _ \ / _` | '__| |  _  |
 | |__| | | | | | | (_| | |    | | | |
  \____/|_| |_| |_|\__,_|_|    \_| |_/
"#;
        format!(
            "{}\nOS: {}\nKernel: {}\nTheme: {}\nMemory: {}MB / {}MB",
            ascii_art,
            self.os_name,
            self.kernel_version,
            self.active_theme,
            self.memory_used_mb,
            self.memory_total_mb
        )
    }
}

/// Omarchy Interactive TUI Menu Selection Applet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyTuiMenuItem {
    pub id: u32,
    pub title: String,
    pub command: String,
}

pub struct OmarchyTuiMenuApplet {
    pub items: Vec<OmarchyTuiMenuItem>,
    pub selected_index: usize,
}

impl OmarchyTuiMenuApplet {
    pub fn new() -> Self {
        let mut items = Vec::new();
        items.push(OmarchyTuiMenuItem {
            id: 1,
            title: "Switch Theme".to_string(),
            command: "sigomarchy theme-next".to_string(),
        });
        items.push(OmarchyTuiMenuItem {
            id: 2,
            title: "Launch Herdr Agent".to_string(),
            command: "sigomarchy herdr".to_string(),
        });
        items.push(OmarchyTuiMenuItem {
            id: 3,
            title: "Reload Hyprland Config".to_string(),
            command: "hyprctl reload".to_string(),
        });

        Self {
            items,
            selected_index: 0,
        }
    }

    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    pub fn get_selected_item(&self) -> Option<&OmarchyTuiMenuItem> {
        self.items.get(self.selected_index)
    }
}

impl Default for OmarchyTuiMenuApplet {
    fn default() -> Self {
        Self::new()
    }
}

/// Power Profile Mode Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmarchyPowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

/// Omarchy Dynamic CPU Power Profile Manager
pub struct OmarchyPowerProfileManager {
    pub active_profile: OmarchyPowerProfile,
    pub cpu_governor: String,
    pub epp_setting: String,
}

impl OmarchyPowerProfileManager {
    pub fn new() -> Self {
        Self {
            active_profile: OmarchyPowerProfile::Balanced,
            cpu_governor: "powersave".to_string(),
            epp_setting: "balance_performance".to_string(),
        }
    }

    pub fn switch_profile(&mut self, profile: OmarchyPowerProfile) -> String {
        self.active_profile = profile;
        match profile {
            OmarchyPowerProfile::Performance => {
                self.cpu_governor = "performance".to_string();
                self.epp_setting = "performance".to_string();
            }
            OmarchyPowerProfile::Balanced => {
                self.cpu_governor = "powersave".to_string();
                self.epp_setting = "balance_performance".to_string();
            }
            OmarchyPowerProfile::PowerSaver => {
                self.cpu_governor = "powersave".to_string();
                self.epp_setting = "power".to_string();
            }
        }
        format!(
            "Switched power profile to {:?} ({})",
            profile, self.cpu_governor
        )
    }
}

impl Default for OmarchyPowerProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Omarchy Dotfiles Snapshot Backup & Version Control Manager
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotfileBackupRecord {
    pub snapshot_id: u32,
    pub timestamp_epoch: u64,
    pub dotfiles_count: usize,
}

pub struct OmarchyDotfilesBackupRestoreEngine {
    pub backups: Vec<DotfileBackupRecord>,
    pub tracked_dotfiles: HashMap<String, String>,
}

impl OmarchyDotfilesBackupRestoreEngine {
    pub fn new() -> Self {
        let mut tracked = HashMap::new();
        tracked.insert(
            "hyprland.conf".to_string(),
            "~/.config/hypr/hyprland.conf".to_string(),
        );
        tracked.insert(
            "alacritty.toml".to_string(),
            "~/.config/alacritty/alacritty.toml".to_string(),
        );

        Self {
            backups: Vec::new(),
            tracked_dotfiles: tracked,
        }
    }

    pub fn create_backup_snapshot(&mut self, timestamp: u64) -> u32 {
        let snap_id = (self.backups.len() + 1) as u32;
        self.backups.push(DotfileBackupRecord {
            snapshot_id: snap_id,
            timestamp_epoch: timestamp,
            dotfiles_count: self.tracked_dotfiles.len(),
        });
        snap_id
    }
}

impl Default for OmarchyDotfilesBackupRestoreEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Omarchy Wallust / Matugen Dynamic Wallpaper Palette Extraction Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WallpaperPalette {
    pub background: String,
    pub foreground: String,
    pub accent_primary: String,
    pub accent_secondary: String,
    pub palette_colors: Vec<String>,
}

pub struct OmarchyWallustMatugenPaletteEngine {
    pub current_palette: WallpaperPalette,
}

impl OmarchyWallustMatugenPaletteEngine {
    pub fn new(bg: &str, fg: &str, accent1: &str, accent2: &str) -> Self {
        Self {
            current_palette: WallpaperPalette {
                background: bg.to_string(),
                foreground: fg.to_string(),
                accent_primary: accent1.to_string(),
                accent_secondary: accent2.to_string(),
                palette_colors: vec![
                    bg.to_string(),
                    fg.to_string(),
                    accent1.to_string(),
                    accent2.to_string(),
                ],
            },
        }
    }

    pub fn generate_kitty_theme(&self) -> String {
        format!(
            "background {}\nforeground {}\ncursor {}\nactive_tab_background {}\n",
            self.current_palette.background,
            self.current_palette.foreground,
            self.current_palette.accent_primary,
            self.current_palette.accent_secondary
        )
    }

    pub fn generate_hyprland_border_colors(&self) -> String {
        format!(
            "col.active_border = rgb({}) rgb({}) 45deg\ncol.inactive_border = rgb({})\n",
            self.current_palette.accent_primary.trim_start_matches('#'),
            self.current_palette.accent_secondary.trim_start_matches('#'),
            self.current_palette.background.trim_start_matches('#')
        )
    }

    pub fn generate_gtk_css(&self) -> String {
        format!(
            "@define-color theme_bg_color {};\n@define-color theme_fg_color {};\n@define-color accent_color {};\n",
            self.current_palette.background,
            self.current_palette.foreground,
            self.current_palette.accent_primary
        )
    }
}

// =========================================================================
// 6. Omarchy omarchy-update Transactional Pipeline & Snapshot Manager
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStageStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct UpdatePipelineStage {
    pub name: String,
    pub status: UpdateStageStatus,
}

pub struct OmarchyUpdatePipelineEngine {
    pub stages: Vec<UpdatePipelineStage>,
    pub snapshot_taken: bool,
    pub is_rollback_needed: bool,
}

impl OmarchyUpdatePipelineEngine {
    pub fn new() -> Self {
        let stages = vec![
            UpdatePipelineStage { name: "Pre-Update Snapshot".to_string(), status: UpdateStageStatus::Pending },
            UpdatePipelineStage { name: "Fetch Packages & Signatures".to_string(), status: UpdateStageStatus::Pending },
            UpdatePipelineStage { name: "Execute Pacman Transaction".to_string(), status: UpdateStageStatus::Pending },
            UpdatePipelineStage { name: "Post-Transaction Hooks".to_string(), status: UpdateStageStatus::Pending },
            UpdatePipelineStage { name: "Bootloader Integrity Check".to_string(), status: UpdateStageStatus::Pending },
        ];

        Self {
            stages,
            snapshot_taken: false,
            is_rollback_needed: false,
        }
    }

    pub fn execute_update_pipeline(&mut self) -> Result<usize, &'static str> {
        let mut completed = 0;
        for stage in &mut self.stages {
            if stage.name == "Pre-Update Snapshot" {
                self.snapshot_taken = true;
            }
            stage.status = UpdateStageStatus::Completed;
            completed += 1;
        }
        Ok(completed)
    }

    pub fn trigger_atomic_rollback(&mut self) -> bool {
        if self.snapshot_taken {
            self.is_rollback_needed = false;
            true
        } else {
            false
        }
    }
}

impl Default for OmarchyUpdatePipelineEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. Omarchy Quickshell Bar Widget Controller
// =========================================================================

#[derive(Debug, Clone)]
pub struct QuickshellWidgetState {
    pub active_workspace: u32,
    pub window_title: String,
    pub cpu_usage_pct: u32,
    pub mem_usage_pct: u32,
    pub herdr_agent_status: String,
}

pub struct OmarchyQuickshellBarController {
    pub state: QuickshellWidgetState,
}

impl OmarchyQuickshellBarController {
    pub fn new() -> Self {
        Self {
            state: QuickshellWidgetState {
                active_workspace: 1,
                window_title: "Terminal".to_string(),
                cpu_usage_pct: 12,
                mem_usage_pct: 34,
                herdr_agent_status: "Active (Idle)".to_string(),
            },
        }
    }

    pub fn update_telemetry(&mut self, workspace: u32, title: &str, cpu: u32, mem: u32) {
        self.state.active_workspace = workspace;
        self.state.window_title = title.to_string();
        self.state.cpu_usage_pct = cpu;
        self.state.mem_usage_pct = mem;
    }

    pub fn render_status_json(&self) -> String {
        format!(
            r#"{{"workspace": {}, "title": "{}", "cpu": {}%, "mem": {}%, "herdr": "{}"}}"#,
            self.state.active_workspace,
            self.state.window_title,
            self.state.cpu_usage_pct,
            self.state.mem_usage_pct,
            self.state.herdr_agent_status
        )
    }
}

impl Default for OmarchyQuickshellBarController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omarchy_fastfetch_engine() {
        let ff = OmarchyFastfetchEngine::new("SigmaOS", "6.8.0", "TokyoNight", 2048, 16384);
        let banner = ff.generate_summary_banner();
        assert!(banner.contains("SigmaOS"));
        assert!(banner.contains("TokyoNight"));
        assert!(banner.contains("2048MB / 16384MB"));
    }

    #[test]
    fn test_omarchy_tui_menu_applet() {
        let mut applet = OmarchyTuiMenuApplet::new();
        assert_eq!(applet.selected_index, 0);
        assert_eq!(applet.get_selected_item().unwrap().title, "Switch Theme");

        applet.select_next();
        assert_eq!(applet.selected_index, 1);
        assert_eq!(
            applet.get_selected_item().unwrap().title,
            "Launch Herdr Agent"
        );
    }

    #[test]
    fn test_omarchy_power_profile_manager() {
        let mut power = OmarchyPowerProfileManager::new();
        let res = power.switch_profile(OmarchyPowerProfile::Performance);
        assert!(res.contains("Performance"));
        assert_eq!(power.cpu_governor, "performance");
        assert_eq!(power.epp_setting, "performance");
    }

    #[test]
    fn test_omarchy_dotfiles_backup_restore_engine() {
        let mut dotfiles = OmarchyDotfilesBackupRestoreEngine::new();
        let snap_id = dotfiles.create_backup_snapshot(1700000000);
        assert_eq!(snap_id, 1);
        assert_eq!(dotfiles.backups.len(), 1);
        assert_eq!(dotfiles.backups[0].dotfiles_count, 2);
    }

    #[test]
    fn test_omarchy_wallust_matugen_palette_engine() {
        let palette = OmarchyWallustMatugenPaletteEngine::new("#1a1b26", "#c0caf5", "#7aa2f7", "#bb9af7");
        let kitty = palette.generate_kitty_theme();
        assert!(kitty.contains("#1a1b26"));
        assert!(kitty.contains("#7aa2f7"));

        let hypr = palette.generate_hyprland_border_colors();
        assert!(hypr.contains("col.active_border = rgb(7aa2f7) rgb(bb9af7) 45deg"));

        let gtk = palette.generate_gtk_css();
        assert!(gtk.contains("@define-color accent_color #7aa2f7;"));
    }

    #[test]
    fn test_omarchy_update_pipeline_engine() {
        let mut update = OmarchyUpdatePipelineEngine::new();
        assert_eq!(update.stages.len(), 5);
        let res = update.execute_update_pipeline().unwrap();
        assert_eq!(res, 5);
        assert!(update.snapshot_taken);
        assert!(update.trigger_atomic_rollback());
    }

    #[test]
    fn test_omarchy_quickshell_bar_controller() {
        let mut bar = OmarchyQuickshellBarController::new();
        bar.update_telemetry(3, "Neovim - main.rs", 25, 42);
        let json = bar.render_status_json();
        assert!(json.contains(r#""workspace": 3"#));
        assert!(json.contains(r#""title": "Neovim - main.rs""#));
        assert!(json.contains(r#""cpu": 25%"#));
    }
}
