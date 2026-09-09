#![allow(dead_code)]
//! SigmaOS Debian Linux Compatibility Adapter
//! Implements APT repositories, SysVinit runlevels, debian alternatives, and debootstrap logic.
//! Zero external dependencies.

use std::string::String;
use std::vec::Vec;

// ==============================================================================
// 1. APT Repository Synchronization & GPG Keyring verification
// ==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebianChannel {
    Stable,
    Testing,
    UnstableSid,
}

pub struct AptRepositorySync {
    pub channel: DebianChannel,
    pub mirror_url: String,
    pub is_gpg_verified: bool,
    pub package_count: usize,
}

impl AptRepositorySync {
    pub fn new(channel: DebianChannel, mirror_url: String) -> Self {
        Self {
            channel,
            mirror_url,
            is_gpg_verified: false,
            package_count: 0,
        }
    }

    pub fn verify_release_keyring(&mut self, gpg_key: &[u8]) -> bool {
        if gpg_key.len() > 0 && gpg_key[0] == 0x99 {
            self.is_gpg_verified = true;
            true
        } else {
            false
        }
    }

    pub fn fetch_package_index(&mut self) -> Result<usize, &'static str> {
        if !self.is_gpg_verified {
            return Err("Unsigned release index: GPG verification failed!");
        }
        self.package_count = 58240; // Simulated package count of Debian repos
        Ok(self.package_count)
    }
}

// ==============================================================================
// 2. SysVinit Runlevels & Service Management (Runlevels 0-6)
// ==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysVRunlevel {
    Halt = 0,
    SingleUser = 1,
    MultiUserConsole = 2,
    MultiUserDefault = 3,
    MultiUserX11 = 4,
    MultiUserFull = 5,
    Reboot = 6,
}

pub struct SysVInitEngine {
    pub current_runlevel: SysVRunlevel,
    pub services_running: usize,
}

impl SysVInitEngine {
    pub fn new() -> Self {
        Self {
            current_runlevel: SysVRunlevel::MultiUserDefault,
            services_running: 0,
        }
    }

    pub fn transition_to_runlevel(&mut self, runlevel: SysVRunlevel) -> bool {
        // Simulates running stop scripts (K*) and start scripts (S*) in rc.d
        self.current_runlevel = runlevel;
        match runlevel {
            SysVRunlevel::Halt => {
                self.services_running = 0;
            }
            SysVRunlevel::SingleUser => {
                self.services_running = 4;
            }
            SysVRunlevel::MultiUserDefault => {
                self.services_running = 18;
            }
            SysVRunlevel::Reboot => {
                self.services_running = 0;
            }
            _ => {
                self.services_running = 24;
            }
        }
        true
    }
}

impl Default for SysVInitEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 3. Debian Alternatives Link Management (update-alternatives parity)
// ==============================================================================
#[derive(Clone)]
pub struct AlternativeLink {
    pub symlink: String, // e.g. "/usr/bin/editor"
    pub target: String,  // e.g. "/usr/bin/nano"
    pub priority: u32,
}

pub struct DebianAlternativesSystem {
    pub link_name: String, // e.g. "editor"
    pub links: Vec<AlternativeLink>,
    pub active_index: Option<usize>,
}

impl DebianAlternativesSystem {
    pub fn new(link_name: String) -> Self {
        Self {
            link_name,
            links: Vec::new(),
            active_index: None,
        }
    }

    pub fn register_alternative(&mut self, symlink: String, target: String, priority: u32) {
        self.links.push(AlternativeLink {
            symlink,
            target,
            priority,
        });
        self.resolve_best_priority();
    }

    pub fn select_manual(&mut self, target: &str) -> bool {
        for (i, link) in self.links.iter().enumerate() {
            if link.target == target {
                self.active_index = Some(i);
                return true;
            }
        }
        false
    }

    fn resolve_best_priority(&mut self) {
        let mut best_idx = None;
        let mut max_priority = 0;
        for (i, link) in self.links.iter().enumerate() {
            if link.priority > max_priority {
                max_priority = link.priority;
                best_idx = Some(i);
            }
        }
        self.active_index = best_idx;
    }

    pub fn get_active_target(&self) -> Option<&str> {
        self.active_index
            .and_then(|idx| self.links.get(idx))
            .map(|link| link.target.as_str())
    }
}

// ==============================================================================
// 4. Debootstrap Minimal Bootstrapping Engine
// ==============================================================================
pub struct DebootstrapEngine {
    pub target_root: String,
    pub is_bootstrapped: bool,
    pub extracted_packages: usize,
}

impl DebootstrapEngine {
    pub fn new(target_root: String) -> Self {
        Self {
            target_root,
            is_bootstrapped: false,
            extracted_packages: 0,
        }
    }

