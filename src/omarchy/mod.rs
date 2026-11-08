// SPDX-License-Identifier: MIT
// SigmaOS Omarchy Command Parity Suite
// Native Rust implementation of Omarchy (Arch + Hyprland) user-facing tools:
// omarchy-theme-set, omarchy-update, and the fuzzy omarchy-menu launcher.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. ThemeFileParser (omarchy .theme file parity)
// ============================================================================

/// Parsed Omarchy theme file (`~/.config/omarchy/themes/<name>.theme`).
/// Omarchy themes are flat KEY=VALUE files with color vars, a wallpaper
/// path, and GTK/Qt scheme hints.
#[derive(Debug, Clone, Default)]
pub struct OmarchyThemeFile {
    /// theme name derived from the file stem
    pub name: String,
    /// mode: "dark" | "light"
    pub mode: String,
    /// wallpaper path or URL
    pub wallpaper: String,
    /// accent color (#rrggbb)
    pub accent: String,
    /// background color
    pub background: String,
    /// foreground color
    pub foreground: String,
    /// terminal palette color0..color15
    pub palette: BTreeMap<u8, String>,
    /// any extra KEY=VALUE pairs preserved verbatim
    pub extras: BTreeMap<String, String>,
}

impl OmarchyThemeFile {
    /// Parse theme file content. `name` is the file stem.
    pub fn parse(name: &str, content: &str) -> Self {
        let mut theme = OmarchyThemeFile {
            name: name.to_string(),
            ..OmarchyThemeFile::default()
        };
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim().to_lowercase();
            let value = value.trim().to_string();
            match key.as_str() {
                "mode" => theme.mode = value,
                "wallpaper" => theme.wallpaper = value,
                "accent" => theme.accent = value,
                "background" => theme.background = value,
                "foreground" => theme.foreground = value,
                _ => {
                    if let Some(rest) = key.strip_prefix("color") {
                        if let Ok(idx) = rest.parse::<u8>() {
                            if idx < 16 {
                                theme.palette.insert(idx, value.clone());
                                continue;
                            }
                        }
                    }
                    theme.extras.insert(key, value);
                }
            }
        }
        theme
    }

    /// Serialize back to .theme file format (stable key order)
    pub fn serialize(&self) -> String {
        let mut out = String::new();
        if !self.mode.is_empty() {
            out.push_str(&format!("mode={}\n", self.mode));
        }
        if !self.wallpaper.is_empty() {
            out.push_str(&format!("wallpaper={}\n", self.wallpaper));
        }
        if !self.accent.is_empty() {
            out.push_str(&format!("accent={}\n", self.accent));
        }
        if !self.background.is_empty() {
            out.push_str(&format!("background={}\n", self.background));
        }
        if !self.foreground.is_empty() {
            out.push_str(&format!("foreground={}\n", self.foreground));
        }
        for (idx, color) in &self.palette {
            out.push_str(&format!("color{}={}\n", idx, color));
        }
        for (key, value) in &self.extras {
            out.push_str(&format!("{}={}\n", key, value));
        }
        out
    }
}

// ============================================================================
// 2. OmarchyThemeSetEngine (omarchy-theme-set parity)
// ============================================================================

/// A file the theme engine writes when a theme is applied
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeFileWrite {
    pub path: String,
    pub content: String,
}

/// The ordered plan produced by `omarchy-theme-set <name>`
#[derive(Debug, Clone, Default)]
pub struct ThemeApplyPlan {
    pub theme_name: String,
    pub writes: Vec<ThemeFileWrite>,
    /// shell commands run after writes to refresh running apps
    /// (omarchy-refresh-* equivalents)
    pub refresh_commands: Vec<String>,
    /// wallpaper to set, if the theme declares one
    pub wallpaper: String,
}

/// Applies an Omarchy theme across GTK, Qt, Hyprland, terminal, shell bar,
/// and wallpaper — the `omarchy-theme-set` pipeline.
#[derive(Debug, Default)]
pub struct OmarchyThemeSetEngine {
    /// config root, e.g. "/home/user/.config"
    pub config_root: String,
}

impl OmarchyThemeSetEngine {
    pub fn new(config_root: &str) -> Self {
        Self {
            config_root: config_root.to_string(),
        }
    }

    fn path(&self, rel: &str) -> String {
        format!("{}/{}", self.config_root.trim_end_matches('/'), rel)
    }

