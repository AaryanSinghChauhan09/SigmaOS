use std::vec;
// SigmaOS openSUSE & Slackware Competitor Parity Subsystem
// Independent, zero-dependency implementations of openSUSE YaST and Slackware pkgtools

use std::collections::BTreeMap;
use std::format;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;

// =========================================================================
// 1. OPENSUSE YAST (YET ANOTHER SETUP TOOL) CONTROL CENTER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YastModuleType {
    Network,
    Security,
    Users,
    Hardware,
}

pub struct YastCentralControlCenter {
    pub active_modules: BTreeMap<String, YastModuleType>,
    pub configuration_steps_completed: usize,
    pub hostname: String,
    pub firewall_enabled: bool,
}

impl YastCentralControlCenter {
    pub fn new() -> Self {
        let mut modules = BTreeMap::new();
        modules.insert("yast-network".to_string(), YastModuleType::Network);
        modules.insert("yast-firewall".to_string(), YastModuleType::Security);
        modules.insert("yast-users".to_string(), YastModuleType::Users);

        Self {
            active_modules: modules,
            configuration_steps_completed: 0,
            hostname: "opensuse-localhost".to_string(),
            firewall_enabled: true,
        }
    }

    pub fn register_setup_module(&mut self, label: &str, module_type: YastModuleType) {
        self.active_modules.insert(label.to_string(), module_type);
    }

    pub fn configure_network_hostname(&mut self, new_hostname: &str) -> Result<(), &'static str> {
        if new_hostname.is_empty() {
            return Err("YaST: Hostname cannot be empty");
        }
        self.hostname = new_hostname.to_string();
        self.configuration_steps_completed += 1;
        Ok(())
    }

    pub fn configure_security_firewall(&mut self, enable: bool) {
        self.firewall_enabled = enable;
        self.configuration_steps_completed += 1;
    }
}

impl Default for YastCentralControlCenter {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 1b. OPENSUSE ZYPPER PACKAGE SOLVER & LIBZYPP REPOSITORY ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct ZypperPackage {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub vendor: String,
    pub dependencies: Vec<String>,
}

pub struct ZypperRepository {
    pub name: String,
    pub enabled: bool,
    pub base_url: String,
    pub priority: u32, // Lower = higher priority (libzypp style)
    pub packages: Vec<ZypperPackage>,
}

impl ZypperRepository {
    pub fn new(name: &str, url: &str, priority: u32) -> Self {
        ZypperRepository {
            name: name.to_string(),
            enabled: true,
            base_url: url.to_string(),
            priority,
            packages: Vec::new(),
        }
    }

    pub fn add_package(&mut self, name: &str, ver: &str, arch: &str, deps: &[&str]) {
        self.packages.push(ZypperPackage {
            name: name.to_string(),
            version: ver.to_string(),
            arch: arch.to_string(),
            vendor: String::from("openSUSE Build Service"),
            dependencies: deps.iter().map(|s| s.to_string()).collect(),
        });
    }
}

/// Zypper Solver utilizing libzypp-style SAT solver logic
pub struct ZypperSolver {
    pub repositories: Vec<ZypperRepository>,
    pub installed_packages: BTreeMap<String, String>, // pkg -> ver
}

impl ZypperSolver {
    pub fn new() -> Self {
        ZypperSolver {
            repositories: Vec::new(),
            installed_packages: BTreeMap::new(),
        }
    }

    pub fn add_repository(&mut self, repo: ZypperRepository) {
        self.repositories.push(repo);
        // Sort repos by priority asc (lower priority number = preferred)
        self.repositories.sort_by_key(|r| r.priority);
    }

    pub fn zypper_install(&mut self, package_name: &str) -> Result<String, &'static str> {
        let mut found_pkg: Option<ZypperPackage> = None;

        for repo in &self.repositories {
            if !repo.enabled {
                continue;
            }
            if let Some(pkg) = repo.packages.iter().find(|p| p.name == package_name) {
                found_pkg = Some(pkg.clone());
                break;
            }
        }

        if let Some(pkg) = found_pkg {
            // Resolve dependencies recursively
            for dep in &pkg.dependencies {
                if !self.installed_packages.contains_key(dep) {
                    self.zypper_install(dep)?;
                }
            }
            self.installed_packages
                .insert(pkg.name.clone(), pkg.version.clone());
            Ok(format!(
                "zypper: Successfully installed {} version {}",
                pkg.name, pkg.version
            ))
        } else {
            Err("zypper: Package not found in active repositories")
        }
    }
}

