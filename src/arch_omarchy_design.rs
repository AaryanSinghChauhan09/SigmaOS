// SPDX-License-Identifier: MIT
// SigmaOS Arch + Omarchy Consolidated Design-Philosophy Module
// (`src/arch_omarchy_design.rs`)
//
// Single in-code source of truth for the design philosophy SigmaOS inherits
// from Arch Linux (https://archlinux.org) and Omarchy
// (https://github.com/basepi/omarchy). The umbrella crate already absorbs
// these ideas across `src/arch_kernel_inspirations.rs`,
// `src/distro/omarchy_inspiration.rs`,
// `src/package/{alpm_engine,arch_aur,aur_integration}.rs`, and
// `pkgbuild/*.PKGBUILD`; this module consolidates that scattered code into one
// declarative entry point the wiki can cite instead of duplicating it.
//
//   - The Arch Way + Omarchy principles -> `ArchOmarchyDesignPhilosophy`
//   - over-engineering review list       -> `KissSimplificationChecklist`
//   - curated opinionated desktop set    -> `CuratedDefaultsManifest`
//   - upstream -> SigmaOS mapping         -> `InspirationMap`

use std::collections::{BTreeMap, BTreeSet};
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. ARCH + OMARCHY DESIGN PHILOSOPHY -> ArchOmarchyDesignPhilosophy
//    A catalog of the design principles SigmaOS absorbs: The Arch Way (KISS,
//    user-centricity, code-correctness over features, rolling release, minimal
//    base, user choice) and Omarchy (curated opinionated defaults,
//    batteries-included but simple).
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhilosophySource {
    /// Drawn from "The Arch Way".
    ArchWay,
    /// Drawn from Omarchy's opinionated desktop defaults.
    Omarchy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignPrinciple {
    pub name: String,
    pub source: PhilosophySource,
    pub summary: String,
}

/// Consolidated catalog of the Arch Way and Omarchy design principles that
/// guide SigmaOS. The single in-code source of truth the wiki cites for "why
/// SigmaOS is built this way".
pub struct ArchOmarchyDesignPhilosophy {
    pub principles: Vec<DesignPrinciple>,
}

impl ArchOmarchyDesignPhilosophy {
    pub fn new() -> Self {
        Self {
            principles: Vec::new(),
        }
    }

    /// Seed the catalog with the canonical Arch Way + Omarchy principles.
    pub fn canonical() -> Self {
        let mut p = Self::new();
        // --- The Arch Way ---
        p.add(
            "KISS",
            PhilosophySource::ArchWay,
            "Keep It Simple, Stupid: prefer the simplest design that solves the problem over a clever or feature-rich one.",
        );
        p.add(
            "User-centricity",
            PhilosophySource::ArchWay,
            "The system serves the user; configuration and control stay in the user's hands, not the vendor's.",
        );
        p.add(
            "Code-correctness over features",
            PhilosophySource::ArchWay,
            "Ship correct, elegant code over more features; correctness and simplicity outrank surface area.",
        );
        p.add(
            "Rolling release",
            PhilosophySource::ArchWay,
            "One continuously updated stream: no reinstall generations, just current packages.",
        );
        p.add(
            "Minimal base system",
            PhilosophySource::ArchWay,
            "A small, clean base install the user builds on; nothing mandatory beyond what the system needs to boot.",
        );
        p.add(
            "User choice / no forced defaults",
            PhilosophySource::ArchWay,
            "Avoid locking users into one option; defaults are a starting point, not a prescription.",
        );
        // --- Omarchy ---
        p.add(
            "Curated opinionated defaults",
            PhilosophySource::Omarchy,
            "Pre-select a coherent, well-tested default set so a fresh install is immediately usable without manual tuning.",
        );
        p.add(
            "Batteries-included but simple",
            PhilosophySource::Omarchy,
            "Ship a complete, working desktop out of the box while keeping the surface area small and comprehensible.",
        );
        p
    }

    pub fn add(&mut self, name: &str, source: PhilosophySource, summary: &str) {
        self.principles.push(DesignPrinciple {
            name: name.to_string(),
            source,
            summary: summary.to_string(),
        });
    }

    pub fn arch_way_count(&self) -> usize {
        self.principles
            .iter()
            .filter(|p| p.source == PhilosophySource::ArchWay)
            .count()
    }

    pub fn omarchy_count(&self) -> usize {
        self.principles
            .iter()
            .filter(|p| p.source == PhilosophySource::Omarchy)
            .count()
    }

    pub fn find(&self, name: &str) -> Option<&DesignPrinciple> {
        self.principles.iter().find(|p| p.name == name)
    }

    pub fn render_summary(&self) -> String {
        format!(
            "ArchOmarchyDesignPhilosophy: {} principles ({} Arch Way, {} Omarchy)",
            self.principles.len(),
            self.arch_way_count(),
            self.omarchy_count()
        )
    }
}

impl Default for ArchOmarchyDesignPhilosophy {
    fn default() -> Self {
        Self::canonical()
    }
}

// =========================================================================
// 2. KISS SIMPLIFICATION CHECKLIST -> KissSimplificationChecklist
//    A reviewable manifest of over-engineering / duplication patterns to
//    simplify, kept in code so the KISS principle is applied to SigmaOS
//    itself. The first entry cites the four duplicated file-manager modules
//    under `src/desktop/`.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KissStatus {
    /// Not yet started.
    Open,
    /// Consolidation in progress.
    Consolidating,
    /// Resolved / consolidated.
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KissSimplificationEntry {
    pub pattern: String,
    pub affected_files: Vec<String>,
    pub suggested_simplification: String,
    pub status: KissStatus,
}

/// Reviewable manifest of over-engineering patterns to simplify.
pub struct KissSimplificationChecklist {
    pub entries: Vec<KissSimplificationEntry>,
}

impl KissSimplificationChecklist {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Seed the manifest with the canonical simplification targets.
    pub fn canonical() -> Self {
        let mut c = Self::new();
        // First entry: the four duplicated file-manager modules.
        c.add(
            "Duplicated file-manager modules",
            vec![
                "src/desktop/file_manager.rs".to_string(),
                "src/desktop/filemanager.rs".to_string(),
                "src/desktop/desktop_file_manager.rs".to_string(),
                "src/desktop/file_manager_extensions.rs".to_string(),
            ],
            "Consolidate into a single file-manager implementation; have the others re-export or delegate to it.",
            KissStatus::Open,
        );
        // Scattered Arch/Omarchy inspiration code (this module is the entry point).
        c.add(
            "Scattered Arch/Omarchy inspiration code",
            vec![
                "src/arch_kernel_inspirations.rs".to_string(),
                "src/distro/omarchy_inspiration.rs".to_string(),
                "src/package/alpm_engine.rs".to_string(),
                "src/package/arch_aur.rs".to_string(),
                "src/package/aur_integration.rs".to_string(),
                "pkgbuild/*.PKGBUILD".to_string(),
            ],
            "Use arch_omarchy_design as the single declarative entry point and reference (not duplicate) the absorbed implementations.",
            KissStatus::Consolidating,
        );
        // Generic YAGNI guard for future module audits.
        c.add(
            "Parallel implementations of the same concern",
            vec!["(review during module audits)".to_string()],
            "Prefer extending the existing implementation over spawning a parallel one (YAGNI).",
            KissStatus::Open,
        );
        c
    }

    pub fn add(
        &mut self,
        pattern: &str,
        affected_files: Vec<String>,
        suggested_simplification: &str,
        status: KissStatus,
    ) {
        self.entries.push(KissSimplificationEntry {
            pattern: pattern.to_string(),
            affected_files,
            suggested_simplification: suggested_simplification.to_string(),
            status,
        });
    }

    pub fn open_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.status == KissStatus::Open)
            .count()
    }

    pub fn find(&self, pattern: &str) -> Option<&KissSimplificationEntry> {
        self.entries.iter().find(|e| e.pattern == pattern)
    }
}