    /// Build the full apply plan for a parsed theme file.
    pub fn plan(&self, theme: &OmarchyThemeFile) -> ThemeApplyPlan {
        let mut writes = Vec::new();

        // GTK3/GTK4 color vars (Adw stylesheet + gtk.css)
        let gtk = format!(
            "/* generated by omarchy-theme-set {} */\n@define-color theme_bg_color {};\n@define-color theme_fg_color {};\n@define-color theme_accent {};\n",
            theme.name, theme.background, theme.foreground, theme.accent
        );
        writes.push(ThemeFileWrite {
            path: self.path("gtk-3.0/gtk.css"),
            content: gtk.clone(),
        });
        writes.push(ThemeFileWrite {
            path: self.path("gtk-4.0/gtk.css"),
            content: gtk,
        });

        // Hyprland window/border/decoration colors
        let accent = theme.accent.trim_start_matches('#');
        let hypr = format!(
            "# generated by omarchy-theme-set {}\ngeneral:col.active_border = {}ff{}\ngeneral:col.inactive_border = 80808080\ndecoration:col.border_active = {}ff{}\n",
            theme.name, accent, accent, accent, accent,
        );
        writes.push(ThemeFileWrite {
            path: self.path("hypr/colors.conf"),
            content: hypr,
        });

        // Ghostty/foot terminal theme
        let mut term = format!(
            "# generated by omarchy-theme-set {}\nbackground = {}\nforeground = {}\ncursor-color = {}\n",
            theme.name,
            theme.background.trim_start_matches('#'),
            theme.foreground.trim_start_matches('#'),
            theme.accent.trim_start_matches('#'),
        );
        for idx in 0..16u8 {
            if let Some(c) = theme.palette.get(&idx) {
                term.push_str(&format!(
                    "palette = {}={}\n",
                    idx,
                    c.trim_start_matches('#')
                ));
            }
        }
        writes.push(ThemeFileWrite {
            path: self.path("ghostty/config"),
            content: term.clone(),
        });
        writes.push(ThemeFileWrite {
            path: self.path("foot/foot.ini"),
            content: format!(
                "[main]\nbackground={}\nforeground={}\n",
                term.lines()
                    .nth(1)
                    .map(|l| l.trim_start_matches("background = "))
                    .unwrap_or(""),
                term.lines()
                    .nth(2)
                    .map(|l| l.trim_start_matches("foreground = "))
                    .unwrap_or("")
            ),
        });

        // Quickshell / waybar palette export for the bar
        let bar = format!(
            "// generated by omarchy-theme-set {}\nQtObject {{\n    property color bg: \"{}\"\n    property color fg: \"{}\"\n    property color accent: \"{}\"\n}}\n",
            theme.name, theme.background, theme.foreground, theme.accent
        );
        writes.push(ThemeFileWrite {
            path: self.path("quickshell/theme.qml"),
            content: bar,
        });

        // Qt color scheme pointer
        writes.push(ThemeFileWrite {
            path: self.path("qt5ct/colors.conf"),
            content: format!(
                "[ColorScheme]\nactive_colors=@Variant(\\0\\0\\0\\x2\\0\\0\\0\\x2\\0\\0\\0\\0\\0\\0\\x2\\0o\\0l\\0o\\0r\\0s\\0)\n[Colors:Window]\nBackgroundNormal={}\n",
                theme.background
            ),
        });

        let refresh_commands = vec![
            "omarchy-refresh-hyprland".to_string(),
            "omarchy-refresh-shell".to_string(),
            "omarchy-refresh-qt".to_string(),
            "omarchy-refresh-gtk".to_string(),
        ];

        ThemeApplyPlan {
            theme_name: theme.name.clone(),
            writes,
            refresh_commands,
            wallpaper: theme.wallpaper.clone(),
        }
    }

    /// Set the active theme name (what omarchy-theme-set records)
    pub fn active_theme_marker(&self, theme_name: &str) -> ThemeFileWrite {
        ThemeFileWrite {
            path: self.path("omarchy/theme"),
            content: format!("{}\n", theme_name),
        }
    }
}

// ============================================================================
// 3. OmarchyUpdateOrchestrator (omarchy-update parity)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStage {
    RefreshMirrors,
    SyncRepos,
    UpdateAUR,
    CleanCache,
    CheckReboot,
}

#[derive(Debug, Clone, Default)]
pub struct OmarchyUpdateResult {
    pub success: bool,
    pub commands_run: Vec<String>,
    pub failed_stage: Option<UpdateStage>,
    pub reboot_required: bool,
}

/// Orchestrates the Omarchy full-system update flow: mirror refresh,
/// `pacman -Syu`, AUR/devel updates, cache cleaning, and reboot prompt.
#[derive(Debug)]
pub struct OmarchyUpdateOrchestrator {
    /// run `omarchy-refresh-pacman` mirror ranking before sync
    pub refresh_mirrors: bool,
    /// include AUR/devel packages in the update
    pub include_devel: bool,
    /// aggressively prune package cache afterwards
    pub clean_cache: bool,
}

impl OmarchyUpdateOrchestrator {
    pub fn new() -> Self {
        Self {
            refresh_mirrors: true,
            include_devel: true,
            clean_cache: true,
        }
    }

    /// Ordered command plan for the update, mirroring `omarchy-update`.
    pub fn plan(&self) -> Vec<(UpdateStage, String)> {
        let mut steps = Vec::new();
        if self.refresh_mirrors {
            steps.push((
                UpdateStage::RefreshMirrors,
                "omarchy-refresh-pacman".to_string(),
            ));
        }
        steps.push((UpdateStage::SyncRepos, "sudo pacman -Syu".to_string()));
        if self.include_devel {
            steps.push((
                UpdateStage::UpdateAUR,
                "yay -Sua --devel --combinedupgrade".to_string(),
            ));
        }
        if self.clean_cache {
            steps.push((UpdateStage::CleanCache, "paccache -rk1".to_string()));
        }
        steps.push((
            UpdateStage::CheckReboot,
            "omarchy-check-update --reboot".to_string(),
        ));
        steps
    }

    /// Execute the plan; `fail_at` simulates a stage failure for testing.
    /// Later stages are skipped but cache cleaning still runs best-effort,
    /// matching omarchy-update's failure tolerance.
    pub fn execute(&self, fail_at: Option<UpdateStage>) -> OmarchyUpdateResult {
        let mut result = OmarchyUpdateResult {
            reboot_required: false,
            ..OmarchyUpdateResult::default()
        };
        for (stage, cmd) in self.plan() {
            if fail_at == Some(stage) {
                result.failed_stage = Some(stage);
                result.success = false;
                // still attempt cache clean like the real script's trap
                if self.clean_cache && stage != UpdateStage::CleanCache {
                    result
                        .commands_run
                        .push("paccache -rk1 (best-effort)".to_string());
                }
                return result;
            }
            result.commands_run.push(cmd);
            if stage == UpdateStage::SyncRepos {
                result.reboot_required = true; // kernel/libc may have changed
            }
        }
        result.success = true;
        result
    }
}

impl Default for OmarchyUpdateOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. OmarchyMenu (omarchy-menu / walker / elephant launcher parity)
// ============================================================================