    pub fn execute_debootstrap(&mut self, sync: &AptRepositorySync) -> Result<bool, &'static str> {
        if !sync.is_gpg_verified {
            return Err("Unverified repository source!");
        }
        // Simulates downloading core base .deb files, unpacking metadata, and resolving dependencies
        self.extracted_packages = 84; // Essential base packages
        self.is_bootstrapped = true;
        Ok(true)
    }
}

// ==============================================================================
// 5. Debconf Package Configuration & Preseed Answer Engine
// ==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebconfQuestionType {
    String,
    Select,
    Multiselect,
    Boolean,
    Password,
    Note,
    Error,
}

#[derive(Debug, Clone)]
pub struct DebconfTemplate {
    pub name: String,
    pub question_type: DebconfQuestionType,
    pub default_value: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct DebconfAnswer {
    pub name: String,
    pub value: String,
    pub seen: bool,
}

pub struct DebconfEngine {
    pub templates: Vec<DebconfTemplate>,
    pub answers: Vec<DebconfAnswer>,
}

impl DebconfEngine {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
            answers: Vec::new(),
        }
    }

    pub fn register_template(&mut self, template: DebconfTemplate) {
        if let Some(ans) = self.answers.iter().find(|a| a.name == template.name) {
            let _ = ans;
        } else {
            self.answers.push(DebconfAnswer {
                name: template.name.clone(),
                value: template.default_value.clone(),
                seen: false,
            });
        }
        self.templates.push(template);
    }

    /// Parse debconf preseed configuration file format (`owner question/type value`)
    pub fn parse_preseed_file(&mut self, text: &str) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let _owner = parts[0];
                let name = parts[1];
                let _qtype = parts[2];
                let value = parts[3..].join(" ");

                self.set_answer(name, &value);
            }
        }
    }

    pub fn set_answer(&mut self, name: &str, value: &str) {
        if let Some(ans) = self.answers.iter_mut().find(|a| a.name == name) {
            ans.value = value.to_string();
            ans.seen = true;
        } else {
            self.answers.push(DebconfAnswer {
                name: name.to_string(),
                value: value.to_string(),
                seen: true,
            });
        }
    }

    pub fn get_answer(&self, name: &str) -> Option<&str> {
        self.answers
            .iter()
            .find(|a| a.name == name)
            .map(|a| a.value.as_str())
    }
}

impl Default for DebconfEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 6. dpkg-statoverride File Mode & Ownership Engine
// ==============================================================================
#[derive(Debug, Clone)]
pub struct StatOverrideEntry {
    pub user: String,
    pub group: String,
    pub mode: u32,
    pub path: String,
}

pub struct DpkgStatOverrideEngine {
    pub overrides: Vec<StatOverrideEntry>,
}

impl DpkgStatOverrideEngine {
    pub fn new() -> Self {
        Self {
            overrides: Vec::new(),
        }
    }

    pub fn add_override(&mut self, user: &str, group: &str, mode: u32, path: &str) {
        self.remove_override(path);
        self.overrides.push(StatOverrideEntry {
            user: user.to_string(),
            group: group.to_string(),
            mode,
            path: path.to_string(),
        });
    }

    pub fn remove_override(&mut self, path: &str) -> bool {
        let initial_len = self.overrides.len();
        self.overrides.retain(|o| o.path != path);
        self.overrides.len() < initial_len
    }

    pub fn get_override(&self, path: &str) -> Option<&StatOverrideEntry> {
        self.overrides.iter().find(|o| o.path == path)
    }

    /// Parse `/var/lib/dpkg/statoverride` lines (`user group mode path`)
    pub fn parse_statoverride_db(&mut self, text: &str) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 4 {
                let user = parts[0];
                let group = parts[1];
                let mode = u32::from_str_radix(parts[2], 8).unwrap_or(0o755);
                let path = parts[3];

                self.add_override(user, group, mode, path);
            }
        }
    }
}

impl Default for DpkgStatOverrideEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 7. APT Pinning & Release Preferences Engine (/etc/apt/preferences)
// ==============================================================================
#[derive(Debug, Clone)]
pub struct AptPinRule {
    pub package_pattern: String,
    pub release_pattern: String,
    pub priority: i32,
}

pub struct AptPinningEngine {
    pub rules: Vec<AptPinRule>,
}

