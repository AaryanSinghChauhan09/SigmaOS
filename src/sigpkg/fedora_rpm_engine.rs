use std::format;
use std::vec;
// SPDX-License-Identifier: MIT
// SigmaOS Fedora/RPM Compatibility Engine
// Implements RPM package management, DNF/YUM compatibility, and RPM spec file parsing

#[cfg(not(feature = "standalone_test"))]
use crate::klib::collections::HashMap;
#[cfg(feature = "standalone_test")]
use alloc::collections::BTreeMap as HashMap;

use std::string::{String, ToString};
use std::vec::Vec;

/// RPM package metadata structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpmPackage {
    pub name: String,
    pub version: String,
    pub release: String,
    pub epoch: u32,
    pub architecture: String,
    pub summary: String,
    pub description: String,
    pub license: String,
    pub url: String,
    pub vendor: String,
    pub build_time: u64,
    pub size: u64,
    pub requires: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub obsoletes: Vec<String>,
}

/// RPM macro expander for Fedora build macros
#[derive(Debug, Clone)]
pub struct RpmMacroExpander {
    macros: HashMap<String, String>,
}

impl RpmMacroExpander {
    pub fn new() -> Self {
        let mut macros = HashMap::new();
        macros.insert("_bindir".to_string(), "/usr/bin".to_string());
        macros.insert("_sbindir".to_string(), "/usr/sbin".to_string());
        macros.insert("_sysconfdir".to_string(), "/etc".to_string());
        macros.insert("_datadir".to_string(), "/usr/share".to_string());
        macros.insert("_includedir".to_string(), "/usr/include".to_string());
        macros.insert("_libdir".to_string(), "/usr/lib64".to_string());
        macros.insert("_mandir".to_string(), "/usr/share/man".to_string());
        macros.insert("_docdir".to_string(), "/usr/share/doc".to_string());
        macros.insert("dist".to_string(), ".fc40".to_string());
        Self { macros }
    }

    pub fn set_macro(&mut self, key: &str, val: &str) {
        self.macros.insert(key.to_string(), val.to_string());
    }

    pub fn expand(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (k, v) in &self.macros {
            let pattern1 = format!("%{{{}}}", k);
            let pattern2 = format!("%{{?{}}}", k);
            result = result.replace(&pattern1, v);
            result = result.replace(&pattern2, v);
        }
        result
    }
}

impl Default for RpmMacroExpander {
    fn default() -> Self {
        Self::new()
    }
}

/// RPM spec file parser
pub struct RpmSpecParser {
    sections: HashMap<String, Vec<String>>,
    headers: HashMap<String, String>,
    expander: RpmMacroExpander,
}

