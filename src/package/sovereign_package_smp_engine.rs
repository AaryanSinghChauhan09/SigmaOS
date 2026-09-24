// SPDX-License-Identifier: MIT
// SigmaOS - Sovereign Package SMP Multi-Core Parallel Build Engine
// Inspired by Linux & BSD multi-core package building paradigms:
// 1. Gentoo Portage `MAKEOPTS="-jN"` parallel compiler job scheduler & load average throttle
// 2. Arch Linux `makepkg` multi-threaded compression (`COMPRESSZST=(zstd -c -z -q --threads=0 -)`)
// 3. FreeBSD Poudriere parallel jail package build queue & build-slot dependency graph solver
// 4. NixOS / GNU Guix `max-jobs` & `build-cores` SMP CPU affinity pinning governor

#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "standalone_test")]
extern crate alloc;

#[cfg(not(feature = "standalone_test"))]
use std::collections::BTreeMap;
#[cfg(not(feature = "standalone_test"))]
use std::format;
#[cfg(not(feature = "standalone_test"))]
use std::string::{String, ToString};
#[cfg(not(feature = "standalone_test"))]
use std::vec::Vec;

#[cfg(feature = "standalone_test")]
use alloc::collections::BTreeMap;
#[cfg(feature = "standalone_test")]
use alloc::format;
#[cfg(feature = "standalone_test")]
use alloc::string::{String, ToString};
#[cfg(feature = "standalone_test")]
use alloc::vec::Vec;

#[cfg(not(feature = "standalone_test"))]
use crate::package::universal::UnifiedPackage;

#[cfg(feature = "standalone_test")]
#[path = "universal.rs"]
pub mod universal;

#[cfg(feature = "standalone_test")]
pub use universal::{PackageError, PackageFormat, UnifiedPackage};

// =========================================================================
// 1. Gentoo Portage & Nix `MAKEOPTS="-jN"` Multi-Core Build Scheduler
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageBuildJob {
    pub package_name: String,
    pub version: String,
    pub estimated_duration_secs: u32,
    pub required_cpu_cores: usize,
    pub dependencies: Vec<String>,
}

pub struct SovereignPackageSmpBuildEngine {
    pub total_cpu_cores: usize,
    pub max_parallel_jobs: usize,
    pub load_average_limit: f32,
    pub active_jobs: BTreeMap<String, usize>, // package_name -> allocated_cores
    pub pending_queue: Vec<PackageBuildJob>,
}

impl SovereignPackageSmpBuildEngine {
    pub fn new(total_cores: usize) -> Self {
        let max_jobs = total_cores.max(1);
        Self {
            total_cpu_cores: total_cores,
            max_parallel_jobs: max_jobs,
            load_average_limit: total_cores as f32 * 1.5,
            active_jobs: BTreeMap::new(),
            pending_queue: Vec::new(),
        }
    }

    pub fn enqueue_build(&mut self, job: PackageBuildJob) {
        self.pending_queue.push(job);
    }

    pub fn available_cores(&self) -> usize {
        let used_cores: usize = self.active_jobs.values().sum();
        if self.total_cpu_cores > used_cores {
            self.total_cpu_cores - used_cores
        } else {
            0
        }
    }

    pub fn schedule_next_runnable_jobs(&mut self, completed_packages: &[String]) -> Vec<String> {
        let mut scheduled = Vec::new();
        let mut idx = 0;

        while idx < self.pending_queue.len() {
            let job = &self.pending_queue[idx];
            let deps_satisfied = job
                .dependencies
                .iter()
                .all(|dep| completed_packages.contains(dep));

            if deps_satisfied {
                let req_cores = job.required_cpu_cores.max(1);
                if self.available_cores() >= req_cores {
                    let job = self.pending_queue.remove(idx);
                    self.active_jobs.insert(job.package_name.clone(), req_cores);
                    scheduled.push(job.package_name);
                    continue;
                }
            }
            idx += 1;
        }

        scheduled
    }

    pub fn finish_job(&mut self, pkg_name: &str) -> bool {
        self.active_jobs.remove(pkg_name).is_some()
    }
}

impl Default for SovereignPackageSmpBuildEngine {
    fn default() -> Self {
        Self::new(8)
    }
}