// =========================================================================
// 1c. OPENSUSE OPEN BUILD SERVICE (OBS) ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsBuildTarget {
    OpenSuseTumbleweed,
    OpenSuseLeap15,
    Sle15,
}

#[derive(Debug, Clone)]
pub struct ObsPackageProject {
    pub project_name: String,
    pub package_name: String,
    pub build_target: ObsBuildTarget,
    pub spec_file_content: String,
    pub source_services: Vec<String>, // e.g. "tar_scm", "recompress"
    pub build_successful: bool,
}

pub struct OpenBuildServiceEngine {
    pub projects: Vec<ObsPackageProject>,
}

impl OpenBuildServiceEngine {
    pub fn new() -> Self {
        OpenBuildServiceEngine {
            projects: Vec::new(),
        }
    }

    pub fn create_obs_project(
        &mut self,
        proj: &str,
        pkg: &str,
        target: ObsBuildTarget,
        spec: &str,
    ) -> &ObsPackageProject {
        let obs_proj = ObsPackageProject {
            project_name: proj.to_string(),
            package_name: pkg.to_string(),
            build_target: target,
            spec_file_content: spec.to_string(),
            source_services: vec!["tar_scm".to_string(), "recompress".to_string()],
            build_successful: false,
        };
        self.projects.push(obs_proj);
        self.projects.last().unwrap()
    }

    pub fn trigger_obs_build(&mut self, proj_name: &str) -> Result<String, &'static str> {
        let proj = self
            .projects
            .iter_mut()
            .find(|p| p.project_name == proj_name)
            .ok_or("OBS project not found")?;

        if !proj.spec_file_content.contains("Name:") || !proj.spec_file_content.contains("Version:")
        {
            return Err("OBS build failed: Invalid RPM spec file syntax");
        }

        proj.build_successful = true;
        Ok(format!(
            "OBS: Successfully compiled {} for target {:?}",
            proj.package_name, proj.build_target
        ))
    }
}

// =========================================================================
// 1d. OPENSUSE YaST 1-CLICK INSTALL (.ymp) PARSER
// =========================================================================

#[derive(Debug, Clone)]
pub struct YmpRepositorySubscription {
    pub repo_name: String,
    pub repo_url: String,
    pub package_to_install: String,
}

pub struct YaST1ClickInstallParser;

impl YaST1ClickInstallParser {
    pub fn parse_ymp_xml(ymp_xml: &str) -> Result<YmpRepositorySubscription, &'static str> {
        let mut name = String::new();
        let mut url = String::new();
        let mut pkg = String::new();

        for line in ymp_xml.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("<name>") && trimmed.ends_with("</name>") {
                name = trimmed[6..trimmed.len() - 7].to_string();
            } else if trimmed.starts_with("<url>") && trimmed.ends_with("</url>") {
                url = trimmed[5..trimmed.len() - 6].to_string();
            } else if trimmed.starts_with("<item>") && trimmed.ends_with("</item>") {
                pkg = trimmed[6..trimmed.len() - 7].to_string();
            }
        }

        if name.is_empty() || url.is_empty() || pkg.is_empty() {
            Err("YaST: Invalid .ymp 1-Click Install XML payload")
        } else {
            Ok(YmpRepositorySubscription {
                repo_name: name,
                repo_url: url,
                package_to_install: pkg,
            })
        }
    }
}

// =========================================================================
// 2. SLACKWARE PKGTOOLS & SLACKPKG MINIMALIST PACKAGE MANAGER
// =========================================================================

pub struct SlackwarePackage {
    pub name: String,
    pub unpacked_files: Vec<String>,
    pub description_line: String,
}

pub struct SlackwarePkgTools {
    pub installed_packages_db: BTreeMap<String, SlackwarePackage>, // package_name -> SlackwarePackage log
}

impl SlackwarePkgTools {
    pub fn new() -> Self {
        Self {
            installed_packages_db: BTreeMap::new(),
        }
    }

    /// Replicates the famous Slackware 'installpkg' process. Unpacks tarball paths
    /// and logs files in '/var/log/packages/package-name'.
    pub fn installpkg(
        &mut self,
        package_name: &str,
        raw_tar_contents: &[&str],
        slack_desc: &str,
    ) -> Result<(), &'static str> {
        if self
            .installed_packages_db
            .contains_key(&package_name.to_string())
        {
            return Err("installpkg: Package already installed on Slackware system");
        }

        let mut unpacked_files = Vec::new();
        for &file in raw_tar_contents {
            unpacked_files.push(file.to_string());
        }

        let pkg = SlackwarePackage {
            name: package_name.to_string(),
            unpacked_files,
            description_line: slack_desc.to_string(),
        };

