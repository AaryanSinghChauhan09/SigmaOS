#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone)]
pub struct VersionToken {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl VersionToken {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        VersionToken { major, minor, patch }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub version: VersionToken,
    pub description: String,
    pub license: String,
    pub maintainers: Vec<String>,
    pub dependencies: Vec<Dependency>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
    pub files: Vec<InstalledFile>,
    pub scripts: Option<PackageScripts>,
    pub source_url: Option<String>,
    pub source_sha256: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_req: String,
}

#[derive(Debug, Clone)]
pub struct InstalledFile {
    pub path: String,
    pub sha256: String,
    pub size: u64,
}

pub struct PackageScripts {
    pub pre_install: Option<String>,
    pub post_install: Option<String>,
    pub pre_remove: Option<String>,
    pub post_remove: Option<String>,
}

pub enum DependencyResolver {
    Topological,
    SatSolver,
    Functional,
}

pub enum PackageBackend {
    Native,
    Ostree,
    Container,
}

pub struct SigmaPackageManager {
    pub backend: PackageBackend,
    pub resolver_type: DependencyResolver,
    pub packages: Vec<PackageMetadata>,
    pub installed: Vec<String>,
    pub store_path: String,
}

impl SigmaPackageManager {
    pub fn new() -> Self {
        SigmaPackageManager {
            backend: PackageBackend::Native,
            resolver_type: DependencyResolver::SatSolver,
            packages: Vec::new(),
            installed: Vec::new(),
            store_path: String::from("/var/sigma-pkg"),
        }
    }

    pub fn register_package(&mut self, package: PackageMetadata) -> Result<(), PackageError> {
        for p in &self.packages {
            if p.name == package.name && p.version.to_string() == package.version.to_string() {
                return Err(PackageError::AlreadyExists);
            }
        }
        self.packages.push(package);
        Ok(())
    }

    pub fn resolve_dependencies(&self, name: &str) -> Result<Vec<String>, PackageError> {
        let mut resolved = Vec::new();
        let mut visited = Vec::new();
        self.resolve_recursive(name, &mut resolved, &mut visited)?;
        Ok(resolved)
    }

    fn resolve_recursive(&self, name: &str, resolved: &mut Vec<String>, visited: &mut Vec<String>) -> Result<(), PackageError> {
        if visited.contains(&name.to_string()) {
            return Ok(());
        }
        visited.push(name.to_string());
        let package = self.packages.iter().find(|p| p.name == name)
            .ok_or(PackageError::NotFound)?;
        for dep in &package.dependencies {
            self.resolve_recursive(&dep.name, resolved, visited)?;
        }
        resolved.push(name.to_string());
        Ok(())
    }

    pub fn install(&mut self, name: &str) -> Result<(), PackageError> {
        let deps = self.resolve_dependencies(name)?;
        for dep_name in deps {
            if !self.installed.contains(&dep_name) {
                self.do_install(&dep_name)?;
            }
        }
        if !self.installed.contains(&name.to_string()) {
            self.do_install(name)?;
        }
        Ok(())
    }

    fn do_install(&mut self, name: &str) -> Result<(), PackageError> {
        let package = self.packages.iter().find(|p| p.name == name)
            .ok_or(PackageError::NotFound)?;
        self.installed.push(name.to_string());
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), PackageError> {
        if let Some(idx) = self.installed.iter().position(|n| n == name) {
            self.installed.remove(idx);
            Ok(())
        } else {
            Err(PackageError::NotFound)
        }
    }

    pub fn upgrade(&mut self, name: &str) -> Result<(), PackageError> {
        self.remove(name)?;
        self.install(name)
    }

    pub fn list_installed(&self) -> &[String] {
        &self.installed
    }

    pub fn search(&self, query: &str) -> Vec<&PackageMetadata> {
        self.packages.iter().filter(|p| p.name.contains(query)).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageError {
    NotFound,
    AlreadyExists,
    DependencyConflict,
    MissingDependency,
    InstallFailed,
    PermissionDenied,
    InvalidPackage,
}

pub struct Generation {
    pub id: String,
    pub creation_time: u64,
    pub kernel: String,
    pub packages: Vec<String>,
    pub system_config: String,
    pub boot_config: String,
    pub prev: Option<String>,
    pub next: Option<String>,
}

impl Generation {
    pub fn new(id: &str) -> Self {
        Generation {
            id: id.to_string(),
            creation_time: 0,
            kernel: String::new(),
            packages: Vec::new(),
            system_config: String::new(),
            boot_config: String::new(),
            prev: None,
            next: None,
        }
    }
}

pub trait SystemProfile: Send + Sync {
    fn hostname(&self) -> &str;
    fn set_hostname(&mut self, hostname: &str);
    fn timezone(&self) -> &str;
    fn set_timezone(&mut self, tz: &str);
    fn locale(&self) -> &str;
    fn set_locale(&mut self, locale: &str);
    fn packages(&self) -> &[String];
    fn add_package(&mut self, package: &str);
    fn remove_package(&mut self, package: &str);
    fn kernel_profile(&self) -> &str;
    fn set_kernel_profile(&mut self, profile: &str);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreemptionMode {
    Voluntary,
    Full,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuGovernor {
    Performance,
    Powersave,
    Schedutil,
}

pub struct KernelProfile {
    pub preemption_mode: PreemptionMode,
    pub tickless_cpus: Vec<u32>,
    pub rcu_lazy: bool,
    pub cpu_governor: CpuGovernor,
}

pub struct SystemConfig {
    pub hostname: String,
    pub timezone: String,
    pub locale: String,
    pub kernel: KernelProfile,
    pub packages: Vec<String>,
    pub security_profile: String,
}

impl SystemConfig {
    pub fn new() -> Self {
        SystemConfig {
            hostname: String::from("sigmaos"),
            timezone: String::from("UTC"),
            locale: String::from("en_US.UTF-8"),
            kernel: KernelProfile {
                preemption_mode: PreemptionMode::Voluntary,
                tickless_cpus: Vec::new(),
                rcu_lazy: false,
                cpu_governor: CpuGovernor::Schedutil,
            },
            packages: Vec::new(),
            security_profile: String::from("default"),
        }
    }
}