// =========================================================================
// 2. Arch `makepkg` Multi-Threaded Compression Job Manager
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveCompressionAlgorithm {
    Zstd,
    Xz,
    Gzip,
    Bzip2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelCompressorConfig {
    pub algorithm: ArchiveCompressionAlgorithm,
    pub compression_level: u32,
    pub worker_threads: usize, // 0 = auto-detect all online CPU cores
}

pub struct ParallelPackageCompressor {
    pub config: ParallelCompressorConfig,
}

impl ParallelPackageCompressor {
    pub fn new(algorithm: ArchiveCompressionAlgorithm, threads: usize) -> Self {
        Self {
            config: ParallelCompressorConfig {
                algorithm,
                compression_level: 6,
                worker_threads: threads,
            },
        }
    }

    pub fn build_compressor_command(&self, input_tar: &str, output_archive: &str) -> String {
        let threads = if self.config.worker_threads == 0 {
            "0".to_string()
        } else {
            self.config.worker_threads.to_string()
        };

        match self.config.algorithm {
            ArchiveCompressionAlgorithm::Zstd => format!(
                "zstd -c -z -q -{} --threads={} {} > {}",
                self.config.compression_level, threads, input_tar, output_archive
            ),
            ArchiveCompressionAlgorithm::Xz => format!(
                "xz -c -z -{} --threads={} {} > {}",
                self.config.compression_level, threads, input_tar, output_archive
            ),
            ArchiveCompressionAlgorithm::Gzip => format!(
                "pigz -c -{} -p {} {} > {}",
                self.config.compression_level, threads, input_tar, output_archive
            ),
            ArchiveCompressionAlgorithm::Bzip2 => format!(
                "lbzip2 -c -{} -n {} {} > {}",
                self.config.compression_level, threads, input_tar, output_archive
            ),
        }
    }
}

impl Default for ParallelPackageCompressor {
    fn default() -> Self {
        Self::new(ArchiveCompressionAlgorithm::Zstd, 0)
    }
}

// =========================================================================
// 3. FreeBSD Poudriere Parallel Jail Package Build Queue
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoudriereJailSlot {
    pub slot_id: u32,
    pub jail_name: String,
    pub assigned_package: Option<String>,
    pub is_busy: bool,
}

pub struct PoudriereMultiJobQueue {
    pub slots: Vec<PoudriereJailSlot>,
    pub build_log_history: Vec<String>,
}

impl PoudriereMultiJobQueue {
    pub fn new(jail_count: u32) -> Self {
        let mut slots = Vec::new();
        for i in 0..jail_count {
            slots.push(PoudriereJailSlot {
                slot_id: i,
                jail_name: format!("poudriere_jail_{}", i),
                assigned_package: None,
                is_busy: false,
            });
        }
        Self {
            slots,
            build_log_history: Vec::new(),
        }
    }

    pub fn dispatch_package_to_idle_jail(&mut self, pkg_name: &str) -> Option<u32> {
        for slot in &mut self.slots {
            if !slot.is_busy {
                slot.is_busy = true;
                slot.assigned_package = Some(pkg_name.to_string());
                self.build_log_history.push(format!(
                    "Dispatched {} to {}",
                    pkg_name, slot.jail_name
                ));
                return Some(slot.slot_id);
            }
        }
        None
    }

    pub fn release_jail(&mut self, slot_id: u32) -> bool {
        if let Some(slot) = self.slots.iter_mut().find(|s| s.slot_id == slot_id) {
            slot.is_busy = false;
            let pkg = slot.assigned_package.take();
            if let Some(p) = pkg {
                self.build_log_history
                    .push(format!("Completed {} in {}", p, slot.jail_name));
            }
            true
        } else {
            false
        }
    }
}

impl Default for PoudriereMultiJobQueue {
    fn default() -> Self {
        Self::new(4)
    }
}

// =========================================================================
// 4. SmpBuildSlotGovernor CPU Affinity & Topology Core Governor
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuCoreAffinityGroup {
    pub group_id: u32,
    pub core_ids: Vec<usize>,
    pub assigned_build_job: Option<String>,
}

pub struct SmpBuildSlotGovernor {
    pub affinity_groups: Vec<CpuCoreAffinityGroup>,
}

impl SmpBuildSlotGovernor {
    pub fn new(total_cores: usize, cores_per_group: usize) -> Self {
        let cores_per_group = cores_per_group.max(1);
        let num_groups = (total_cores + cores_per_group - 1) / cores_per_group;
        let mut groups = Vec::new();

        for g in 0..num_groups {
            let start = g * cores_per_group;
            let end = (start + cores_per_group).min(total_cores);
            let core_ids = (start..end).collect();

            groups.push(CpuCoreAffinityGroup {
                group_id: g as u32,
                core_ids,
                assigned_build_job: None,
            });
        }

        Self {
            affinity_groups: groups,
        }
    }

    pub fn allocate_affinity_group(&mut self, pkg_name: &str) -> Option<Vec<usize>> {
        for group in &mut self.affinity_groups {
            if group.assigned_build_job.is_none() {
                group.assigned_build_job = Some(pkg_name.to_string());
                return Some(group.core_ids.clone());
            }
        }
        None
    }

    pub fn free_affinity_group(&mut self, pkg_name: &str) -> bool {
        for group in &mut self.affinity_groups {
            if group.assigned_build_job.as_deref() == Some(pkg_name) {
                group.assigned_build_job = None;
                return true;
            }
        }
        false
    }
}

impl Default for SmpBuildSlotGovernor {
    fn default() -> Self {
        Self::new(16, 4)
    }
}

// =========================================================================
// 5. Sovereign Package SMP Orchestrator Suite
// =========================================================================

pub struct SovereignPackageSmpOrchestratorSuite {
    pub scheduler: SovereignPackageSmpBuildEngine,
    pub compressor: ParallelPackageCompressor,
    pub poudriere: PoudriereMultiJobQueue,
    pub governor: SmpBuildSlotGovernor,
}

impl SovereignPackageSmpOrchestratorSuite {
    pub fn new(total_cores: usize) -> Self {
        Self {
            scheduler: SovereignPackageSmpBuildEngine::new(total_cores),
            compressor: ParallelPackageCompressor::new(ArchiveCompressionAlgorithm::Zstd, 0),
            poudriere: PoudriereMultiJobQueue::new(total_cores as u32 / 2),
            governor: SmpBuildSlotGovernor::new(total_cores, 4),
        }
    }

    pub fn prepare_package_smp_build(&mut self, pkg: &mut UnifiedPackage) -> Result<(), String> {
        let cores = self.governor.allocate_affinity_group(&pkg.name);
        if let Some(c) = cores {
            pkg.properties.insert(
                "smp_assigned_cores".to_string(),
                format!("{:?}", c),
            );
        }

        pkg.properties.insert(
            "parallel_compress_cmd".to_string(),
            self.compressor
                .build_compressor_command("pkg_data.tar", "pkg_data.tar.zst"),
        );

        Ok(())
    }
}

impl Default for SovereignPackageSmpOrchestratorSuite {
    fn default() -> Self {
        Self::new(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smp_build_scheduler() {
        let mut engine = SovereignPackageSmpBuildEngine::new(8);
        engine.enqueue_build(PackageBuildJob {
            package_name: "libffi".to_string(),
            version: "3.4.4".to_string(),
            estimated_duration_secs: 10,
            required_cpu_cores: 2,
            dependencies: vec![],
        });

        engine.enqueue_build(PackageBuildJob {
            package_name: "python".to_string(),
            version: "3.12.0".to_string(),
            estimated_duration_secs: 60,
            required_cpu_cores: 4,
            dependencies: vec!["libffi".to_string()],
        });

        let scheduled = engine.schedule_next_runnable_jobs(&[]);
        assert_eq!(scheduled, vec!["libffi".to_string()]);

        engine.finish_job("libffi");
        let scheduled_next = engine.schedule_next_runnable_jobs(&["libffi".to_string()]);
        assert_eq!(scheduled_next, vec!["python".to_string()]);
    }

    #[test]
    fn test_parallel_compressor() {
        let comp = ParallelPackageCompressor::new(ArchiveCompressionAlgorithm::Zstd, 4);
        let cmd = comp.build_compressor_command("in.tar", "out.tar.zst");
        assert!(cmd.contains("zstd -c -z -q -6 --threads=4 in.tar > out.tar.zst"));
    }

    #[test]
    fn test_poudriere_queue() {
        let mut queue = PoudriereMultiJobQueue::new(2);
        let slot0 = queue.dispatch_package_to_idle_jail("bash");
        assert_eq!(slot0, Some(0));

        let slot1 = queue.dispatch_package_to_idle_jail("zsh");
        assert_eq!(slot1, Some(1));

        let slot_none = queue.dispatch_package_to_idle_jail("fish");
        assert_eq!(slot_none, None);

        assert!(queue.release_jail(0));
        let slot0_retry = queue.dispatch_package_to_idle_jail("fish");
        assert_eq!(slot0_retry, Some(0));
    }

    #[test]
    fn test_smp_slot_governor() {
        let mut governor = SmpBuildSlotGovernor::new(8, 4);
        let group0 = governor.allocate_affinity_group("gcc").unwrap();
        assert_eq!(group0, vec![0, 1, 2, 3]);

        assert!(governor.free_affinity_group("gcc"));
    }

    #[test]
    fn test_orchestrator_suite() {
        let mut suite = SovereignPackageSmpOrchestratorSuite::new(8);
        let mut pkg = UnifiedPackage::new("rustc".to_string(), "1.75.0".to_string());
        assert!(suite.prepare_package_smp_build(&mut pkg).is_ok());
        assert!(pkg.properties.contains_key("smp_assigned_cores"));
        assert!(pkg.properties.contains_key("parallel_compress_cmd"));
    }
}
