// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/virtualization/SovereignHypervisor.rs — Type-1 Hypervisor (VMX)
//
// Implements an Intel VT-x (VMX) based micro-hypervisor for the SigmaOS
// Sovereign virtualization layer.
//
// Inspired by:
//   - Linux KVM (arch/x86/kvm/vmx.c)
//   - Firecracker's minimal VMM design
//   - xhyve / bhyve VMX initialization sequences
//
// Supports:
//   - CPUID VMX capability detection
//   - VMXON / VMXOFF lifecycle
//   - VMCS allocation and initialization
//   - Guest state setup (CR0/CR3/CR4, segment registers, RIP/RSP)
//   - VMLAUNCH / VMRESUME
//   - VM-exit reason decoding
//
// Language: Rust #![no_std] — no alloc, no external crates.
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── VMX MSR Indices (Intel SDM Vol 3C §A) ────────────────────────────────────
const MSR_IA32_VMX_BASIC:        u32 = 0x480;
const MSR_IA32_FEATURE_CONTROL:  u32 = 0x03A;
const MSR_IA32_VMX_CR0_FIXED0:   u32 = 0x486;
const MSR_IA32_VMX_CR0_FIXED1:   u32 = 0x487;
const MSR_IA32_VMX_CR4_FIXED0:   u32 = 0x488;
const MSR_IA32_VMX_CR4_FIXED1:   u32 = 0x489;

// IA32_FEATURE_CONTROL bits
const FEATURE_CTRL_LOCK:     u64 = 1 << 0;
const FEATURE_CTRL_VMXON:    u64 = 1 << 2;

// CPUID leaf for VMX support
const CPUID_VMX_FEATURE: u32 = 1 << 5; // ECX bit 5

// ── VMCS Encoding Fields (Intel SDM Vol 3C §B) ───────────────────────────────
// Guest state area
const VMCS_GUEST_ES_SEL:        u32 = 0x0800;
const VMCS_GUEST_CS_SEL:        u32 = 0x0802;
const VMCS_GUEST_SS_SEL:        u32 = 0x0804;
const VMCS_GUEST_DS_SEL:        u32 = 0x0806;
const VMCS_GUEST_FS_SEL:        u32 = 0x0808;
const VMCS_GUEST_GS_SEL:        u32 = 0x080A;
const VMCS_GUEST_TR_SEL:        u32 = 0x080E;
const VMCS_GUEST_CS_LIMIT:      u32 = 0x4802;
const VMCS_GUEST_SS_LIMIT:      u32 = 0x4804;
const VMCS_GUEST_DS_LIMIT:      u32 = 0x4806;
const VMCS_GUEST_TR_LIMIT:      u32 = 0x480E;
const VMCS_GUEST_GDTR_LIMIT:    u32 = 0x4810;
const VMCS_GUEST_IDTR_LIMIT:    u32 = 0x4812;
const VMCS_GUEST_CS_ACCESS:     u32 = 0x4816;
const VMCS_GUEST_DS_ACCESS:     u32 = 0x4818;
const VMCS_GUEST_SS_ACCESS:     u32 = 0x481A;
const VMCS_GUEST_TR_ACCESS:     u32 = 0x481E;
const VMCS_GUEST_CR0:           u32 = 0x6800;
const VMCS_GUEST_CR3:           u32 = 0x6802;
const VMCS_GUEST_CR4:           u32 = 0x6804;
const VMCS_GUEST_GDTR_BASE:     u32 = 0x6816;
const VMCS_GUEST_IDTR_BASE:     u32 = 0x6818;
const VMCS_GUEST_RIP:           u32 = 0x681E;
const VMCS_GUEST_RSP:           u32 = 0x6820;
const VMCS_GUEST_RFLAGS:        u32 = 0x6822;
const VMCS_GUEST_ACTIVITY:      u32 = 0x4826; // 0=active

