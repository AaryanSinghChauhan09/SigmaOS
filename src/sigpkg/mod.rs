// SigmaPkg - SigmaOS Package Manager
// Zero-dependency, zero-allocation-ready, safe Rust package manager

pub mod arch_compat;
pub mod aur;
pub mod aur_helper;
pub mod importer;
pub mod linux_compat;
pub mod makepkg;
pub mod nix_shell;
pub mod pacman;
pub mod recipe;
pub mod resolver;
pub mod rpm_compat;
pub mod spec;
pub mod store;
pub mod transaction;
pub mod universal_adapter;
pub mod verifier;

pub use arch_compat::{AurRecipeCompiler, PacmanDbAdapter, RollingSyncManager};
pub use aur::{AurClient, AurPackage as AurPkg, AurPackageQuery, Version as AurVersion};
pub use aur_helper::{AurHelper, AurPackage, AurParser};
pub use makepkg::{MakepkgSandbox, PkgbuildParser};
pub use nix_shell::{DevEnvironment, NixShellManager, PredefinedEnvironments};
pub use linux_compat::{
    DebianPackageTranslator, LinuxPackageCompatManager, LinuxPackageType, RpmPackageTranslator,
    TranslatedMetadata, TranslatorError,
};
pub use pacman::{MakePkgEngine, PacmanError, PacmanManager, PkgBuildScript};
pub use recipe::{BuildSystem, PackageRecipe, RecipeError, RecipeManager};
pub use resolver::SatSolver;
pub use rpm_compat::{PackageSourceFormat, RpmPackageTranslator, SpecMetadata};
pub use store::ContentAddressedStore;
pub use transaction::Transaction;
pub use universal_adapter::{
    AptDebManifest, FlatpakManifest, PacmanPkgbuild, SnapcraftManifest, UniversalPackageAdapter,
};
pub use importer::{PackageImporter, DebPackageImporter, RpmPackageImporter, PacmanPackageImporter};
pub use verifier::CryptoVerifier;

/// Package version using SemVer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parses version input safely with a zero-allocation, stateless next() token iterator over '.' separators
    pub fn parse(version_str: &str) -> Result<Self, ParseError> {
        let mut parts = version_str.split('.');

        let major_str = parts.next().ok_or(ParseError::InvalidFormat)?;
        let minor_str = parts.next().ok_or(ParseError::InvalidFormat)?;
        let patch_str = parts.next().ok_or(ParseError::InvalidFormat)?;

        if parts.next().is_some() {
            return Err(ParseError::InvalidFormat);
        }

        let major = major_str
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;
        let minor = minor_str
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;
        let patch = patch_str
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;

        Ok(Version::new(major, minor, patch))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat,
    InvalidNumber,
}

/// Package metadata
#[derive(Debug, Clone)]
pub struct Package {
    pub name: crate::klib::String,
    pub version: Version,
    pub description: crate::klib::String,
    pub dependencies: Vec<Dependency>,
    pub checksum: crate::klib::String,
    pub mirrors: Vec<crate::klib::String>,
    pub signing_keys: Vec<crate::klib::String>,
    pub licenses: Vec<crate::klib::String>,
    pub maintainers: Vec<crate::klib::String>,
    pub changelogs: Vec<crate::klib::String>,
    pub source: crate::klib::String,
}

impl Package {
    pub fn new(
        name: crate::klib::String,
        version: Version,
        description: crate::klib::String,
        dependencies: Vec<Dependency>,
        checksum: crate::klib::String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            dependencies,
            checksum,
            mirrors: Vec::new(),
            signing_keys: Vec::new(),
            licenses: Vec::new(),
            maintainers: Vec::new(),
            changelogs: Vec::new(),
            source: crate::klib::String::new(),
        }
    }
}

impl Package {
    pub fn new(name: String, version: Version, description: String, dependencies: Vec<Dependency>, checksum: String) -> Self {
        Package {
            name,
            version,
            description,
            dependencies,
            checksum,
        }
    }
}

impl Package {
    pub fn new(
        name: String,
        version: Version,
        description: String,
        dependencies: Vec<Dependency>,
        checksum: String,
    ) -> Self {
        Self {
            name,
            version,
            description,
            dependencies,
            checksum,
        }
    }
}

impl Package {
    pub fn new(name: String, version: Version, description: String, dependencies: Vec<Dependency>, checksum: String) -> Self {
        Package {
            name,
            version,
            description,
            dependencies,
            checksum,
        }
    }
}

/// Package dependency
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
}

/// Version constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    Exact(Version),
    GreaterThan(Version),
    LessThan(Version),
    GreaterOrEqual(Version),
    LessOrEqual(Version),
    Any,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_creation() {
        let version = Version::new(1, 2, 3);
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_version_parsing() {
        let version = Version::parse("1.2.3").unwrap();
        assert_eq!(version, Version::new(1, 2, 3));
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 2, 4);
        assert!(v1 < v2);
    }
}
