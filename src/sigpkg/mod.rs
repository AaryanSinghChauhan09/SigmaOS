// SigmaPkg - SigmaOS Package Manager
// Zero-dependency, zero-allocation-ready, safe Rust package manager

pub mod recipe;
pub mod resolver;
pub mod store;
pub mod transaction;
pub mod verifier;
pub mod importer;
pub mod universal_oop_system;
pub mod universal_adapter;
pub mod universal_engine;

pub use importer::{DebPackageImporter, RpmPackageImporter, PacmanPackageImporter, PackageImporter};
pub use recipe::{BuildSystem, PackageRecipe, RecipeError, RecipeManager};
pub use resolver::SatSolver;
pub use spec::{
    ManagerCapability, PackageCapability, PackageDependency, PackageError as SpecPackageError,
    PackageInfo, PackageManager as SpecPackageManager, PackageStats, PackageVersion,
    SimplePackage, SimplePackageManager,
};
pub use universal_engine::{
    ApkPackageAdapter, AptPackageAdapter, CachyCpuDetector, CachyosPackageAdapter, CpuArchLevel,
    EbuildPackageAdapter, FlatpakPackageAdapter, NixPackageAdapter,
    PackageAdapterFactory,
    PacmanPackageAdapter, SnapPackageAdapter,
    TxzPackageAdapter, UniversalPackage, UniversalPackageType, UserDefinedPackageHook,
    XbpsPackageAdapter,
};
pub use store::ContentAddressedStore;
pub use transaction::Transaction;
pub use verifier::CryptoVerifier;

/// Package version using SemVer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
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
    pub pqc_signature: Option<String>,
    pub gpg_key_id: Option<String>,
    pub supported_architectures: Vec<String>,
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
            pqc_signature: None,
            gpg_key_id: None,
            supported_architectures: Vec::new(),
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

    #[test]
    fn test_package_rich_metadata_and_pqc_trust() {
        let mut pkg = Package::new(
            "linux-rt-kernel".to_string(),
            Version::new(6, 9, 3),
            "Real-time preempt-rt microkernel variant for SigmaOS".to_string(),
            Vec::new(),
            "sha256:d83d102e3b74".to_string(),
        );

        // Populate rich metadata standard fields
        pkg.licenses.push("GPL-2.0-only".to_string());
        pkg.maintainers
            .push("Sovereign Maintainers <maintainers@sigmaos.dev>".to_string());
        pkg.mirrors
            .push("https://mirrors.sigmaos.org/pkgs/".to_string());
        pkg.signing_keys
            .push("dilithium5:pubkey_root_ca".to_string());
        pkg.changelogs
            .push("v6.9.3: RT preemption schedulers stabilization".to_string());

        assert_eq!(pkg.name, "linux-rt-kernel");
        assert_eq!(pkg.licenses[0], "GPL-2.0-only");
        assert_eq!(
            pkg.maintainers[0],
            "Sovereign Maintainers <maintainers@sigmaos.dev>"
        );
        assert_eq!(pkg.mirrors[0], "https://mirrors.sigmaos.org/pkgs/");
        assert_eq!(pkg.signing_keys[0], "dilithium5:pubkey_root_ca");
        assert_eq!(
            pkg.changelogs[0],
            "v6.9.3: RT preemption schedulers stabilization"
        );
    }
}
