// SPDX-License-Identifier: MIT
// Multi-Vendor Hardware Virtualization Subsystem for SigmaOS (`src/virtualization/vendor_hardware.rs`)
// Inspired by Linux KVM (arch/x86/kvm/vmx/, arch/x86/kvm/svm/), FreeBSD bhyve (sys/amd64/vmm/), and OpenBSD vmm(4).
// Supports Intel VT-x (VMCS execution & EPT 2D page tables), AMD-V (SVM VMCB & SEV-SNP encrypted state),
// and NVIDIA vGPU (VFIO-mdev GRID virtual GPU slicing).

use std::collections::HashMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Hardware Virtualization Vendor Category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualizationVendorType {
    IntelVtxEpt,     // Intel VT-x with Extended Page Tables (EPT)
    AmdSvmSevSnp,    // AMD-V SVM with Secure Encrypted Virtualization (SEV-SNP)
    NvidiaVgpuGrid,  // NVIDIA vGPU / VFIO Mediated Device (mdev)
}

/// Intel VT-x VMCS Execution State
#[derive(Debug, Clone)]
pub struct IntelVmcsExecutionState {
    pub vmcs_phys_addr: u64,
    pub pin_based_controls: u32,
    pub cpu_based_controls: u32,
    pub ept_pointer: u64,
    pub guest_rip: u64,
    pub guest_rsp: u64,
    pub is_vmx_active: bool,
}

/// AMD-V SVM VMCB Execution Control Block
#[derive(Debug, Clone)]
pub struct AmdVmcbExecutionBlock {
    pub vmcb_phys_addr: u64,
    pub intercept_read_cr3: bool,
    pub intercept_write_cr3: bool,
    pub npt_pointer: u64,             // Nested Page Table (NPT) pointer
    pub sev_asid: u32,                // SEV-SNP Address Space Identifier
    pub is_sev_snp_encrypted: bool,
}

/// NVIDIA vGPU Mediated Device (VFIO-mdev) Instance
#[derive(Debug, Clone)]
pub struct NvidiaVgpuMediatedInstance {
    pub mdev_uuid: String,
    pub vgpu_type: String,            // e.g. "grid_p100-2q" or "rtx6000-4q"
    pub vram_allocated_mb: u64,
    pub max_display_heads: u32,
    pub is_vfio_bound: bool,
}

/// Multi-Vendor Virtualization Engine
pub struct MultiVendorVirtualizationEngine {
    pub active_intel_vmcs: HashMap<u32, IntelVmcsExecutionState>,  // VMID -> VMCS
    pub active_amd_vmcb: HashMap<u32, AmdVmcbExecutionBlock>,       // VMID -> VMCB
    pub active_nvidia_vgpu: HashMap<String, NvidiaVgpuMediatedInstance>, // UUID -> vGPU
    pub is_hardware_supported: bool,
}

impl MultiVendorVirtualizationEngine {
    pub fn new() -> Self {
        Self {
            active_intel_vmcs: HashMap::new(),
            active_amd_vmcb: HashMap::new(),
            active_nvidia_vgpu: HashMap::new(),
            is_hardware_supported: true,
        }
    }

    /// Provision Intel VT-x VMCS with EPT 2D Page Table Pointer
    pub fn create_intel_vmcs(&mut self, vmid: u32, vmcs_phys: u64, ept_pml4_phys: u64) -> IntelVmcsExecutionState {
        let vmcs = IntelVmcsExecutionState {
            vmcs_phys_addr: vmcs_phys,
            pin_based_controls: 0x0000001F,
            cpu_based_controls: 0x80000000, // Secondary controls enabled for EPT
            ept_pointer: ept_pml4_phys | 0x1E, // WB memory type, 4-level walk
            guest_rip: 0xFFFE_0000,            // Reset vector
            guest_rsp: 0x7FFF_0000,
            is_vmx_active: true,
        };

        self.active_intel_vmcs.insert(vmid, vmcs.clone());
        vmcs
    }

    /// Provision AMD-V SVM VMCB with SEV-SNP Memory Encryption
    pub fn create_amd_vmcb(&mut self, vmid: u32, vmcb_phys: u64, npt_phys: u64, sev_asid: u32) -> AmdVmcbExecutionBlock {
        let vmcb = AmdVmcbExecutionBlock {
            vmcb_phys_addr: vmcb_phys,
            intercept_read_cr3: false,
            intercept_write_cr3: false,
            npt_pointer: npt_phys,
            sev_asid,
            is_sev_snp_encrypted: sev_asid > 0,
        };

        self.active_amd_vmcb.insert(vmid, vmcb.clone());
        vmcb
    }

    /// Provision NVIDIA vGPU Mediated Device (VFIO-mdev GRID slicing)
    pub fn create_nvidia_vgpu_instance(&mut self, uuid: &str, vgpu_type: &str, vram_mb: u64) -> NvidiaVgpuMediatedInstance {
        let vgpu = NvidiaVgpuMediatedInstance {
            mdev_uuid: uuid.to_string(),
            vgpu_type: vgpu_type.to_string(),
            vram_allocated_mb: vram_mb,
            max_display_heads: 4,
            is_vfio_bound: true,
        };

        self.active_nvidia_vgpu.insert(uuid.to_string(), vgpu.clone());
        vgpu
    }
}

impl Default for MultiVendorVirtualizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_vmcs_creation() {
        let mut engine = MultiVendorVirtualizationEngine::new();
        let vmcs = engine.create_intel_vmcs(1, 0x100000, 0x200000);

        assert_eq!(vmcs.vmcs_phys_addr, 0x100000);
        assert!(vmcs.is_vmx_active);
        assert_eq!(engine.active_intel_vmcs.len(), 1);
    }

    #[test]
    fn test_amd_vmcb_sev_snp() {
        let mut engine = MultiVendorVirtualizationEngine::new();
        let vmcb = engine.create_amd_vmcb(2, 0x300000, 0x400000, 5);

        assert_eq!(vmcb.sev_asid, 5);
        assert!(vmcb.is_sev_snp_encrypted);
        assert_eq!(engine.active_amd_vmcb.len(), 1);
    }

    #[test]
    fn test_nvidia_vgpu_slicing() {
        let mut engine = MultiVendorVirtualizationEngine::new();
        let vgpu = engine.create_nvidia_vgpu_instance("uuid-1234-5678", "grid_p100-2q", 2048);

        assert_eq!(vgpu.vram_allocated_mb, 2048);
        assert!(vgpu.is_vfio_bound);
        assert_eq!(engine.active_nvidia_vgpu.len(), 1);
    }
}
