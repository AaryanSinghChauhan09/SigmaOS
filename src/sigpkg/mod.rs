// SigmaPkg - SigmaOS Package Manager
// Zero-dependency, zero-allocation-ready, safe Rust package manager

pub mod arch_compat;
pub mod debian_defeater;
pub mod importer;
pub mod makepkg;
pub mod nix_shell;
pub mod portage;
pub mod recipe;
pub mod resolver;
pub mod rpm_compat;
pub mod spec;
pub mod store;
pub mod transaction;
pub mod verifier;
pub mod zero_alloc_resolver;
pub mod universal_adapter;
pub mod sovereign_sigpkg;

pub use zero_alloc_resolver::{
    PackageDependencyResolver, MAX_RECIPE_DEPENDENCIES,
};
pub use universal_adapter::{
    PackageFormatAdapter, UniversalPackageManager, AdapterError, DebAdapter, RpmAdapter, PacmanAdapter, ApkAdapter, NixAdapter, EbuildAdapter,
};
pub use sovereign_sigpkg::*;

pub use arch_compat::{AlpmHook, AlpmHookManager, AurRecipeCompiler, MakepkgBuilder, MkinitcpioBuilder, PacmanDbAdapter, RollingSyncManager};
pub use importer::{PackageImporter, DebPackageImporter, RpmPackageImporter, PacmanPackageImporter};
pub use debian_defeater::{
    AlternativeCandidate, AptMirror, MaintainerScriptPhase, SovereignAlternativesSystem,
    SovereignDeltaGenerator, SovereignMaintainerSandbox, SovereignMirrorSelector,
};
pub use portage::{EbuildSpec, PortageResolver, Slot, UseFlag};
pub use spec::{
    ManagerCapability, PackageCapability,
    PackageDependency, PackageError as SpecPackageError, PackageInfo, PackageManager as SpecPackageManager, PackageStats, PackageVersion,
    SimplePackage, SimplePackageManager,
    CachyCpuDetector, CachyosPackageAdapter, CpuArchLevel,
    UniversalPackage, UniversalPackageType, UserDefinedPackageHook,
};
pub use recipe::{BuildSystem, PackageRecipe, RecipeError, RecipeManager};
pub use resolver::SatSolver;
pub use store::ContentAddressedStore;
pub use transaction::Transaction;
pub use verifier::CryptoVerifier;

/// Package version using SemVer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}


impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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
    pub mirrors: Vec<String>,
    pub signing_keys: Vec<String>,
    pub licenses: Vec<String>,
    pub maintainers: Vec<String>,
    pub changelogs: Vec<String>,
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
            mirrors: Vec::new(),
            signing_keys: Vec::new(),
            licenses: Vec::new(),
            maintainers: Vec::new(),
            changelogs: Vec::new(),
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
