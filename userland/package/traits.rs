// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired package manager traits for SigmaOS
// Zero-allocation, performance-optimized package management

/// Core package manager trait
pub trait PackageManager {
    /// Initialize package manager
    fn init(&mut self) -> Result<(), PackageError>;
    
    /// Get package manager name
    fn name(&self) -> &str;
    
    /// Install package
    fn install(&mut self, package: &Package) -> Result<(), PackageError>;
    
    /// Remove package
    fn remove(&mut self, package: &Package) -> Result<(), PackageError>;
    
    /// Update package
    fn update(&mut self, package: &Package) -> Result<(), PackageError>;
    
    /// Upgrade all packages
    fn upgrade_all(&mut self) -> Result<(), PackageError>;
    
    /// Search for package
    fn search(&self, query: &str) -> Result<Vec<Package>, PackageError>;
    
    /// Get package info
    fn info(&self, package: &Package) -> Result<PackageInfo, PackageError>;
    
    /// List installed packages
    fn list_installed(&self) -> Result<Vec<Package>, PackageError>;
    
    /// List available packages
    fn list_available(&self) -> Result<Vec<Package>, PackageError>;
    
    /// Resolve dependencies
    fn resolve_dependencies(&self, package: &Package) -> Result<Vec<Package>, PackageError>;
}

/// Package database trait
pub trait PackageDatabase {
    /// Add package to database
    fn add_package(&mut self, package: &Package) -> Result<(), PackageError>;
    
    /// Remove package from database
    fn remove_package(&mut self, package: &Package) -> Result<(), PackageError>;
    
    /// Get package from database
    fn get_package(&self, name: &str) -> Option<Package>;
    
    /// List all packages
    fn list_packages(&self) -> Vec<Package>;
    
    /// Query packages
    fn query(&self, query: PackageQuery) -> Vec<Package>;
}

/// Package repository trait
pub trait PackageRepository {
    /// Sync repository
    fn sync(&mut self) -> Result<(), PackageError>;
    
    /// Get package from repository
    fn get_package(&self, name: &str) -> Option<Package>;
    
    /// List packages in repository
    fn list_packages(&self) -> Vec<Package>;
    
    /// Download package
    fn download(&self, package: &Package) -> Result<Vec<u8>, PackageError>;
    
    /// Get repository URL
    fn url(&self) -> &str;
}

/// Dependency resolver trait
pub trait DependencyResolver {
    /// Resolve dependencies for package
    fn resolve(&self, package: &Package) -> Result<DependencyGraph, PackageError>;
    
    /// Check for conflicts
    fn check_conflicts(&self, packages: &[Package]) -> Result<(), PackageError>;
    
    /// Find upgrade path
    fn find_upgrade_path(&self, from: &Package, to: &Package) -> Result<Vec<Package>, PackageError>;
}

/// Package builder trait
pub trait PackageBuilder {
    /// Build package from source
    fn build(&mut self, spec: &PackageSpec) -> Result<Package, PackageError>;
    
    /// Install package
    fn install(&mut self, package: &Package) -> Result<(), PackageError>;
    
    /// Clean build artifacts
    fn clean(&mut self) -> Result<(), PackageError>;
}

/// Package structure
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<PackageDependency>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub size: u64,
    pub installed_size: u64,
    pub checksum: String,
}

/// Package dependency
#[derive(Debug, Clone)]
pub struct PackageDependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
    pub optional: bool,
}

/// Version constraint
#[derive(Debug, Clone)]
pub enum VersionConstraint {
    Any,
    Exact(String),
    GreaterThan(String),
    LessThan(String),
    GreaterOrEqual(String),
    LessOrEqual(String),
    Range(String, String),
}

/// Package specification for building
#[derive(Debug, Clone)]
pub struct PackageSpec {
    pub name: String,
    pub version: String,
    pub source_url: String,
    pub build_commands: Vec<String>,
    pub dependencies: Vec<PackageDependency>,
    pub install_commands: Vec<String>,
}

/// Package information
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub maintainer: String,
    pub license: String,
    pub homepage: String,
    pub dependencies: Vec<PackageDependency>,
    pub files: Vec<String>,
    pub install_date: Option<u64>,
}

/// Package query
#[derive(Debug, Clone)]
pub enum PackageQuery {
   ByName(String),
    ByDescription(String),
    ByDependency(String),
    ByMaintainer(String),
    ByLicense(String),
    All,
}

/// Dependency graph
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    pub packages: Vec<Package>,
    pub edges: Vec<(String, String)>, // (depends_on, depends_from)
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
            edges: Vec::new(),
        }
    }
    
    pub fn add_package(&mut self, package: Package) {
        self.packages.push(package);
    }
    
    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges.push((from, to));
    }
    
    pub fn get_dependencies(&self, package: &str) -> Vec<&Package> {
        self.edges
            .iter()
            .filter(|(dep, pkg)| pkg == package)
            .filter_map(|(dep, _)| {
                self.packages.iter().find(|p| p.name == *dep)
            })
            .collect()
    }
}

/// Package error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageError {
    PackageNotFound,
    DependencyNotFound,
    DependencyConflict,
    BuildFailed,
    InstallFailed,
    DownloadFailed,
    InvalidPackage,
    InvalidVersion,
    PermissionDenied,
    DiskFull,
    NetworkError,
    CorruptedDatabase,
    LockHeld,
    Other,
}

/// Package manager configuration
pub struct PackageManagerConfig {
    pub cache_dir: String,
    pub install_dir: String,
    pub config_dir: String,
    pub repositories: Vec<String>,
    pub parallel_downloads: usize,
    pub parallel_builds: usize,
}

impl PackageManagerConfig {
    pub fn new() -> Self {
        Self {
            cache_dir: "/var/cache/sigma-pkg".to_string(),
            install_dir: "/".to_string(),
            config_dir: "/etc/sigma-pkg".to_string(),
            repositories: vec![
                "https://repo.sigmaos.org/core".to_string(),
                "https://repo.sigmaos.org/community".to_string(),
            ],
            parallel_downloads: 4,
            parallel_builds: 2,
        }
    }
}

/// Package lock for preventing concurrent operations
pub struct PackageLock {
    locked: core::sync::atomic::AtomicBool,
}

impl PackageLock {
    pub const fn new() -> Self {
        Self {
            locked: core::sync::atomic::AtomicBool::new(false),
        }
    }
    
    pub fn acquire(&self) -> bool {
        !self.locked.swap(true, core::sync::atomic::Ordering::Acquire)
    }
    
    pub fn release(&self) {
        self.locked.store(false, core::sync::atomic::Ordering::Release);
    }
    
    pub fn is_locked(&self) -> bool {
        self.locked.load(core::sync::atomic::Ordering::Acquire)
    }
}

/// Package cache for storing downloaded packages
pub struct PackageCache {
    packages: Vec<Package>,
    max_size: usize,
}

impl PackageCache {
    pub const fn new(max_size: usize) -> Self {
        Self {
            packages: Vec::new(),
            max_size,
        }
    }
    
    pub fn add(&mut self, package: Package) -> Result<(), PackageError> {
        if self.packages.len() >= self.max_size {
            return Err(PackageError::DiskFull);
        }
        self.packages.push(package);
        Ok(())
    }
    
    pub fn get(&self, name: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == name)
    }
    
    pub fn remove(&mut self, name: &str) -> bool {
        if let Some(pos) = self.packages.iter().position(|p| p.name == name) {
            self.packages.remove(pos);
            true
        } else {
            false
        }
    }
    
    pub fn clear(&mut self) {
        self.packages.clear();
    }
}
