// SigmaOS Sovereign Omarchy Linux Repository Innovations Engine
// Inspired by the Omarchy Linux repository: Hyprland dynamic tiling & animations, Quickshell status applets,
// Ghostty & Kitty font studio, Omakase Neovim LSP setup, Herdr multi-agent AI orchestrator,
// GNU Stow dotfiles & theme studio, and Walker fuzzy application launcher.
//
// 100% Safe-Rust zero-dependency implementation using alloc primitives.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Omarchy 4 Dynamic Hyprland Layout & Custom Animation Curve Engine
#[derive(Debug, Clone)]
pub struct HyprlandWindowRule {
    pub window_class: String,
    pub target_workspace: u32,
    pub is_floating: bool,
    pub opacity: f32,
}

#[derive(Debug, Clone)]
pub struct HyprlandAnimationCurve {
    pub curve_name: String,
    pub p1_x: f32,
    pub p1_y: f32,
    pub p2_x: f32,
    pub p2_y: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Omarchy4DynamicHyprlandLayoutEngine {
    pub active_layout: String, // "dwindle" or "master"
    pub window_rules: Vec<HyprlandWindowRule>,
    pub animation_curves: BTreeMap<String, HyprlandAnimationCurve>,
}

impl Omarchy4DynamicHyprlandLayoutEngine {
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.active_layout = "dwindle".to_string();

        engine.window_rules.push(HyprlandWindowRule {
            window_class: "Ghostty".to_string(),
            target_workspace: 1,
            is_floating: false,
            opacity: 0.95,
        });
        engine.window_rules.push(HyprlandWindowRule {
            window_class: "Chromium".to_string(),
            target_workspace: 2,
            is_floating: false,
            opacity: 1.0,
        });

        engine.animation_curves.insert(
            "md3_decel".to_string(),
            HyprlandAnimationCurve {
                curve_name: "md3_decel".to_string(),
                p1_x: 0.05,
                p1_y: 0.7,
                p2_x: 0.1,
                p2_y: 1.0,
            },
        );

        engine
    }

    pub fn set_layout(&mut self, layout: &str) -> Result<String, &'static str> {
        if layout == "dwindle" || layout == "master" {
            self.active_layout = layout.to_string();
            Ok(format!("Hyprland layout switched to {}", layout))
        } else {
            Err("Unsupported layout mode")
        }
    }

    pub fn query_rule_for_window(&self, class_name: &str) -> Option<&HyprlandWindowRule> {
        self.window_rules.iter().find(|r| r.window_class == class_name)
    }
}

/// Omarchy Quickshell QML & Waybar JSON/CSS Status Bar Applet Studio Engine
#[derive(Debug, Clone)]
pub struct QuickshellModuleSpec {
    pub module_id: String,
    pub module_type: String, // "Clock", "Workspaces", "CpuRamMeter", "HerdrAiStatus", "Battery"
    pub update_interval_ms: u32,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct OmarchyQuickshellStatusStudioEngine {
    pub active_preset: String,
    pub modules: BTreeMap<String, QuickshellModuleSpec>,
}

impl OmarchyQuickshellStatusStudioEngine {
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.active_preset = "omarchy-glass-topbar".to_string();

        engine.add_module("clock", "Clock", 1000);
        engine.add_module("workspaces", "Workspaces", 0);
        engine.add_module("sys_meter", "CpuRamMeter", 2000);
        engine.add_module("herdr_ai", "HerdrAiStatus", 5000);
        engine
    }

    pub fn add_module(&mut self, id: &str, mod_type: &str, interval_ms: u32) {
        let spec = QuickshellModuleSpec {
            module_id: id.to_string(),
            module_type: mod_type.to_string(),
            update_interval_ms: interval_ms,
            is_enabled: true,
        };
        self.modules.insert(id.to_string(), spec);
    }

    pub fn render_quickshell_json(&self) -> String {
        format!(
            "{{\"preset\":\"{}\",\"active_modules_count\":{}}}",
            self.active_preset,
            self.modules.len()
        )
    }
}

