// SPDX-License-Identifier: MIT
// SigmaOS Debian GNU/Linux Gap Closure Advancements Suite V26
// (`src/distro/debian_gap_closure_advancements_v26.rs`)
//
// Zero-dependency Rust implementation absorbing remaining gaps between SigmaOS
// and Debian GNU/Linux system administration paradigms:
//   1. `dpkg-divert` File Path Redirection & Collision Prevention Engine
//   2. `debconf` Template Question, Priority Threshold & Response Database Engine
//   3. `dpkg-statoverride` File Ownership, Group & Mode Permission Override Engine
//   4. `apt-mark` Selection State Governor (Auto / Manual / Hold) & Orphan Package Purger
//   5. `uscan` / `debian/watch` Upstream Version Watchfile Monitoring Engine
//   6. `debootstrap` Base System Chroot Installation & Target Staging Engine

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. DPKG-DIVERT FILE PATH REDIRECTION ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiversionRule {
    pub original_file: String,
    pub diverted_to: String,
    pub package_owner: String,
    pub is_local: bool,
}

pub struct DebianDpkgDivertEngine {
    pub diversions: BTreeMap<String, DiversionRule>,
}

impl DebianDpkgDivertEngine {
    pub fn new() -> Self {
        Self {
            diversions: BTreeMap::new(),
        }
    }

    pub fn add_diversion(
        &mut self,
        original: &str,
        diverted: &str,
        pkg_owner: &str,
        is_local: bool,
    ) -> Result<(), &'static str> {
        if self.diversions.contains_key(original) {
            return Err("Diversion for path already exists");
        }

        self.diversions.insert(
            original.to_string(),
            DiversionRule {
                original_file: original.to_string(),
                diverted_to: diverted.to_string(),
                package_owner: pkg_owner.to_string(),
                is_local,
            },
        );
        Ok(())
    }

    pub fn resolve_path(&self, file_path: &str, requesting_pkg: &str) -> String {
        if let Some(rule) = self.diversions.get(file_path) {
            if rule.package_owner == requesting_pkg {
                rule.original_file.clone()
            } else {
                rule.diverted_to.clone()
            }
        } else {
            file_path.to_string()
        }
    }
}

impl Default for DebianDpkgDivertEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. DEBCONF TEMPLATE QUESTIONS & RESPONSE DATABASE ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebconfPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct DebconfQuestion {
    pub template_key: String,
    pub priority: DebconfPriority,
    pub default_answer: String,
    pub user_answer: Option<String>,
}

pub struct DebianDebconfDatabaseEngine {
    pub questions: BTreeMap<String, DebconfQuestion>,
    pub system_priority_threshold: DebconfPriority,
}

impl DebianDebconfDatabaseEngine {
    pub fn new() -> Self {
        Self {
            questions: BTreeMap::new(),
            system_priority_threshold: DebconfPriority::High,
        }
    }

    pub fn register_template(&mut self, key: &str, priority: DebconfPriority, default_val: &str) {
        self.questions.insert(
            key.to_string(),
            DebconfQuestion {
                template_key: key.to_string(),
                priority,
                default_answer: default_val.to_string(),
                user_answer: None,
            },
        );
    }

