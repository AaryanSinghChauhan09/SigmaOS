// SigmaOS BSD Clean-Room Parity Subsystem
// Independent, zero-dependency implementations of BSD (FreeBSD/OpenBSD) core tooling

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;

/// Jailed Execution Environment in FreeBSD virtualization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BsdJail {
    pub jid: u32,
    pub hostname: String,
    pub ip_address: String,
    pub path: String,
    pub is_running: bool,
    pub sysv_ipc_enabled: bool,
}

/// FreeBsdJailManager emulates FreeBSD's lightweight jail OS-level virtualization.
pub struct FreeBsdJailManager {
    pub jails: BTreeMap<u32, BsdJail>,
    pub next_jid: u32,
}

impl FreeBsdJailManager {
    pub fn new() -> Self {
        Self {
            jails: BTreeMap::new(),
            next_jid: 1,
        }
    }

    pub fn create_jail(
        &mut self,
        hostname: &str,
        ip: &str,
        root_path: &str,
    ) -> Result<u32, &'static str> {
        if root_path.is_empty() {
            return Err("Jail path cannot be empty");
        }

        let jid = self.next_jid;
        self.next_jid += 1;

        let jail = BsdJail {
            jid,
            hostname: hostname.to_string(),
            ip_address: ip.to_string(),
            path: root_path.to_string(),
            is_running: true,
            sysv_ipc_enabled: false,
        };

        self.jails.insert(jid, jail);
        Ok(jid)
    }

    pub fn stop_jail(&mut self, jid: u32) -> Result<(), &'static str> {
        if let Some(jail) = self.jails.get_mut(&jid) {
            jail.is_running = false;
            Ok(())
        } else {
            Err("Jail ID not found")
        }
    }

    pub fn enable_sysv_ipc(&mut self, jid: u32) -> Result<(), &'static str> {
        if let Some(jail) = self.jails.get_mut(&jid) {
            jail.sysv_ipc_enabled = true;
            Ok(())
        } else {
            Err("Jail ID not found")
        }
    }

    pub fn check_network_allowed(&self, jid: u32, target_ip: &str) -> bool {
        if let Some(jail) = self.jails.get(&jid) {
            if !jail.is_running {
                return false;
            }
            // Simple rule: jail can talk to its own IP or standard interfaces
            target_ip == jail.ip_address || target_ip == "127.0.0.1"
        } else {
            false
        }
    }
}

impl Default for FreeBsdJailManager {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBsdSysctlKernelMib emulates OpenBSD's sysctl Management Information Base tree.
/// Specifically focuses on securelevel lockdown states (e.g. kern.securelevel).
pub struct OpenBsdSysctlKernelMib {
    pub mib_tree: BTreeMap<String, String>,
}

impl OpenBsdSysctlKernelMib {
    pub fn new() -> Self {
        let mut mib = BTreeMap::new();
        mib.insert("kern.securelevel".to_string(), "0".to_string()); // Standard insecure
        mib.insert("kern.ostype".to_string(), "OpenBSD".to_string());
        mib.insert("hw.ncpu".to_string(), "8".to_string());
        mib.insert("hw.physmem".to_string(), "17179869184".to_string()); // 16GB
        mib.insert("hw.pagesize".to_string(), "4096".to_string());

        Self { mib_tree: mib }
    }

    pub fn query_mib(&self, key: &str) -> Result<String, &'static str> {
        self.mib_tree
            .get(key)
            .cloned()
            .ok_or("MIB key not found in sysctl tree")
    }

    pub fn write_mib(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        if key == "kern.securelevel" {
            let current_level = self.query_mib(key)?.parse::<i32>().unwrap_or(0);
            let next_level = value.parse::<i32>().unwrap_or(0);

            // OpenBSD securelevel constraint: securelevel can ONLY be raised, never lowered
            if next_level < current_level {
                return Err("Operation not permitted: securelevel can only be raised");
            }
        }

        self.mib_tree.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn is_raw_disk_write_allowed(&self) -> bool {
        let securelevel = self
            .query_mib("kern.securelevel")
            .unwrap_or_else(|_| "0".to_string())
            .parse::<i32>()
            .unwrap_or(0);

        // securelevel >= 1 blocks writing directly to raw disk devices
        securelevel < 1
    }
}

impl Default for OpenBsdSysctlKernelMib {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// NetBSD Rump Kernel Hypercall Translation Router
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RumpHypercall {
    Syscall,
    DriverAccess,
    MemoryAlloc,
}

pub struct NetBsdRumpKernelRouter;

impl NetBsdRumpKernelRouter {
    pub fn dispatch_hypercall(call_type: RumpHypercall, param: u64) -> u64 {
        match call_type {
            RumpHypercall::Syscall => param.wrapping_add(1),
            RumpHypercall::DriverAccess => param ^ 0xFF00FF00,
            RumpHypercall::MemoryAlloc => (param + 4095) & !4095,
        }
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freebsd_jail_manager() {
        let mut fjm = FreeBsdJailManager::new();

        // Create jails
        let jid = fjm.create_jail("webserver.local", "192.168.10.15", "/jails/web").unwrap();
        assert_eq!(jid, 1);
        assert!(fjm.jails.get(&1).unwrap().is_running);

        // Check networking segregation
        assert!(fjm.check_network_allowed(1, "192.168.10.15"));
        assert!(!fjm.check_network_allowed(1, "192.168.10.99"));

        // Enable SysV IPC
        assert!(!fjm.jails.get(&1).unwrap().sysv_ipc_enabled);
        assert!(fjm.enable_sysv_ipc(1).is_ok());
        assert!(fjm.jails.get(&1).unwrap().sysv_ipc_enabled);

        // Stop jail
        assert!(fjm.stop_jail(1).is_ok());
        assert!(!fjm.jails.get(&1).unwrap().is_running);
        assert!(!fjm.check_network_allowed(1, "192.168.10.15"));
    }

    #[test]
    fn test_openbsd_sysctl_mib() {
        let mut sysctl = OpenBsdSysctlKernelMib::new();

        // Query default MIBs
        assert_eq!(sysctl.query_mib("kern.ostype").unwrap(), "OpenBSD");
        assert_eq!(sysctl.query_mib("hw.ncpu").unwrap(), "8");

        // Write non-constrained MIB
        assert!(sysctl.write_mib("hw.ncpu", "16").is_ok());
        assert_eq!(sysctl.query_mib("hw.ncpu").unwrap(), "16");

        // Verify write securelevel transitions
        assert_eq!(sysctl.query_mib("kern.securelevel").unwrap(), "0");
        assert!(sysctl.is_raw_disk_write_allowed());

        // Raise securelevel to 1 (lockdown mode)
        assert!(sysctl.write_mib("kern.securelevel", "1").is_ok());
        assert_eq!(sysctl.query_mib("kern.securelevel").unwrap(), "1");
        assert!(!sysctl.is_raw_disk_write_allowed());

        // Attempt to lower securelevel (blocked)
        assert!(sysctl.write_mib("kern.securelevel", "0").is_err());
        assert_eq!(sysctl.query_mib("kern.securelevel").unwrap(), "1");
    }

    #[test]
    fn test_netbsd_rump_router() {
        assert_eq!(NetBsdRumpKernelRouter::dispatch_hypercall(RumpHypercall::Syscall, 100), 101);
        assert_eq!(NetBsdRumpKernelRouter::dispatch_hypercall(RumpHypercall::MemoryAlloc, 5000), 8192);
    }
}