impl Default for KissSimplificationChecklist {
    fn default() -> Self {
        Self::canonical()
    }
}

// =========================================================================
// 3. CURATED DESKTOP DEFAULTS -> CuratedDefaultsManifest
//    Declarative opinionated desktop defaults mirroring the desktop-facing
//    portion of the `depends=(...)` list in `pkgbuild/sigmaos.PKGBUILD`, so the
//    curated set has an in-code source of truth alongside the PKGBUILD.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CuratedDefaultsEntry {
    pub role: String,
    pub package: String,
    pub category: String,
}

/// Opinionated desktop defaults mirrored from `pkgbuild/sigmaos.PKGBUILD`'s
/// `depends=(...)` list.
pub struct CuratedDefaultsManifest {
    pub entries: Vec<CuratedDefaultsEntry>,
}

impl CuratedDefaultsManifest {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Seed the manifest with the curated desktop defaults from the PKGBUILD.
    pub fn canonical() -> Self {
        let mut m = Self::new();
        // Wayland compositor stack
        m.add("compositor", "hyprland", "wayland");
        m.add("screen-lock", "hyprlock", "wayland");
        m.add("idle-daemon", "hypridle", "wayland");
        m.add("screen-lock-alt", "swaylock", "wayland");
        m.add("display-manager", "sddm", "login");
        m.add("bootsplash", "plymouth", "boot");
        m.add("wayland", "wayland", "wayland");
        m.add("wayland-utils", "wayland-utils", "wayland");
        m.add("xwayland", "xorg-server-xwayland", "wayland");
        m.add("session", "uwsm", "wayland");
        m.add("seat", "seatd", "wayland");
        // Terminals
        m.add("terminal", "foot", "terminal");
        m.add("terminal-alt", "ghostty", "terminal");
        // Launcher & shell
        m.add("launcher", "wofi", "launcher");
        m.add("status-bar", "waybar", "shell");
        m.add("notifications", "mako", "shell");
        // Audio
        m.add("audio-server", "pipewire", "audio");
        m.add("audio-pulse", "pipewire-pulse", "audio");
        m.add("audio-alsa", "pipewire-alsa", "audio");
        m.add("audio-manager", "wireplumber", "audio");
        m.add("alsa-utils", "alsa-utils", "audio");
        m.add("volume-cli", "pamixer", "audio");
        m.add("volume-ui", "pavucontrol", "audio");
        // Input
        m.add("input", "libinput", "input");
        m.add("xdo", "xdotool", "input");
        m.add("ydo", "ydotool", "input");
        // Capture & clipboard
        m.add("screenshot", "grim", "capture");
        m.add("region-select", "slurp", "capture");
        m.add("screen-record", "wf-recorder", "capture");
        m.add("clipboard", "wl-clipboard", "capture");
        // File manager
        m.add("file-manager", "nautilus", "files");
        m.add("vfs", "gvfs", "files");
        m.add("vfs-mtp", "gvfs-mtp", "files");
        m.add("vfs-nfs", "gvfs-nfs", "files");
        // Browsers
        m.add("browser", "chromium", "web");
        m.add("browser-alt", "firefox", "web");
        // Editors
        m.add("editor", "micro", "editor");
        m.add("editor-alt", "helix", "editor");
        // Telemetry & power
        m.add("sysinfo", "fastfetch", "telemetry");
        m.add("process-monitor", "btop", "telemetry");
        m.add("brightness", "brightnessctl", "telemetry");
        m.add("power", "power-profiles-daemon", "telemetry");
        // CLI utilities
        m.add("prompt-tui", "gum", "cli");
        m.add("fuzzy-finder", "fzf", "cli");
        m.add("vcs", "git", "cli");
        m.add("http", "curl", "cli");
        m.add("http-alt", "wget", "cli");
        m.add("archive-unzip", "unzip", "cli");
        m.add("archive-zip", "zip", "cli");
        m.add("archive-tar", "tar", "cli");
        m.add("sync", "rsync", "cli");
        // Shells
        m.add("shell", "bash", "shell-runtime");
        m.add("shell-completion", "bash-completion", "shell-runtime");
        m.add("shell-alt", "zsh", "shell-runtime");
        // Fonts
        m.add("font", "noto-fonts", "fonts");
        m.add("font-emoji", "noto-fonts-emoji", "fonts");
        m.add("font-mono", "ttf-jetbrains-mono-nerd", "fonts");
        // Toolkits
        m.add("toolkit-gtk4", "gtk4", "toolkits");
        m.add("toolkit-gtk3", "gtk3", "toolkits");
        m.add("toolkit-qt5", "qt5-wayland", "toolkits");
        m.add("toolkit-qt6", "qt6-wayland", "toolkits");
        // Icons
        m.add("icons", "adwaita-icon-theme", "icons");
        m.add("icons-alt", "papirus-icon-theme", "icons");
        m.add("cursors", "cursor-themes", "icons");
        // Networking
        m.add("network", "networkmanager", "network");
        m.add("network-applet", "network-manager-applet", "network");
        // Bluetooth
        m.add("bluetooth", "bluez", "bluetooth");
        m.add("bluetooth-utils", "bluez-utils", "bluetooth");
        m.add("bluetooth-ui", "blueman", "bluetooth");
        // Printing
        m.add("printing", "cups", "printing");
        m.add("printing-helper", "cups-pk-helper", "printing");
        // Security / polkit
        m.add("polkit-agent", "polkit-kde-agent", "security");
        m.add("keyring", "gnome-keyring", "security");
        m.add("tpm-tools", "tpm2-tools", "security");
        m.add("tpm-stack", "tpm2-tss", "security");
        // Accessibility
        m.add("a11y-speakup", "speakup", "accessibility");
        m.add("a11y-espeakup", "espeakup", "accessibility");
        m.add("a11y-espeak", "espeak-ng", "accessibility");
        m
    }