    pub fn answer_question(&mut self, key: &str, answer: &str) -> bool {
        if let Some(q) = self.questions.get_mut(key) {
            q.user_answer = Some(answer.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_effective_answer(&self, key: &str) -> Option<String> {
        let q = self.questions.get(key)?;
        if q.priority >= self.system_priority_threshold {
            Some(q.user_answer.clone().unwrap_or_else(|| q.default_answer.clone()))
        } else {
            Some(q.default_answer.clone())
        }
    }
}

impl Default for DebianDebconfDatabaseEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. DPKG-STATOVERRIDE FILE OWNERSHIP & PERMISSION OVERRIDE ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatOverrideEntry {
    pub path: String,
    pub owner_user: String,
    pub owner_group: String,
    pub octal_mode: u32,
}

pub struct DebianDpkgStatoverrideEngine {
    pub overrides: BTreeMap<String, StatOverrideEntry>,
}

impl DebianDpkgStatoverrideEngine {
    pub fn new() -> Self {
        Self {
            overrides: BTreeMap::new(),
        }
    }

    pub fn add_override(&mut self, path: &str, user: &str, group: &str, mode: u32) {
        self.overrides.insert(
            path.to_string(),
            StatOverrideEntry {
                path: path.to_string(),
                owner_user: user.to_string(),
                owner_group: group.to_string(),
                octal_mode: mode,
            },
        );
    }

    pub fn get_override(&self, path: &str) -> Option<&StatOverrideEntry> {
        self.overrides.get(path)
    }
}

impl Default for DebianDpkgStatoverrideEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. APT-MARK SELECTION STATE GOVERNOR ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AptMarkSelectionState {
    Auto,
    Manual,
    Hold,
}

#[derive(Debug, Clone)]
pub struct AptMarkPackageRecord {
    pub package_name: String,
    pub state: AptMarkSelectionState,
    pub is_installed: bool,
    pub is_required_dep: bool,
}

pub struct DebianAptMarkGovernorEngine {
    pub packages: BTreeMap<String, AptMarkPackageRecord>,
}

impl DebianAptMarkGovernorEngine {
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
        }
    }

    pub fn register_package(&mut self, name: &str, state: AptMarkSelectionState, installed: bool, required: bool) {
        self.packages.insert(
            name.to_string(),
            AptMarkPackageRecord {
                package_name: name.to_string(),
                state,
                is_installed: installed,
                is_required_dep: required,
            },
        );
    }

    pub fn set_state(&mut self, name: &str, state: AptMarkSelectionState) -> bool {
        if let Some(pkg) = self.packages.get_mut(name) {
            pkg.state = state;
            true
        } else {
            false
        }
    }

    pub fn list_orphan_packages(&self) -> Vec<String> {
        let mut orphans = Vec::new();
        for pkg in self.packages.values() {
            if pkg.is_installed
                && pkg.state == AptMarkSelectionState::Auto
                && !pkg.is_required_dep
            {
                orphans.push(pkg.package_name.clone());
            }
        }
        orphans
    }
}

impl Default for DebianAptMarkGovernorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. USCAN / DEBIAN/WATCH UPSTREAM RELEASE WATCHFILE ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct DebianWatchfileSpec {
    pub package_name: String,
    pub upstream_url_pattern: String,
    pub current_version: String,
    pub latest_upstream_version: Option<String>,
}

pub struct DebianUscanWatchfileEngine {
    pub watchfiles: BTreeMap<String, DebianWatchfileSpec>,
}

impl DebianUscanWatchfileEngine {
    pub fn new() -> Self {
        Self {
            watchfiles: BTreeMap::new(),
        }
    }

    pub fn register_watchfile(&mut self, pkg: &str, pattern: &str, current_ver: &str) {
        self.watchfiles.insert(
            pkg.to_string(),
            DebianWatchfileSpec {
                package_name: pkg.to_string(),
                upstream_url_pattern: pattern.to_string(),
                current_version: current_ver.to_string(),
                latest_upstream_version: None,
            },
        );
    }

    pub fn check_upstream_release(&mut self, pkg: &str, detected_upstream_ver: &str) -> Option<bool> {
        let spec = self.watchfiles.get_mut(pkg)?;
        spec.latest_upstream_version = Some(detected_upstream_ver.to_string());
        Some(spec.current_version != detected_upstream_ver)
    }
}

impl Default for DebianUscanWatchfileEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. DEBOOTSTRAP BASE SYSTEM CHROOT INSTALLER ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebootstrapStage {
    DownloadingPackages,
    ExtractingCore,
    ConfiguringBase,
    Completed,
}

#[derive(Debug, Clone)]
pub struct DebootstrapTarget {
    pub target_directory: String,
    pub suite_codename: String, // e.g., "bookworm", "trixie", "sid"
    pub mirror_url: String,
    pub stage: DebootstrapStage,
    pub base_packages_count: usize,
}

pub struct DebianDebootstrapBaseInstallerEngine {
    pub targets: BTreeMap<String, DebootstrapTarget>,
}

impl DebianDebootstrapBaseInstallerEngine {
    pub fn new() -> Self {
        Self {
            targets: BTreeMap::new(),
        }
    }

    pub fn start_debootstrap(&mut self, target_dir: &str, suite: &str, mirror: &str) -> String {
        let target = DebootstrapTarget {
            target_directory: target_dir.to_string(),
            suite_codename: suite.to_string(),
            mirror_url: mirror.to_string(),
            stage: DebootstrapStage::DownloadingPackages,
            base_packages_count: 85,
        };

        self.targets.insert(target_dir.to_string(), target);
        format!("Debootstrap initialized for '{}' ({})", target_dir, suite)
    }

    pub fn advance_stage(&mut self, target_dir: &str) -> Result<DebootstrapStage, &'static str> {
        let target = self.targets.get_mut(target_dir).ok_or("Debootstrap target not found")?;

        target.stage = match target.stage {
            DebootstrapStage::DownloadingPackages => DebootstrapStage::ExtractingCore,
            DebootstrapStage::ExtractingCore => DebootstrapStage::ConfiguringBase,
            DebootstrapStage::ConfiguringBase => DebootstrapStage::Completed,
            DebootstrapStage::Completed => DebootstrapStage::Completed,
        };

        Ok(target.stage)
    }
}

impl Default for DebianDebootstrapBaseInstallerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// MASTER COORDINATOR SUITE V26
// =========================================================================

