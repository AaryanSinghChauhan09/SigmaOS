// Debian Linux Distro Compatibility Subsystem
// Formats, parsers, and registry models matching Debian systems (dpkg, apt, .deb)

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebControl {
    pub package: String,
    pub version: String,
    pub depends: Vec<String>,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
}

impl DebControl {
    pub fn parse(text: &str) -> Result<Self, &'static str> {
        let mut package = String::new();
        let mut version = String::new();
        let mut depends = Vec::new();
        let mut architecture = String::new();
        let mut maintainer = String::new();
        let mut description = String::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim();
                let val = line[pos + 1..].trim();
                match key {
                    "Package" => package = val.to_string(),
                    "Version" => version = val.to_string(),
                    "Architecture" => architecture = val.to_string(),
                    "Maintainer" => maintainer = val.to_string(),
                    "Description" => description = val.to_string(),
                    "Depends" => {
                        for dep in val.split(',') {
                            let dep_trimmed = dep.trim();
                            if !dep_trimmed.is_empty() {
                                depends.push(dep_trimmed.to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if package.is_empty() || version.is_empty() {
            return Err("Package and Version fields are mandatory in Debian control file");
        }

        // Validate package name (alphanumeric, '+', '-', '.' only, no relative traversal or path separators)
        if package.starts_with('.')
            || package.contains('/')
            || package.contains('\\')
            || package.contains("..")
        {
            return Err("Invalid or unsafe package name in Debian control file");
        }
        for b in package.bytes() {
            if !b.is_ascii_alphanumeric() && b != b'+' && b != b'-' && b != b'.' {
                return Err("Debian package name contains illegal characters");
            }
        }

        Ok(Self {
            package,
            version,
            depends,
            architecture,
            maintainer,
            description,
        })
    }
}

pub struct DebPackage {
    pub control: DebControl,
    pub data_size: usize,
}

impl DebPackage {
    /// Read standard .deb file containing ar archive entries (debian-binary, control, data)
    pub fn parse_binary(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 8 {
            return Err("Buffer is too small for Debian package ar signature");
        }
        if &data[0..8] != b"!<arch>\n" {
            return Err("Invalid Debian package ar signature");
        }

        let mut offset = 8;
        let mut control_text = None;
        let mut data_size = 0;

        while offset + 60 <= data.len() {
            let header = &data[offset..offset + 60];
            if &header[58..60] != b"\x60\x0A" {
                break;
            }

            let size_str = core::str::from_utf8(&header[48..58])
                .map_err(|_| "Invalid size encoding in ar header")?
                .trim();
            let size = size_str
                .parse::<usize>()
                .map_err(|_| "Failed to parse ar member size")?;

            let name = core::str::from_utf8(&header[0..16])
                .map_err(|_| "Invalid name encoding in ar header")?
                .trim();

            if offset + 60 + size > data.len() {
                return Err("ar member size extends beyond binary bounds");
            }
            let member_data = &data[offset + 60..offset + 60 + size];

            if name.starts_with("debian-binary") {
                let clean_ver = core::str::from_utf8(member_data)
                    .map_err(|_| "debian-binary contains invalid characters")?
                    .trim();
                if clean_ver != "2.0" {
                    return Err("Unsupported Debian package ar binary version");
                }
            } else if name.starts_with("control") {
                control_text = Some(
                    core::str::from_utf8(member_data).map_err(|_| "Control file must be UTF-8")?,
                );
            } else if name.starts_with("data") {
                data_size = size;
            }

            offset += 60 + size;
            if offset % 2 != 0 {
                offset += 1; // ar members are aligned to even bytes
            }
        }

        let control = if let Some(txt) = control_text {
            DebControl::parse(txt)?
        } else {
            // Fallback default
            DebControl {
                package: "debian-helper".to_string(),
                version: "1.0".to_string(),
                depends: Vec::new(),
                architecture: "all".to_string(),
                maintainer: "Debian team".to_string(),
                description: "Debian simulation helper".to_string(),
            }
        };

        Ok(Self { control, data_size })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptSource {
    pub is_source: bool, // true if deb-src, false if deb
    pub uri: String,
    pub suite: String,
    pub components: Vec<String>,
}

/// Parse /etc/apt/sources.list lines
pub fn parse_sources_list(text: &str) -> Result<Vec<AptSource>, &'static str> {
    let mut sources = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }

        let is_source = match parts[0] {
            "deb" => false,
            "deb-src" => true,
            _ => continue, // Ignore invalid types
        };

        let uri = parts[1].to_string();
        let suite = parts[2].to_string();
        let mut components = Vec::new();
        for &comp in &parts[3..] {
            components.push(comp.to_string());
        }

        sources.push(AptSource {
            is_source,
            uri,
            suite,
            components,
        });
    }

    Ok(sources)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpkgStatusEntry {
    pub package: String,
    pub status: String, // e.g. "install ok installed"
    pub priority: String,
    pub section: String,
    pub installed_size: usize,
    pub maintainer: String,
    pub architecture: String,
    pub version: String,
    pub description: String,
}

/// Parse /var/lib/dpkg/status records
pub fn parse_dpkg_status(text: &str) -> Vec<DpkgStatusEntry> {
    let mut entries = Vec::new();
    let mut current_package = String::new();
    let mut current_status = String::new();
    let mut current_priority = String::new();
    let mut current_section = String::new();
    let mut current_installed_size = 0;
    let mut current_maintainer = String::new();
    let mut current_architecture = String::new();
    let mut current_version = String::new();
    let mut current_description = String::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !current_package.is_empty() {
                entries.push(DpkgStatusEntry {
                    package: current_package.clone(),
                    status: current_status.clone(),
                    priority: current_priority.clone(),
                    section: current_section.clone(),
                    installed_size: current_installed_size,
                    maintainer: current_maintainer.clone(),
                    architecture: current_architecture.clone(),
                    version: current_version.clone(),
                    description: current_description.clone(),
                });
                // Reset fields
                current_package.clear();
                current_status.clear();
                current_priority.clear();
                current_section.clear();
                current_installed_size = 0;
                current_maintainer.clear();
                current_architecture.clear();
                current_version.clear();
                current_description.clear();
            }
            continue;
        }

        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim();
            let val = line[pos + 1..].trim();
            match key {
                "Package" => current_package = val.to_string(),
                "Status" => current_status = val.to_string(),
                "Priority" => current_priority = val.to_string(),
                "Section" => current_section = val.to_string(),
                "Installed-Size" => current_installed_size = val.parse::<usize>().unwrap_or(0),
                "Maintainer" => current_maintainer = val.to_string(),
                "Architecture" => current_architecture = val.to_string(),
                "Version" => current_version = val.to_string(),
                "Description" => current_description = val.to_string(),
                _ => {}
            }
        }
    }

    if !current_package.is_empty() {
        entries.push(DpkgStatusEntry {
            package: current_package,
            status: current_status,
            priority: current_priority,
            section: current_section,
            installed_size: current_installed_size,
            maintainer: current_maintainer,
            architecture: current_architecture,
            version: current_version,
            description: current_description,
        });
    }

    entries
}

// =========================================================================
// 1. DEBIAN update-alternatives MASTER SYMLINK PRIORITY ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeChoice {
    pub path: String,
    pub priority: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeGroup {
    pub name: String,
    pub master_link: String,
    pub choices: Vec<AlternativeChoice>,
    pub selected_path: Option<String>,
    pub is_auto: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DebianUpdateAlternativesEngine {
    pub groups: BTreeMap<String, AlternativeGroup>,
}

impl DebianUpdateAlternativesEngine {
    pub fn new() -> Self {
        Self {
            groups: BTreeMap::new(),
        }
    }

    pub fn install_alternative(
        &mut self,
        name: &str,
        master_link: &str,
        path: &str,
        priority: i32,
    ) {
        let choice = AlternativeChoice {
            path: path.to_string(),
            priority,
        };

        let entry = self.groups.entry(name.to_string()).or_insert_with(|| AlternativeGroup {
            name: name.to_string(),
            master_link: master_link.to_string(),
            choices: Vec::new(),
            selected_path: None,
            is_auto: true,
        });

        if !entry.choices.iter().any(|c| c.path == path) {
            entry.choices.push(choice);
        }

        if entry.is_auto {
            self.recalculate_auto_selection(name);
        }
    }

    pub fn recalculate_auto_selection(&mut self, name: &str) {
        if let Some(group) = self.groups.get_mut(name) {
            if group.is_auto {
                group.selected_path = group
                    .choices
                    .iter()
                    .max_by_key(|c| c.priority)
                    .map(|c| c.path.clone());
            }
        }
    }

    pub fn set_manual_selection(&mut self, name: &str, path: &str) -> Result<(), &'static str> {
        let group = self
            .groups
            .get_mut(name)
            .ok_or("update-alternatives: Group not found")?;

        if !group.choices.iter().any(|c| c.path == path) {
            return Err("update-alternatives: Path not registered in group choices");
        }

        group.selected_path = Some(path.to_string());
        group.is_auto = false;
        Ok(())
    }

    pub fn auto_select(&mut self, name: &str) -> Result<(), &'static str> {
        let group = self
            .groups
            .get_mut(name)
            .ok_or("update-alternatives: Group not found")?;

        group.is_auto = true;
        self.recalculate_auto_selection(name);
        Ok(())
    }

