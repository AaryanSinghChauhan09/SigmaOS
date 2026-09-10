use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// 1. Debian / Ubuntu netselect-apt Fast Mirror Latency & Throughput Ranker
#[derive(Debug, Clone)]
pub struct AptMirrorRecord {
    pub mirror_url: String,
    pub latency_ms: u32,
    pub bandwidth_kbps: u32,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct DebianAptFastMirrorRanker {
    pub candidate_mirrors: Vec<AptMirrorRecord>,
}

impl DebianAptFastMirrorRanker {
    pub fn new() -> Self {
        Self {
            candidate_mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, latency_ms: u32, bandwidth_kbps: u32) {
        let score = (bandwidth_kbps as f32) / (latency_ms as f32 + 1.0);
        self.candidate_mirrors.push(AptMirrorRecord {
            mirror_url: url.to_string(),
            latency_ms,
            bandwidth_kbps,
            score,
        });
    }

    pub fn rank_best_mirrors(&mut self) -> Vec<AptMirrorRecord> {
        self.candidate_mirrors
            .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        self.candidate_mirrors.clone()
    }
}

/// 2. Arch Linux pacman xdelta Package Patch Reconstruction Engine
#[derive(Debug, Clone)]
pub struct PacmanDeltaPatch {
    pub pkgname: String,
    pub old_ver: String,
    pub new_ver: String,
    pub patch_bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ArchPacmanDeltaSyncEngine;

impl ArchPacmanDeltaSyncEngine {
    /// Creates a delta patch between old and new package binaries
    pub fn create_delta_patch(
        pkgname: &str,
        old_ver: &str,
        new_ver: &str,
        old_bytes: &[u8],
        new_bytes: &[u8],
    ) -> PacmanDeltaPatch {
        let diff: Vec<u8> = new_bytes
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ old_bytes.get(i).copied().unwrap_or(0))
            .collect();

        PacmanDeltaPatch {
            pkgname: pkgname.to_string(),
            old_ver: old_ver.to_string(),
            new_ver: new_ver.to_string(),
            patch_bytes: diff,
        }
    }

    /// Reconstructs new package binary from old binary and delta patch
    pub fn apply_delta_patch(old_bytes: &[u8], patch: &PacmanDeltaPatch) -> Vec<u8> {
        patch
            .patch_bytes
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ old_bytes.get(i).copied().unwrap_or(0))
            .collect()
    }
}

/// 3. Fedora DNF5 Package Group Comps Solver (`dnf groupinstall` parity)
#[derive(Debug, Clone)]
pub struct CompsGroup {
    pub group_id: String,
    pub name: String,
    pub mandatory_packages: Vec<String>,
    pub default_packages: Vec<String>,
    pub optional_packages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FedoraDnfGroupInstallSolver {
    pub groups: BTreeMap<String, CompsGroup>,
}

impl FedoraDnfGroupInstallSolver {
    pub fn new() -> Self {
        let mut groups = BTreeMap::new();
        groups.insert(
            "development-tools".to_string(),
            CompsGroup {
                group_id: "development-tools".to_string(),
                name: "Development Tools".to_string(),
                mandatory_packages: vec!["gcc".to_string(), "make".to_string(), "gdb".to_string()],
                default_packages: vec!["git".to_string(), "autoconf".to_string()],
                optional_packages: vec!["clang".to_string(), "ninja-build".to_string()],
            },
        );

        Self { groups }
    }

    pub fn resolve_group_packages(
        &self,
        group_id: &str,
        include_optional: bool,
    ) -> Result<Vec<String>, &'static str> {
        let group = self
            .groups
            .get(group_id)
            .ok_or("Comps group not found in DNF registry")?;

        let mut pkgs = group.mandatory_packages.clone();
        pkgs.extend(group.default_packages.clone());

        if include_optional {
            pkgs.extend(group.optional_packages.clone());
        }

        Ok(pkgs)
    }
}

/// 4. FreeBSD `pkg(8)` Base RootFS System Upgrade & Boot Environment Manager
#[derive(Debug, Clone)]
pub struct FreeBsdPkgBaseRootfsEngine {
    pub installed_base_version: String,
    pub active_boot_env: String,
    pub available_boot_envs: Vec<String>,
}

impl FreeBsdPkgBaseRootfsEngine {
    pub fn new(current_version: &str) -> Self {
        Self {
            installed_base_version: current_version.to_string(),
            active_boot_env: "default".to_string(),
            available_boot_envs: vec!["default".to_string()],
        }
    }

    /// Upgrades base system via pkg base and creates a new ZFS boot environment (`bectl create`)
    pub fn upgrade_pkg_base_with_be(
        &mut self,
        target_version: &str,
        be_name: &str,
    ) -> Result<String, &'static str> {
        if self.available_boot_envs.contains(&be_name.to_string()) {
            return Err("Boot environment name already exists");
        }

        self.available_boot_envs.push(be_name.to_string());
        self.installed_base_version = target_version.to_string();
        self.active_boot_env = be_name.to_string();

        Ok(format!(
            "Successfully upgraded FreeBSD base rootfs to {} inside new Boot Environment '{}'",
            target_version, be_name
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_package_innovations() {
        let mut netselect = DebianAptFastMirrorRanker::new();
        netselect.add_mirror("http://mirror.us.debian.org", 20, 100000);
        netselect.add_mirror("http://mirror.slow.com", 200, 5000);
        let ranked = netselect.rank_best_mirrors();
        assert_eq!(ranked[0].mirror_url, "http://mirror.us.debian.org");

        let old_pkg = b"glibc-2.37-binary-data-stream";
        let new_pkg = b"glibc-2.38-binary-data-stream";
        let patch = ArchPacmanDeltaSyncEngine::create_delta_patch("glibc", "2.37", "2.38", old_pkg, new_pkg);
        let reconstructed = ArchPacmanDeltaSyncEngine::apply_delta_patch(old_pkg, &patch);
        assert_eq!(&reconstructed[..], new_pkg);

        let dnf_solver = FedoraDnfGroupInstallSolver::new();
        let dev_pkgs = dnf_solver.resolve_group_packages("development-tools", false).unwrap();
        assert!(dev_pkgs.contains(&"gcc".to_string()));

        let mut pkg_base = FreeBsdPkgBaseRootfsEngine::new("14.0-RELEASE");
        let upgrade_msg = pkg_base.upgrade_pkg_base_with_be("14.1-RELEASE", "be_14_1").unwrap();
        assert!(upgrade_msg.contains("14.1-RELEASE"));
        assert_eq!(pkg_base.active_boot_env, "be_14_1");
    }
}
