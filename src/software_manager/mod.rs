//! SigmaOS Software Manager — Inspired by Linux Mint Software Manager (mintinstall), 
//! GNOME Software, KDE Discover, Omarchy ISO package tooling.
//! Principles: Layered Design, Open-Closed, Plug-in Driver Model, DRY, KISS.

use std::collections::HashMap;

/// Package source backend (Plug-in Driver Model — new backends via trait impl).
pub trait PackageBackend: Send + Sync {
    fn name(&self) -> &'static str;
    fn search(&self, query: &str) -> Vec<PackageInfo>;
    fn install(&self, pkg: &PackageInfo) -> Result<(), InstallError>;
    fn remove(&self, pkg_name: &str) -> Result<(), InstallError>;
    fn is_installed(&self, pkg_name: &str) -> bool;
    fn list_installed(&self) -> Vec<PackageInfo>;
}

/// Package metadata (Uniform Access — same struct for APT, Flatpak, Snap, AppImage).
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: PackageCategory,
    pub size_kb: u64,
    pub icon_name: String,
    pub backend: &'static str,
    pub rating: f32,
}

/// Package category taxonomy (Discoverability principle).
#[derive(Debug, Clone, PartialEq)]
pub enum PackageCategory {
    Internet,
    Multimedia,
    Graphics,
    Office,
    Games,
    Development,
    Education,
    System,
    Accessories,
    Science,
    Other,
}

impl PackageCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Internet     => "Internet",
            Self::Multimedia   => "Multimedia",
            Self::Graphics     => "Graphics",
            Self::Office       => "Office",
            Self::Games        => "Games",
            Self::Development  => "Development",
            Self::Education    => "Education",
            Self::System       => "System",
            Self::Accessories  => "Accessories",
            Self::Science      => "Science",
            Self::Other        => "Other",
        }
    }
}

/// Software Manager — aggregates multiple backends (Composition over Inheritance).
pub struct SoftwareManager {
    backends: Vec<Box<dyn PackageBackend>>,
    featured: Vec<PackageInfo>,
}

impl SoftwareManager {
    pub fn new() -> Self {
        Self { backends: Vec::new(), featured: Vec::new() }
    }

    /// Register a backend (Inversion of Control / Plug-in Driver Model).
    pub fn register_backend(&mut self, backend: Box<dyn PackageBackend>) {
        self.backends.push(backend);
    }

    /// Search across all registered backends (aggregated results).
    pub fn search(&self, query: &str) -> Vec<PackageInfo> {
        let q = query.to_lowercase();
        self.backends.iter()
            .flat_map(|b| b.search(&q))
            .filter(|p| !p.name.is_empty())
            .collect()
    }

    /// Install a package via its designated backend.
    pub fn install(&self, pkg: &PackageInfo) -> Result<(), InstallError> {
        let backend = self.backends.iter()
            .find(|b| b.name() == pkg.backend)
            .ok_or(InstallError::BackendNotFound(pkg.backend.to_string()))?;
        backend.install(pkg)
    }

    /// Remove a package (searched across all backends).
    pub fn remove(&self, pkg_name: &str) -> Result<(), InstallError> {
        for b in &self.backends {
            if b.is_installed(pkg_name) {
                return b.remove(pkg_name);
            }
        }
        Err(InstallError::NotInstalled(pkg_name.to_string()))
    }

    /// Check if a package is installed in any backend.
    pub fn is_installed(&self, pkg_name: &str) -> bool {
        self.backends.iter().any(|b| b.is_installed(pkg_name))
    }

    pub fn featured_apps(&self) -> &[PackageInfo] {
        &self.featured
    }

    pub fn backend_count(&self) -> usize {
        self.backends.len()
    }
}

impl Default for SoftwareManager {
    fn default() -> Self { Self::new() }
}

#[derive(Debug)]
pub enum InstallError {
    BackendNotFound(String),
    NotInstalled(String),
    PermissionDenied,
    NetworkError(String),
    DependencyConflict(String),
}

impl std::fmt::Display for InstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackendNotFound(b) => write!(f, "backend not found: {b}"),
            Self::NotInstalled(p)    => write!(f, "package not installed: {p}"),
            Self::PermissionDenied   => write!(f, "permission denied"),
            Self::NetworkError(m)    => write!(f, "network error: {m}"),
            Self::DependencyConflict(m) => write!(f, "dependency conflict: {m}"),
        }
    }
}

/// A stub APT backend for testing (Deterministic Testing — no real system calls).
#[cfg(test)]
pub struct StubAptBackend {
    installed: std::collections::HashSet<String>,
    packages: Vec<PackageInfo>,
}

#[cfg(test)]
impl StubAptBackend {
    pub fn new() -> Self {
        Self {
            installed: std::collections::HashSet::new(),
            packages: vec![
                PackageInfo {
                    name: "firefox".into(),
                    version: "120.0".into(),
                    description: "Web browser".into(),
                    category: PackageCategory::Internet,
                    size_kb: 80_000,
                    icon_name: "firefox".into(),
                    backend: "apt",
                    rating: 4.5,
                },
            ],
        }
    }
}

#[cfg(test)]
impl PackageBackend for StubAptBackend {
    fn name(&self) -> &'static str { "apt" }
    fn search(&self, query: &str) -> Vec<PackageInfo> {
        self.packages.iter()
            .filter(|p| p.name.contains(query) || p.description.to_lowercase().contains(query))
            .cloned().collect()
    }
    fn install(&self, _pkg: &PackageInfo) -> Result<(), InstallError> { Ok(()) }
    fn remove(&self, _pkg: &str) -> Result<(), InstallError> { Ok(()) }
    fn is_installed(&self, pkg_name: &str) -> bool { self.installed.contains(pkg_name) }
    fn list_installed(&self) -> Vec<PackageInfo> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manager_with_apt() -> SoftwareManager {
        let mut mgr = SoftwareManager::new();
        mgr.register_backend(Box::new(StubAptBackend::new()));
        mgr
    }

    #[test]
    fn test_backend_registered() {
        let mgr = manager_with_apt();
        assert_eq!(mgr.backend_count(), 1);
    }

    #[test]
    fn test_search_returns_results() {
        let mgr = manager_with_apt();
        let results = mgr.search("firefox");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "firefox");
    }

    #[test]
    fn test_search_empty_query() {
        let mgr = manager_with_apt();
        // empty string matches all
        let results = mgr.search("nonexistent-package-xyz");
        assert!(results.is_empty());
    }

    #[test]
    fn test_not_installed_initially() {
        let mgr = manager_with_apt();
        assert!(!mgr.is_installed("firefox"));
    }

    #[test]
    fn test_remove_not_installed_error() {
        let mgr = manager_with_apt();
        assert!(matches!(mgr.remove("firefox"), Err(InstallError::NotInstalled(_))));
    }

    #[test]
    fn test_category_labels() {
        assert_eq!(PackageCategory::Internet.label(), "Internet");
        assert_eq!(PackageCategory::Games.label(), "Games");
        assert_eq!(PackageCategory::Development.label(), "Development");
    }
}