/// Omarchy Ghostty & Kitty Terminal Emulator Config & Font Studio
#[derive(Debug, Clone)]
pub struct TerminalFontProfile {
    pub font_family: String,
    pub font_size_pt: f32,
    pub enable_ligatures: bool,
    pub cell_height_ratio: f32,
}

#[derive(Debug, Clone, Default)]
pub struct OmarchyGhosttyKittyFontStudioEngine {
    pub active_terminal: String, // "Ghostty" or "Kitty"
    pub font_profile: Option<TerminalFontProfile>,
}

impl OmarchyGhosttyKittyFontStudioEngine {
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.active_terminal = "Ghostty".to_string();
        engine.font_profile = Some(TerminalFontProfile {
            font_family: "JetBrains Mono Nerd Font".to_string(),
            font_size_pt: 12.5,
            enable_ligatures: true,
            cell_height_ratio: 1.1,
        });
        engine
    }

    pub fn generate_ghostty_config(&self) -> String {
        if let Some(font) = &self.font_profile {
            format!(
                "font-family = \"{}\"\nfont-size = {}\nfont-thicken = true\ncursor-style = block",
                font.font_family, font.font_size_pt
            )
        } else {
            "font-family = monospace".to_string()
        }
    }
}

/// Omarchy Omakase Neovim Preset, Treesitter, & Mason LSP Engine
#[derive(Debug, Clone)]
pub struct NeovimLspServer {
    pub server_name: String,
    pub language_id: String,
    pub is_installed_via_mason: bool,
}

#[derive(Debug, Clone, Default)]
pub struct OmarchyOmakaseNeovimLspStudioEngine {
    pub preset_name: String,
    pub installed_lsps: BTreeMap<String, NeovimLspServer>,
    pub treesitter_parsers: Vec<String>,
}

impl OmarchyOmakaseNeovimLspStudioEngine {
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.preset_name = "Omakase-Ide-Pro".to_string();

        engine.install_lsp("rust_analyzer", "rust");
        engine.install_lsp("clangd", "cpp");
        engine.install_lsp("ts_ls", "typescript");

        engine.treesitter_parsers.push("rust".to_string());
        engine.treesitter_parsers.push("python".to_string());
        engine.treesitter_parsers.push("lua".to_string());

        engine
    }

    pub fn install_lsp(&mut self, name: &str, lang: &str) {
        let server = NeovimLspServer {
            server_name: name.to_string(),
            language_id: lang.to_string(),
            is_installed_via_mason: true,
        };
        self.installed_lsps.insert(name.to_string(), server);
    }

    pub fn get_preset_summary(&self) -> String {
        format!(
            "Neovim Preset: {} | Installed LSPs: {} | Treesitter Parsers: {}",
            self.preset_name,
            self.installed_lsps.len(),
            self.treesitter_parsers.len()
        )
    }
}

/// Omarchy Herdr Multi-Agent LLM AI Coding Orchestrator Engine
#[derive(Debug, Clone)]
pub struct HerdrAgentTask {
    pub task_id: String,
    pub agent_role: String, // "Architect", "Coder", "Reviewer", "TestWriter"
    pub prompt: String,
    pub status: String,
}

#[derive(Debug, Clone, Default)]
pub struct OmarchyHerdrMultiAgentOrchestratorEngine {
    pub active_agents_count: usize,
    pub task_queue: Vec<HerdrAgentTask>,
}

impl OmarchyHerdrMultiAgentOrchestratorEngine {
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.active_agents_count = 4;
        engine
    }

    pub fn dispatch_coding_task(&mut self, id: &str, role: &str, prompt: &str) -> String {
        let task = HerdrAgentTask {
            task_id: id.to_string(),
            agent_role: role.to_string(),
            prompt: prompt.to_string(),
            status: "Processing".to_string(),
        };
        self.task_queue.push(task);
        format!("Herdr agent [{}] assigned to task {}", role, id)
    }

    pub fn complete_task(&mut self, id: &str) -> bool {
        for task in self.task_queue.iter_mut() {
            if task.task_id == id {
                task.status = "Completed".to_string();
                return true;
            }
        }
        false
    }
}