/// A launchable command shown in the Omarchy menu
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuEntry {
    pub id: String,
    pub label: String,
    pub command: String,
    /// optional category for submenus (e.g. "System", "Apps")
    pub category: String,
}

/// Fuzzy-matched menu result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuMatch {
    pub entry: MenuEntry,
    pub score: i32,
}

/// The Omarchy application/command launcher (`omarchy-menu`, powered in
/// real Omarchy by walker/elephant over fuzzel). Ranks entries by fuzzy
/// subsequence match quality, then label length, then insertion order.
#[derive(Debug, Default)]
pub struct OmarchyMenu {
    pub entries: Vec<MenuEntry>,
}

impl OmarchyMenu {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: MenuEntry) {
        self.entries.push(entry);
    }

    /// Score how well `query` fuzzy-matches `candidate`.
    /// Returns None when the query is not a subsequence of the candidate.
    fn fuzzy_score(query: &str, candidate: &str) -> Option<i32> {
        let q: Vec<char> = query.to_lowercase().chars().collect();
        let c: Vec<char> = candidate.to_lowercase().chars().collect();
        if q.is_empty() {
            return Some(0);
        }
        let mut score = 0i32;
        let mut qi = 0usize;
        let mut prev_idx: Option<usize> = None;
        let mut consecutive = 0i32;
        for (ci, ch) in c.iter().enumerate() {
            if qi >= q.len() {
                break;
            }
            if *ch == q[qi] {
                score += 10;
                if let Some(p) = prev_idx {
                    if ci == p + 1 {
                        consecutive += 1;
                        score += consecutive * 5; // reward runs
                    } else {
                        consecutive = 0;
                    }
                }
                // word-start bonus
                if ci == 0 || c[ci - 1] == ' ' || c[ci - 1] == '-' || c[ci - 1] == '_' {
                    score += 8;
                }
                prev_idx = Some(ci);
                qi += 1;
            }
        }
        if qi < q.len() {
            return None;
        }
        // shorter candidates rank higher on ties
        score -= (c.len() as i32) / 10;
        Some(score)
    }

    /// Return matching entries sorted by score descending.
    pub fn search(&self, query: &str) -> Vec<MenuMatch> {
        let mut matches: Vec<MenuMatch> = self
            .entries
            .iter()
            .filter_map(|entry| {
                Self::fuzzy_score(query, &entry.label)
                    .or_else(|| Self::fuzzy_score(query, &entry.command))
                    .or_else(|| Self::fuzzy_score(query, &entry.id))
                    .map(|score| MenuMatch {
                        entry: entry.clone(),
                        score,
                    })
            })
            .collect();
        matches.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then_with(|| a.entry.label.cmp(&b.entry.label))
        });
        matches
    }

    /// Entries grouped by category for menu rendering
    pub fn categories(&self) -> Vec<(String, Vec<&MenuEntry>)> {
        let mut cats: Vec<(String, Vec<&MenuEntry>)> = Vec::new();
        for entry in &self.entries {
            if let Some(cat) = cats.iter_mut().find(|(c, _)| c == &entry.category) {
                cat.1.push(entry);
            } else {
                cats.push((entry.category.clone(), vec![entry]));
            }
        }
        cats
    }
}

// ============================================================================
// 5. OmarchyToggleEngine (omarchy-toggle-* parity)
// ============================================================================

/// Toggleable Omarchy desktop features
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OmarchyToggle {
    /// omarchy-toggle-theme / dark-light switch
    DarkLight,
    /// omarchy-toggle-animations
    Animations,
    /// omarchy-toggle-nightlight
    Nightlight,
    /// omarchy-toggle-input-device (mic mute)
    MicMute,
    /// omarchy-toggle-bluetooth
    Bluetooth,
    /// omarchy-toggle-wifi
    Wifi,
}

impl OmarchyToggle {
    pub fn cli_name(&self) -> &'static str {
        match self {
            Self::DarkLight => "omarchy-toggle-theme",
            Self::Animations => "omarchy-toggle-animations",
            Self::Nightlight => "omarchy-toggle-nightlight",
            Self::MicMute => "omarchy-toggle-input-device",
            Self::Bluetooth => "omarchy-toggle-bluetooth",
            Self::Wifi => "omarchy-toggle-wifi",
        }
    }
}

/// State of all Omarchy toggles plus the command each emits when flipped
#[derive(Debug, Clone, Default)]
pub struct OmarchyToggleEngine {
    /// toggle -> current on/off state
    pub states: BTreeMap<OmarchyToggle, bool>,
}

impl OmarchyToggleEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_on(&self, toggle: OmarchyToggle) -> bool {
        self.states.get(&toggle).copied().unwrap_or(false)
    }

    /// Flip a toggle and return the shell commands the real
    /// `omarchy-toggle-*` script would run.
    pub fn flip(&mut self, toggle: OmarchyToggle) -> Vec<String> {
        let now_on = !self.is_on(toggle);
        self.states.insert(toggle, now_on);
        match toggle {
            OmarchyToggle::DarkLight => vec![
                "omarchy-theme-set --toggle-mode".to_string(),
                "omarchy-refresh-shell".to_string(),
            ],
            OmarchyToggle::Animations => vec![format!(
                "hyprctl keyword general:animations {}",
                if now_on { "1" } else { "0" }
            )],
            OmarchyToggle::Nightlight => vec![format!(
                "omarchy-nightlight {}",
                if now_on { "on" } else { "off" }
            )],
            OmarchyToggle::MicMute => vec![format!(
                "wpctl set-mute @DEFAULT_AUDIO_SOURCE@ {}",
                if now_on { "0" } else { "1" }
            )],
            OmarchyToggle::Bluetooth => vec![format!(
                "bluetoothctl {}",
                if now_on { "power on" } else { "power off" }
            )],
            OmarchyToggle::Wifi => vec![format!(
                "nmcli radio wifi {}",
                if now_on { "on" } else { "off" }
            )],
        }
    }

    /// All registered toggles with their current state
    pub fn snapshot(&self) -> Vec<(OmarchyToggle, bool)> {
        [
            OmarchyToggle::DarkLight,
            OmarchyToggle::Animations,
            OmarchyToggle::Nightlight,
            OmarchyToggle::MicMute,
            OmarchyToggle::Bluetooth,
            OmarchyToggle::Wifi,
        ]
        .iter()
        .map(|t| (*t, self.is_on(*t)))
        .collect()
    }
}

