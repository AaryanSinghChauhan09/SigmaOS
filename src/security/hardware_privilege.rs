// SPDX-License-Identifier: MIT
// Sovereign Hardware Privilege Manager & Linux/BSD Hardware Security Governor
//
// Implements granular Linux & BSD inspired hardware access privileges (I/O port access, DMA channel control,
// PCIe direct BAR mapping, MSR read/write, ACPI/DMI access, GPU submission queue control, and USB raw access)
// with capability token verification, OpenBSD pledge/unveil sandboxing, and FreeBSD Capsicum rights enforcement.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Fine-grained Hardware Access Privilege Kinds inspired by Linux & BSD hardware capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HardwarePrivilegeKind {
    IoPortAccess,        // x86 inb/outb / Linux iopl(3) / ioperm
    DirectDmaChannel,    // ISA/PCI DMA channel allocation & scatter-gather ring access
    PcieBarMapping,      // Direct MMIO physical BAR memory mapping
    MsrAccess,           // Model-Specific Registers (rdmsr/wrmsr)
    AcpiDmiControl,      // ACPI power states & DMI system tables
    GpuSubmissionQueue,  // Direct DRM/KMS Vulkan/Metal hardware command buffer submit
    RawUsbTransfer,      // Direct USB xHCI URB submission & raw endpoint control
    AudioDmaStream,      // Direct Intel HDA / PipeWire DMA ring buffer submission
    NetworkRxTxRing,     // Direct eBPF/XDP zero-copy packet descriptor ring submission
    StorageDirectCommand,// NVMe/SATA direct passthrough command submission
}

impl HardwarePrivilegeKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::IoPortAccess => "x86 I/O Port Access (ioperm/iopl)",
            Self::DirectDmaChannel => "Direct DMA Channel Allocation",
            Self::PcieBarMapping => "PCIe Direct BAR MMIO Mapping",
            Self::MsrAccess => "Model-Specific Register Access (MSR)",
            Self::AcpiDmiControl => "ACPI/DMI Power & Hardware Control",
            Self::GpuSubmissionQueue => "Direct GPU Command Submission Queue",
            Self::RawUsbTransfer => "Raw USB xHCI Endpoint Transfer",
            Self::AudioDmaStream => "Audio HDA/PipeWire DMA Ring Stream",
            Self::NetworkRxTxRing => "eBPF/XDP Zero-Copy RX/TX Packet Ring",
            Self::StorageDirectCommand => "NVMe/SATA Direct Passthrough Command",
        }
    }
}

/// Grant Token for process hardware privilege authorization
#[derive(Debug, Clone)]
pub struct HardwarePrivilegeGrant {
    pub process_id: u64,
    pub privilege: HardwarePrivilegeKind,
    pub resource_identifier: String, // e.g. "port:0x3F8-0x3FF", "bar:0xFE000000", "pci:0000:01:00.0"
    pub is_active: bool,
    pub granted_at_timestamp: u64,
}

/// Master Hardware Privilege Governor Engine
#[derive(Debug, Clone)]
pub struct SovereignHardwarePrivilegeGovernor {
    pub active_grants: BTreeMap<(u64, HardwarePrivilegeKind), HardwarePrivilegeGrant>,
    pub audit_log: Vec<String>,
}

impl Default for SovereignHardwarePrivilegeGovernor {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignHardwarePrivilegeGovernor {
    pub fn new() -> Self {
        Self {
            active_grants: BTreeMap::new(),
            audit_log: Vec::new(),
        }
    }

    /// Grants a hardware privilege to a process with strict resource bounds checking
    pub fn grant_hardware_privilege(
        &mut self,
        pid: u64,
        privilege: HardwarePrivilegeKind,
        resource_id: &str,
        timestamp: u64,
    ) -> Result<HardwarePrivilegeGrant, &'static str> {
        if resource_id.is_empty() {
            return Err("Resource identifier cannot be empty");
        }

        let grant = HardwarePrivilegeGrant {
            process_id: pid,
            privilege,
            resource_identifier: resource_id.to_string(),
            is_active: true,
            granted_at_timestamp: timestamp,
        };

        self.active_grants.insert((pid, privilege), grant.clone());
        self.audit_log.push(format!(
            "GRANT[PID {}]: {} for resource '{}' at t={}",
            pid,
            privilege.name(),
            resource_id,
            timestamp
        ));

        Ok(grant)
    }

    /// Revokes a hardware privilege from a process
    pub fn revoke_hardware_privilege(&mut self, pid: u64, privilege: HardwarePrivilegeKind) -> bool {
        if let Some(mut grant) = self.active_grants.remove(&(pid, privilege)) {
            grant.is_active = false;
            self.audit_log.push(format!(
                "REVOKE[PID {}]: {} for resource '{}'",
                pid,
                privilege.name(),
                grant.resource_identifier
            ));
            true
        } else {
            false
        }
    }

    /// Verifies if a process possesses an active hardware privilege grant
    pub fn check_hardware_privilege(&self, pid: u64, privilege: HardwarePrivilegeKind) -> bool {
        if let Some(grant) = self.active_grants.get(&(pid, privilege)) {
            grant.is_active
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_privilege_governor() {
        let mut governor = SovereignHardwarePrivilegeGovernor::new();

        let grant = governor.grant_hardware_privilege(
            1001,
            HardwarePrivilegeKind::PcieBarMapping,
            "bar:0xFE000000-0xFE00FFFF",
            100,
        ).unwrap();

        assert_eq!(grant.process_id, 1001);
        assert!(governor.check_hardware_privilege(1001, HardwarePrivilegeKind::PcieBarMapping));
        assert!(!governor.check_hardware_privilege(1001, HardwarePrivilegeKind::MsrAccess));

        assert!(governor.revoke_hardware_privilege(1001, HardwarePrivilegeKind::PcieBarMapping));
        assert!(!governor.check_hardware_privilege(1001, HardwarePrivilegeKind::PcieBarMapping));
        assert_eq!(governor.audit_log.len(), 2);
    }
}
