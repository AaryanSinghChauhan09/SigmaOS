//! Community Outreach and Distributed Repositories module for SigmaOS
//! Governs the registration, validation, and metadata for community-driven package repos.

use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoSecurityLevel {
    CommunitySigned, // Signed by verified PGP community key
    Experimental,    // Unsigned experimental packages
    VettedCore,      // Vetted by core developers (Dilithium-5)
}

pub struct CommunityRepository {
    pub name: &'static str,
    pub url: &'static str,
    pub security_level: RepoSecurityLevel,
    pub package_count: usize,
}

impl CommunityRepository {
    pub const fn new(name: &'static str, url: &'static str, level: RepoSecurityLevel) -> Self {
        Self {
            name,
            url,
            security_level: level,
            package_count: 0,
        }
    }

    pub fn verify_trust_score(&self) -> u32 {
        match self.security_level {
            RepoSecurityLevel::VettedCore => 100,
            RepoSecurityLevel::CommunitySigned => 80,
            RepoSecurityLevel::Experimental => 30,
        }
    }
}