// ============================================================================
// 6. OmarchyDiskSpeedTest (omarchy-disk-speed-test parity)
// ============================================================================

/// Result of a single disk benchmark pass
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiskBenchmarkResult {
    pub operation: &'static str, // "seq_read" | "seq_write" | "rand_read" | "rand_write"
    pub mib_per_sec: f64,
    pub iops: f64,
}

/// Full disk speed report (`omarchy disk speed test` output model)
#[derive(Debug, Clone, Default)]
pub struct DiskSpeedReport {
    pub device: String,
    pub results: Vec<DiskBenchmarkResult>,
}

impl DiskSpeedReport {
    /// Average sequential throughput across read+write, MiB/s
    pub fn avg_seq_mib(&self) -> f64 {
        let seq: Vec<f64> = self
            .results
            .iter()
            .filter(|r| r.operation.starts_with("seq"))
            .map(|r| r.mib_per_sec)
            .collect();
        if seq.is_empty() {
            0.0
        } else {
            seq.iter().sum::<f64>() / seq.len() as f64
        }
    }

    /// Plain-text rendering like omarchy's speed test summary
    pub fn render(&self) -> String {
        let mut out = format!("Device: {}\n", self.device);
        for r in &self.results {
            out.push_str(&format!(
                "{:<10} {:>8.1} MiB/s  {:>10.0} IOPS\n",
                r.operation, r.mib_per_sec, r.iops
            ));
        }
        out.push_str(&format!("avg seq: {:.1} MiB/s\n", self.avg_seq_mib()));
        out
    }
}

/// Models `omarchy-disk-speed-test`: runs the four standard fio-style
/// passes and produces a report. In production this shells out to fio;
/// here the measurement is injected so logic stays testable.
#[derive(Debug)]
pub struct OmarchyDiskSpeedTest {
    pub device: String,
    pub block_size_kib: u32,
    pub runtime_secs: u32,
}

impl OmarchyDiskSpeedTest {
    pub fn new(device: &str) -> Self {
        Self {
            device: device.to_string(),
            block_size_kib: 1024,
            runtime_secs: 10,
        }
    }

    /// Build the fio command lines the real script would run
    pub fn fio_commands(&self) -> Vec<String> {
        let bs = self.block_size_kib;
        let rt = self.runtime_secs;
        vec![
            format!("fio --name=seq_read --rw=read --bs={}k --runtime={} --direct=1 {}", bs, rt, self.device),
            format!("fio --name=seq_write --rw=write --bs={}k --runtime={} --direct=1 {}", bs, rt, self.device),
            format!("fio --name=rand_read --rw=randread --bs=4k --runtime={} --direct=1 --iodepth=32 {}", rt, self.device),
            format!("fio --name=rand_write --rw=randwrite --bs=4k --runtime={} --direct=1 --iodepth=32 {}", rt, self.device),
        ]
    }

    /// Assemble a report from measured pass results (injection point for
    /// real fio output parsing)
    pub fn report(&self, measured: Vec<DiskBenchmarkResult>) -> DiskSpeedReport {
        DiskSpeedReport {
            device: self.device.clone(),
            results: measured,
        }
    }
}

// ============================================================================
// 7. OmarchyDevEnvInstaller (omarchy-install-dev-env parity)
// ============================================================================

/// A development tool group installed by `omarchy install dev env`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevToolGroup {
    pub name: String,
    pub packages: Vec<String>,
}

/// Plans the `omarchy dev env` installation: base toolchain plus
/// language-specific groups, all via pacman/AUR.
#[derive(Debug)]
pub struct OmarchyDevEnvInstaller {
    pub groups: Vec<DevToolGroup>,
}

impl OmarchyDevEnvInstaller {
    pub fn new() -> Self {
        Self {
            groups: vec![
                DevToolGroup {
                    name: "base".to_string(),
                    packages: vec![
                        "git".to_string(),
                        "github-cli".to_string(),
                        "fd".to_string(),
                        "ripgrep".to_string(),
                        "jq".to_string(),
                        "fzf".to_string(),
                        "bat".to_string(),
                        "eza".to_string(),
                        "zoxide".to_string(),
                        "direnv".to_string(),
                        "mise".to_string(),
                    ],
                },
                DevToolGroup {
                    name: "rust".to_string(),
                    packages: vec![
                        "rust".to_string(),
                        "cargo".to_string(),
                        "rust-analyzer".to_string(),
                    ],
                },
                DevToolGroup {
                    name: "node".to_string(),
                    packages: vec![
                        "nodejs".to_string(),
                        "npm".to_string(),
                        "pnpm".to_string(),
                        "bun".to_string(),
                    ],
                },
                DevToolGroup {
                    name: "python".to_string(),
                    packages: vec![
                        "python".to_string(),
                        "python-pip".to_string(),
                        "uv".to_string(),
                    ],
                },
                DevToolGroup {
                    name: "go".to_string(),
                    packages: vec!["go".to_string()],
                },
            ],
        }
    }

    /// Group names available for selective install
    pub fn group_names(&self) -> Vec<&str> {
        self.groups.iter().map(|g| g.name.as_str()).collect()
    }

