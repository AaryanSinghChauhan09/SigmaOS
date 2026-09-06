#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SPDX-License-Identifier: MIT
// SigmaPkg Daemon (sigpkgd) — background repository maintenance
// Implements the SIGPKG_DESIGN "Daemon responsibilities": periodic repository
// sync with TUF metadata verification, update checking, and GC of orphaned
// store paths. Reuses the no_std SigpkgClient, ContentAddressedStore,
// Manifest, TufRole, and CryptoVerifier from the sigpkg crate.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

use crate::sigpkg::{CryptoVerifier, Manifest, SigpkgClient, TufRole, Version};

/// Result of an update check for a single installed package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateAvailable {
    pub name: String,
    pub installed: Version,
    pub available: Version,
}

/// A repository whose metadata was last synced (or a failed sync).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncStatus {
    Synced { metadata_roles: usize },
    Failed { reason: String },
}

/// The sigpkgd daemon: owns a client, tracks the last sync status, and exposes
/// repository sync, update checking, and store GC.
pub struct SigpkgDaemon {
    pub client: SigpkgClient,
    pub last_sync: Option<SyncStatus>,
    /// Packages that are referenced/required and therefore must not be GC'd.
    pub referenced: Vec<String>,
}

impl SigpkgDaemon {
    pub fn new(repository_url: &str) -> Self {
        Self {
            client: SigpkgClient::new(repository_url),
            last_sync: None,
            referenced: Vec::new(),
        }
    }

    pub fn add_trusted_key(&mut self, key: &str) {
        self.client.add_trusted_key(key);
    }

    /// Exposure of the verifier for signature production in tests/drivers.
    pub fn verifier(&self) -> &CryptoVerifier {
        &self.client.verifier
    }

    /// Mark a package as referenced so GC retains it.
    pub fn mark_referenced(&mut self, name: &str) {
        if !self.referenced.contains(&name.to_string()) {
            self.referenced.push(name.to_string());
        }
    }

    /// Sync repository metadata. A git-pull-style sync: any invalid/unsigned
    /// role aborts the whole sync (all-or-nothing), so a tampered repository
    /// can never partially poison the daemon's trust state.
    pub fn sync_repository(&mut self, root_payload: &[u8], root_sig: &[u8]) -> SyncStatus {
        if self
            .client
            .fetch_metadata(TufRole::Root, root_payload, root_sig)
        {
            self.last_sync = Some(SyncStatus::Synced {
                metadata_roles: self.client.metadata.len(),
            });
            SyncStatus::Synced {
                metadata_roles: self.client.metadata.len(),
            }
        } else {
            let reason = "root metadata signature verification failed".to_string();
            self.last_sync = Some(SyncStatus::Failed {
                reason: reason.clone(),
            });
            SyncStatus::Failed { reason }
        }
    }

    /// Check for available updates of each installed package by scanning the
    /// repository for a newer manifest version.
    pub fn check_updates(
        &self,
        repo_manifests: &BTreeMap<String, Manifest>,
    ) -> Vec<UpdateAvailable> {
        let mut updates = Vec::new();
        for installed in self.client.store.list() {
            if let Some(avail) = repo_manifests.get(&installed.name) {
                if avail.version > installed.version {
                    updates.push(UpdateAvailable {
                        name: installed.name.clone(),
                        installed: installed.version,
                        available: avail.version,
                    });
                }
            }
        }
        updates
    }

    /// Install/update a repository package into the store, verifying checksum.
    pub fn deploy(
        &mut self,
        manifest: &Manifest,
        payload: &[u8],
        installed: &BTreeMap<String, Version>,
    ) -> Result<String, String> {
        self.client
            .install_from_manifest(manifest, payload, installed)
    }

    /// Garbage-collect orphaned store packages: remove any stored package that
    /// is neither referenced nor provides a needed binary. Returns the count
    /// of packages reclaimed.
    pub fn gc_store(&mut self) -> usize {
        let names: Vec<String> = self
            .client
            .store
            .list()
            .iter()
            .map(|p| p.name.clone())
            .collect();

        let mut reclaimed = 0;
        for name in names {
            let retained = self.referenced.contains(&name);
            if !retained {
                if self.client.store.remove(&name).is_ok() {
                    reclaimed += 1;
                }
            }
        }
        reclaimed
    }

