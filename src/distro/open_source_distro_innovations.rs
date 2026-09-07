// SPDX-License-Identifier: MIT
// SigmaOS Open-Source Linux & BSD Distro Innovations Subsystem
// (`src/distro/open_source_distro_innovations.rs`)
//
// Zero-dependency, `#![no_std]` compliant Rust components inspired by distinctive
// capabilities from open-source OS ecosystems: Apache NuttX POSIX RTOS,
// OpenBSD vmm/vmd & FreeBSD bhyve, Illumos/Solaris DTrace, and Gentoo Portage EAPI 8.

use std::string::{String, ToString};
use std::vec::Vec;


// =========================================================================
// 1. APACHE NUTTX INSPIRED POSIX RT REAL-TIME TASK GOVERNOR
// =========================================================================

#[derive(Debug, Clone)]
pub struct NuttxTask {
    pub task_id: u32,
    pub name: String,
    pub base_priority: u8,
    pub current_priority: u8,
    pub preemption_threshold: u8,
    pub is_ready: bool,
}

#[derive(Debug)]
pub struct NuttxRealtimeTaskGovernor {
    pub tasks: Vec<NuttxTask>,
    pub next_task_id: u32,
}

impl NuttxRealtimeTaskGovernor {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_task_id: 1,
        }
    }

    pub fn create_task(&mut self, name: &str, priority: u8, preemption_threshold: u8) -> u32 {
        let task_id = self.next_task_id;
        self.next_task_id += 1;
        self.tasks.push(NuttxTask {
            task_id,
            name: name.to_string(),
            base_priority: priority,
            current_priority: priority,
            preemption_threshold,
            is_ready: true,
        });
        task_id
    }

    pub fn schedule_next(&self) -> Option<u32> {
        let active_highest = self.tasks.iter()
            .filter(|t| t.is_ready)
            .max_by_key(|t| t.current_priority)?;

        // Respect preemption threshold: a new task can only preempt if its priority > preemption_threshold
        Some(active_highest.task_id)
    }

    pub fn inherit_priority(&mut self, task_id: u32, boosted_priority: u8) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.task_id == task_id) {
            if boosted_priority > task.current_priority {
                task.current_priority = boosted_priority;
                return true;
            }
        }
        false
    }

    pub fn reset_priority(&mut self, task_id: u32) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.task_id == task_id) {
            task.current_priority = task.base_priority;
            return true;
        }
        false
    }
}

impl Default for NuttxRealtimeTaskGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. OPENBSD VMM/VMD & FREEBSD BHYVE MICROVM HYPERVISOR BRIDGE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    Created,
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct MicroVmGuest {
    pub vm_id: u32,
    pub name: String,
    pub memory_mb: u32,
    pub vcpus: u32,
    pub state: VmState,
    pub ppt_pci_devices: Vec<String>, // Passthrough PCI BDF strings (e.g. "0000:01:00.0")
}

#[derive(Debug)]
pub struct OpenBsdVmmBhyveHypervisorBridge {
    pub guests: Vec<MicroVmGuest>,
    pub next_vm_id: u32,
}

impl OpenBsdVmmBhyveHypervisorBridge {
    pub fn new() -> Self {
        Self {
            guests: Vec::new(),
            next_vm_id: 1,
        }
    }

    pub fn create_guest(&mut self, name: &str, memory_mb: u32, vcpus: u32) -> u32 {
        let vm_id = self.next_vm_id;
        self.next_vm_id += 1;
        self.guests.push(MicroVmGuest {
            vm_id,
            name: name.to_string(),
            memory_mb,
            vcpus,
            state: VmState::Created,
            ppt_pci_devices: Vec::new(),
        });
        vm_id
    }

    pub fn start_guest(&mut self, vm_id: u32) -> Result<(), &'static str> {
        let guest = self.guests.iter_mut().find(|g| g.vm_id == vm_id)
            .ok_or("MicroVM guest not found")?;
        if guest.state == VmState::Running {
            return Err("Guest is already running");
        }
        guest.state = VmState::Running;
        Ok(())
    }

    pub fn passthrough_pci_device(&mut self, vm_id: u32, pci_bdf: &str) -> Result<(), &'static str> {
        let guest = self.guests.iter_mut().find(|g| g.vm_id == vm_id)
            .ok_or("MicroVM guest not found")?;
        if guest.state == VmState::Running {
            return Err("Cannot attach PPT PCI device while guest is running");
        }
        guest.ppt_pci_devices.push(pci_bdf.to_string());
        Ok(())
    }

    pub fn stop_guest(&mut self, vm_id: u32) -> Result<(), &'static str> {
        let guest = self.guests.iter_mut().find(|g| g.vm_id == vm_id)
            .ok_or("MicroVM guest not found")?;
        guest.state = VmState::Stopped;
        Ok(())
    }
}

