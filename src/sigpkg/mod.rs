// SigmaPkg - SigmaOS Package Manager
// Zero-dependency, zero-allocation-ready, safe Rust package manager

<<<<<<< HEAD
||||||| 23ef22a4a
pub mod linux_compat;
pub mod importer;
pub mod pacman;
=======
pub mod arch_compat;
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub mod recipe;
pub mod resolver;
pub mod store;
pub mod transaction;
pub mod verifier;
pub mod rpm_compat;
pub mod universal_adapter;
pub mod arch_compat;

<<<<<<< HEAD
||||||| 23ef22a4a
pub use importer::{
    DebPackageImporter, PackageImporter, PacmanPackageImporter, RpmPackageImporter,
};
pub use importer::{PackageImporter, DebPackageImporter, RpmPackageImporter, PacmanPackageImporter};
pub use pacman::{MakePkgEngine, PacmanError, PacmanManager, PkgBuildScript};
=======
pub use arch_compat::{AurRecipeCompiler, PacmanDbAdapter, RollingSyncManager};
pub use spec::{
    AptPackageAdapter, ManagerCapability, PackageAdapterFactory, PackageCapability,
    PackageDependency, PackageError as SpecPackageError, PackageInfo, PackageManager as SpecPackageManager, PackageStats, PackageVersion,
    PacmanPackageAdapter, SimplePackage, SimplePackageManager, SnapPackageAdapter,
    NixPackageAdapter, EbuildPackageAdapter, ApkPackageAdapter, FlatpakPackageAdapter,
    TxzPackageAdapter, XbpsPackageAdapter,
    CachyCpuDetector, CachyosPackageAdapter, CpuArchLevel,
    UniversalPackage, UniversalPackageType, UserDefinedPackageHook,
};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub use recipe::{BuildSystem, PackageRecipe, RecipeError, RecipeManager};
pub use resolver::SatSolver;
pub use store::ContentAddressedStore;
pub use transaction::Transaction;
pub use verifier::CryptoVerifier;
pub use zero_alloc_resolver::{PackageDependencyResolver, MAX_RECIPE_DEPENDENCIES};
pub use universal_adapter::{
    PackageFormatAdapter, UniversalPackageManager as UniversalAdapterManager, AdapterError,
    DebAdapter, RpmAdapter, PacmanAdapter,
};
<<<<<<< HEAD
||||||| 23ef22a4a
pub use importer::{
    PackageImporter, DebPackageImporter, RpmPackageImporter, PacmanPackageImporter,
};
=======
pub use universal_oop_system::{
    IPackage, IPackageParser, PackageFormat, PackageMetadata,
    PackageParserFactory, UniversalPackageManager,
    DebAdapter as OopDebAdapter, RpmAdapter as OopRpmAdapter, PacmanAdapter as OopPacmanAdapter,
    UserDefinedHook, ParseError, InstallError, HookError,
};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

/// Package version using SemVer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
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
    pub name: String,
    pub version: Version,
    pub description: String,
    pub dependencies: Vec<Dependency>,
    pub checksum: String,
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
