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
pub mod store;
pub mod transaction;
pub mod universal_adapter;
pub mod debian_crusher;

pub use arch_compat::{AurRecipeCompiler, PacmanDbAdapter, RollingSyncManager};
pub use debian_defeater::{SovereignMirrorSelector, SovereignTransactionManager, SovereignSandboxEnforcer, SovereignDeltaGenerator, TransactionStatus};
pub mod spec;
pub use spec::{
    ManagerCapability, PackageCapability,
    PackageDependency, PackageError as SpecPackageError, PackageInfo, PackageManager as SpecPackageManager, PackageStats, PackageVersion,
    SimplePackage, SimplePackageManager,
    CachyCpuDetector, CachyosPackageAdapter, CpuArchLevel,
    UniversalPackage, UniversalPackageType, UserDefinedPackageHook,
};
pub use recipe::{BuildSystem, PackageRecipe, RecipeError, RecipeManager};
pub use resolver::SatSolver;
pub use rpm_compat::{PackageSourceFormat, RpmPackageTranslator, SpecMetadata};
pub use store::ContentAddressedStore;
pub use transaction::Transaction;
pub use verifier::CryptoVerifier;
pub use zero_alloc_resolver::{PackageDependencyResolver, MAX_RECIPE_DEPENDENCIES};
pub mod universal_adapter;
pub mod universal_oop_system;
pub use universal_adapter::{
    PackageFormatAdapter, UniversalPackageManager as UniversalAdapterManager, AdapterError,
    DebAdapter, RpmAdapter, PacmanAdapter,
};
pub use universal_oop_system::{
    IPackage, IPackageParser, PackageFormat, PackageMetadata,
    PackageParserFactory, UniversalPackageManager,
    DebAdapter as OopDebAdapter, RpmAdapter as OopRpmAdapter, PacmanAdapter as OopPacmanAdapter,
    UserDefinedHook, ParseError as OopParseError, InstallError, HookError,
};
pub use verifier::CryptoVerifier;

/// Package version using SemVer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
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

    pub fn parse(version_str: &str) -> Result<Self, ParseError> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.is_empty() || parts.len() > 3 {
            return Err(ParseError::InvalidFormat);
        }

        let major = parts[0]
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;
        let minor = if parts.len() >= 2 {
            parts[1]
                .parse::<u64>()
                .map_err(|_| ParseError::InvalidNumber)?
        } else {
            0
        };
        let patch = if parts.len() >= 3 {
            parts[2]
                .parse::<u64>()
                .map_err(|_| ParseError::InvalidNumber)?
        } else {
            0
        };

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
