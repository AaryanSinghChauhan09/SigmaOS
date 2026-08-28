extern crate alloc;
// SPDX-License-Identifier: MIT
// SigmaOS Stable Linux Distro Parity Subsystem
// Inspired by RHEL/Rocky/AlmaLinux, Debian dpkg, Alpine lbu/apk, and Enterprise systemd cgroupv2

#[cfg(not(target_os = "none"))]
use std::vec::Vec;

#[cfg(target_os = "none")]

#[cfg(target_os = "none")]
use alloc::vec::Vec;

// ============================================================================
// 1. RHEL / Rocky / AlmaLinux Subscription & Entitlement Manager
// ============================================================================

/// Subscription Pool definition
#[derive(Debug, Clone)]
pub struct SubscriptionPool {
    pub pool_id: &'static str,
    pub product_name: &'static str,
    pub total_quantity: usize,
    pub consumed_quantity: usize,
}

/// Entitlement certificate attached to system
#[derive(Debug, Clone)]
pub struct EntitlementCertificate {
    pub sku: &'static str,
    pub serial_number: u64,
    pub active: bool,
}

/// RHEL / Rocky / AlmaLinux Subscription Entitlement Manager
#[derive(Debug)]
pub struct RhelSubscriptionEntitlementManager {
    pub system_uuid: u64,
    is_registered: bool,
    pools: Vec<SubscriptionPool>,
    entitlements: Vec<EntitlementCertificate>,
}

impl RhelSubscriptionEntitlementManager {
    pub fn new(system_uuid: u64) -> Self {
        let mut mgr = Self {
            system_uuid,
            is_registered: false,
            pools: Vec::new(),
            entitlements: Vec::new(),
        };

        // Standard RHEL Server Enterprise Pool
        mgr.pools.push(SubscriptionPool {
            pool_id: "RH-SERVER-POOL-8800",
            product_name: "Red Hat Enterprise Linux Server, Standard",
            total_quantity: 100,
            consumed_quantity: 10,
        });

        mgr
    }

    pub fn register_system(&mut self, org_key: &str) -> Result<(), &'static str> {
        if org_key.is_empty() {
            return Err("Invalid Organization Activation Key");
        }
        self.is_registered = true;
        Ok(())
    }

    pub fn attach_pool(&mut self, pool_id: &str) -> Result<u64, &'static str> {
        if !self.is_registered {
            return Err("System is not registered with subscription-manager");
        }

        if let Some(pool) = self.pools.iter_mut().find(|p| p.pool_id == pool_id) {
            if pool.consumed_quantity >= pool.total_quantity {
                return Err("Subscription Pool is fully consumed");
            }
            pool.consumed_quantity += 1;

            let serial = self.system_uuid ^ (pool.consumed_quantity as u64);
            self.entitlements.push(EntitlementCertificate {
                sku: pool.pool_id,
                serial_number: serial,
                active: true,
            });
            return Ok(serial);
        }

        Err("Subscription Pool ID not found")
    }

    pub fn verify_entitlement(&self) -> bool {
        self.is_registered && self.entitlements.iter().any(|cert| cert.active)
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }
}

// ============================================================================
// 2. Debian dpkg Database & Package Trigger Simulator
// ============================================================================

/// Debian dpkg package states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpkgPackageStatus {
    NotInstalled,
    HalfInstalled,
    Unpacked,
    HalfConfigured,
    Installed,
}

/// Package record entry in /var/lib/dpkg/status
#[derive(Debug, Clone)]
pub struct DpkgPackageRecord {
    pub name: &'static str,
    pub version: &'static str,
    pub architecture: &'static str,
    pub status: DpkgPackageStatus,
}

/// Debian dpkg Database Manager
#[derive(Debug)]
pub struct DebianDpkgDbSimulator {
    packages: Vec<DpkgPackageRecord>,
    pending_triggers: Vec<&'static str>,
}

impl DebianDpkgDbSimulator {
    pub fn new() -> Self {
        let mut db = Self {
            packages: Vec::new(),
            pending_triggers: Vec::new(),
        };

        // Pre-populate core Debian base-files package
        db.packages.push(DpkgPackageRecord {
            name: "base-files",
            version: "12.2+deb12u1",
            architecture: "amd64",
            status: DpkgPackageStatus::Installed,
        });

        db
    }

    pub fn register_package(
        &mut self,
        name: &'static str,
        version: &'static str,
        arch: &'static str,
    ) {
        if let Some(pkg) = self.packages.iter_mut().find(|p| p.name == name) {
            pkg.version = version;
            pkg.architecture = arch;
            pkg.status = DpkgPackageStatus::Unpacked;
        } else {
            self.packages.push(DpkgPackageRecord {
                name,
                version,
                architecture: arch,
                status: DpkgPackageStatus::Unpacked,
            });
        }
    }

    pub fn transition_status(
        &mut self,
        name: &str,
        new_status: DpkgPackageStatus,
    ) -> Result<(), &'static str> {
        if let Some(pkg) = self.packages.iter_mut().find(|p| p.name == name) {
            pkg.status = new_status;
            if new_status == DpkgPackageStatus::Installed {
                self.pending_triggers.push("man-db");
                self.pending_triggers.push("libc-bin");
            }
            return Ok(());
        }
        Err("Package entry not found in dpkg status database")
    }

    pub fn query_status(&self, name: &str) -> DpkgPackageStatus {
        self.packages
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.status)
            .unwrap_or(DpkgPackageStatus::NotInstalled)
    }

    pub fn execute_pending_triggers(&mut self) -> usize {
        let count = self.pending_triggers.len();
        self.pending_triggers.clear();
        count
    }
}

// ============================================================================
// 3. Alpine Linux APK Index & LBU Overlay Engine
// ============================================================================