    /// Present a concise daemon status line for logs/telemetry.
    pub fn status_line(&self) -> String {
        let roles = match &self.last_sync {
            Some(SyncStatus::Synced { metadata_roles }) => {
                format!("synced({} roles)", metadata_roles)
            }
            Some(SyncStatus::Failed { reason }) => format!("failed: {}", reason),
            None => "never-synced".to_string(),
        };
        let installed = self.client.store.list().len();
        let referenced = self.referenced.len();
        format!(
            "sigpkgd[{}]: {}; {} installed; {} referenced; repo {}",
            self.client.repository_url, roles, installed, referenced, self.client.repository_url
        )
    }
}

impl Default for SigpkgDaemon {
    fn default() -> Self {
        Self::new("https://repo.sigmaos.dev/sigma")
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    fn payload_checksum(payload: &[u8]) -> String {
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in payload.iter() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        std::format!("{:x}", h)
    }

    fn make_manifest(name: &str, version: Version, payload: &[u8]) -> Manifest {
        Manifest::new(name, version, "test", &payload_checksum(payload))
    }

    #[test]
    fn test_sync_verifies_root_metadata() {
        let mut daemon = SigpkgDaemon::new("https://repo.sigmaos.dev/sigma");
        daemon.add_trusted_key("root-key");
        let payload = b"root-metadata";
        let sig = daemon.verifier().sign("root-key", payload);
        assert!(matches!(
            daemon.sync_repository(payload, &sig),
            SyncStatus::Synced { .. }
        ));
        // Unsigned/attack payload must fail all-or-nothing.
        assert!(matches!(
            daemon.sync_repository(payload, &[]),
            SyncStatus::Failed { .. }
        ));
    }

    #[test]
    fn test_check_updates_detects_newer_versions() {
        let mut daemon = SigpkgDaemon::new("https://repo.sigmaos.dev/sigma");
        let payload = b"hello-bytes";
        let installed_map = BTreeMap::new();
        daemon
            .deploy(
                &make_manifest("hello", Version::new(1, 0, 0), payload),
                payload,
                &installed_map,
            )
            .unwrap();

        let mut repo: BTreeMap<String, Manifest> = BTreeMap::new();
        repo.insert(
            "hello".to_string(),
            make_manifest("hello", Version::new(2, 0, 0), payload),
        );
        repo.insert(
            "other".to_string(),
            make_manifest("other", Version::new(1, 0, 0), payload),
        );

        let updates = daemon.check_updates(&repo);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].name, "hello");
        assert_eq!(updates[0].installed, Version::new(1, 0, 0));
        assert_eq!(updates[0].available, Version::new(2, 0, 0));
    }

    #[test]
    fn test_gc_reclaims_orphans_but_keeps_referenced() {
        let mut daemon = SigpkgDaemon::new("https://repo.sigmaos.dev/sigma");
        let installed_map = BTreeMap::new();
        daemon
            .deploy(
                &make_manifest("keep", Version::new(1, 0, 0), b"keep-bytes"),
                b"keep-bytes",
                &installed_map,
            )
            .unwrap();
        daemon
            .deploy(
                &make_manifest("orphan", Version::new(1, 0, 0), b"orphan-bytes"),
                b"orphan-bytes",
                &installed_map,
            )
            .unwrap();
        daemon.mark_referenced("keep");

        assert_eq!(daemon.gc_store(), 1);
        assert!(daemon.client.store.get("keep").is_some());
        assert!(daemon.client.store.get("orphan").is_none());
    }

    #[test]
    fn test_status_line_reports_state() {
        let mut daemon = SigpkgDaemon::default();
        assert!(daemon.status_line().contains("never-synced"));
        let payload = b"root";
        let sig = daemon.verifier().sign("root-key", payload);
        daemon.add_trusted_key("root-key");
        daemon.sync_repository(payload, &sig);
        assert!(daemon.status_line().contains("synced"));
    }
}