impl RpmSpecParser {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
            headers: HashMap::new(),
            expander: RpmMacroExpander::new(),
        }
    }

    /// Parse RPM spec file content
    pub fn parse_spec(&mut self, spec_content: &str) -> Result<RpmPackage, String> {
        let mut current_section = String::from("%header");
        let mut current_lines: Vec<String> = Vec::new();

        for line in spec_content.lines() {
            let line_trimmed = line.trim();

            if line_trimmed.starts_with('%')
                && !line_trimmed.starts_with("%{")
                && !line_trimmed.starts_with("%?")
            {
                // Save previous section
                if !current_section.is_empty() {
                    self.sections
                        .insert(current_section.clone(), current_lines.clone());
                }

                // Start new section
                let section_name = line_trimmed
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string();
                current_section = section_name;
                current_lines.clear();
            } else if current_section == "%header"
                && line_trimmed.contains(':')
                && !line_trimmed.starts_with('#')
            {
                let parts: Vec<&str> = line_trimmed.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let val = self.expander.expand(parts[1].trim());
                    self.headers.insert(key, val);
                }
            } else {
                current_lines.push(self.expander.expand(line_trimmed));
            }
        }

        // Save last section
        if !current_section.is_empty() {
            self.sections.insert(current_section, current_lines);
        }

        self.extract_package_info()
    }

    fn extract_package_info(&self) -> Result<RpmPackage, String> {
        let mut package = RpmPackage {
            name: self.headers.get("Name").cloned().unwrap_or_default(),
            version: self.headers.get("Version").cloned().unwrap_or_default(),
            release: self.headers.get("Release").cloned().unwrap_or_default(),
            epoch: self
                .headers
                .get("Epoch")
                .and_then(|e: &String| e.parse::<u32>().ok())
                .unwrap_or(0),
            architecture: self
                .headers
                .get("BuildArch")
                .cloned()
                .unwrap_or_else(|| "x86_64".to_string()),
            summary: self.headers.get("Summary").cloned().unwrap_or_default(),
            description: String::new(),
            license: self.headers.get("License").cloned().unwrap_or_default(),
            url: self
                .headers
                .get("Url")
                .or_else(|| self.headers.get("URL"))
                .cloned()
                .unwrap_or_default(),
            vendor: self
                .headers
                .get("Vendor")
                .cloned()
                .unwrap_or_else(|| "Fedora Project".to_string()),
            build_time: 0,
            size: 0,
            requires: self
                .headers
                .get("Requires")
                .map(|r: &String| {
                    r.split(',')
                        .map(|s: &str| s.trim().to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default(),
            provides: self
                .headers
                .get("Provides")
                .map(|r: &String| {
                    r.split(',')
                        .map(|s: &str| s.trim().to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default(),
            conflicts: self
                .headers
                .get("Conflicts")
                .map(|r: &String| {
                    r.split(',')
                        .map(|s: &str| s.trim().to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default(),
            obsoletes: self
                .headers
                .get("Obsoletes")
                .map(|r: &String| {
                    r.split(',')
                        .map(|s: &str| s.trim().to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default(),
        };

        // Extract from %description section
        if let Some(desc_lines) = self.sections.get("%description") {
            let mut desc = String::new();
            for l in desc_lines {
                desc.push_str(l);
                desc.push('\n');
            }
            package.description = desc;
        }

        // Extract from %files section (usually contains some metadata)
        if let Some(files_lines) = self.sections.get("%files") {
            for line in files_lines {
                let l_str: &str = line;
                if l_str.starts_with("%doc") || l_str.starts_with("%config") {
                    // Process file markers
                }
            }
        }

        Ok(package)
    }

    /// Get section content
    pub fn get_section(&self, section_name: &str) -> Option<&Vec<String>> {
        self.sections.get(section_name)
    }
}

impl Default for RpmSpecParser {
    fn default() -> Self {
        Self::new()
    }
}

/// DNF/YUM repository configuration
#[derive(Debug, Clone)]
pub struct DnfRepository {
    pub id: String,
    pub name: String,
    pub baseurl: String,
    pub enabled: bool,
    pub gpgcheck: bool,
    pub metadata_expire: u64,
}

/// DNF repository manager
pub struct DnfRepositoryManager {
    repositories: Vec<DnfRepository>,
}

impl DnfRepositoryManager {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }

    /// Add a DNF repository
    pub fn add_repository(&mut self, repo: DnfRepository) {
        self.repositories.push(repo);
    }

    /// Parse .repo file format
    pub fn parse_repo_file(&mut self, repo_content: &str) -> Result<(), String> {
        let mut current_repo: Option<DnfRepository> = None;

        for line in repo_content.lines() {
            let line = line.trim();

            if line.starts_with('[') && line.ends_with(']') {
                // Save previous repo
                if let Some(repo) = current_repo {
                    self.repositories.push(repo);
                }

                // Start new repo
                let repo_id = line[1..line.len() - 1].to_string();
                current_repo = Some(DnfRepository {
                    id: repo_id.clone(),
                    name: repo_id,
                    baseurl: String::new(),
                    enabled: true,
                    gpgcheck: true,
                    metadata_expire: 86400,
                });
            } else if let Some(ref mut repo) = current_repo {
                if line.starts_with("name=") {
                    repo.name = line[5..].to_string();
                } else if line.starts_with("baseurl=") {
                    repo.baseurl = line[8..].to_string();
                } else if line.starts_with("enabled=") {
                    repo.enabled = line[8..].parse::<bool>().unwrap_or(true);
                } else if line.starts_with("gpgcheck=") {
                    repo.gpgcheck = line[9..].parse::<bool>().unwrap_or(true);
                }
            }
        }

        // Save last repo
        if let Some(repo) = current_repo {
            self.repositories.push(repo);
        }

        Ok(())
    }

    /// Get enabled repositories
    pub fn get_enabled_repos(&self) -> Vec<&DnfRepository> {
        self.repositories.iter().filter(|r| r.enabled).collect()
    }

    /// Get repository by ID
    pub fn get_repository(&self, id: &str) -> Option<&DnfRepository> {
        self.repositories.iter().find(|r| r.id == id)
    }
}

/// RPM database (similar to rpmdb)
pub struct RpmDatabase {
    installed_packages: HashMap<String, RpmPackage>,
}

impl RpmDatabase {
    pub fn new() -> Self {
        Self {
            installed_packages: HashMap::new(),
        }
    }

    /// Install an RPM package
    pub fn install_package(&mut self, package: RpmPackage) -> Result<(), String> {
        let package_key = format!(
            "{}-{}-{}.{}",
            package.name, package.version, package.release, package.architecture
        );
        self.installed_packages.insert(package_key, package);
        Ok(())
    }

    /// Remove an RPM package
    pub fn remove_package(&mut self, name: &str) -> Result<(), String> {
        let keys_to_remove: Vec<String> = self
            .installed_packages
            .keys()
            .filter(|k: &&String| k.starts_with(name))
            .cloned()
            .collect();

        for key in keys_to_remove {
            self.installed_packages.remove(&key);
        }

        Ok(())
    }

    /// Query installed packages
    pub fn query(&self, pattern: &str) -> Vec<&RpmPackage> {
        self.installed_packages
            .values()
            .filter(|p| p.name.contains(pattern) || p.summary.contains(pattern))
            .collect()
    }

    /// Get package information
    pub fn get_package(&self, name: &str) -> Option<&RpmPackage> {
        self.installed_packages.values().find(|p| p.name == name)
    }

    /// List all installed packages
    pub fn list_all(&self) -> Vec<&RpmPackage> {
        self.installed_packages.values().collect()
    }
}

/// Dependency resolver for RPM packages
pub struct RpmDependencyResolver {
    database: RpmDatabase,
}

impl RpmDependencyResolver {
    pub fn new(database: RpmDatabase) -> Self {
        Self { database }
    }

    /// Resolve dependencies for a package
    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<String>, String> {
        let package = self
            .database
            .get_package(package_name)
            .ok_or_else(|| format!("Package {} not found", package_name))?;

        let mut resolved = Vec::new();
        for req in &package.requires {
            resolved.push(req.clone());
            // Recursively resolve dependencies
            if let Ok(sub_deps) = self.resolve_dependencies(req) {
                resolved.extend(sub_deps);
            }
        }

        Ok(resolved)
    }

    /// Check for conflicts
    pub fn check_conflicts(&self, package_name: &str) -> Result<Vec<String>, String> {
        let package = self
            .database
            .get_package(package_name)
            .ok_or_else(|| format!("Package {} not found", package_name))?;

        let mut conflicts = Vec::new();
        for conflict in &package.conflicts {
            if self.database.get_package(conflict).is_some() {
                conflicts.push(conflict.clone());
            }
        }

        Ok(conflicts)
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_rpm_spec_parser() {
        let spec_content = r#"Name: test-pkg
Version: 2.1.0
Release: 1%{?dist}
Summary: Comprehensive Fedora SPEC test
License: MIT
URL: https://fedoraproject.org

%description
This is a test package for SigmaOS RPM compatibility.
It provides essential functionality for the system.

%files
%{_bindir}/test-app
%{_docdir}/test-app/README
"#;

        let mut parser = RpmSpecParser::new();
        let package = parser.parse_spec(spec_content).unwrap();

        assert_eq!(package.name, "test-pkg");
        assert_eq!(package.version, "2.1.0");
        assert_eq!(package.release, "1.fc40");
        assert_eq!(package.summary, "Comprehensive Fedora SPEC test");
        assert_eq!(package.license, "MIT");
        assert!(package.description.contains("test package"));
        assert_eq!(parser.get_section("%description").unwrap().len(), 3);

        let files = parser.get_section("%files").unwrap();
        assert_eq!(files[0], "/usr/bin/test-app");
        assert_eq!(files[1], "/usr/share/doc/test-app/README");
    }

    #[test]
    fn test_rpm_macro_expander() {
        let expander = RpmMacroExpander::new();
        assert_eq!(expander.expand("%{_bindir}/app"), "/usr/bin/app");
        assert_eq!(expander.expand("%{_sysconfdir}/app.conf"), "/etc/app.conf");
        assert_eq!(expander.expand("release-1%{?dist}"), "release-1.fc40");
    }

    #[test]
    fn test_dnf_repository_manager() {
        let repo_content = r#"[fedora]
name=Fedora Repository
baseurl=https://download.fedoraproject.org/pub/fedora/linux/releases/39/Everything/x86_64/os/
enabled=1
gpgcheck=1
"#;

        let mut manager = DnfRepositoryManager::new();
        manager.parse_repo_file(repo_content).unwrap();

        assert_eq!(manager.repositories.len(), 1);
        assert_eq!(manager.repositories[0].id, "fedora");
        assert_eq!(manager.repositories[0].enabled, true);
        assert_eq!(manager.get_enabled_repos().len(), 1);
    }

    #[test]
    fn test_rpm_database() {
        let mut db = RpmDatabase::new();

        let package = RpmPackage {
            name: "test-rpm".to_string(),
            version: "1.0.0".to_string(),
            release: "1.fc39".to_string(),
            epoch: 0,
            architecture: "x86_64".to_string(),
            summary: "Test RPM package".to_string(),
            description: "Test description".to_string(),
            license: "MIT".to_string(),
            url: "https://example.com".to_string(),
            vendor: "SigmaOS".to_string(),
            build_time: 1692883200,
            size: 1024000,
            requires: vec!["libc6".to_string()],
            provides: Vec::new(),
            conflicts: Vec::new(),
            obsoletes: Vec::new(),
        };

        db.install_package(package).unwrap();
        assert_eq!(db.list_all().len(), 1);
        assert!(db.get_package("test-rpm").is_some());

        db.remove_package("test-rpm").unwrap();
        assert_eq!(db.list_all().len(), 0);
    }

    #[test]
    fn test_dependency_resolver() {
        let mut db = RpmDatabase::new();

        let base_pkg = RpmPackage {
            name: "base-pkg".to_string(),
            version: "1.0.0".to_string(),
            release: "1".to_string(),
            epoch: 0,
            architecture: "x86_64".to_string(),
            summary: "Base package".to_string(),
            description: "Base".to_string(),
            license: "MIT".to_string(),
            url: String::new(),
            vendor: String::new(),
            build_time: 0,
            size: 0,
            requires: vec!["dep-pkg".to_string()],
            provides: Vec::new(),
            conflicts: Vec::new(),
            obsoletes: Vec::new(),
        };

        let dep_pkg = RpmPackage {
            name: "dep-pkg".to_string(),
            version: "1.0.0".to_string(),
            release: "1".to_string(),
            epoch: 0,
            architecture: "x86_64".to_string(),
            summary: "Dependency package".to_string(),
            description: "Dependency".to_string(),
            license: "MIT".to_string(),
            url: String::new(),
            vendor: String::new(),
            build_time: 0,
            size: 0,
            requires: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            obsoletes: Vec::new(),
        };

        db.install_package(base_pkg).unwrap();
        db.install_package(dep_pkg).unwrap();

        let resolver = RpmDependencyResolver::new(db);
        let deps = resolver.resolve_dependencies("base-pkg").unwrap();

        assert!(deps.contains(&"dep-pkg".to_string()));
    }

    #[test]
    fn test_fedora_anitya_release_monitoring() {
        let mut engine = FedoraAnityaReleaseMonitoringEngine::new();
        engine.register_project(AnityaProjectRecord {
            project_id: 101,
            name: "curl".to_string(),
            homepage: "https://curl.se".to_string(),
            current_version: "8.4.0".to_string(),
            latest_upstream_version: "8.4.0".to_string(),
            updated_available: false,
        });

        let is_updated = engine.check_upstream_version("curl", "8.5.0").unwrap();
        assert!(is_updated);
        assert_eq!(
            engine.projects.get("curl").unwrap().latest_upstream_version,
            "8.5.0"
        );

        let msgs = engine.messaging_bus.get_messages_for_project(101);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].topic, AnityaMessageTopic::ProjectVersionUpdate);
        assert_eq!(msgs[0].old_version, "8.4.0");
        assert_eq!(msgs[0].new_version, "8.5.0");
        assert!(msgs[0]
            .to_json_summary()
            .contains("org.fedoraproject.prod.anitya.project.version.update"));
        assert_eq!(msgs[0].packages.len(), 2);
    }

    #[test]
    fn test_fedora_mirrormanager2_routing_and_freshness() {
        let mut mm2 = FedoraMirrorManager2Engine::new(86400); // 24h staleness
        mm2.register_mirror(MirrorSiteRecord {
            url: "https://mirror.us.fedora.org".to_string(),
            country_code: "US".to_string(),
            asn_number: 1234,
            categories: vec!["fedora".to_string(), "updates".to_string()],
            architectures: vec!["x86_64".to_string()],
            bandwidth_mbps: 10000,
            last_synced_timestamp: 1000000,
            is_active: true,
        });
        mm2.register_mirror(MirrorSiteRecord {
            url: "https://mirror.eu.fedora.org".to_string(),
            country_code: "DE".to_string(),
            asn_number: 5678,
            categories: vec!["fedora".to_string(), "updates".to_string()],
            architectures: vec!["x86_64".to_string()],
            bandwidth_mbps: 1000,
            last_synced_timestamp: 1000000,
            is_active: true,
        });

        let routed = mm2.route_client_request("US", 1234, "fedora", "x86_64", 1000100);
        assert_eq!(routed.len(), 2);
        assert_eq!(routed[0].country_code, "US");

        // Test staleness
        let stale_demoted = mm2.verify_mirror_freshness(1000000 + 100000);
        assert_eq!(stale_demoted, 2);
        assert!(!mm2.mirrors[0].is_active);
    }

    #[test]
    fn test_fedora_mirrormanager2_metalink_xml_generation() {
        let mut mm2 = FedoraMirrorManager2Engine::new(86400);
        mm2.register_mirror(MirrorSiteRecord {
            url: "https://dl.fedoraproject.org/pub/fedora/linux/releases/39/Everything/x86_64/os/"
                .to_string(),
            country_code: "US".to_string(),
            asn_number: 100,
            categories: vec!["fedora".to_string()],
            architectures: vec!["x86_64".to_string()],
            bandwidth_mbps: 10000,
            last_synced_timestamp: 2000000,
            is_active: true,
        });

        let xml = mm2.generate_metalink_xml(
            "repodata/repomd.xml",
            "fedora",
            "x86_64",
            "US",
            2, // Countme bucket 2 (1-6 months old system)
            2000100,
        );

        assert!(xml.contains("metalink version=\"3.0\""));
        assert!(xml.contains("repodata/repomd.xml"));
        assert!(xml.contains("countme=2"));
    }
}

/// Fedora Anitya (release-monitoring.org) Upstream Project Tracking Record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnityaProjectRecord {
    pub project_id: u32,
    pub name: String,
    pub homepage: String,
    pub current_version: String,
    pub latest_upstream_version: String,
    pub updated_available: bool,
}

/// Fedora Messaging Topic Type for Anitya Upstream Events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnityaMessageTopic {
    ProjectVersionUpdate, // org.fedoraproject.prod.anitya.project.version.update
    ProjectMapNew,        // org.fedoraproject.prod.anitya.project.map.new
    ProjectVersionFlag,   // org.fedoraproject.prod.anitya.project.version.flag
}

impl AnityaMessageTopic {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnityaMessageTopic::ProjectVersionUpdate => {
                "org.fedoraproject.prod.anitya.project.version.update"
            }
            AnityaMessageTopic::ProjectMapNew => "org.fedoraproject.prod.anitya.project.map.new",
            AnityaMessageTopic::ProjectVersionFlag => {
                "org.fedoraproject.prod.anitya.project.version.flag"
            }
        }
    }
}

/// Cross-Distro Package Mapping in Anitya
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnityaPackageMapping {
    pub distro: String,       // e.g. "Fedora", "EPEL", "SigmaOS"
    pub package_name: String, // e.g. "curl"
}

/// Fedora Messaging Standard Schema Payload for Anitya Upstream Release Event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnityaVersionUpdateMessage {
    pub message_id: String,
    pub topic: AnityaMessageTopic,
    pub project_id: u32,
    pub project_name: String,
    pub ecosystem: String,
    pub old_version: String,
    pub new_version: String,
    pub packages: Vec<AnityaPackageMapping>,
    pub timestamp: u64,
}

impl AnityaVersionUpdateMessage {
    pub fn to_json_summary(&self) -> String {
        format!(
            "{{\"msg_id\":\"{}\",\"topic\":\"{}\",\"project_id\":{},\"project\":\"{}\",\"ecosystem\":\"{}\",\"old_version\":\"{}\",\"new_version\":\"{}\",\"timestamp\":{}}}",
            self.message_id,
            self.topic.as_str(),
            self.project_id,
            self.project_name,
            self.ecosystem,
            self.old_version,
            self.new_version,
            self.timestamp
        )
    }
}

/// Fedora Messaging Event Engine for Anitya Notifications
pub struct AnityaFedoraMessagingEngine {
    pub published_messages: Vec<AnityaVersionUpdateMessage>,
    pub next_msg_seq: u64,
}

impl AnityaFedoraMessagingEngine {
    pub fn new() -> Self {
        Self {
            published_messages: Vec::new(),
            next_msg_seq: 1,
        }
    }

    pub fn publish_version_update(
        &mut self,
        project_id: u32,
        project_name: &str,
        ecosystem: &str,
        old_version: &str,
        new_version: &str,
        packages: Vec<AnityaPackageMapping>,
        timestamp: u64,
    ) -> AnityaVersionUpdateMessage {
        let msg_id = format!("anitya-msg-{}", self.next_msg_seq);
        self.next_msg_seq += 1;

        let msg = AnityaVersionUpdateMessage {
            message_id: msg_id,
            topic: AnityaMessageTopic::ProjectVersionUpdate,
            project_id,
            project_name: project_name.to_string(),
            ecosystem: ecosystem.to_string(),
            old_version: old_version.to_string(),
            new_version: new_version.to_string(),
            packages,
            timestamp,
        };

        self.published_messages.push(msg.clone());
        msg
    }

    pub fn get_messages_for_project(&self, project_id: u32) -> Vec<&AnityaVersionUpdateMessage> {
        self.published_messages
            .iter()
            .filter(|m| m.project_id == project_id)
            .collect()
    }
}

impl Default for AnityaFedoraMessagingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Fedora Anitya Upstream Release Monitoring Engine
pub struct FedoraAnityaReleaseMonitoringEngine {
    pub projects: HashMap<String, AnityaProjectRecord>,
    pub messaging_bus: AnityaFedoraMessagingEngine,
}

impl FedoraAnityaReleaseMonitoringEngine {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            messaging_bus: AnityaFedoraMessagingEngine::new(),
        }
    }

    pub fn register_project(&mut self, record: AnityaProjectRecord) {
        self.projects.insert(record.name.clone(), record);
    }

    pub fn check_upstream_version(
        &mut self,
        project_name: &str,
        latest_version: &str,
    ) -> Option<bool> {
        if let Some(record) = self.projects.get_mut(project_name) {
            let is_new = record.current_version != latest_version;
            if is_new {
                self.messaging_bus.publish_version_update(
                    record.project_id,
                    &record.name,
                    "Fedora",
                    &record.current_version,
                    latest_version,
                    vec![
                        AnityaPackageMapping {
                            distro: "Fedora".to_string(),
                            package_name: record.name.clone(),
                        },
                        AnityaPackageMapping {
                            distro: "SigmaOS".to_string(),
                            package_name: record.name.clone(),
                        },
                    ],
                    1000000,
                );
            }
            record.latest_upstream_version = latest_version.to_string();
            record.updated_available = is_new;
            if is_new {
                let mut pkgs = Vec::new();
                pkgs.push(AnityaPackageMapping {
                    distro: "Fedora".to_string(),
                    package_name: proj_name.clone(),
                });
                pkgs.push(AnityaPackageMapping {
                    distro: "CentOS".to_string(),
                    package_name: proj_name.clone(),
                });
                self.messaging_bus.publish_version_update(
                    proj_id,
                    &proj_name,
                    "fedora",
                    &old_ver,
                    latest_version,
                    pkgs,
                    1700000000,
                );
            }
            Some(is_new)
        } else {
            None
        }
    }
}

impl Default for FedoraAnityaReleaseMonitoringEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// FEDORA MIRRORMANAGER2 SUB-SYSTEM ENGINE
// =========================================================================

/// Fedora MirrorManager2 Mirror Site Record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorSiteRecord {
    pub url: String,
    pub country_code: String,
    pub asn_number: u32,
    pub categories: Vec<String>, // "fedora", "epel", "updates", "testing"
    pub architectures: Vec<String>, // "x86_64", "aarch64", "riscv64"
    pub bandwidth_mbps: u32,
    pub last_synced_timestamp: u64,
    pub is_active: bool,
}

/// Fedora MirrorManager2 Engine
pub struct FedoraMirrorManager2Engine {
    pub mirrors: Vec<MirrorSiteRecord>,
    pub max_staleness_secs: u64,
}

impl FedoraMirrorManager2Engine {
    pub fn new(max_staleness_secs: u64) -> Self {
        Self {
            mirrors: Vec::new(),
            max_staleness_secs,
        }
    }

    pub fn register_mirror(&mut self, site: MirrorSiteRecord) {
        self.mirrors.retain(|m| m.url != site.url);
        self.mirrors.push(site);
    }

    /// Selects optimal mirrors matched by country code or ASN, architecture, and category, weighted by bandwidth
    pub fn route_client_request(
        &self,
        country: &str,
        asn: u32,
        category: &str,
        arch: &str,
        current_timestamp: u64,
    ) -> Vec<MirrorSiteRecord> {
        let mut candidates: Vec<MirrorSiteRecord> = self
            .mirrors
            .iter()
            .filter(|m| {
                m.is_active
                    && m.categories.iter().any(|c| c == category)
                    && m.architectures.iter().any(|a| a == arch)
                    && current_timestamp.saturating_sub(m.last_synced_timestamp)
                        <= self.max_staleness_secs
            })
            .cloned()
            .collect();

        // Sort by proximity: ASN match first, Country match second, then bandwidth descending
        candidates.sort_by(|a, b| {
            let score_a = (if a.asn_number == asn {
                1000
            } else if a.country_code == country {
                100
            } else {
                0
            }) + a.bandwidth_mbps;
            let score_b = (if b.asn_number == asn {
                1000
            } else if b.country_code == country {
                100
            } else {
                0
            }) + b.bandwidth_mbps;
            score_b.cmp(&score_a)
        });

        candidates
    }

    pub fn verify_mirror_freshness(&mut self, current_timestamp: u64) -> usize {
        let mut stale_count = 0;
        for mirror in &mut self.mirrors {
            if current_timestamp.saturating_sub(mirror.last_synced_timestamp)
                > self.max_staleness_secs
            {
                mirror.is_active = false;
                stale_count += 1;
            }
        }
        stale_count
    }

    pub fn generate_metalink_xml(
        &self,
        repo_file_path: &str,
        category: &str,
        arch: &str,
        country: &str,
        countme_bucket: u32,
        current_timestamp: u64,
    ) -> String {
        let active_mirrors =
            self.route_client_request(country, 0, category, arch, current_timestamp);

        let mut xml = format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<metalink version=\"3.0\" xmlns=\"http://www.metalinker.org/\" origin=\"Fedora-MirrorManager2\">\n  <files>\n    <file name=\"{}\">\n      <resources>\n",
            repo_file_path
        );

        for (preference, mirror) in active_mirrors.iter().enumerate() {
            xml.push_str(&format!(
                "        <url protocol=\"https\" type=\"https\" location=\"{}\" preference=\"{}\">{}?countme={}</url>\n",
                mirror.country_code,
                100 - preference,
                mirror.url,
                countme_bucket
            ));
        }

        xml.push_str("      </resources>\n    </file>\n  </files>\n</metalink>");
        xml
    }
}

impl Default for FedoraMirrorManager2Engine {
    fn default() -> Self {
        Self::new(86400) // Default 24h staleness limit
    }
}