    pub fn add(&mut self, role: &str, package: &str, category: &str) {
        self.entries.push(CuratedDefaultsEntry {
            role: role.to_string(),
            package: package.to_string(),
            category: category.to_string(),
        });
    }

    /// All curated package names, deduplicated and sorted.
    pub fn packages(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for e in &self.entries {
            set.insert(e.package.clone());
        }
        set.into_iter().collect()
    }

    /// Group entries by category.
    pub fn grouped_by_category(&self) -> BTreeMap<String, Vec<&CuratedDefaultsEntry>> {
        let mut map: BTreeMap<String, Vec<&CuratedDefaultsEntry>> = BTreeMap::new();
        for e in &self.entries {
            map.entry(e.category.clone()).or_default().push(e);
        }
        map
    }

    pub fn has_package(&self, package: &str) -> bool {
        self.entries.iter().any(|e| e.package == package)
    }

    pub fn render_summary(&self) -> String {
        format!(
            "CuratedDefaultsManifest: {} curated packages across {} categories",
            self.packages().len(),
            self.grouped_by_category().len()
        )
    }
}

impl Default for CuratedDefaultsManifest {
    fn default() -> Self {
        Self::canonical()
    }
}

// =========================================================================
// 4. INSPIRATION MAP -> InspirationMap
//    The exact upstream -> SigmaOS mapping this consolidation applies. Each
//    entry holds the upstream name, the target SigmaOS component, and a
//    one-line rationale. Multi-target upstreams are split into one entry per
//    target component so "contains every named target component" is a direct
//    containment check.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspirationMapEntry {
    pub upstream_name: String,
    pub target_component: String,
    pub rationale: String,
}