/// Omarchy GNU Stow Dotfiles Profile Manager & Live Theme Switcher Studio
#[derive(Debug, Clone)]
pub struct DotfileStowProfile {
    pub profile_name: String, // "tokyo-night", "catppuccin-mocha", "gruvbox-dark"
    pub target_home_dir: String,
    pub is_stowed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct OmarchyDotfilesStowThemeStudioEngine {
    pub active_theme: String,
    pub stow_profiles: BTreeMap<String, DotfileStowProfile>,
}

impl OmarchyDotfilesStowThemeStudioEngine {
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.active_theme = "TokyoNight".to_string();

        engine.stow_profiles.insert(
            "tokyo-night".to_string(),
            DotfileStowProfile {
                profile_name: "tokyo-night".to_string(),
                target_home_dir: "/home/user/.config".to_string(),
                is_stowed: true,
            },
        );
        engine.stow_profiles.insert(
            "catppuccin-mocha".to_string(),
            DotfileStowProfile {
                profile_name: "catppuccin-mocha".to_string(),
                target_home_dir: "/home/user/.config".to_string(),
                is_stowed: false,
            },
        );

        engine
    }

    pub fn switch_theme_live(&mut self, theme_name: &str) -> Result<String, &'static str> {
        self.active_theme = theme_name.to_string();
        Ok(format!("Omarchy live theme switched to {}", theme_name))
    }
}

/// Omarchy Walker Application Launcher & Fuzzy Search Indexer
#[derive(Debug, Clone)]
pub struct WalkerAppEntry {
    pub app_id: String,
    pub display_name: String,
    pub exec_cmd: String,
    pub frecent_score: u32,
}

#[derive(Debug, Clone, Default)]
pub struct OmarchyWalkerFuzzyLauncherEngine {
    pub app_index: Vec<WalkerAppEntry>,
}

impl OmarchyWalkerFuzzyLauncherEngine {
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.app_index.push(WalkerAppEntry {
            app_id: "ghostty.desktop".to_string(),
            display_name: "Ghostty Terminal".to_string(),
            exec_cmd: "ghostty".to_string(),
            frecent_score: 95,
        });
        engine.app_index.push(WalkerAppEntry {
            app_id: "sigma-browser.desktop".to_string(),
            display_name: "SigmaOS Web Shell Browser".to_string(),
            exec_cmd: "sigma-browser".to_string(),
            frecent_score: 100,
        });
        engine.app_index.push(WalkerAppEntry {
            app_id: "neovim.desktop".to_string(),
            display_name: "Omakase Neovim".to_string(),
            exec_cmd: "nvim".to_string(),
            frecent_score: 80,
        });
        engine
    }

    pub fn fuzzy_search(&self, query: &str) -> Vec<&WalkerAppEntry> {
        let mut matched: Vec<&WalkerAppEntry> = self
            .app_index
            .iter()
            .filter(|app| {
                app.display_name.to_lowercase().contains(&query.to_lowercase())
                    || app.exec_cmd.contains(query)
            })
            .collect();

        matched.sort_by(|a, b| b.frecent_score.cmp(&a.frecent_score));
        matched
    }
}

/// Sovereign Omarchy Repository Innovations Master Suite
#[derive(Debug, Clone, Default)]
pub struct SovereignOmarchyRepositoryInnovationsSuite {
    pub hyprland: Omarchy4DynamicHyprlandLayoutEngine,
    pub quickshell: OmarchyQuickshellStatusStudioEngine,
    pub terminal_font: OmarchyGhosttyKittyFontStudioEngine,
    pub neovim_lsp: OmarchyOmakaseNeovimLspStudioEngine,
    pub herdr_ai: OmarchyHerdrMultiAgentOrchestratorEngine,
    pub dotfiles_theme: OmarchyDotfilesStowThemeStudioEngine,
    pub walker_launcher: OmarchyWalkerFuzzyLauncherEngine,
}

