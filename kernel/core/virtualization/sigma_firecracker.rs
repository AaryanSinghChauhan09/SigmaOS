// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/virtualization/sigma_firecracker.rs — Firecracker MicroVM Integration
//
// Implements Firecracker-inspired microVM sandboxing for SigmaOS.
// Provides per-app microVM isolation with minimal overhead.
// Inspired by: Firecracker (AWS), Qubes OS compartmentalization
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum number of microVMs.
const MAX_MICROVMS: SigmaUsize = 64;
/// MicroVM ID length.
const VM_ID_LEN: SigmaUsize = 64;
/// Kernel path length.
const KERNEL_PATH_LEN: SigmaUsize = 256;
/// Rootfs path length.
const ROOTFS_PATH_LEN: SigmaUsize = 256;
/// Default memory size in MB.
const DEFAULT_MEMORY_MB: SigmaU32 = 512;
/// Default vCPU count.
const DEFAULT_VCPUS: SigmaU32 = 2;

// ── MicroVM State ───────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MicroVMState {
    /// VM not created.
    Stopped = 0,
    /// VM starting.
    Starting = 1,
    /// VM running.
    Running = 2,
    /// VM paused.
    Paused = 3,
    /// VM shutting down.
    ShuttingDown = 4,
    /// VM stopped.
    Halted = 5,
}

// ── MicroVM Configuration ───────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MicroVMConfig {
    pub vm_id: [SigmaU8; VM_ID_LEN],
    pub kernel_path: [SigmaU8; KERNEL_PATH_LEN],
    pub rootfs_path: [SigmaU8; ROOTFS_PATH_LEN],
    pub memory_mb: SigmaU32,
    pub vcpus: SigmaU32,
    pub enable_network: SigmaBool,
    pub enable_vsock: SigmaBool,
    pub enable_firewall: SigmaBool,
    pub _pad: [SigmaU8; 7],
}

impl MicroVMConfig {
    pub const fn new() -> Self {
        Self {
            vm_id: [0u8; VM_ID_LEN],
            kernel_path: [0u8; KERNEL_PATH_LEN],
            rootfs_path: [0u8; ROOTFS_PATH_LEN],
            memory_mb: DEFAULT_MEMORY_MB,
            vcpus: DEFAULT_VCPUS,
            enable_network: true,
            enable_vsock: true,
            enable_firewall: true,
            _pad: [0u8; 7],
        }
    }
}

// ── MicroVM Instance ───────────────────────────────────────────────────────
#[repr(C)]
pub struct MicroVM {
    pub config: MicroVMConfig,
    pub state: MicroVMState,
    pub pid: SigmaU32,
    pub vsock_port: SigmaU32,
    pub created_at: SigmaU64,
    pub active: SigmaBool,
    pub _pad: [SigmaU8; 7],
}

impl MicroVM {
    pub const fn new() -> Self {
        Self {
            config: MicroVMConfig::new(),
            state: MicroVMState::Stopped,
            pid: 0,
            vsock_port: 0,
            created_at: 0,
            active: false,
            _pad: [0u8; 7],
        }
    }
}

// ── MicroVM Manager ─────────────────────────────────────────────────────────
pub struct MicroVMManager {
    vms: [MicroVM; MAX_MICROVMS],
    count: SigmaUsize,
    next_vm_id: SigmaU32,
    next_vsock_port: SigmaU32,
    initialized: SigmaBool,
}