    pub fn get_active_path(&self, name: &str) -> Option<String> {
        self.groups.get(name).and_then(|g| g.selected_path.clone())
    }
}

// =========================================================================
// 2. DEBIAN apt-mark PACKAGE MARK STATUS MANAGER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageMarkStatus {
    Auto,
    Manual,
    Hold,
}

#[derive(Debug, Clone, Default)]
pub struct DebianAptMarkEngine {
    pub marks: BTreeMap<String, PackageMarkStatus>,
}

impl DebianAptMarkEngine {
    pub fn new() -> Self {
        Self {
            marks: BTreeMap::new(),
        }
    }

    pub fn mark_package(&mut self, pkg: &str, status: PackageMarkStatus) {
        self.marks.insert(pkg.to_string(), status);
    }

    pub fn get_mark_status(&self, pkg: &str) -> PackageMarkStatus {
        self.marks
            .get(pkg)
            .copied()
            .unwrap_or(PackageMarkStatus::Manual)
    }

    pub fn is_held(&self, pkg: &str) -> bool {
        self.get_mark_status(pkg) == PackageMarkStatus::Hold
    }
}

// =========================================================================
// 3. DEBIAN dpkg-trigger DEFERRED PACKAGE HOOK QUEUE
// =========================================================================

#[derive(Debug, Clone, Default)]
pub struct DpkgTrigger {
    pub name: String,
    pub interested_packages: Vec<String>,
    pub is_activated: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DebianDpkgTriggerEngine {
    pub registered_triggers: BTreeMap<String, DpkgTrigger>,
}

impl DebianDpkgTriggerEngine {
    pub fn new() -> Self {
        Self {
            registered_triggers: BTreeMap::new(),
        }
    }

