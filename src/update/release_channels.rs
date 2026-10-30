//! Release-channel policy for a usable SigmaOS distribution.
//!
//! Omarchy's package repository separates `edge`, `rc`, and `stable`, and
//! publishes architecture-specific artifacts that only move forward. SigmaOS
//! keeps that operational model independent from the package backend so APT,
//! ALPM, or a future image updater can consume the same release contract.

use std::collections::BTreeSet;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseChannel {
    Edge,
    ReleaseCandidate,
    Stable,
}

impl ReleaseChannel {
    pub fn can_promote_to(self, target: Self) -> bool {
        target > self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseArchitecture {
    X86_64,
    Aarch64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifact {
    pub package_name: String,
    pub version: String,
    pub architecture: ReleaseArchitecture,
    pub sha256: String,
    pub signature_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub version: String,
    pub channel: ReleaseChannel,
    pub architecture: ReleaseArchitecture,
    pub artifacts: Vec<ReleaseArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseValidationError {
    EmptyVersion,
    EmptyPackageName,
    InvalidSha256,
    UnverifiedSignature,
    ArchitectureMismatch,
    DuplicatePackage,
    InvalidPromotion,
    HealthChecksFailed,
}

impl ReleaseManifest {
    pub fn validate(&self) -> Result<(), ReleaseValidationError> {
        if self.version.trim().is_empty() {
            return Err(ReleaseValidationError::EmptyVersion);
        }

        let mut package_names = BTreeSet::new();
        for artifact in &self.artifacts {
            if artifact.package_name.trim().is_empty() {
                return Err(ReleaseValidationError::EmptyPackageName);
            }
            if artifact.architecture != self.architecture {
                return Err(ReleaseValidationError::ArchitectureMismatch);
            }
            if artifact.sha256.len() != 64
                || !artifact.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
            {
                return Err(ReleaseValidationError::InvalidSha256);
            }
            if !artifact.signature_verified {
                return Err(ReleaseValidationError::UnverifiedSignature);
            }
            if !package_names.insert(artifact.package_name.as_str()) {
                return Err(ReleaseValidationError::DuplicatePackage);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleasePromotion {
    pub manifest: ReleaseManifest,
    pub previous_channel: Option<ReleaseChannel>,
}

impl ReleasePromotion {
    pub fn promote(
        manifest: ReleaseManifest,
        previous_channel: Option<ReleaseChannel>,
        health_checks_passed: bool,
    ) -> Result<Self, ReleaseValidationError> {
        manifest.validate()?;
        if !health_checks_passed {
            return Err(ReleaseValidationError::HealthChecksFailed);
        }
        if let Some(previous) = previous_channel {
            if !previous.can_promote_to(manifest.channel) {
                return Err(ReleaseValidationError::InvalidPromotion);
            }
        }
        Ok(Self {
            manifest,
            previous_channel,
        })
    }
}

impl Default for ReleaseChannel {
    fn default() -> Self {
        Self::Edge
    }
}

impl ToString for ReleaseChannel {
    fn to_string(&self) -> String {
        match self {
            Self::Edge => String::from("edge"),
            Self::ReleaseCandidate => String::from("rc"),
            Self::Stable => String::from("stable"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn artifact(name: &str, architecture: ReleaseArchitecture) -> ReleaseArtifact {
        ReleaseArtifact {
            package_name: name.to_string(),
            version: String::from("1.0.0"),
            architecture,
            sha256: String::from(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            ),
            signature_verified: true,
        }
    }

    fn manifest(channel: ReleaseChannel) -> ReleaseManifest {
        ReleaseManifest {
            version: String::from("1.0.0"),
            channel,
            architecture: ReleaseArchitecture::X86_64,
            artifacts: vec![artifact("sigma-init", ReleaseArchitecture::X86_64)],
        }
    }

    #[test]
    fn validates_signed_architecture_specific_manifest() {
        assert!(manifest(ReleaseChannel::Edge).validate().is_ok());
    }

    #[test]
    fn rejects_unverified_or_duplicate_artifacts() {
        let mut invalid = manifest(ReleaseChannel::Stable);
        invalid.artifacts[0].signature_verified = false;
        assert_eq!(
            invalid.validate(),
            Err(ReleaseValidationError::UnverifiedSignature)
        );

        let mut duplicate = manifest(ReleaseChannel::Stable);
        duplicate
            .artifacts
            .push(artifact("sigma-init", ReleaseArchitecture::X86_64));
        assert_eq!(
            duplicate.validate(),
            Err(ReleaseValidationError::DuplicatePackage)
        );
    }

    #[test]
    fn promotes_only_after_health_checks_and_in_order() {
        assert!(ReleasePromotion::promote(
            manifest(ReleaseChannel::ReleaseCandidate),
            Some(ReleaseChannel::Edge),
            true
        )
        .is_ok());
        assert_eq!(
            ReleasePromotion::promote(
                manifest(ReleaseChannel::Stable),
                Some(ReleaseChannel::Stable),
                true
            ),
            Err(ReleaseValidationError::InvalidPromotion)
        );
        assert_eq!(
            ReleasePromotion::promote(manifest(ReleaseChannel::Stable), None, false),
            Err(ReleaseValidationError::HealthChecksFailed)
        );
    }
}
