// SigmaOS Debian/Ubuntu Parity Implementation
// Implements Debian packaging system, APT, and Ubuntu-specific features

#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "standalone_test")]
extern crate alloc;

#[cfg(not(feature = "standalone_test"))]
use crate::klib::Vec;

#[cfg(feature = "standalone_test")]
use alloc::vec::Vec;

use core::cell::Cell;
use std::format;

/// Debian package management with APT parity
pub struct DebianPackageManager {
    pub sources_list: Vec<String>,
    pub installed_packages: Vec<String>,
    pub cache_updated: Cell<bool>,
}

impl DebianPackageManager {
    pub fn new() -> Self {
        DebianPackageManager {
            sources_list: Vec::new(),
            installed_packages: Vec::new(),
            cache_updated: Cell::new(false),
        }
    }

    /// Add repository to sources.list
    pub fn add_repository(&mut self, repo: &str) {
        self.sources_list.push(String::from(repo));
    }

    /// Update package cache (apt-get update equivalent)
    pub fn update_cache(&self) {
        self.cache_updated.set(true);
    }

    /// Install package (apt-get install equivalent)
    pub fn install_package(&mut self, package: &str) -> bool {
        self.installed_packages.push(String::from(package));
        true
    }

    /// Remove package (apt-get remove equivalent)
    pub fn remove_package(&mut self, package: &str) -> bool {
        let package_str = String::from(package);
        for i in 0..self.installed_packages.len() {
            if self.installed_packages[i] == package_str {
                self.installed_packages.remove(i);
                return true;
            }
        }
        false
    }

    /// Search for packages (apt-cache search equivalent)
    pub fn search_packages(&self, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        let search_str = String::from(query);
        for pkg in &self.installed_packages {
            if pkg.contains(&search_str) {
                results.push(pkg.clone());
            }
        }
        results
    }
}

/// Ubuntu Snap package manager parity
pub struct SnapPackageManager {
    pub installed_snaps: Vec<String>,
    pub snap_channels: Vec<String>,
}

impl SnapPackageManager {
    pub fn new() -> Self {
        SnapPackageManager {
            installed_snaps: Vec::new(),
            snap_channels: Vec::new(),
        }
    }

    /// Install snap package
    pub fn install_snap(&mut self, snap: &str) -> bool {
        self.installed_snaps.push(String::from(snap));
        true
    }

    /// Remove snap package
    pub fn remove_snap(&mut self, snap: &str) -> bool {
        let snap_str = String::from(snap);
        for i in 0..self.installed_snaps.len() {
            if self.installed_snaps[i] == snap_str {
                self.installed_snaps.remove(i);
                return true;
            }
        }
        false
    }

    /// List installed snaps
    pub fn list_snaps(&self) -> &Vec<String> {
        &self.installed_snaps
    }
}

/// Debian Control file parser for .deb packages
pub struct DebianControl {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
    pub depends: Vec<String>,
}

impl DebianControl {
    pub fn new() -> Self {
        DebianControl {
            package: String::new(),
            version: String::new(),
            architecture: String::new(),
            maintainer: String::new(),
            description: String::new(),
            depends: Vec::new(),
        }
    }