    pub fn register_trigger_interest(&mut self, trigger_name: &str, pkg_name: &str) {
        let trigger = self
            .registered_triggers
            .entry(trigger_name.to_string())
            .or_insert_with(|| DpkgTrigger {
                name: trigger_name.to_string(),
                interested_packages: Vec::new(),
                is_activated: false,
            });

        if !trigger.interested_packages.contains(&pkg_name.to_string()) {
            trigger.interested_packages.push(pkg_name.to_string());
        }
    }

    pub fn activate_trigger(&mut self, trigger_name: &str) -> usize {
        if let Some(trigger) = self.registered_triggers.get_mut(trigger_name) {
            trigger.is_activated = true;
            trigger.interested_packages.len()
        } else {
            0
        }
    }

    pub fn flush_pending_triggers(&mut self) -> Vec<(String, String)> {
        let mut notifications = Vec::new();
        for trigger in self.registered_triggers.values_mut() {
            if trigger.is_activated {
                for pkg in &trigger.interested_packages {
                    notifications.push((pkg.clone(), trigger.name.clone()));
                }
                trigger.is_activated = false;
            }
        }
        notifications
    }
}

// =========================================================================
// 4. DEBIAN MULTI-ARCH ARCHITECTURE PATH RESOLVER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiarchTriplet {
    X86_64LinuxGnu,
    AArch64LinuxGnu,
    RiscV64LinuxGnu,
    I386LinuxGnu,
}

pub struct DebianMultiarchPathResolver;