    /// Install plan for selected groups (empty selection = all groups).
    /// Returns ordered pacman/AUR commands.
    pub fn install_plan(&self, selected: &[&str]) -> Vec<String> {
        let mut cmds = Vec::new();
        for group in &self.groups {
            if !selected.is_empty() && !selected.contains(&group.name.as_str()) {
                continue;
            }
            let pkgs = group.packages.join(" ");
            cmds.push(format!("sudo pacman -S --needed --noconfirm {}", pkgs));
            // AUR-only helpers go through the AUR helper
            let aur_pkgs: Vec<&String> = group
                .packages
                .iter()
                .filter(|p| {
                    matches!(
                        p.as_str(),
                        "mise" | "eza" | "zoxide" | "bat" | "fzf" | "direnv" | "bun" | "uv"
                    )
                })
                .collect();
            if !aur_pkgs.is_empty() {
                cmds.push(format!(
                    "yay -S --noconfirm {}",
                    aur_pkgs
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
        }
        cmds
    }
}

impl Default for OmarchyDevEnvInstaller {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 8. OmarchyRefreshRegistry (omarchy-refresh-* parity)
// ============================================================================

/// A registered `omarchy-refresh-<target>` command
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshCommand {
    pub name: String,
    pub description: String,
    /// shell command the refresh script runs
    pub command: String,
    /// whether it needs a running graphical session
    pub needs_session: bool,
}

/// Registry of Omarchy refresh commands (`omarchy-refresh-*` family).
/// These re-apply generated configs without reinstalling anything.
#[derive(Debug)]
pub struct OmarchyRefreshRegistry {
    pub commands: Vec<RefreshCommand>,
}

impl OmarchyRefreshRegistry {
    pub fn new() -> Self {
        Self {
            commands: vec![
                RefreshCommand {
                    name: "omarchy-refresh-hyprland".to_string(),
                    description: "Reload Hyprland config and borders".to_string(),
                    command: "hyprctl reload".to_string(),
                    needs_session: true,
                },
                RefreshCommand {
                    name: "omarchy-refresh-shell".to_string(),
                    description: "Restart quickshell bar/overlays".to_string(),
                    command: "qs -c omarchy-shell kill; qs -c omarchy-shell".to_string(),
                    needs_session: true,
                },
                RefreshCommand {
                    name: "omarchy-refresh-pacman".to_string(),
                    description: "Re-rank mirrors and refresh pacman db".to_string(),
                    command: "reflector --latest 5 --sort rate | sudo tee /etc/pacman.d/mirrorlist && sudo pacman -Syy".to_string(),
                    needs_session: false,
                },
                RefreshCommand {
                    name: "omarchy-refresh-gtk".to_string(),
                    description: "Reload GTK theme".to_string(),
                    command: "gsettings set org.gnome.desktop.interface gtk-theme".to_string(),
                    needs_session: false,
                },
                RefreshCommand {
                    name: "omarchy-refresh-qt".to_string(),
                    description: "Reload Qt color scheme".to_string(),
                    command: "qt5ct".to_string(),
                    needs_session: false,
                },
                RefreshCommand {
                    name: "omarchy-refresh-terminal".to_string(),
                    description: "Reload terminal font/theme".to_string(),
                    command: "omarchy-launch-terminal --reload-config".to_string(),
                    needs_session: true,
                },
            ],
        }
    }

    /// Look up a refresh command by exact or prefix name
    pub fn find(&self, name: &str) -> Option<&RefreshCommand> {
        self.commands.iter().find(|c| {
            c.name == name
                || c.name
                    .ends_with(&format!("-{}", name.trim_start_matches("omarchy-refresh-")))
        })
    }

    /// Commands runnable without a graphical session
    pub fn headless_commands(&self) -> Vec<&RefreshCommand> {
        self.commands.iter().filter(|c| !c.needs_session).collect()
    }
}

impl Default for OmarchyRefreshRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 9. OmarchyPowerMenu (omarchy-menu power / omarchy-power-* parity)
// ============================================================================

/// System power actions offered by the Omarchy menu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    Shutdown,
    Reboot,
    Suspend,
    Hibernate,
    Logout,
    Lock,
}

impl PowerAction {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Shutdown => "Shut Down",
            Self::Reboot => "Restart",
            Self::Suspend => "Suspend",
            Self::Hibernate => "Hibernate",
            Self::Logout => "Log Out",
            Self::Lock => "Lock Screen",
        }
    }

    /// systemd/logind command the action maps to
    pub fn command(&self) -> &'static str {
        match self {
            Self::Shutdown => "systemctl poweroff",
            Self::Reboot => "systemctl reboot",
            Self::Suspend => "systemctl suspend",
            Self::Hibernate => "systemctl hibernate",
            Self::Logout => "loginctl terminate-session $XDG_SESSION_ID",
            Self::Lock => "omarchy-lock",
        }
    }

    /// Actions that need polkit authorization
    pub fn needs_auth(&self) -> bool {
        !matches!(self, Self::Lock | Self::Logout)
    }
}

/// Ordered power menu entries with optional confirmation gating
#[derive(Debug)]
pub struct OmarchyPowerMenu;

impl OmarchyPowerMenu {
    pub fn entries(&self) -> Vec<PowerAction> {
        vec![
            PowerAction::Lock,
            PowerAction::Suspend,
            PowerAction::Logout,
            PowerAction::Reboot,
            PowerAction::Shutdown,
            PowerAction::Hibernate,
        ]
    }

    /// Destructive actions that should show a confirm dialog first
    pub fn needs_confirmation(&self, action: PowerAction) -> bool {
        matches!(
            action,
            PowerAction::Shutdown | PowerAction::Reboot | PowerAction::Hibernate
        )
    }