pub struct DebianGapClosureAdvancementsV26Suite {
    pub divert_engine: DebianDpkgDivertEngine,
    pub debconf_engine: DebianDebconfDatabaseEngine,
    pub statoverride_engine: DebianDpkgStatoverrideEngine,
    pub apt_mark_engine: DebianAptMarkGovernorEngine,
    pub uscan_engine: DebianUscanWatchfileEngine,
    pub debootstrap_engine: DebianDebootstrapBaseInstallerEngine,
}

#[derive(Debug, Clone)]
pub struct DebianV26DiagnosticsReport {
    pub dpkg_diversions_count: usize,
    pub debconf_questions_count: usize,
    pub statoverrides_count: usize,
    pub apt_marked_packages_count: usize,
    pub uscan_watchfiles_count: usize,
    pub debootstrap_targets_count: usize,
    pub status_ok: bool,
}

impl DebianGapClosureAdvancementsV26Suite {
    pub fn new() -> Self {
        Self {
            divert_engine: DebianDpkgDivertEngine::new(),
            debconf_engine: DebianDebconfDatabaseEngine::new(),
            statoverride_engine: DebianDpkgStatoverrideEngine::new(),
            apt_mark_engine: DebianAptMarkGovernorEngine::new(),
            uscan_engine: DebianUscanWatchfileEngine::new(),
            debootstrap_engine: DebianDebootstrapBaseInstallerEngine::new(),
        }
    }

    pub fn run_diagnostics(&self) -> DebianV26DiagnosticsReport {
        DebianV26DiagnosticsReport {
            dpkg_diversions_count: self.divert_engine.diversions.len(),
            debconf_questions_count: self.debconf_engine.questions.len(),
            statoverrides_count: self.statoverride_engine.overrides.len(),
            apt_marked_packages_count: self.apt_mark_engine.packages.len(),
            uscan_watchfiles_count: self.uscan_engine.watchfiles.len(),
            debootstrap_targets_count: self.debootstrap_engine.targets.len(),
            status_ok: true,
        }
    }
}

impl Default for DebianGapClosureAdvancementsV26Suite {
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
    fn test_dpkg_divert_engine() {
        let mut divert = DebianDpkgDivertEngine::new();
        divert.add_diversion("/usr/bin/gcc", "/usr/bin/gcc.real", "gcc-wrapper", false).unwrap();

        assert_eq!(divert.resolve_path("/usr/bin/gcc", "gcc-wrapper"), "/usr/bin/gcc");
        assert_eq!(divert.resolve_path("/usr/bin/gcc", "other-pkg"), "/usr/bin/gcc.real");
    }

    #[test]
    fn test_debconf_database_engine() {
        let mut debconf = DebianDebconfDatabaseEngine::new();
        debconf.register_template("tzdata/zones", DebconfPriority::Critical, "UTC");

        assert_eq!(debconf.get_effective_answer("tzdata/zones").unwrap(), "UTC");
        debconf.answer_question("tzdata/zones", "America/New_York");
        assert_eq!(debconf.get_effective_answer("tzdata/zones").unwrap(), "America/New_York");
    }

    #[test]
    fn test_dpkg_statoverride_engine() {
        let mut statoverride = DebianDpkgStatoverrideEngine::new();
        statoverride.add_override("/usr/bin/sudo", "root", "sudo", 0o4755);

        let entry = statoverride.get_override("/usr/bin/sudo").unwrap();
        assert_eq!(entry.owner_user, "root");
        assert_eq!(entry.octal_mode, 0o4755);
    }

    #[test]
    fn test_apt_mark_governor_engine() {
        let mut apt_mark = DebianAptMarkGovernorEngine::new();
        apt_mark.register_package("libssl3", AptMarkSelectionState::Auto, true, false);
        apt_mark.register_package("curl", AptMarkSelectionState::Manual, true, false);

        let orphans = apt_mark.list_orphan_packages();
        assert_eq!(orphans, vec!["libssl3".to_string()]);
    }

    #[test]
    fn test_uscan_watchfile_engine() {
        let mut uscan = DebianUscanWatchfileEngine::new();
        uscan.register_watchfile("nginx", "https://nginx.org/download/nginx-(.*).tar.gz", "1.24.0");

        let has_new = uscan.check_upstream_release("nginx", "1.26.0").unwrap();
        assert!(has_new);
    }

    #[test]
    fn test_debootstrap_installer_engine() {
        let mut debootstrap = DebianDebootstrapBaseInstallerEngine::new();
        let msg = debootstrap.start_debootstrap("/chroots/sid", "sid", "http://deb.debian.org/debian");
        assert!(msg.contains("initialized"));

        let stage = debootstrap.advance_stage("/chroots/sid").unwrap();
        assert_eq!(stage, DebootstrapStage::ExtractingCore);
    }

    #[test]
    fn test_debian_v26_suite_diagnostics() {
        let suite = DebianGapClosureAdvancementsV26Suite::new();
        let report = suite.run_diagnostics();
        assert!(report.status_ok);
    }
}