/// Mapping of Arch/Omarchy upstreams to the SigmaOS components that absorb
/// them.
pub struct InspirationMap {
    pub entries: Vec<InspirationMapEntry>,
}

impl InspirationMap {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Seed the map with the exact mapping this consolidation applies.
    pub fn canonical() -> Self {
        let mut m = Self::new();
        // pacman -> packages kit + tools
        m.add(
            "pacman",
            "SigmaOS-packages-kit",
            "Pacman's transactional package-manager model absorbed into the packages-kit CLI.",
        );
        m.add(
            "pacman",
            "SigmaOS-packages-tools",
            "Pacman's repo/AUR tooling absorbed into the packages-tools CLI.",
        );
        // AUR -> plugin marketplace + registry
        m.add(
            "AUR",
            "SigmaOS-plugin-marketplace",
            "The Arch User Repository's community-contribution model mapped to a decentralized plugin marketplace.",
        );
        m.add(
            "AUR",
            "SigmaOS-plugin-registry",
            "AUR's metadata/indexing absorbed into the plugin registry.",
        );
        // PKGBUILD/makepkg -> ISO
        m.add(
            "PKGBUILD/makepkg",
            "SigmaOS-ISO",
            "Reproducible PKGBUILD/makepkg build definitions mirrored into the ISO image builder.",
        );
        // archiso -> ISO
        m.add(
            "archiso",
            "SigmaOS-ISO",
            "archiso's live-image generation pipeline reimagined as the SigmaOS ISO builder.",
        );
        // Hyprland -> compositor + window-manager + quickshell
        m.add(
            "Hyprland",
            "SigmaOS-compositor",
            "Hyprland's dynamic Wayland compositor split off as the SigmaOS-compositor.",
        );
        m.add(
            "Hyprland",
            "SigmaOS-window-manager",
            "Hyprland's tiling/window management absorbed into SigmaOS-window-manager.",
        );
        m.add(
            "Hyprland",
            "SigmaOS-quickshell",
            "Hyprland's shell integration mapped to the SigmaOS-quickshell unified shell.",
        );
        // omadots -> dots-configuration
        m.add(
            "omadots",
            "SigmaOS-dots-configuration",
            "Omarchy's opinionated dotfile bundle mapped to the sovereign dots-configuration module.",
        );
        // omafind -> file-manager + file-manager-system
        m.add(
            "omafind",
            "SigmaOS-file-manager",
            "Omarchy's omafind file finder absorbed into the desktop file manager.",
        );
        m.add(
            "omafind",
            "SigmaOS-file-manager-system",
            "omafind's backend search absorbed into the file-manager-system backend.",
        );
        // omareel -> reel + screen-recorder
        m.add(
            "omareel",
            "SigmaOS-reel",
            "Omarchy's omareel screen capture absorbed into SigmaOS-reel.",
        );
        m.add(
            "omareel",
            "SigmaOS-screen-recorder",
            "omareel's recording pipeline mapped to the SigmaOS-screen-recorder component.",
        );
        // omarchy-mac -> mac-run
        m.add(
            "omarchy-mac",
            "SigmaOS-mac-run",
            "Omarchy's macOS-look preset mapped to the mac-run compatibility shim.",
        );
        // Arch Wiki -> docs + wiki
        m.add(
            "Arch Wiki",
            "SigmaOS-docs",
            "Arch Wiki's documentation culture mirrored into the SigmaOS-docs source set.",
        );
        m.add(
            "Arch Wiki",
            "SigmaOS-wiki",
            "Arch Wiki's published-knowledge model mirrored into the SigmaOS published wiki.",
        );
        m
    }

