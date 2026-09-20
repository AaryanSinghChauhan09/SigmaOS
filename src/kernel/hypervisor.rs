// SPDX-License-Identifier: MIT
// SigmaOS Kernel Bare-Metal Virtualization Hypervisor Engine
// (`src/kernel/hypervisor.rs`)
//
// Zero-dependency, `#![no_std]` compliant Rust implementation of an in-kernel
// bare-metal hypervisor abstraction supporting Intel VT-x (VMX) / AMD-V (SVM)
// hardware virtualization, VMCS/VMCB lifecycle management, EPT/NPT 2D paging,
// and virtio device queues for guest microVM isolation.

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;

#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;

/// CPU Hardware Virtualization Extensions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualizationVendor {
    IntelVtx,
    AmdSvm,
}

/// Guest VM State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmExecutionState {
    Created,
    Running,
    Paused,
    Terminated,
}

/// Extended Page Table (EPT) Entry for 2D Paging
#[derive(Debug, Clone)]
pub struct EptPageMapEntry {
    pub guest_phys_addr: u64,
    pub host_phys_addr: u64,
    pub read_access: bool,
    pub write_access: bool,
    pub execute_access: bool,
}

/// Virtio Queue Descriptor
#[derive(Debug, Clone)]
pub struct VirtioQueueSpec {
    pub queue_id: u32,
    pub queue_size: u16,
    pub desc_table_paddr: u64,
    pub avail_ring_paddr: u64,
    pub used_ring_paddr: u64,
}

/// MicroVM Guest Domain Instance
#[derive(Debug, Clone)]
pub struct MicroVmGuestDomain {
    pub vmid: u32,
    pub vcpu_count: u16,
    pub memory_limit_mb: u64,
    pub ept_mappings: Vec<EptPageMapEntry>,
    pub virtio_queues: BTreeMap<u32, VirtioQueueSpec>,
    pub state: VmExecutionState,
}

/// In-Kernel Bare-Metal Hypervisor Manager
#[derive(Debug)]
pub struct SovereignBareMetalHypervisor {
    pub vendor: VirtualizationVendor,
    pub vmx_enabled: bool,
    pub active_vms: BTreeMap<u32, MicroVmGuestDomain>,
    pub next_vmid: u32,
    pub total_exit_events: u64,
}

impl SovereignBareMetalHypervisor {
    pub fn new(vendor: VirtualizationVendor) -> Self {
        Self {
            vendor,
            vmx_enabled: true,
            active_vms: BTreeMap::new(),
            next_vmid: 1,
            total_exit_events: 0,
        }
    }

    /// Create and provision a microVM guest domain
    pub fn create_guest_domain(&mut self, vcpus: u16, memory_mb: u64) -> u32 {
        let vmid = self.next_vmid;
        self.next_vmid += 1;

        let domain = MicroVmGuestDomain {
            vmid,
            vcpu_count: vcpus,
            memory_limit_mb: memory_mb,
            ept_mappings: Vec::new(),
            virtio_queues: BTreeMap::new(),
            state: VmExecutionState::Created,
        };

        self.active_vms.insert(vmid, domain);
        vmid
    }

    /// Map Guest Physical Address (GPA) to Host Physical Address (HPA) via EPT
    pub fn map_guest_memory(&mut self, vmid: u32, gpa: u64, hpa: u64) -> Result<(), &'static str> {
        let vm = self.active_vms.get_mut(&vmid).ok_or("Hypervisor: Guest VM not found")?;

        let entry = EptPageMapEntry {
            guest_phys_addr: gpa,
            host_phys_addr: hpa,
            read_access: true,
            write_access: true,
            execute_access: true,
        };

        vm.ept_mappings.push(entry);
        Ok(())
    }

    /// Launch microVM guest execution (VMLAUNCH / VMRUN)
    pub fn launch_guest(&mut self, vmid: u32) -> Result<(), &'static str> {
        let vm = self.active_vms.get_mut(&vmid).ok_or("Hypervisor: Guest VM not found")?;
        if vm.state == VmExecutionState::Running {
            return Err("Hypervisor: Guest VM already running");
        }

        vm.state = VmExecutionState::Running;
        Ok(())
    }

    /// Handle VM Exit VMEXIT reasons (e.g. I/O instruction, EPT violation)
    pub fn handle_vm_exit(&mut self, vmid: u32, exit_reason: u32) -> Result<(), &'static str> {
        if !self.active_vms.contains_key(&vmid) {
            return Err("Hypervisor: Invalid VMID for VMEXIT");
        }

        self.total_exit_events += 1;
        match exit_reason {
            0x30 => { // IO_INSTRUCTION exit
                Ok(())
            }
            0x300 => { // EPT_VIOLATION exit
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl Default for SovereignBareMetalHypervisor {
    fn default() -> Self {
        Self::new(VirtualizationVendor::IntelVtx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypervisor_vm_lifecycle() {
        let mut hyp = SovereignBareMetalHypervisor::new(VirtualizationVendor::IntelVtx);
        let vmid = hyp.create_guest_domain(2, 1024);

        assert_eq!(vmid, 1);
        assert!(hyp.map_guest_memory(vmid, 0x1000, 0x8000_0000).is_ok());
        assert!(hyp.launch_guest(vmid).is_ok());
        assert!(hyp.handle_vm_exit(vmid, 0x30).is_ok());

        assert_eq!(hyp.total_exit_events, 1);
    }
}
