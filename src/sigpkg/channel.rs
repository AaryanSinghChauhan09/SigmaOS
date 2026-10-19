//! SigmaOS Package Repository Channels
//!
//! Following principles: PRACTICAL_LICENSING_AWARENESS, MERITOCRACY_AND_INCLUSIVITY,
//! STABLE_APIS, OPEN_STANDARD_AND_INTEROPERABILITY, COMMUNITY_ORIENTED_DEVELOPMENT,
//! VERSION_CONTROL_PRACTICES, DON'T_REPEAT_YOURSELF, SECURE_DEFAULTS,
//! FAIL_FAST_SYSTEM, DEFENSIVE_PROGRAMMING, PRIVILEGE_SEPARATION.
//!
//! Competitive with: Arch pacman, Mint apt, Omarchy package channels (edge/rc/stable).
//!
//! Channel system:
//! - edge: bleeding edge, built from PRs
//! - rc: release candidate, tested
//! - stable: production-ready, signed packages

use core::fmt;

/// Package release channel
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Channel {
    /// Edge channel: built from every PR, unsigned, for testing
    Edge,
    /// RC channel: release candidate, tested, signed
    Rc,
    /// Stable channel: production-ready, signed, long-term support
    Stable,
}

impl Channel {
    /// Get the channel name as a string
    pub fn name(&self) -> &str {
        match self {
            Channel::Edge => "edge",
            Channel::Rc => "rc",
            Channel::Stable => "stable",
        }
    }

    /// Check if packages from this channel require signatures
    pub fn requires_signature(&self) -> bool {
        match self {
            Channel::Edge => false,
            Channel::Rc => true,
            Channel::Stable => true,
        }
    }

    /// Check if packages can be promoted to this channel
    pub fn can_promote_from(&self, from: Channel) -> bool {
        match (self, from) {
            (Channel::Edge, _) => true, // Edge accepts everything
            (Channel::Rc, Channel::Edge) => true, // Edge -> RC
            (Channel::Stable, Channel::Rc) => true, // RC -> Stable
            (Channel::Stable, Channel::Edge) => false, // Edge cannot go directly to Stable
            _ => false,
        }
    }

    /// Get the promotion path description
    pub fn promotion_path(&self) -> Vec<Channel> {
        match self {
            Channel::Edge => vec![Channel::Edge],
            Channel::Rc => vec![Channel::Edge, Channel::Rc],
            Channel::Stable => vec![Channel::Edge, Channel::Rc, Channel::Stable],
        }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Channel::Stable
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Package metadata with channel information
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub release: u32,
    pub channel: Channel,
    pub architecture: String,
    pub checksum: String,
    pub signed: bool,
    pub dependencies: Vec<String>,
    pub description: String,
}

impl PackageMetadata {
    /// Create new package metadata
    pub fn new(name: String, version: String, channel: Channel) -> Self {
        Self {
            name,
            version,
            release: 1,
            channel,
            architecture: "x86_64".to_string(),
            checksum: String::new(),
            signed: channel.requires_signature(),
            dependencies: Vec::new(),
            description: String::new(),
        }
    }

    /// Get the full package identifier
    pub fn identifier(&self) -> String {
        format!("{}-{}-{}", self.name, self.version, self.release)
    }

    /// Check if this package can be installed on stable channel
    pub fn is_stable_ready(&self) -> bool {
        self.channel == Channel::Stable && self.signed && !self.checksum.is_empty()
    }
}

/// Repository configuration
#[derive(Debug, Clone)]
pub struct RepositoryConfig {
    pub name: String,
    pub url: String,
    pub channel: Channel,
    pub enabled: bool,
    pub priority: u32,
    pub gpg_key: Option<String>,
}

impl RepositoryConfig {
    /// Create a new repository configuration
    pub fn new(name: String, url: String, channel: Channel) -> Self {
        Self {
            name,
            url,
            channel,
            enabled: true,
            priority: 100,
            gpg_key: if channel.requires_signature() {
                Some(format!("{}-gpg-key", channel.name()))
            } else {
                None
            },
        }
    }
}

impl fmt::Display for PackageMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({}) [{}]",
            self.name, self.version, self.release, self.channel
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_names() {
        assert_eq!(Channel::Edge.name(), "edge");
        assert_eq!(Channel::Rc.name(), "rc");
        assert_eq!(Channel::Stable.name(), "stable");
    }

    #[test]
    fn test_channel_signature_requirements() {
        assert!(!Channel::Edge.requires_signature());
        assert!(Channel::Rc.requires_signature());
        assert!(Channel::Stable.requires_signature());
    }

    #[test]
    fn test_channel_promotion() {
        assert!(Channel::Rc.can_promote_from(Channel::Edge));
        assert!(Channel::Stable.can_promote_from(Channel::Rc));
        assert!(!Channel::Stable.can_promote_from(Channel::Edge));
    }

    #[test]
    fn test_package_metadata() {
        let pkg = PackageMetadata::new(
            "sigmaos-kernel".to_string(),
            "6.8.0".to_string(),
            Channel::Stable,
        );
        assert_eq!(pkg.identifier(), "sigmaos-kernel-6.8.0-1");
        assert!(pkg.is_stable_ready());
    }

    #[test]
    fn test_edge_package_not_stable() {
        let pkg = PackageMetadata::new(
            "test-pkg".to_string(),
            "0.1.0".to_string(),
            Channel::Edge,
        );
        assert!(!pkg.is_stable_ready());
    }
}