impl DebianMultiarchPathResolver {
    pub fn resolve_triplet_str(triplet: MultiarchTriplet) -> &'static str {
        match triplet {
            MultiarchTriplet::X86_64LinuxGnu => "x86_64-linux-gnu",
            MultiarchTriplet::AArch64LinuxGnu => "aarch64-linux-gnu",
            MultiarchTriplet::RiscV64LinuxGnu => "riscv64-linux-gnu",
            MultiarchTriplet::I386LinuxGnu => "i386-linux-gnu",
        }
    }

    pub fn resolve_lib_dir(triplet: MultiarchTriplet) -> String {
        format!("/usr/lib/{}", Self::resolve_triplet_str(triplet))
    }

    pub fn resolve_include_dir(triplet: MultiarchTriplet) -> String {
        format!("/usr/include/{}", Self::resolve_triplet_str(triplet))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::format;
    use std::vec;

    #[test]
    fn test_debian_control_parsing() {
        let control_text = "Package: nginx\n\
                            Version: 1.22.1-1\n\
                            Architecture: amd64\n\
                            Depends: libc6, libssl3, zlib1g\n\
                            Maintainer: Debian Nginx Maintainers <pkg-nginx-maintainers@alioth-lists.debian.net>\n\
                            Description: high-performance web server\n";

        let control = DebControl::parse(control_text).unwrap();
        assert_eq!(control.package, "nginx");
        assert_eq!(control.version, "1.22.1-1");
        assert_eq!(control.architecture, "amd64");
        assert_eq!(control.depends.len(), 3);
        assert_eq!(control.depends[0], "libc6");
        assert_eq!(control.depends[1], "libssl3");
        assert_eq!(control.depends[2], "zlib1g");
        assert_eq!(
            control.maintainer,
            "Debian Nginx Maintainers <pkg-nginx-maintainers@alioth-lists.debian.net>"
        );
        assert_eq!(control.description, "high-performance web server");
    }

    #[test]
    fn test_debian_deb_package_binary_parsing() {
        // Construct a programmatically valid mock .deb package (ar archive format)
        let mut binary_data = Vec::new();
        // 1. Signature
        binary_data.extend_from_slice(b"!<arch>\n");

        // 2. Member: debian-binary (size = 4 bytes)
        let mut header1 = [b' '; 60];
        header1[0..13].copy_from_slice(b"debian-binary");
        header1[48..58].copy_from_slice(b"4         ");
        header1[58..60].copy_from_slice(b"\x60\x0A");
        binary_data.extend_from_slice(&header1);
        binary_data.extend_from_slice(b"2.0\n");

        // 3. Member: control (size = dynamic)
        let control_data = "Package: curl\nVersion: 7.88.1-8\nDescription: command line tool for transferring data\n";
        let mut header2 = [b' '; 60];
        header2[0..7].copy_from_slice(b"control");
        let size_str = format!("{:<10}", control_data.len());
        header2[48..58].copy_from_slice(size_str.as_bytes());
        header2[58..60].copy_from_slice(b"\x60\x0A");
        binary_data.extend_from_slice(&header2);
        binary_data.extend_from_slice(control_data.as_bytes());

        // Align to even byte boundary if odd
        if control_data.len() % 2 != 0 {
            binary_data.push(0);
        }

        let deb = DebPackage::parse_binary(&binary_data).unwrap();
        assert_eq!(deb.control.package, "curl");
        assert_eq!(deb.control.version, "7.88.1-8");
        assert_eq!(
            deb.control.description,
            "command line tool for transferring data"
        );
    }

    #[test]
    fn test_debian_control_unsafe_package_name_rejected() {
        let unsafe_control = "Package: ../../etc/passwd\nVersion: 1.0\n";
        assert!(DebControl::parse(unsafe_control).is_err());

        let invalid_char_control = "Package: nginx;rm -rf /\nVersion: 1.0\n";
        assert!(DebControl::parse(invalid_char_control).is_err());

        let dot_start_control = "Package: .hidden_pkg\nVersion: 1.0\n";
        assert!(DebControl::parse(dot_start_control).is_err());
    }

    #[test]
    fn test_debian_sources_list_parsing() {
        let sources_text = "# Mock Debian Sources List\n\
                            deb http://deb.debian.org/debian bookworm main contrib non-free\n\
                            deb-src http://security.debian.org/debian-security bookworm-security main\n";

        let sources = parse_sources_list(sources_text).unwrap();
        assert_eq!(sources.len(), 2);

        // First source
        assert!(!sources[0].is_source);
        assert_eq!(sources[0].uri, "http://deb.debian.org/debian");
        assert_eq!(sources[0].suite, "bookworm");
        assert_eq!(sources[0].components.len(), 3);
        assert_eq!(sources[0].components[0], "main");
        assert_eq!(sources[0].components[2], "non-free");

        // Second source
        assert!(sources[1].is_source);
        assert_eq!(sources[1].uri, "http://security.debian.org/debian-security");
        assert_eq!(sources[1].suite, "bookworm-security");
        assert_eq!(sources[1].components.len(), 1);
        assert_eq!(sources[1].components[0], "main");
    }

    #[test]
    fn test_debian_dpkg_status_parsing() {
        let status_text = "Package: libc6\n\
                           Status: install ok installed\n\
                           Priority: required\n\
                           Section: libs\n\
                           Installed-Size: 12000\n\
                           Architecture: amd64\n\
                           Version: 2.36-9\n\
                           Description: GNU C Library: Shared libraries\n\
                           \n\
                           Package: dpkg\n\
                           Status: install ok installed\n\
                           Priority: required\n\
                           Section: admin\n\
                           Installed-Size: 8500\n\
                           Architecture: amd64\n\
                           Version: 1.21.21\n\
                           Description: Debian package management system\n";

        let entries = parse_dpkg_status(status_text);
        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].package, "libc6");
        assert_eq!(entries[0].status, "install ok installed");
        assert_eq!(entries[0].installed_size, 12000);
        assert_eq!(entries[0].architecture, "amd64");
        assert_eq!(entries[0].version, "2.36-9");

        assert_eq!(entries[1].package, "dpkg");
        assert_eq!(entries[1].status, "install ok installed");
        assert_eq!(entries[1].installed_size, 8500);
        assert_eq!(entries[1].version, "1.21.21");
    }

    #[test]
    fn test_debian_update_alternatives_engine() {
        let mut alts = DebianUpdateAlternativesEngine::new();
        alts.install_alternative("editor", "/usr/bin/editor", "/usr/bin/vim.basic", 50);
        alts.install_alternative("editor", "/usr/bin/editor", "/usr/bin/nano", 40);

        // Highest priority (50) auto-selected
        assert_eq!(alts.get_active_path("editor"), Some("/usr/bin/vim.basic".to_string()));

        // Install higher priority choice (100)
        alts.install_alternative("editor", "/usr/bin/editor", "/usr/bin/neovim", 100);
        assert_eq!(alts.get_active_path("editor"), Some("/usr/bin/neovim".to_string()));

        // Manual override
        assert!(alts.set_manual_selection("editor", "/usr/bin/nano").is_ok());
        assert_eq!(alts.get_active_path("editor"), Some("/usr/bin/nano".to_string()));

        // Return to auto mode
        assert!(alts.auto_select("editor").is_ok());
        assert_eq!(alts.get_active_path("editor"), Some("/usr/bin/neovim".to_string()));
    }

    #[test]
    fn test_debian_apt_mark_engine() {
        let mut mark = DebianAptMarkEngine::new();
        assert_eq!(mark.get_mark_status("curl"), PackageMarkStatus::Manual);

        mark.mark_package("libssl3", PackageMarkStatus::Auto);
        mark.mark_package("kernel-hold", PackageMarkStatus::Hold);

        assert_eq!(mark.get_mark_status("libssl3"), PackageMarkStatus::Auto);
        assert!(mark.is_held("kernel-hold"));
        assert!(!mark.is_held("libssl3"));
    }

    #[test]
    fn test_debian_dpkg_trigger_engine() {
        let mut triggers = DebianDpkgTriggerEngine::new();
        triggers.register_trigger_interest("ldconfig", "libc6");
        triggers.register_trigger_interest("ldconfig", "libssl3");

        assert_eq!(triggers.activate_trigger("ldconfig"), 2);

        let pending = triggers.flush_pending_triggers();
        assert_eq!(pending.len(), 2);
        assert_eq!(pending[0], ("libc6".to_string(), "ldconfig".to_string()));
        assert_eq!(pending[1], ("libssl3".to_string(), "ldconfig".to_string()));

        // After flushing, queue is cleared
        assert!(triggers.flush_pending_triggers().is_empty());
    }

    #[test]
    fn test_debian_multiarch_path_resolver() {
        assert_eq!(
            DebianMultiarchPathResolver::resolve_lib_dir(MultiarchTriplet::X86_64LinuxGnu),
            "/usr/lib/x86_64-linux-gnu"
        );
        assert_eq!(
            DebianMultiarchPathResolver::resolve_include_dir(MultiarchTriplet::AArch64LinuxGnu),
            "/usr/include/aarch64-linux-gnu"
        );
    }
}
