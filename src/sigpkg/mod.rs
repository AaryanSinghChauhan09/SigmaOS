// SigmaPkg - SigmaOS Package Manager
// Zero-dependency, zero-allocation-ready, safe Rust package manager

pub mod recipe;
pub mod resolver;
pub mod store;
pub mod transaction;
pub mod verifier;

pub use recipe::{BuildSystem, PackageRecipe, RecipeError, RecipeManager};
pub use resolver::SatSolver;
pub use store::ContentAddressedStore;
pub use transaction::Transaction;
pub use verifier::CryptoVerifier;

/// Package version using SemVer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
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
        // Optimized to be entirely allocation-free by using inline parsing with iterators.
        // This avoids heap-allocated collections like Vec inside utility version parsing.
        let mut parts = version_str.split('.');

        let major = parts
            .next()
            .ok_or(ParseError::InvalidFormat)?
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;

        let minor = parts
            .next()
            .ok_or(ParseError::InvalidFormat)?
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;

        let patch = parts
            .next()
            .ok_or(ParseError::InvalidFormat)?
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;

        if parts.next().is_some() {
            return Err(ParseError::InvalidFormat);
        }
<<<<<<< HEAD
=======

        let major = parts[0]
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;
        let minor = parts[1]
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;
        let patch = parts[2]
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber)?;

>>>>>>> origin/jules-9057756713964855410-d59a7b65
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
}

/// Package dependency
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version_constraint: VersionConstraint,
}

/// Version constraint
<<<<<<< HEAD
<<<<<<< HEAD
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
=======
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
>>>>>>> origin/jules-8662134349396449944-dbc9966d
=======
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
>>>>>>> origin/jules-9057756713964855410-d59a7b65
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