// Host state area
const VMCS_HOST_CR0:            u32 = 0x6C00;
const VMCS_HOST_CR3:            u32 = 0x6C02;
const VMCS_HOST_CR4:            u32 = 0x6C04;
const VMCS_HOST_RSP:            u32 = 0x6C14;
const VMCS_HOST_RIP:            u32 = 0x6C16;
const VMCS_HOST_CS_SEL:         u32 = 0x0C02;
const VMCS_HOST_TR_SEL:         u32 = 0x0C0C;

// VM execution controls
const VMCS_PIN_EXEC_CTRL:       u32 = 0x4000;
const VMCS_PRI_PROC_EXEC_CTRL:  u32 = 0x4002;
const VMCS_EXCEPTION_BITMAP:    u32 = 0x4004;
const VMCS_EXIT_CTRL:           u32 = 0x400C;
const VMCS_ENTRY_CTRL:          u32 = 0x4012;

// VM-exit reason register
const VMCS_EXIT_REASON:         u32 = 0x4402;
const VMCS_EXIT_QUAL:           u32 = 0x6400;

// ── VM-exit reason codes (partial list) ──────────────────────────────────────
const EXIT_REASON_CPUID:   u32 = 10;
const EXIT_REASON_HLT:     u32 = 12;
const EXIT_REASON_VMCALL:  u32 = 18;
const EXIT_REASON_IO:      u32 = 30;
const EXIT_REASON_EPT:     u32 = 48;

// ── VMCS Region ───────────────────────────────────────────────────────────────
/// A VMCS region must be 4 KiB aligned and <= 4 KiB size.
/// We use repr(C, align(4096)) to satisfy VMX requirements.
#[repr(C, align(4096))]
pub struct VmcsRegion {
    /// VMCS revision identifier (from MSR_IA32_VMX_BASIC[31:0]).
    pub revision_id:      SigmaU32,
    /// VMX-abort indicator (written by CPU on abort).
    pub abort_indicator:  SigmaU32,
    /// VMCS data (hardware-managed).
    pub data:             [SigmaU8; 4088],
}

impl VmcsRegion {
    pub const fn zeroed() -> Self {
        Self { revision_id: 0, abort_indicator: 0, data: [0u8; 4088] }
    }
}

// ── GuestConfig — configuration for a guest VM ────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GuestConfig {
    /// Guest physical address where execution starts (entry point).
    pub entry_rip:  SigmaU64,
    /// Guest initial RSP value.
    pub entry_rsp:  SigmaU64,
    /// Guest CR3 (page table root physical address).
    pub cr3:        SigmaU64,
    /// Guest memory base (physical).
    pub mem_base:   SigmaU64,
    /// Guest memory size in bytes.
    pub mem_size:   SigmaU64,
    /// vCPU identifier.
    pub vcpu_id:    SigmaU32,
    pub _pad:       [SigmaU8; 4],
}

impl GuestConfig {
    pub const fn default_real_mode() -> Self {
        Self {
            entry_rip: 0x7C00,     // bootloader convention
            entry_rsp: 0x7C00,
            cr3:       0,
            mem_base:  0,
            mem_size:  16 * 1024 * 1024, // 16 MiB guest RAM
            vcpu_id:   0,
            _pad:      [0u8; 4],
        }
    }
}

// ── Static VMX regions ────────────────────────────────────────────────────────
static mut VMXON_REGION: VmcsRegion = VmcsRegion::zeroed();
static mut VMCS_REGION:  VmcsRegion = VmcsRegion::zeroed();
/// Host stack for VM-exit handler (4 KiB).
static mut HOST_STACK: [SigmaU8; 4096] = [0u8; 4096];

// ── Driver State ──────────────────────────────────────────────────────────────
pub struct SovereignHypervisor {
    pub initialized:  SigmaBool,
    pub vmx_enabled:  SigmaBool,
    pub guest_active: SigmaBool,
    pub vmx_revision: SigmaU32,
    pub guest_cfg:    GuestConfig,
    pub exit_count:   SigmaU32,
}

