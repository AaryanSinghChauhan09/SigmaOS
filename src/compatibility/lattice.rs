// SPDX-License-Identifier: MIT
// SigmaOS Kernel Feature Lattice & Syscall Tracker
// Encapsulates fine-grained feature path selection and lifecycle-aware syscall tracking

use std::collections::BTreeMap;
use std::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LatticeFeature {
    LegacyMemoryModel,
    PredictiveScheduling,
    AncientAddressing,
    ZeroTrustNetwork,
}

pub struct KernelLattice {
    pub enabled_features: BTreeMap<LatticeFeature, bool>,
}

impl KernelLattice {
    pub fn new() -> Self {
        let mut lattice = KernelLattice {
            enabled_features: BTreeMap::new(),
        };
        lattice
            .enabled_features
            .insert(LatticeFeature::LegacyMemoryModel, false);
        lattice
            .enabled_features
            .insert(LatticeFeature::PredictiveScheduling, true);
        lattice
    }

    pub fn enable_feature(&mut self, feature: LatticeFeature) {
        self.enabled_features.insert(feature, true);
    }

    pub fn disable_feature(&mut self, feature: LatticeFeature) {
        self.enabled_features.insert(feature, false);
    }

    pub fn is_feature_active(&self, feature: LatticeFeature) -> bool {
        *self.enabled_features.get(&feature).unwrap_or(&false)
    }
}

// =========================================================================
// SYSCALL EVOLUTION TRACKER IMPLEMENTATION
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallLifecycle {
    Introduced,
    Modified,
    Deprecated,
}

pub struct SyscallHistory {
    pub name: String,
    pub lifecycle: SyscallLifecycle,
    pub since_kernel_version: String,
}

pub struct SyscallTracker {
    pub tracking_pool: BTreeMap<u32, SyscallHistory>,
}

impl SyscallTracker {
    pub fn new() -> Self {
        let mut tracker = SyscallTracker {
            tracking_pool: BTreeMap::new(),
        };
        // Seed default trace history
        tracker.register_syscall(
            1,
            "sys_exit".to_string(),
            SyscallLifecycle::Deprecated,
            "Linux 2.6".to_string(),
        );
        tracker.register_syscall(
            60,
            "sys_exit_group".to_string(),
            SyscallLifecycle::Introduced,
            "Linux 3.x".to_string(),
        );
        tracker
    }

    pub fn register_syscall(
        &mut self,
        num: u32,
        name: String,
        lifecycle: SyscallLifecycle,
        version: String,
    ) {
        self.tracking_pool.insert(
            num,
            SyscallHistory {
                name,
                lifecycle,
                since_kernel_version: version,
            },
        );
    }

    pub fn query_lifecycle(&self, num: u32) -> Option<&SyscallHistory> {
        self.tracking_pool.get(&num)
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_lattice_features() {
        let mut lattice = KernelLattice::new();
        assert!(!lattice.is_feature_active(LatticeFeature::LegacyMemoryModel));
        assert!(lattice.is_feature_active(LatticeFeature::PredictiveScheduling));

        lattice.enable_feature(LatticeFeature::LegacyMemoryModel);
        assert!(lattice.is_feature_active(LatticeFeature::LegacyMemoryModel));
    }

    #[test]
    fn test_syscall_tracker_lifecycle() {
        let tracker = SyscallTracker::new();
        let history = tracker.query_lifecycle(1).unwrap();
        assert_eq!(history.name, "sys_exit");
        assert_eq!(history.lifecycle, SyscallLifecycle::Deprecated);
        assert_eq!(history.since_kernel_version, "Linux 2.6");
    }
}