/// Alpine overlay file entry
#[derive(Debug, Clone)]
pub struct ApkOverlayFile {
    pub path_hash: u64,
    pub is_volatile_ram: bool,
    pub size_bytes: usize,
}

/// Alpine Local Backup (lbu) & APK Overlay Engine
#[derive(Debug)]
pub struct AlpineApkOverlayEngine {
    overlay_files: Vec<ApkOverlayFile>,
    committed_apkovl_count: usize,
}

impl AlpineApkOverlayEngine {
    pub fn new() -> Self {
        Self {
            overlay_files: Vec::new(),
            committed_apkovl_count: 0,
        }
    }

    pub fn add_overlay_file(&mut self, path: &str, size_bytes: usize) {
        let mut path_hash: u64 = 0xcbf29ce484222325;
        for &byte in path.as_bytes() {
            path_hash ^= u64::from(byte);
            path_hash = path_hash.wrapping_mul(0x100000001b3);
        }

        if let Some(file) = self
            .overlay_files
            .iter_mut()
            .find(|f| f.path_hash == path_hash)
        {
            file.size_bytes = size_bytes;
        } else {
            self.overlay_files.push(ApkOverlayFile {
                path_hash,
                is_volatile_ram: true,
                size_bytes,
            });
        }
    }

    pub fn commit_lbu_changes(&mut self) -> usize {
        for file in &mut self.overlay_files {
            file.is_volatile_ram = false;
        }
        self.committed_apkovl_count += 1;
        self.overlay_files.len()
    }

    pub fn get_apkovl_count(&self) -> usize {
        self.committed_apkovl_count
    }
}

// ============================================================================
// 4. Systemd Unified Cgroup v2 Governor
// ============================================================================

/// Cgroup v2 Resource Limit parameters
#[derive(Debug, Clone, Copy)]
pub struct CgroupV2Limits {
    pub cpu_weight: u32,
    pub memory_max_bytes: u64,
    pub io_weight: u32,
}

/// Systemd Cgroup v2 Accounting Counters
#[derive(Debug, Clone, Copy)]
pub struct CgroupV2Accounting {
    pub cpu_usage_ns: u64,
    pub memory_current_bytes: u64,
}

/// Unit Cgroup Controller
#[derive(Debug)]
pub struct SystemdCgroupGovernor {
    pub limits: CgroupV2Limits,
    pub accounting: CgroupV2Accounting,
}

impl SystemdCgroupGovernor {
    pub fn new(limits: CgroupV2Limits) -> Self {
        Self {
            limits,
            accounting: CgroupV2Accounting {
                cpu_usage_ns: 0,
                memory_current_bytes: 0,
            },
        }
    }

    pub fn consume_memory(&mut self, bytes: u64) -> Result<(), &'static str> {
        if self.accounting.memory_current_bytes + bytes > self.limits.memory_max_bytes {
            return Err("OOM: Memory limit exceeded (cgroupv2 memory.max)");
        }
        self.accounting.memory_current_bytes += bytes;
        Ok(())
    }

    pub fn record_cpu_ns(&mut self, duration_ns: u64) {
        self.accounting.cpu_usage_ns += duration_ns;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhel_subscription_manager() {
        let mut mgr = RhelSubscriptionEntitlementManager::new(0xABCD1234EF567890);
        assert!(!mgr.is_registered());
        assert!(!mgr.verify_entitlement());

        // Register system
        assert!(mgr.register_system("ORG-KEY-SIGMA-SERVER").is_ok());
        assert!(mgr.is_registered());

        // Attach subscription pool
        let serial = mgr.attach_pool("RH-SERVER-POOL-8800").unwrap();
        assert!(serial > 0);
        assert!(mgr.verify_entitlement());
    }

    #[test]
    fn test_debian_dpkg_db() {
        let mut db = DebianDpkgDbSimulator::new();

        // Check pre-populated base-files status
        assert_eq!(db.query_status("base-files"), DpkgPackageStatus::Installed);

        // Register new package
        db.register_package("nginx", "1.22.1-9", "amd64");
        assert_eq!(db.query_status("nginx"), DpkgPackageStatus::Unpacked);

        // Transition status to Installed
        assert!(db
            .transition_status("nginx", DpkgPackageStatus::Installed)
            .is_ok());
        assert_eq!(db.query_status("nginx"), DpkgPackageStatus::Installed);

        // Execute pending triggers
        let triggers_run = db.execute_pending_triggers();
        assert_eq!(triggers_run, 2);
    }

    #[test]
    fn test_alpine_apk_overlay_engine() {
        let mut overlay = AlpineApkOverlayEngine::new();

        overlay.add_overlay_file("/etc/network/interfaces", 256);
        overlay.add_overlay_file("/etc/apk/world", 128);

        assert_eq!(overlay.get_apkovl_count(), 0);

        let committed = overlay.commit_lbu_changes();
        assert_eq!(committed, 2);
        assert_eq!(overlay.get_apkovl_count(), 1);
    }

    #[test]
    fn test_systemd_cgroup_governor() {
        let limits = CgroupV2Limits {
            cpu_weight: 100,
            memory_max_bytes: 1024 * 1024 * 10, // 10 MB
            io_weight: 100,
        };

        let mut governor = SystemdCgroupGovernor::new(limits);

        // Consume memory within limit
        assert!(governor.consume_memory(1024 * 1024 * 5).is_ok());

        // Exceed memory limit
        assert!(governor.consume_memory(1024 * 1024 * 10).is_err());

        // Record CPU accounting
        governor.record_cpu_ns(500_000_000);
        assert_eq!(governor.accounting.cpu_usage_ns, 500_000_000);
    }
}