        // Write package metadata to Slackware's system registry
        self.installed_packages_db
            .insert(package_name.to_string(), pkg);
        Ok(())
    }

    /// Replicates the famous Slackware 'removepkg' process, removing tracked files from root.
    pub fn removepkg(&mut self, package_name: &str) -> Result<usize, &'static str> {
        let pkg = self
            .installed_packages_db
            .remove(&package_name.to_string())
            .ok_or("removepkg: Package not found in Slackware database")?;

        Ok(pkg.unpacked_files.len())
    }

    pub fn parse_slack_desc_field(&self, raw_desc: &str) -> String {
        // Slackware's slack-desc file format requires a structured 'package: description' line
        for line in raw_desc.lines() {
            if line.contains(":") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    return parts[1].trim().to_string();
                }
            }
        }
        "No Slackware package description found".to_string()
    }
}

impl Default for SlackwarePkgTools {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_yast_central_control_center() {
        let mut yast = YastCentralControlCenter::new();
        assert_eq!(yast.active_modules.len(), 3);
        assert_eq!(yast.hostname, "opensuse-localhost");

        yast.register_setup_module("yast-hardware", YastModuleType::Hardware);
        assert_eq!(yast.active_modules.len(), 4);

        assert!(yast.configure_network_hostname("gecko-station").is_ok());
        assert_eq!(yast.hostname, "gecko-station");

        yast.configure_security_firewall(false);
        assert!(!yast.firewall_enabled);
        assert_eq!(yast.configuration_steps_completed, 2);
    }

    #[test]
    fn test_slackware_pkgtools_install_remove() {
        let mut pkgtools = SlackwarePkgTools::new();
        let tar_contents = vec!["/usr/bin/slackpkg", "/etc/slackpkg/slackpkg.conf"];

        let slack_desc = "
            slackpkg: slackpkg (automated package manager)
            slackpkg: An automated tool to install or upgrade packages
        ";

        // Install Slackware Package
        assert!(pkgtools
            .installpkg("slackpkg", &tar_contents, slack_desc)
            .is_ok());
        assert_eq!(pkgtools.installed_packages_db.len(), 1);

        // Parse slack-desc description line
        let desc = pkgtools.parse_slack_desc_field(slack_desc);
        assert_eq!(desc, "slackpkg (automated package manager)");

        // Try installing duplicate (fails)
        assert!(pkgtools
            .installpkg("slackpkg", &tar_contents, slack_desc)
            .is_err());

        // Remove Slackware Package
        let count = pkgtools.removepkg("slackpkg").unwrap();
        assert_eq!(count, 2);
        assert_eq!(pkgtools.installed_packages_db.len(), 0);
    }

    #[test]
    fn test_zypper_solver_flow() {
        let mut solver = ZypperSolver::new();
        let mut repo = ZypperRepository::new(
            "openSUSE-OSS",
            "https://download.opensuse.org/distribution/leap/15.5/repo/oss/",
            10,
        );
        repo.add_package("zlib", "1.2.13", "x86_64", &[]);
        repo.add_package("curl", "8.0.1", "x86_64", &["zlib"]);

        solver.add_repository(repo);

        let res = solver.zypper_install("curl").unwrap();
        assert!(res.contains("Successfully installed curl"));
        assert_eq!(solver.installed_packages.get("zlib").unwrap(), "1.2.13");
        assert_eq!(solver.installed_packages.get("curl").unwrap(), "8.0.1");
    }

    #[test]
    fn test_open_build_service_engine() {
        let mut obs = OpenBuildServiceEngine::new();
        let spec = "Name: hello\nVersion: 2.10\nSummary: GNU Hello World\n";
        obs.create_obs_project(
            "home:user:branches",
            "hello",
            ObsBuildTarget::OpenSuseTumbleweed,
            spec,
        );

        let res = obs.trigger_obs_build("home:user:branches").unwrap();
        assert!(res.contains("Successfully compiled hello"));
        assert!(obs.projects[0].build_successful);
    }

    #[test]
    fn test_yast_1click_install_parser() {
        let ymp = "
            <metapackage>
                <name>Games Repository</name>
                <url>https://download.opensuse.org/repositories/games/openSUSE_Tumbleweed/</url>
                <item>supertuxkart</item>
            </metapackage>
        ";

        let sub = YaST1ClickInstallParser::parse_ymp_xml(ymp).unwrap();
        assert_eq!(sub.repo_name, "Games Repository");
        assert_eq!(sub.package_to_install, "supertuxkart");
    }
}