impl SovereignOmarchyRepositoryInnovationsSuite {
    pub fn new() -> Self {
        Self {
            hyprland: Omarchy4DynamicHyprlandLayoutEngine::new(),
            quickshell: OmarchyQuickshellStatusStudioEngine::new(),
            terminal_font: OmarchyGhosttyKittyFontStudioEngine::new(),
            neovim_lsp: OmarchyOmakaseNeovimLspStudioEngine::new(),
            herdr_ai: OmarchyHerdrMultiAgentOrchestratorEngine::new(),
            dotfiles_theme: OmarchyDotfilesStowThemeStudioEngine::new(),
            walker_launcher: OmarchyWalkerFuzzyLauncherEngine::new(),
        }
    }

    pub fn calculate_omarchy_parity_score(&self) -> u32 {
        let mut score = 0;
        if self.hyprland.active_layout == "dwindle" {
            score += 15;
        }
        if self.quickshell.modules.len() >= 4 {
            score += 15;
        }
        if self.terminal_font.font_profile.is_some() {
            score += 15;
        }
        if self.neovim_lsp.installed_lsps.len() >= 3 {
            score += 15;
        }
        if self.herdr_ai.active_agents_count >= 4 {
            score += 15;
        }
        if self.dotfiles_theme.stow_profiles.len() >= 2 {
            score += 15;
        }
        if self.walker_launcher.app_index.len() >= 3 {
            score += 10;
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyprland_layout_engine() {
        let mut engine = Omarchy4DynamicHyprlandLayoutEngine::new();
        assert_eq!(engine.active_layout, "dwindle");
        assert!(engine.set_layout("master").is_ok());
        assert_eq!(engine.active_layout, "master");
        let rule = engine.query_rule_for_window("Ghostty").unwrap();
        assert_eq!(rule.target_workspace, 1);
    }

    #[test]
    fn test_quickshell_studio() {
        let engine = OmarchyQuickshellStatusStudioEngine::new();
        let json = engine.render_quickshell_json();
        assert!(json.contains("omarchy-glass-topbar"));
    }

    #[test]
    fn test_terminal_font_studio() {
        let engine = OmarchyGhosttyKittyFontStudioEngine::new();
        let config = engine.generate_ghostty_config();
        assert!(config.contains("JetBrains Mono Nerd Font"));
    }

    #[test]
    fn test_omakase_neovim_lsp() {
        let engine = OmarchyOmakaseNeovimLspStudioEngine::new();
        let summary = engine.get_preset_summary();
        assert!(summary.contains("Installed LSPs: 3"));
    }

    #[test]
    fn test_herdr_ai_orchestrator() {
        let mut engine = OmarchyHerdrMultiAgentOrchestratorEngine::new();
        let msg = engine.dispatch_coding_task("t-1", "Coder", "Write safe Rust kernel shard");
        assert!(msg.contains("Herdr agent"));
        assert!(engine.complete_task("t-1"));
    }

    #[test]
    fn test_dotfiles_theme_studio() {
        let mut engine = OmarchyDotfilesStowThemeStudioEngine::new();
        assert_eq!(engine.active_theme, "TokyoNight");
        assert!(engine.switch_theme_live("CatppuccinMocha").is_ok());
        assert_eq!(engine.active_theme, "CatppuccinMocha");
    }

    #[test]
    fn test_walker_fuzzy_launcher() {
        let engine = OmarchyWalkerFuzzyLauncherEngine::new();
        let results = engine.fuzzy_search("Ghostty");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].exec_cmd, "ghostty");
    }

    #[test]
    fn test_omarchy_repository_suite() {
        let suite = SovereignOmarchyRepositoryInnovationsSuite::new();
        let score = suite.calculate_omarchy_parity_score();
        assert_eq!(score, 100);
    }
}