    /// Execute an action: returns the command plus whether it ran through
    /// polkit. Caller decides whether to show a confirmation first.
    pub fn execute(&self, action: PowerAction, confirmed: bool) -> Option<String> {
        if self.needs_confirmation(action) && !confirmed {
            return None;
        }
        Some(action.command().to_string())
    }
}

impl Default for OmarchyPowerMenu {
    fn default() -> Self {
        Self
    }
}

// ============================================================================
// 10. OmarchyAppLauncher (omarchy-launch-* parity)
// ============================================================================

/// A launchable application entry (`omarchy-launch-*` / walker entries)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchableApp {
    pub id: String,
    pub name: String,
    /// desktop file id or binary name
    pub exec: String,
    /// icon name for menu/launcher display
    pub icon: String,
    /// terminal apps need a terminal wrapper
    pub terminal: bool,
}

/// Registry of Omarchy's curated default applications and their launch
/// wrappers (`omarchy-launch-browser`, `omarchy-launch-terminal`, etc.)
#[derive(Debug)]
pub struct OmarchyAppLauncher {
    pub apps: Vec<LaunchableApp>,
}

impl OmarchyAppLauncher {
    pub fn new() -> Self {
        Self {
            apps: vec![
                LaunchableApp {
                    id: "browser".to_string(),
                    name: "Browser".to_string(),
                    exec: "omarchy-launch-browser".to_string(),
                    icon: "web-browser".to_string(),
                    terminal: false,
                },
                LaunchableApp {
                    id: "terminal".to_string(),
                    name: "Terminal".to_string(),
                    exec: "ghostty".to_string(),
                    icon: "utilities-terminal".to_string(),
                    terminal: true,
                },
                LaunchableApp {
                    id: "files".to_string(),
                    name: "Files".to_string(),
                    exec: "nautilus".to_string(),
                    icon: "system-file-manager".to_string(),
                    terminal: false,
                },
                LaunchableApp {
                    id: "editor".to_string(),
                    name: "Editor".to_string(),
                    exec: "omarchy-launch-editor".to_string(),
                    icon: "accessories-text-editor".to_string(),
                    terminal: true,
                },
                LaunchableApp {
                    id: "music".to_string(),
                    name: "Music".to_string(),
                    exec: "spotify".to_string(),
                    icon: "multimedia-audio-player".to_string(),
                    terminal: false,
                },
                LaunchableApp {
                    id: "photos".to_string(),
                    name: "Photos".to_string(),
                    exec: "omarchy-launch-photos".to_string(),
                    icon: "multimedia-photo-viewer".to_string(),
                    terminal: false,
                },
            ],
        }
    }

    /// Find an app by id or name (case-insensitive)
    pub fn find(&self, query: &str) -> Option<&LaunchableApp> {
        let q = query.to_lowercase();
        self.apps.iter().find(|a| {
            a.id.to_lowercase() == q || a.name.to_lowercase() == q || a.exec.to_lowercase() == q
        })
    }

    /// Build the shell command to launch an app, wrapping terminal apps
    /// in the configured terminal like the real omarchy-launch-* scripts
    pub fn launch_command(&self, app_id: &str, args: &[&str]) -> Option<String> {
        let app = self.find(app_id)?;
        let mut cmd = if app.terminal {
            format!("{} -e {}", self.terminal_command(), app.exec)
        } else {
            app.exec.clone()
        };
        for arg in args {
            cmd.push(' ');
            cmd.push_str(arg);
        }
        Some(cmd)
    }

    fn terminal_command(&self) -> String {
        "ghostty".to_string()
    }
}

impl Default for OmarchyAppLauncher {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 11. OmarchyMigrationHelper (omarchy-migrate-* parity)
// ============================================================================

/// Source distro a user is migrating from
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationSource {
    MacOS,
    Windows,
    Ubuntu,
    Fedora,
    Other,
}

/// Post-install migration checklist generated by `omarchy-migrate-*`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationStep {
    pub description: String,
    pub command: String,
    /// steps marked optional are suggested but not required
    pub optional: bool,
}

/// Generates migration steps for bringing settings/files over from a
/// previous OS install (`omarchy-migrate-from-*` helpers).
#[derive(Debug)]
pub struct OmarchyMigrationHelper;

impl OmarchyMigrationHelper {
    pub fn new() -> Self {
        Self
    }

    /// Steps to migrate user config from another Linux distro
    pub fn linux_steps(&self) -> Vec<MigrationStep> {
        vec![
            MigrationStep {
                description: "Copy dotfiles and shell config".to_string(),
                command: "rsync -av /mnt/oldhome/$USER/{.bashrc,.zshrc,.config} ~/".to_string(),
                optional: false,
            },
            MigrationStep {
                description: "Import GNOME/other DE settings via omarchy-theme-set".to_string(),
                command: "omarchy-theme-set --import ~/.config/omarchy/themes".to_string(),
                optional: true,
            },
            MigrationStep {
                description: "Reinstall AUR packages from old pacman log".to_string(),
                command: "grep 'installed ' /mnt/oldvar/log/pacman.log | awk '{print $4}' | yay -S --needed".to_string(),
                optional: true,
            },
            MigrationStep {
                description: "Mount old home and verify data".to_string(),
                command: "omarchy-mount-old /mnt/oldhome && ls /mnt/oldhome/$USER".to_string(),
                optional: false,
            },
        ]
    }