impl SovereignHypervisor {
    pub const fn new() -> Self {
        Self {
            initialized:  false,
            vmx_enabled:  false,
            guest_active: false,
            vmx_revision: 0,
            guest_cfg:    GuestConfig::default_real_mode(),
            exit_count:   0,
        }
    }

    // ── x86 helper intrinsics ─────────────────────────────────────────────────

    #[inline(always)]
    unsafe fn rdmsr(msr: u32) -> u64 {
        let lo: u32;
        let hi: u32;
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack)
        );
        (hi as u64) << 32 | lo as u64
    }

    #[inline(always)]
    unsafe fn wrmsr(msr: u32, val: u64) {
        core::arch::asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") val as u32,
            in("edx") (val >> 32) as u32,
            options(nomem, nostack)
        );
    }

    #[inline(always)]
    unsafe fn read_cr0() -> u64 {
        let v: u64;
        core::arch::asm!("mov {}, cr0", out(reg) v, options(nomem, nostack));
        v
    }

    #[inline(always)]
    unsafe fn write_cr0(v: u64) {
        core::arch::asm!("mov cr0, {}", in(reg) v, options(nomem, nostack));
    }

    #[inline(always)]
    unsafe fn read_cr4() -> u64 {
        let v: u64;
        core::arch::asm!("mov {}, cr4", out(reg) v, options(nomem, nostack));
        v
    }

    #[inline(always)]
    unsafe fn write_cr4(v: u64) {
        core::arch::asm!("mov cr4, {}", in(reg) v, options(nomem, nostack));
    }

    // ── VMCS access via VMWRITE / VMREAD ─────────────────────────────────────

    #[inline(always)]
    unsafe fn vmwrite(field: u32, val: u64) -> bool {
        let ret: u64;
        core::arch::asm!(
            "vmwrite {1}, {0}",
            "setna {2}",
            in(reg) val,
            in(reg) field as u64,
            out(reg_byte) ret,
            options(nomem, nostack)
        );
        ret == 0
    }

    #[inline(always)]
    unsafe fn vmread(field: u32) -> u64 {
        let val: u64;
        core::arch::asm!(
            "vmread {0}, {1}",
            out(reg) val,
            in(reg) field as u64,
            options(nomem, nostack)
        );
        val
    }

    // ── CPUID VMX check ───────────────────────────────────────────────────────

    unsafe fn check_vmx_support() -> bool {
        let ecx: u32;
        core::arch::asm!(
            "cpuid",
            inout("eax") 1u32 => _,
            out("ebx") _,
            out("ecx") ecx,
            out("edx") _,
            options(nomem, nostack)
        );
        ecx & CPUID_VMX_FEATURE != 0
    }

    // ── Core VMX lifecycle ────────────────────────────────────────────────────

    pub unsafe fn init(&mut self) {
        if self.initialized { return; }

        // 1. Check CPU supports VMX.
        if !Self::check_vmx_support() {
            // VMX not available — fall back to software emulation mode.
            self.initialized  = true;
            self.vmx_enabled  = false;
            return;
        }

        // 2. Read VMX basic MSR — get revision ID for VMCS.
        let vmx_basic    = Self::rdmsr(MSR_IA32_VMX_BASIC);
        self.vmx_revision = (vmx_basic & 0x7FFF_FFFF) as SigmaU32;

        // 3. Enable VMX in CR4.VMXE (bit 13).
        let cr4 = Self::read_cr4();
        Self::write_cr4(cr4 | (1 << 13));

        // 4. Set required CR0 bits from VMX fixed MSRs.
        let cr0_fixed0 = Self::rdmsr(MSR_IA32_VMX_CR0_FIXED0);
        let cr0_fixed1 = Self::rdmsr(MSR_IA32_VMX_CR0_FIXED1);
        let cr0 = (Self::read_cr0() | cr0_fixed0) & cr0_fixed1;
        Self::write_cr0(cr0);

        // 5. Enable VMX in IA32_FEATURE_CONTROL MSR (if not locked).
        let fc = Self::rdmsr(MSR_IA32_FEATURE_CONTROL);
        if fc & FEATURE_CTRL_LOCK == 0 {
            Self::wrmsr(MSR_IA32_FEATURE_CONTROL,
                fc | FEATURE_CTRL_LOCK | FEATURE_CTRL_VMXON);
        }

        // 6. Initialize VMXON region with revision ID.
        VMXON_REGION.revision_id = self.vmx_revision;

        // 7. VMXON — enter VMX root operation.
        let vmxon_phys = &VMXON_REGION as *const VmcsRegion as u64;
        let mut vmxon_success: u8 = 0;
        core::arch::asm!(
            "vmxon ({0})",
            "setna {1}",
            in(reg) &vmxon_phys as *const u64 as u64,
            out(reg_byte) vmxon_success,
            options(nostack)
        );

        self.vmx_enabled = vmxon_success == 0;
        self.initialized = true;
    }

    pub unsafe fn hypervisor_init(&mut self) { self.init(); }

    /// Setup and launch a guest virtual machine.
    pub unsafe fn boot_guest_vm(&mut self) {
        if !self.vmx_enabled { return; }

        // 1. Initialize VMCS region.
        VMCS_REGION.revision_id = self.vmx_revision;

        let vmcs_phys = &VMCS_REGION as *const VmcsRegion as u64;

        // 2. VMCLEAR — put VMCS in clear state.
        core::arch::asm!(
            "vmclear ({0})",
            in(reg) &vmcs_phys as *const u64 as u64,
            options(nostack)
        );

        // 3. VMPTRLD — make this VMCS active.
        core::arch::asm!(
            "vmptrld ({0})",
            in(reg) &vmcs_phys as *const u64 as u64,
            options(nostack)
        );

        // 4. Program guest state area.
        let g = &self.guest_cfg;
        Self::vmwrite(VMCS_GUEST_CR0,    0x20); // PE=0, PG=0 (real-mode emulation)
        Self::vmwrite(VMCS_GUEST_CR3,    g.cr3);
        Self::vmwrite(VMCS_GUEST_CR4,    0x2000); // VMXE
        Self::vmwrite(VMCS_GUEST_RIP,    g.entry_rip);
        Self::vmwrite(VMCS_GUEST_RSP,    g.entry_rsp);
        Self::vmwrite(VMCS_GUEST_RFLAGS, 0x0002); // Reserved bit 1 always set
        Self::vmwrite(VMCS_GUEST_CS_SEL, 0x0000);
        Self::vmwrite(VMCS_GUEST_DS_SEL, 0x0000);
        Self::vmwrite(VMCS_GUEST_SS_SEL, 0x0000);
        Self::vmwrite(VMCS_GUEST_TR_SEL, 0x0000);
        Self::vmwrite(VMCS_GUEST_ACTIVITY, 0); // active
        Self::vmwrite(VMCS_GUEST_GDTR_BASE,  0);
        Self::vmwrite(VMCS_GUEST_GDTR_LIMIT, 0xFFFF);
        Self::vmwrite(VMCS_GUEST_IDTR_BASE,  0);
        Self::vmwrite(VMCS_GUEST_IDTR_LIMIT, 0xFFFF);

        // 5. Program host state area (where we return on VM-exit).
        let host_rsp = HOST_STACK.as_ptr() as u64 + HOST_STACK.len() as u64;
        Self::vmwrite(VMCS_HOST_CR0,    Self::read_cr0());
        Self::vmwrite(VMCS_HOST_CR3,    0); // Filled by actual paging setup
        Self::vmwrite(VMCS_HOST_CR4,    Self::read_cr4());
        Self::vmwrite(VMCS_HOST_RSP,    host_rsp);
        Self::vmwrite(VMCS_HOST_RIP,    vm_exit_handler as u64);
        Self::vmwrite(VMCS_HOST_CS_SEL, 0x08);
        Self::vmwrite(VMCS_HOST_TR_SEL, 0x18);

        // 6. Execution controls — intercept HLT, I/O.
        Self::vmwrite(VMCS_PIN_EXEC_CTRL,      0x00000016); // defaults
        Self::vmwrite(VMCS_PRI_PROC_EXEC_CTRL, 0x0401E172); // HLT + IO exits
        Self::vmwrite(VMCS_EXIT_CTRL,          0x00036DFF); // 64-bit host
        Self::vmwrite(VMCS_ENTRY_CTRL,         0x000011FF);
        Self::vmwrite(VMCS_EXCEPTION_BITMAP,   0x00000000);

        // 7. VMLAUNCH — transfer control to guest.
        self.guest_active = true;
        let launch_ret: u64;
        core::arch::asm!(
            "vmlaunch",
            "xor {0}, {0}", // We only reach here on failure
            out(reg) launch_ret,
            options(nostack)
        );
        // If we reach here, VMLAUNCH failed.
        self.guest_active = false;
    }

    pub unsafe fn hypervisor_boot_guest(&mut self) { self.boot_guest_vm(); }

    /// Handle a VM-exit event (called from host RIP after exit).
    pub unsafe fn handle_exit(&mut self) {
        self.exit_count += 1;
        let reason = Self::vmread(VMCS_EXIT_REASON) as u32 & 0xFFFF;
        match reason {
            EXIT_REASON_HLT    => { self.guest_active = false; }
            EXIT_REASON_CPUID  => { /* Emulate CPUID — advanced: ignored here */ }
            EXIT_REASON_VMCALL => { /* Hypercall handler */ }
            EXIT_REASON_IO     => { /* Port I/O emulation */ }
            _ => {}
        }
        // VMRESUME to continue guest if still active.
        if self.guest_active {
            core::arch::asm!("vmresume", options(nostack));
        }
    }
}

