// SigmaPkg - SigmaOS Package Manager
// Zero-dependency, zero-allocation-ready, safe Rust package manager

pub mod recipe;
pub mod resolver;
pub mod rpm_compat;
pub mod store;
pub mod transaction;
pub mod universal_adapter;
pub mod verifier;

pub use recipe::{BuildSystem, PackageRecipe, RecipeError, RecipeManager};
pub use rpm_compat::{RpmPackageTranslator, SpecMetadata, PackageSourceFormat};
pub use resolver::SatSolver;
pub use store::ContentAddressedStore;
pub use transaction::Transaction;
pub use universal_adapter::{
    AptDebManifest, FlatpakManifest, PacmanPkgbuild, SnapcraftManifest, UniversalPackageAdapter,
};
pub use verifier::CryptoVerifier;
pub use universal_adapter::{
    AptDebManifest, PacmanPkgbuild, SnapcraftManifest, FlatpakManifest, UniversalPackageAdapter,
};

/// Package version using SemVer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        if parts.len() != 3 {
            return Err(ParseError::InvalidFormat);
        }

        let major = parts[0]
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;
        let minor = parts[1]
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;
        let patch = parts[2]
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;

        Ok(Version::new(major, minor, patch))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