    /// Steps to migrate from macOS (files, browser data, keys)
    pub fn macos_steps(&self) -> Vec<MigrationStep> {
        vec![
            MigrationStep {
                description: "Copy SSH keys and GPG keys".to_string(),
                command:
                    "mkdir -p ~/.ssh && cp /Volumes/MacHome/.ssh/* ~/.ssh/ && chmod 700 ~/.ssh"
                        .to_string(),
                optional: false,
            },
            MigrationStep {
                description: "Import macOS fonts".to_string(),
                command:
                    "cp /Volumes/MacHome/Library/Fonts/*.ttf ~/.local/share/fonts/ && fc-cache -fv"
                        .to_string(),
                optional: true,
            },
            MigrationStep {
                description: "Copy documents and desktop files".to_string(),
                command: "rsync -av /Volumes/MacHome/{Documents,Desktop,Pictures} ~/".to_string(),
                optional: false,
            },
        ]
    }

    /// Steps to migrate from Windows (NTFS mount + WSL data)
    pub fn windows_steps(&self) -> Vec<MigrationStep> {
        vec![
            MigrationStep {
                description: "Mount Windows partition read-write via ntfs3".to_string(),
                command: "sudo mount -t ntfs3 /dev/nvme0n1p3 /mnt/windows".to_string(),
                optional: false,
            },
            MigrationStep {
                description: "Copy browser profiles from Windows".to_string(),
                command: "rsync -av /mnt/windows/Users/*/AppData/Local/Google/Chrome/User\\ Data/Default ~/".to_string(),
                optional: true,
            },
            MigrationStep {
                description: "Copy Documents/Desktop from Windows profile".to_string(),
                command: "rsync -av /mnt/windows/Users/*/Documents ~/".to_string(),
                optional: false,
            },
        ]
    }

    /// Pick the right step list for a migration source
    pub fn steps_for(&self, source: MigrationSource) -> Vec<MigrationStep> {
        match source {
            MigrationSource::MacOS => self.macos_steps(),
            MigrationSource::Windows => self.windows_steps(),
            MigrationSource::Ubuntu | MigrationSource::Fedora | MigrationSource::Other => {
                self.linux_steps()
            }
        }
    }
}

impl Default for OmarchyMigrationHelper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_file_roundtrip() {
        let content = "mode=dark\naccent=#7aa2f7\nbackground=#1a1b26\nforeground=#c0caf5\ncolor0=#15161e\ncolor15=#b4befe\nwallpaper=/tmp/wall.png\ncustom_key=42\n";
        let theme = OmarchyThemeFile::parse("catppuccin", content);
        assert_eq!(theme.mode, "dark");
        assert_eq!(theme.accent, "#7aa2f7");
        assert_eq!(theme.palette.get(&0), Some(&"#15161e".to_string()));
        assert_eq!(theme.palette.get(&15), Some(&"#b4befe".to_string()));
        assert_eq!(theme.wallpaper, "/tmp/wall.png");
        assert_eq!(theme.extras.get("custom_key"), Some(&"42".to_string()));