impl AptPinningEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_pin(&mut self, package_pattern: &str, release_pattern: &str, priority: i32) {
        self.rules.push(AptPinRule {
            package_pattern: package_pattern.to_string(),
            release_pattern: release_pattern.to_string(),
            priority,
        });
    }

    /// Parse `/etc/apt/preferences` format (`Package: ...`, `Pin: release ...`, `Pin-Priority: ...`)
    pub fn parse_preferences(&mut self, text: &str) {
        let mut current_package = String::from("*");
        let mut current_release = String::from("*");
        let mut current_priority = 500;

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                if current_priority != 500 || current_package != "*" || current_release != "*" {
                    self.add_pin(&current_package, &current_release, current_priority);
                    current_package = String::from("*");
                    current_release = String::from("*");
                    current_priority = 500;
                }
                continue;
            }

            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                match key {
                    "Package" => current_package = val.to_string(),
                    "Pin" => {
                        if let Some(rel) = val.strip_prefix("release") {
                            current_release = rel.trim().to_string();
                        } else {
                            current_release = val.to_string();
                        }
                    }
                    "Pin-Priority" => {
                        current_priority = val.parse::<i32>().unwrap_or(500);
                    }
                    _ => {}
                }
            }
        }

        if current_priority != 500 || current_package != "*" || current_release != "*" {
            self.add_pin(&current_package, &current_release, current_priority);
        }
    }

    pub fn get_pin_priority(&self, package: &str, release: &str) -> i32 {
        let mut best_priority = 500; // Default APT pin priority
        for rule in &self.rules {
            let pkg_match = rule.package_pattern == "*" || rule.package_pattern == package;
            let rel_match = rule.release_pattern == "*"
                || release.contains(&rule.release_pattern)
                || rule.release_pattern.contains(release);

            if pkg_match && rel_match {
                best_priority = rule.priority;
            }
        }
        best_priority
    }
}

impl Default for AptPinningEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 8. APT Build-Dep Source Dependency Resolver Engine
// ==============================================================================
pub struct AptBuildDepResolver;

impl AptBuildDepResolver {
    pub fn new() -> Self {
        Self
    }

    /// Parses Debian source package `.dsc` / `debian/control` `Build-Depends:` fields
    pub fn resolve_build_deps(&self, control_text: &str) -> Vec<String> {
        let mut build_deps = Vec::new();

        for line in control_text.lines() {
            let line = line.trim();
            if line.starts_with("Build-Depends:") || line.starts_with("Build-Depends-Indep:") {
                let pos = line.find(':').unwrap_or(0);
                let val = &line[pos + 1..];
                for dep in val.split(',') {
                    let clean = dep.split('(').next().unwrap_or(dep).trim();
                    if !clean.is_empty() {
                        build_deps.push(clean.to_string());
                    }
                }
            }
        }

        build_deps
    }
}

impl Default for AptBuildDepResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debconf_preseed_parsing() {
        let mut debconf = DebconfEngine::new();
        let preseed_content = r#"
            # Preseed configuration for automated installation
            tzdata tzdata/Zones/Asia string Kolkata
            locales locales/default_environment_locale select en_US.UTF-8
            openssh-server openssh-server/permit-root-login boolean false
        "#;

        debconf.parse_preseed_file(preseed_content);
        assert_eq!(debconf.get_answer("tzdata/Zones/Asia"), Some("Kolkata"));
        assert_eq!(
            debconf.get_answer("locales/default_environment_locale"),
            Some("en_US.UTF-8")
        );
        assert_eq!(
            debconf.get_answer("openssh-server/permit-root-login"),
            Some("false")
        );
    }

    #[test]
    fn test_dpkg_statoverride_engine() {
        let mut engine = DpkgStatOverrideEngine::new();
        let statoverride_data = r#"
            root statd 02755 /usr/sbin/statd
            root crontab 04755 /usr/bin/crontab
        "#;

        engine.parse_statoverride_db(statoverride_data);
        let crontab_override = engine.get_override("/usr/bin/crontab").unwrap();
        assert_eq!(crontab_override.user, "root");
        assert_eq!(crontab_override.group, "crontab");
        assert_eq!(crontab_override.mode, 0o4755);

        engine.add_override("root", "shadow", 0o2755, "/usr/bin/expiry");
        assert!(engine.get_override("/usr/bin/expiry").is_some());
    }

    #[test]
    fn test_apt_pinning_engine() {
        let mut pinning = AptPinningEngine::new();
        let prefs = r#"
            Package: *
            Pin: release a=testing
            Pin-Priority: 900

            Package: firefox
            Pin: release a=unstable
            Pin-Priority: 1001
        "#;

        pinning.parse_preferences(prefs);
        assert_eq!(pinning.get_pin_priority("bash", "testing"), 900);
        assert_eq!(pinning.get_pin_priority("firefox", "unstable"), 1001);
    }

    #[test]
    fn test_apt_build_dep_resolver() {
        let resolver = AptBuildDepResolver::new();
        let dsc_content = r#"
            Source: nginx
            Section: httpd
            Priority: optional
            Build-Depends: debhelper-compat (= 13), libssl-dev (>= 3.0), zlib1g-dev, libpcre2-dev
        "#;

        let deps = resolver.resolve_build_deps(dsc_content);
        assert_eq!(deps.len(), 4);
        assert_eq!(deps[0], "debhelper-compat");
        assert_eq!(deps[1], "libssl-dev");
        assert_eq!(deps[2], "zlib1g-dev");
        assert_eq!(deps[3], "libpcre2-dev");
    }
}