    pub fn add(&mut self, upstream_name: &str, target_component: &str, rationale: &str) {
        self.entries.push(InspirationMapEntry {
            upstream_name: upstream_name.to_string(),
            target_component: target_component.to_string(),
            rationale: rationale.to_string(),
        });
    }

    /// All distinct target SigmaOS components covered by the map.
    pub fn target_components(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for e in &self.entries {
            set.insert(e.target_component.clone());
        }
        set.into_iter().collect()
    }

    /// All distinct upstream names covered by the map.
    pub fn upstream_names(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for e in &self.entries {
            set.insert(e.upstream_name.clone());
        }
        set.into_iter().collect()
    }

    pub fn has_target(&self, component: &str) -> bool {
        self.entries.iter().any(|e| e.target_component == component)
    }

    pub fn has_upstream(&self, upstream: &str) -> bool {
        self.entries.iter().any(|e| e.upstream_name == upstream)
    }

    pub fn render_summary(&self) -> String {
        format!(
            "InspirationMap: {} upstreams -> {} SigmaOS components",
            self.upstream_names().len(),
            self.target_components().len()
        )
    }
}

impl Default for InspirationMap {
    fn default() -> Self {
        Self::canonical()
    }
}

// =========================================================================
// Unit tests (kept in parity with sibling inspiration modules; verified via
// `cargo test --lib arch_omarchy_design`).
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn philosophy_catalog_is_non_empty_and_balanced() {
        let p = ArchOmarchyDesignPhilosophy::canonical();
        assert!(!p.principles.is_empty());
        // The Arch Way must be represented (6 named principles)...
        assert!(p.arch_way_count() >= 6);
        // ...and Omarchy too (2 named principles).
        assert!(p.omarchy_count() >= 2);
        // Spot-check the canonical KISS principle exists and is Arch-sourced.
        let kiss = p.find("KISS").expect("KISS principle present");
        assert_eq!(kiss.source, PhilosophySource::ArchWay);
        // render_summary uses format! and reflects the counts.
        assert!(p.render_summary().contains("ArchOmarchyDesignPhilosophy"));
    }

    #[test]
    fn kiss_checklist_cites_duplicated_file_managers_first() {
        let c = KissSimplificationChecklist::canonical();
        assert!(!c.entries.is_empty());
        let first = &c.entries[0];
        assert_eq!(first.pattern, "Duplicated file-manager modules");
        // The four duplicated file-manager modules must all be cited.
        for f in [
            "src/desktop/file_manager.rs",
            "src/desktop/filemanager.rs",
            "src/desktop/desktop_file_manager.rs",
            "src/desktop/file_manager_extensions.rs",
        ] {
            assert!(
                first.affected_files.iter().any(|af| af == f),
                "missing cited file: {}",
                f
            );
        }
        assert!(c.open_count() >= 1);
    }

    #[test]
    fn curated_defaults_enumerate_the_curated_set() {
        let m = CuratedDefaultsManifest::canonical();
        assert!(!m.entries.is_empty());
        // Every package the FEAT names as part of the curated set must appear.
        for pkg in [
            "hyprland",
            "hyprlock",
            "hypridle",
            "foot",
            "ghostty",
            "wofi",
            "waybar",
            "mako",
            "pipewire",
            "wireplumber",
            "grim",
            "slurp",
            "nautilus",
            "fastfetch",
            "btop",
            "networkmanager",
            "bluez",
        ] {
            assert!(
                m.has_package(pkg),
                "curated defaults missing package: {}",
                pkg
            );
        }
        // packages() is deduplicated and sorted (BTreeSet-backed).
        let pkgs = m.packages();
        assert!(!pkgs.is_empty());
        assert!(pkgs.windows(2).all(|w| w[0] <= w[1]));
        // grouped_by_category returns at least the wayland category.
        let grouped = m.grouped_by_category();
        assert!(grouped.contains_key("wayland"));
        assert!(m.render_summary().contains("CuratedDefaultsManifest"));
    }

    #[test]
    fn inspiration_map_contains_every_named_target_component() {
        let m = InspirationMap::canonical();
        assert!(!m.entries.is_empty());
        // Every named target SigmaOS component from the FEAT mapping.
        for component in [
            "SigmaOS-packages-kit",
            "SigmaOS-packages-tools",
            "SigmaOS-plugin-marketplace",
            "SigmaOS-plugin-registry",
            "SigmaOS-ISO",
            "SigmaOS-compositor",
            "SigmaOS-window-manager",
            "SigmaOS-quickshell",
            "SigmaOS-dots-configuration",
            "SigmaOS-file-manager",
            "SigmaOS-file-manager-system",
            "SigmaOS-reel",
            "SigmaOS-screen-recorder",
            "SigmaOS-mac-run",
            "SigmaOS-docs",
            "SigmaOS-wiki",
        ] {
            assert!(
                m.has_target(component),
                "inspiration map missing target: {}",
                component
            );
        }
        // Every named upstream from the FEAT mapping.
        for upstream in [
            "pacman",
            "AUR",
            "PKGBUILD/makepkg",
            "archiso",
            "Hyprland",
            "omadots",
            "omafind",
            "omareel",
            "omarchy-mac",
            "Arch Wiki",
        ] {
            assert!(
                m.has_upstream(upstream),
                "inspiration map missing upstream: {}",
                upstream
            );
        }
        // 16 distinct target components (SigmaOS-ISO is reached by two upstreams).
        assert_eq!(m.target_components().len(), 16);
        assert!(m.render_summary().contains("InspirationMap"));
    }
}