        let serialized = theme.serialize();
        let reparsed = OmarchyThemeFile::parse("catppuccin", &serialized);
        assert_eq!(reparsed.accent, theme.accent);
        assert_eq!(reparsed.palette.len(), theme.palette.len());
    }

    #[test]
    fn test_theme_set_plan() {
        let engine = OmarchyThemeSetEngine::new("/home/u/.config");
        let theme = OmarchyThemeFile::parse(
            "nord",
            "mode=dark\naccent=#88c0d0\nbackground=#2e3440\nforeground=#d8dee9\ncolor4=#81a1c1\n",
        );
        let plan = engine.plan(&theme);
        assert_eq!(plan.theme_name, "nord");
        assert!(plan
            .writes
            .iter()
            .any(|w| w.path.contains("gtk-3.0/gtk.css")));
        assert!(plan
            .writes
            .iter()
            .any(|w| w.path.contains("hypr/colors.conf")));
        assert!(plan
            .writes
            .iter()
            .any(|w| w.path.contains("ghostty/config")));
        assert!(plan
            .writes
            .iter()
            .any(|w| w.path.contains("quickshell/theme.qml")));
        assert!(plan.refresh_commands.iter().any(|c| c.contains("hyprland")));
        let ghostty = plan
            .writes
            .iter()
            .find(|w| w.path.contains("ghostty"))
            .unwrap();
        assert!(ghostty.content.contains("background = 2e3440"));
        assert!(ghostty.content.contains("palette = 4=81a1c1"));
    }

    #[test]
    fn test_omarchy_update_flow() {
        let orch = OmarchyUpdateOrchestrator::new();
        let result = orch.execute(None);
        assert!(result.success);
        assert!(result.reboot_required);
        assert!(result
            .commands_run
            .iter()
            .any(|c| c.contains("pacman -Syu")));
        assert!(result.commands_run.iter().any(|c| c.contains("yay")));
        assert!(result.commands_run.iter().any(|c| c.contains("paccache")));

        let failed = orch.execute(Some(UpdateStage::SyncRepos));
        assert!(!failed.success);
        assert_eq!(failed.failed_stage, Some(UpdateStage::SyncRepos));
        // best-effort cache clean still recorded
        assert!(failed.commands_run.iter().any(|c| c.contains("paccache")));
    }

    #[test]
    fn test_menu_fuzzy_search() {
        let mut menu = OmarchyMenu::new();
        menu.add(MenuEntry {
            id: "browser".to_string(),
            label: "Firefox".to_string(),
            command: "firefox".to_string(),
            category: "Apps".to_string(),
        });
        menu.add(MenuEntry {
            id: "files".to_string(),
            label: "File Manager".to_string(),
            command: "nautilus".to_string(),
            category: "Apps".to_string(),
        });
        menu.add(MenuEntry {
            id: "term".to_string(),
            label: "Terminal".to_string(),
            command: "ghostty".to_string(),
            category: "Apps".to_string(),
        });

        let res = menu.search("ff");
        assert_eq!(res[0].entry.id, "browser");

        let res2 = menu.search("term");
        assert_eq!(res2[0].entry.id, "term");

        // subsequence match: "flm" -> "File Manager"
        let res3 = menu.search("flm");
        assert_eq!(res3[0].entry.id, "files");

        assert!(menu.search("zzzz").is_empty());

        let cats = menu.categories();
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0].1.len(), 3);
    }

    #[test]
    fn test_toggle_engine() {
        let mut toggles = OmarchyToggleEngine::new();
        assert!(!toggles.is_on(OmarchyToggle::Animations));

        let cmds = toggles.flip(OmarchyToggle::Animations);
        assert!(toggles.is_on(OmarchyToggle::Animations));
        assert_eq!(cmds, vec!["hyprctl keyword general:animations 1"]);

        let cmds2 = toggles.flip(OmarchyToggle::DarkLight);
        assert!(cmds2.iter().any(|c| c.contains("theme-set")));

        let cmds3 = toggles.flip(OmarchyToggle::Nightlight);
        assert!(cmds3.iter().any(|c| c.contains("nightlight on")));

        toggles.flip(OmarchyToggle::MicMute);
        assert!(toggles.is_on(OmarchyToggle::MicMute));

        let snap = toggles.snapshot();
        assert_eq!(snap.len(), 6);
        for toggle in [
            OmarchyToggle::DarkLight,
            OmarchyToggle::Animations,
            OmarchyToggle::Nightlight,
            OmarchyToggle::MicMute,
        ] {
            assert!(snap.iter().any(|(t, on)| *t == toggle && *on));
        }
        assert!(snap.iter().any(|(t, on)| *t == OmarchyToggle::Wifi && !*on));
    }

    #[test]
    fn test_disk_speed_test() {
        let test = OmarchyDiskSpeedTest::new("/dev/nvme0n1");
        let cmds = test.fio_commands();
        assert_eq!(cmds.len(), 4);
        assert!(cmds[0].contains("seq_read"));
        assert!(cmds[3].contains("randwrite"));

        let report = test.report(vec![
            DiskBenchmarkResult {
                operation: "seq_read",
                mib_per_sec: 3500.0,
                iops: 900_000.0,
            },
            DiskBenchmarkResult {
                operation: "seq_write",
                mib_per_sec: 3000.0,
                iops: 800_000.0,
            },
            DiskBenchmarkResult {
                operation: "rand_read",
                mib_per_sec: 80.0,
                iops: 20_000.0,
            },
            DiskBenchmarkResult {
                operation: "rand_write",
                mib_per_sec: 60.0,
                iops: 15_000.0,
            },
        ]);
        assert_eq!(report.avg_seq_mib(), 3250.0);
        let rendered = report.render();
        assert!(rendered.contains("3500.0 MiB/s"));
        assert!(rendered.contains("avg seq: 3250.0"));
    }

    #[test]
    fn test_dev_env_installer() {
        let installer = OmarchyDevEnvInstaller::new();
        assert!(installer.group_names().contains(&"rust"));

        let all = installer.install_plan(&[]);
        assert!(all.iter().any(|c| c.contains("pacman -S")));
        assert!(all.iter().any(|c| c.contains("yay -S")));

        let only_rust = installer.install_plan(&["rust"]);
        assert_eq!(only_rust.len(), 1);
        assert!(only_rust[0].contains("rust-analyzer"));
    }

    #[test]
    fn test_refresh_registry() {
        let reg = OmarchyRefreshRegistry::new();
        let hypr = reg.find("omarchy-refresh-hyprland").unwrap();
        assert_eq!(hypr.command, "hyprctl reload");
        assert!(hypr.needs_session);

        let pacman = reg.find("pacman").unwrap();
        assert!(!pacman.needs_session);
        assert!(pacman.command.contains("mirrorlist"));

        let headless = reg.headless_commands();
        assert_eq!(headless.len(), 3);
        assert!(reg.find("nonexistent").is_none());
    }

    #[test]
    fn test_power_menu() {
        let menu = OmarchyPowerMenu;
        let entries = menu.entries();
        assert_eq!(entries.len(), 6);
        assert_eq!(entries[0], PowerAction::Lock);

        // destructive actions require confirmation
        assert!(menu.execute(PowerAction::Shutdown, false).is_none());
        assert!(menu
            .execute(PowerAction::Shutdown, true)
            .unwrap()
            .contains("poweroff"));
        // lock needs no confirmation
        assert!(menu.execute(PowerAction::Lock, false).is_some());
        assert!(PowerAction::Reboot.needs_auth());
        assert!(!PowerAction::Lock.needs_auth());
    }

    #[test]
    fn test_app_launcher() {
        let launcher = OmarchyAppLauncher::new();
        assert!(launcher.find("browser").is_some());
        assert!(launcher.find("Terminal").is_some());
        assert!(launcher.find("nonexistent").is_none());

        // terminal apps get wrapped in the terminal
        let term_cmd = launcher.launch_command("terminal", &[]).unwrap();
        assert!(term_cmd.contains("ghostty"));
        // GUI apps launch directly
        let gui_cmd = launcher.launch_command("files", &["~/Documents"]).unwrap();
        assert_eq!(gui_cmd, "nautilus ~/Documents");
    }

    #[test]
    fn test_migration_helper() {
        let helper = OmarchyMigrationHelper::new();
        let mac = helper.steps_for(MigrationSource::MacOS);
        assert!(mac.iter().any(|s| s.command.contains(".ssh")));
        let win = helper.steps_for(MigrationSource::Windows);
        assert!(win.iter().any(|s| s.command.contains("ntfs3")));
        let linux = helper.steps_for(MigrationSource::Ubuntu);
        assert!(linux.iter().any(|s| s.command.contains("pacman.log")));
        // every step has a command
        assert!(mac
            .iter()
            .chain(win.iter())
            .chain(linux.iter())
            .all(|s| !s.command.is_empty()));
    }
}