// ── VM-exit trampoline (host RIP target) ──────────────────────────────────────
/// This function is the host RIP programmed into the VMCS.
/// On VM-exit the CPU transfers here from guest context.
unsafe extern "C" fn vm_exit_handler() {
    INSTANCE.handle_exit();
}

// ── Global Driver Instance ────────────────────────────────────────────────────
static mut INSTANCE: SovereignHypervisor = SovereignHypervisor::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn hypervisor_init() { INSTANCE.init(); }

#[no_mangle]
pub unsafe extern "C" fn init() { INSTANCE.init(); }

#[no_mangle]
pub unsafe extern "C" fn hypervisor_boot_guest() { INSTANCE.boot_guest_vm(); }

#[no_mangle]
pub unsafe extern "C" fn boot_guest_vm() { INSTANCE.boot_guest_vm(); }

/// Returns 1 if VMX hardware is available and enabled.
#[no_mangle]
pub unsafe extern "C" fn hypervisor_vmx_available() -> SigmaU32 {
    if INSTANCE.vmx_enabled { 1 } else { 0 }
}

/// Returns 1 if a guest is currently running.
#[no_mangle]
pub unsafe extern "C" fn hypervisor_guest_active() -> SigmaU32 {
    if INSTANCE.guest_active { 1 } else { 0 }
}

/// Returns total VM-exit event count.
#[no_mangle]
pub unsafe extern "C" fn hypervisor_exit_count() -> SigmaU32 {
    INSTANCE.exit_count
}

/// Configure the next guest to boot.
#[no_mangle]
pub unsafe extern "C" fn hypervisor_set_guest(cfg: *const GuestConfig) {
    if !cfg.is_null() {
        INSTANCE.guest_cfg = core::ptr::read(cfg);
    }
}