    /// Parse debian/control file format
    pub fn parse_control(&mut self, control_content: &str) {
        let lines: Vec<&str> = control_content.lines().collect();

        for line in lines {
            if line.contains(':') {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let field = parts[0].trim();
                    let value = parts[1].trim();

                    match field {
                        "Package" => self.package = String::from(value),
                        "Version" => self.version = String::from(value),
                        "Architecture" => self.architecture = String::from(value),
                        "Maintainer" => self.maintainer = String::from(value),
                        "Description" => self.description = String::from(value),
                        "Depends" => {
                            let deps: Vec<&str> = value.split(',').collect();
                            for dep in deps {
                                self.depends.push(String::from(dep.trim()));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// Ubuntu Unity/GNOME desktop integration
pub struct UbuntuDesktopIntegration {
    pub unity_launcher: Vec<String>,
    pub gnome_extensions: Vec<String>,
    pub desktop_files: Vec<String>,
}

impl UbuntuDesktopIntegration {
    pub fn new() -> Self {
        UbuntuDesktopIntegration {
            unity_launcher: Vec::new(),
            gnome_extensions: Vec::new(),
            desktop_files: Vec::new(),
        }
    }

    /// Add application to Unity launcher
    pub fn add_to_launcher(&mut self, app: &str) {
        self.unity_launcher.push(String::from(app));
    }

    /// Install GNOME extension
    pub fn install_extension(&mut self, extension: &str) {
        self.gnome_extensions.push(String::from(extension));
    }

    /// Create desktop file
    pub fn create_desktop_file(&mut self, filename: &str) {
        self.desktop_files.push(String::from(filename));
    }
}

// =========================================================================
// 1. Debian dpkg-triggers Execution Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpkgTriggerInterest {
    pub package_name: String,
    pub trigger_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpkgTriggerActivation {
    pub trigger_name: String,
    pub activating_package: String,
}

pub struct DebianDpkgTriggersEngine {
    pub interests: Vec<DpkgTriggerInterest>,
    pub pending_activations: Vec<DpkgTriggerActivation>,
    pub processed_actions: Vec<String>,
}

impl DebianDpkgTriggersEngine {
    pub fn new() -> Self {
        Self {
            interests: Vec::new(),
            pending_activations: Vec::new(),
            processed_actions: Vec::new(),
        }
    }

    pub fn register_interest(&mut self, package_name: &str, trigger_name: &str) {
        self.interests.push(DpkgTriggerInterest {
            package_name: package_name.to_string(),
            trigger_name: trigger_name.to_string(),
        });
    }

    pub fn activate_trigger(&mut self, trigger_name: &str, activating_package: &str) {
        self.pending_activations.push(DpkgTriggerActivation {
            trigger_name: trigger_name.to_string(),
            activating_package: activating_package.to_string(),
        });
    }

    pub fn process_pending_triggers(&mut self) -> usize {
        let mut count = 0;
        let activations = self.pending_activations.clone();
        self.pending_activations.clear();

        for act in activations {
            for interest in &self.interests {
                if interest.trigger_name == act.trigger_name {
                    let action = format!(
                        "Processing trigger '{}' for package '{}' (activated by '{}')",
                        act.trigger_name, interest.package_name, act.activating_package
                    );
                    self.processed_actions.push(action);
                    count += 1;
                }
            }
        }
        count
    }
}

impl Default for DebianDpkgTriggersEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. Debian apt-listbugs Release Critical Bug Auditor
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BugSeverity {
    Critical,
    Grave,
    Serious,
    Normal,
    Minor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptBugReport {
    pub bug_id: u32,
    pub package_name: String,
    pub severity: BugSeverity,
    pub title: String,
}

pub struct DebianAptListbugsAuditor {
    pub reported_bugs: Vec<AptBugReport>,
    pub held_packages: Vec<String>,
}

impl DebianAptListbugsAuditor {
    pub fn new() -> Self {
        Self {
            reported_bugs: Vec::new(),
            held_packages: Vec::new(),
        }
    }

    pub fn report_bug(
        &mut self,
        bug_id: u32,
        package_name: &str,
        severity: BugSeverity,
        title: &str,
    ) {
        self.reported_bugs.push(AptBugReport {
            bug_id,
            package_name: package_name.to_string(),
            severity,
            title: title.to_string(),
        });
    }

    pub fn get_critical_bugs_for_package(&self, package_name: &str) -> Vec<AptBugReport> {
        self.reported_bugs
            .iter()
            .filter(|b| {
                b.package_name == package_name
                    && (b.severity == BugSeverity::Critical
                        || b.severity == BugSeverity::Grave
                        || b.severity == BugSeverity::Serious)
            })
            .cloned()
            .collect()
    }

    pub fn audit_and_hold_buggy_packages(&mut self, package_list: &[&str]) -> usize {
        let mut held_count = 0;
        for &pkg in package_list {
            let rc_bugs = self.get_critical_bugs_for_package(pkg);
            if !rc_bugs.is_empty() {
                if !self.held_packages.contains(&pkg.to_string()) {
                    self.held_packages.push(pkg.to_string());
                    held_count += 1;
                }
            }
        }
        held_count
    }
}

impl Default for DebianAptListbugsAuditor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. Debian update-alternatives System
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlternativeMode {
    Auto,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeTarget {
    pub path: String,
    pub priority: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeGroup {
    pub name: String,
    pub master_link: String,
    pub mode: AlternativeMode,
    pub choices: Vec<AlternativeTarget>,
    pub manual_selection: Option<String>,
}

pub struct DebianAlternativesSystem {
    pub groups: Vec<AlternativeGroup>,
}

impl DebianAlternativesSystem {
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    pub fn register_alternative(
        &mut self,
        name: &str,
        master_link: &str,
        target_path: &str,
        priority: u32,
    ) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.name == name) {
            if !group.choices.iter().any(|c| c.path == target_path) {
                group.choices.push(AlternativeTarget {
                    path: target_path.to_string(),
                    priority,
                });
            }
        } else {
            let mut choices = Vec::new();
            choices.push(AlternativeTarget {
                path: target_path.to_string(),
                priority,
            });
            self.groups.push(AlternativeGroup {
                name: name.to_string(),
                master_link: master_link.to_string(),
                mode: AlternativeMode::Auto,
                choices,
                manual_selection: None,
            });
        }
    }

    pub fn set_manual(&mut self, name: &str, target_path: &str) -> bool {
        if let Some(group) = self.groups.iter_mut().find(|g| g.name == name) {
            if group.choices.iter().any(|c| c.path == target_path) {
                group.mode = AlternativeMode::Manual;
                group.manual_selection = Some(target_path.to_string());
                return true;
            }
        }
        false
    }

    pub fn set_auto(&mut self, name: &str) -> bool {
        if let Some(group) = self.groups.iter_mut().find(|g| g.name == name) {
            group.mode = AlternativeMode::Auto;
            group.manual_selection = None;
            return true;
        }
        false
    }

    pub fn get_active_target(&self, name: &str) -> Option<String> {
        let group = self.groups.iter().find(|g| g.name == name)?;
        match group.mode {
            AlternativeMode::Manual => group.manual_selection.clone(),
            AlternativeMode::Auto => group
                .choices
                .iter()
                .max_by_key(|c| c.priority)
                .map(|c| c.path.clone()),
        }
    }
}

impl Default for DebianAlternativesSystem {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Debian Popularity-Contest (popcon) Telemetry Reporter
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopconPackageEntry {
    pub package_name: String,
    pub last_access_timestamp: u64,
    pub executable_path: String,
}

pub struct DebianPopconReporter {
    pub host_id: String,
    pub package_entries: Vec<PopconPackageEntry>,
}

impl DebianPopconReporter {
    pub fn new(host_id: &str) -> Self {
        Self {
            host_id: host_id.to_string(),
            package_entries: Vec::new(),
        }
    }

    pub fn record_access(&mut self, package_name: &str, executable_path: &str, timestamp: u64) {
        if let Some(entry) = self
            .package_entries
            .iter_mut()
            .find(|e| e.package_name == package_name)
        {
            entry.last_access_timestamp = timestamp;
            entry.executable_path = executable_path.to_string();
        } else {
            self.package_entries.push(PopconPackageEntry {
                package_name: package_name.to_string(),
                last_access_timestamp: timestamp,
                executable_path: executable_path.to_string(),
            });
        }
    }

    pub fn generate_popcon_report(&self, current_time: u64) -> String {
        let mut report = format!("POPULARITY-CONTEST-0.1 HOST:{}\n", self.host_id);
        for entry in &self.package_entries {
            let status = if current_time.saturating_sub(entry.last_access_timestamp) < 30 * 86400 {
                "RECENT"
            } else {
                "OLD"
            };
            report.push_str(&format!(
                "{} {} {} {}\n",
                entry.last_access_timestamp, entry.package_name, entry.executable_path, status
            ));
        }
        report.push_str("END-POPULARITY-CONTEST\n");
        report
    }
}

// =========================================================================
// 5. Debian Backports & APT Pinning Preference Manager
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptPinRule {
    pub package_pattern: String,
    pub release_target: String,
    pub priority: i32,
}

pub struct DebianBackportsPinningManager {
    pub pin_rules: Vec<AptPinRule>,
}

impl DebianBackportsPinningManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            pin_rules: Vec::new(),
        };
        // Default debian-backports pin priority rule
        mgr.add_pin_rule("*", "bookworm-backports", 100);
        mgr
    }

    pub fn add_pin_rule(&mut self, package_pattern: &str, release_target: &str, priority: i32) {
        self.pin_rules.push(AptPinRule {
            package_pattern: package_pattern.to_string(),
            release_target: release_target.to_string(),
            priority,
        });
    }

    pub fn calculate_effective_priority(&self, package: &str, release: &str) -> i32 {
        let mut best_priority = 500; // Default Debian APT priority
        for rule in &self.pin_rules {
            if (rule.package_pattern == "*" || rule.package_pattern == package)
                && rule.release_target == release
            {
                best_priority = rule.priority;
            }
        }
        best_priority
    }

    pub fn generate_preferences_file(&self) -> String {
        let mut pref = String::from("# /etc/apt/preferences.d/sovereign-backports\n");
        for rule in &self.pin_rules {
            pref.push_str(&format!(
                "Package: {}\nPin: release a={}\nPin-Priority: {}\n\n",
                rule.package_pattern, rule.release_target, rule.priority
            ));
        }
        pref
    }
}

impl Default for DebianBackportsPinningManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DebianPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SnapPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DebianControl {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for UbuntuDesktopIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpkg_triggers_engine() {
        let mut engine = DebianDpkgTriggersEngine::new();
        engine.register_interest("man-db", "usr/share/man");
        engine.activate_trigger("usr/share/man", "vim");

        let processed = engine.process_pending_triggers();
        assert_eq!(processed, 1);
        assert_eq!(engine.processed_actions.len(), 1);
        assert!(engine.processed_actions[0].contains("usr/share/man"));
    }

    #[test]
    fn test_apt_listbugs_auditor() {
        let mut auditor = DebianAptListbugsAuditor::new();
        auditor.report_bug(
            1001,
            "openssl",
            BugSeverity::Critical,
            "Buffer overflow in TLS",
        );
        auditor.report_bug(1002, "bash", BugSeverity::Minor, "Typo in man page");

        let held = auditor.audit_and_hold_buggy_packages(&["openssl", "bash"]);
        assert_eq!(held, 1);
        assert_eq!(auditor.held_packages, vec!["openssl".to_string()]);
    }

    #[test]
    fn test_update_alternatives_system() {
        let mut alts = DebianAlternativesSystem::new();
        alts.register_alternative("editor", "/usr/bin/editor", "/usr/bin/vim.basic", 50);
        alts.register_alternative("editor", "/usr/bin/editor", "/usr/bin/nano", 40);

        // Auto mode picks highest priority (/usr/bin/vim.basic)
        assert_eq!(
            alts.get_active_target("editor"),
            Some("/usr/bin/vim.basic".to_string())
        );

        // Manual override
        assert!(alts.set_manual("editor", "/usr/bin/nano"));
        assert_eq!(
            alts.get_active_target("editor"),
            Some("/usr/bin/nano".to_string())
        );

        // Reset to auto
        assert!(alts.set_auto("editor"));
        assert_eq!(
            alts.get_active_target("editor"),
            Some("/usr/bin/vim.basic".to_string())
        );
    }

    #[test]
    fn test_popcon_reporter() {
        let mut popcon = DebianPopconReporter::new("sovereign-host-01");
        popcon.record_access("coreutils", "/bin/ls", 1700000000);

        let report = popcon.generate_popcon_report(1700000100);
        assert!(report.contains("POPULARITY-CONTEST-0.1"));
        assert!(report.contains("coreutils /bin/ls RECENT"));
    }

    #[test]
    fn test_backports_pinning_manager() {
        let mut pin_mgr = DebianBackportsPinningManager::new();
        assert_eq!(
            pin_mgr.calculate_effective_priority("linux-image", "bookworm-backports"),
            100
        );

        pin_mgr.add_pin_rule("linux-image", "bookworm-backports", 500);
        assert_eq!(
            pin_mgr.calculate_effective_priority("linux-image", "bookworm-backports"),
            500
        );

        let pref_file = pin_mgr.generate_preferences_file();
        assert!(pref_file.contains("Package: linux-image"));
        assert!(pref_file.contains("Pin-Priority: 500"));
    }
}