impl MicroVMManager {
    pub const fn new() -> Self {
        Self {
            vms: [MicroVM::new(); MAX_MICROVMS],
            count: 0,
            next_vm_id: 1,
            next_vsock_port: 1024,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    fn copy_str(dst: &mut [SigmaU8], src: &[SigmaU8]) {
        let len = src.len().min(dst.len() - 1);
        let mut i = 0;
        while i < len { dst[i] = src[i]; i += 1; }
        dst[len] = 0;
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Create a new microVM.
    pub fn create_vm(&mut self, config: MicroVMConfig) -> SigmaU32 {
        if self.count >= MAX_MICROVMS {
            return 0;
        }

        let idx = self.count;
        let vm_id = self.next_vm_id;
        self.next_vm_id += 1;

        let mut vm = MicroVM::new();
        vm.config = config;
        vm.state = MicroVMState::Starting;
        vm.vsock_port = self.next_vsock_port;
        self.next_vsock_port += 1;
        vm.created_at = 0; // In production: get timestamp
        vm.active = true;

        // In production: spawn Firecracker process
        // For now, simulate successful creation
        vm.state = MicroVMState::Running;
        vm.pid = vm_id + 1000; // Simulated PID

        self.vms[idx] = vm;
        self.count += 1;
        vm_id
    }

    /// Start a microVM.
    pub fn start_vm(&mut self, vm_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.count {
            if self.vms[i].config.vm_id[0] as SigmaU32 == vm_id || self.vms[i].pid == vm_id {
                if self.vms[i].state == MicroVMState::Stopped {
                    self.vms[i].state = MicroVMState::Starting;
                    // In production: start Firecracker process
                    self.vms[i].state = MicroVMState::Running;
                    return 0;
                }
                return -1; // Already running or invalid state
            }
        }
        -1 // VM not found
    }

    /// Stop a microVM.
    pub fn stop_vm(&mut self, vm_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.count {
            if self.vms[i].config.vm_id[0] as SigmaU32 == vm_id || self.vms[i].pid == vm_id {
                self.vms[i].state = MicroVMState::ShuttingDown;
                // In production: send shutdown signal to Firecracker
                self.vms[i].state = MicroVMState::Halted;
                return 0;
            }
        }
        -1 // VM not found
    }

    /// Pause a microVM.
    pub fn pause_vm(&mut self, vm_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.count {
            if self.vms[i].config.vm_id[0] as SigmaU32 == vm_id || self.vms[i].pid == vm_id {
                if self.vms[i].state == MicroVMState::Running {
                    self.vms[i].state = MicroVMState::Paused;
                    // In production: pause Firecracker
                    return 0;
                }
                return -1;
            }
        }
        -1
    }

    /// Resume a microVM.
    pub fn resume_vm(&mut self, vm_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.count {
            if self.vms[i].config.vm_id[0] as SigmaU32 == vm_id || self.vms[i].pid == vm_id {
                if self.vms[i].state == MicroVMState::Paused {
                    self.vms[i].state = MicroVMState::Running;
                    // In production: resume Firecracker
                    return 0;
                }
                return -1;
            }
        }
        -1
    }

    /// Destroy a microVM.
    pub fn destroy_vm(&mut self, vm_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.count {
            if self.vms[i].config.vm_id[0] as SigmaU32 == vm_id || self.vms[i].pid == vm_id {
                self.vms[i] = MicroVM::new();
                self.count -= 1;
                return 0;
            }
        }
        -1
    }

    /// Get microVM state.
    pub fn get_vm_state(&self, vm_id: SigmaU32) -> MicroVMState {
        for i in 0..self.count {
            if self.vms[i].config.vm_id[0] as SigmaU32 == vm_id || self.vms[i].pid == vm_id {
                return self.vms[i].state;
            }
        }
        MicroVMState::Stopped
    }

    /// List all microVMs.
    pub fn list_vms(&self, out: *mut MicroVM, max: SigmaUsize) -> SigmaUsize {
        let mut written = 0;
        for i in 0..self.count {
            if written >= max { break; }
            unsafe { core::ptr::write(out.add(written), self.vms[i]); }
            written += 1;
        }
        written
    }

    /// Get microVM count.
    pub fn vm_count(&self) -> SigmaUsize {
        self.count
    }

    /// Create disposable browser VM.
    pub fn create_browser_vm(&mut self) -> SigmaU32 {
        let mut config = MicroVMConfig::new();
        Self::copy_str(&mut config.vm_id, b"browser-disposable");
        Self::copy_str(&mut config.kernel_path, b"/boot/vmlinux-browser");
        Self::copy_str(&mut config.rootfs_path, b"/var/lib/microvms/browser-rootfs.ext4");
        config.memory_mb = 1024;
        config.vcpus = 2;
        config.enable_network = true;
        config.enable_vsock = true;
        config.enable_firewall = true;
        self.create_vm(config)
    }

    /// Create sandboxed app VM.
    pub fn create_app_vm(&mut self, app_name: &[SigmaU8]) -> SigmaU32 {
        let mut config = MicroVMConfig::new();
        Self::copy_str(&mut config.vm_id, app_name);
        Self::copy_str(&mut config.kernel_path, b"/boot/vmlinux-app");
        Self::copy_str(&mut config.rootfs_path, b"/var/lib/microvms/app-rootfs.ext4");
        config.memory_mb = 512;
        config.vcpus = 1;
        config.enable_network = false;
        config.enable_vsock = true;
        config.enable_firewall = true;
        self.create_vm(config)
    }
}

static mut G_VM_MGR: MicroVMManager = MicroVMManager::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_init() {
    G_VM_MGR.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_create_vm(
    vm_id: *const SigmaU8,
    vm_id_len: SigmaUsize,
    kernel_path: *const SigmaU8,
    kernel_len: SigmaUsize,
    rootfs_path: *const SigmaU8,
    rootfs_len: SigmaUsize,
    memory_mb: SigmaU32,
    vcpus: SigmaU32,
    enable_network: SigmaU32,
    enable_vsock: SigmaU32,
    enable_firewall: SigmaU32,
) -> SigmaU32 {
    let vid = core::slice::from_raw_parts(vm_id, vm_id_len.min(VM_ID_LEN));
    let kp = core::slice::from_raw_parts(kernel_path, kernel_len.min(KERNEL_PATH_LEN));
    let rp = core::slice::from_raw_parts(rootfs_path, rootfs_len.min(ROOTFS_PATH_LEN));

    let mut config = MicroVMConfig::new();
    MicroVMManager::copy_str(&mut config.vm_id, vid);
    MicroVMManager::copy_str(&mut config.kernel_path, kp);
    MicroVMManager::copy_str(&mut config.rootfs_path, rp);
    config.memory_mb = memory_mb;
    config.vcpus = vcpus;
    config.enable_network = enable_network != 0;
    config.enable_vsock = enable_vsock != 0;
    config.enable_firewall = enable_firewall != 0;

    G_VM_MGR.create_vm(config)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_start_vm(vm_id: SigmaU32) -> SigmaI32 {
    G_VM_MGR.start_vm(vm_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_stop_vm(vm_id: SigmaU32) -> SigmaI32 {
    G_VM_MGR.stop_vm(vm_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_pause_vm(vm_id: SigmaU32) -> SigmaI32 {
    G_VM_MGR.pause_vm(vm_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_resume_vm(vm_id: SigmaU32) -> SigmaI32 {
    G_VM_MGR.resume_vm(vm_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_destroy_vm(vm_id: SigmaU32) -> SigmaI32 {
    G_VM_MGR.destroy_vm(vm_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_get_vm_state(vm_id: SigmaU32) -> SigmaU32 {
    G_VM_MGR.get_vm_state(vm_id) as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_list_vms(
    out: *mut MicroVM,
    max: SigmaU32,
) -> SigmaU32 {
    G_VM_MGR.list_vms(out, max as SigmaUsize) as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_vm_count() -> SigmaU32 {
    G_VM_MGR.vm_count() as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_create_browser_vm() -> SigmaU32 {
    G_VM_MGR.create_browser_vm()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_firecracker_create_app_vm(
    app_name: *const SigmaU8,
    app_len: SigmaUsize,
) -> SigmaU32 {
    let name = core::slice::from_raw_parts(app_name, app_len.min(VM_ID_LEN));
    G_VM_MGR.create_app_vm(name)
}