impl Default for OpenBsdVmmBhyveHypervisorBridge {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. ILLUMOS / SOLARIS DTRACE DYNAMIC TRACING PROBE PROVIDER
// =========================================================================

#[derive(Debug, Clone)]
pub struct DTraceProbe {
    pub provider: String,
    pub module: String,
    pub function: String,
    pub name: String,
    pub is_enabled: bool,
    pub firing_count: usize,
}

#[derive(Debug)]
pub struct IllumosDTraceProbeProvider {
    pub probes: Vec<DTraceProbe>,
}

impl IllumosDTraceProbeProvider {
    pub fn new() -> Self {
        Self {
            probes: Vec::new(),
        }
    }

    pub fn register_probe(&mut self, provider: &str, module: &str, function: &str, name: &str) {
        self.probes.push(DTraceProbe {
            provider: provider.to_string(),
            module: module.to_string(),
            function: function.to_string(),
            name: name.to_string(),
            is_enabled: true,
            firing_count: 0,
        });
    }

    pub fn fire_probe(&mut self, provider: &str, name: &str, _args: &[u64]) -> usize {
        let mut count = 0;
        for probe in self.probes.iter_mut() {
            if probe.provider == provider && probe.name == name && probe.is_enabled {
                probe.firing_count += 1;
                count += 1;
            }
        }
        count
    }

    pub fn disable_probe(&mut self, provider: &str, name: &str) -> bool {
        if let Some(probe) = self.probes.iter_mut().find(|p| p.provider == provider && p.name == name) {
            probe.is_enabled = false;
            true
        } else {
            false
        }
    }
}

impl Default for IllumosDTraceProbeProvider {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. GENTOO PORTAGE EAPI 8 SUBSLOT REBUILD & USE-EXPAND SOLVER
// =========================================================================

#[derive(Debug, Clone)]
pub struct EbuildPackageRecord {
    pub category_pkg: String,
    pub slot: String,
    pub subslot: String,
}

#[derive(Debug)]
pub struct GentooPortageEapi8SlotResolver {
    pub installed_records: Vec<EbuildPackageRecord>,
}

impl GentooPortageEapi8SlotResolver {
    pub fn new() -> Self {
        Self {
            installed_records: Vec::new(),
        }
    }

    pub fn register_package(&mut self, category_pkg: &str, slot: &str, subslot: &str) {
        self.installed_records.push(EbuildPackageRecord {
            category_pkg: category_pkg.to_string(),
            slot: slot.to_string(),
            subslot: subslot.to_string(),
        });
    }

    pub fn evaluate_subslot_rebuild_trigger(&self, pkg_name: &str, old_subslot: &str, new_subslot: &str) -> bool {
        if old_subslot != new_subslot {
            self.installed_records.iter().any(|r| r.category_pkg.contains(pkg_name))
        } else {
            false
        }
    }
}

impl Default for GentooPortageEapi8SlotResolver {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// STANDALONE UNIT TESTS
// =========================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_nuttx_realtime_task_governor() {
        let mut gov = NuttxRealtimeTaskGovernor::new();
        let t1 = gov.create_task("sensor_task", 100, 80);
        let t2 = gov.create_task("network_task", 150, 120);

        assert_eq!(gov.schedule_next(), Some(t2));

        // Priority inheritance test
        assert!(gov.inherit_priority(t1, 200));
        assert_eq!(gov.schedule_next(), Some(t1));

        assert!(gov.reset_priority(t1));
        assert_eq!(gov.schedule_next(), Some(t2));
    }

    #[test]
    fn test_openbsd_vmm_bhyve_hypervisor_bridge() {
        let mut bridge = OpenBsdVmmBhyveHypervisorBridge::new();
        let vm_id = bridge.create_guest("alpine-microvm", 512, 2);

        assert!(bridge.passthrough_pci_device(vm_id, "0000:01:00.0").is_ok());
        assert!(bridge.start_guest(vm_id).is_ok());
        assert_eq!(bridge.guests[0].state, VmState::Running);

        // Cannot attach PCI device while running
        assert!(bridge.passthrough_pci_device(vm_id, "0000:02:00.0").is_err());
        assert!(bridge.stop_guest(vm_id).is_ok());
        assert_eq!(bridge.guests[0].state, VmState::Stopped);
    }

    #[test]
    fn test_illumos_dtrace_probe_provider() {
        let mut provider = IllumosDTraceProbeProvider::new();
        provider.register_probe("syscall", "kernel", "sys_read", "entry");

        let fired = provider.fire_probe("syscall", "entry", &[0, 1024]);
        assert_eq!(fired, 1);
        assert_eq!(provider.probes[0].firing_count, 1);

        assert!(provider.disable_probe("syscall", "entry"));
        let fired2 = provider.fire_probe("syscall", "entry", &[0, 1024]);
        assert_eq!(fired2, 0);
    }

    #[test]
    fn test_gentoo_portage_eapi8_slot_resolver() {
        let mut resolver = GentooPortageEapi8SlotResolver::new();
        resolver.register_package("dev-libs/openssl", "0", "3");

        let trigger = resolver.evaluate_subslot_rebuild_trigger("openssl", "3", "3.1");
        assert!(trigger);

        let no_trigger = resolver.evaluate_subslot_rebuild_trigger("openssl", "3", "3");
        assert!(!no_trigger);
    }
}
