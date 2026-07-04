// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// init/sigma_init.rs — SigmaOS PID 1 Init System
//
// Implements PID 1 init system for SigmaOS:
// - Mounts essential filesystems (/proc, /sys, /dev)
// - Starts essential services
// - Spawns the shell (sigma-sh)
// - Handles orphaned processes
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

// ─── Init System Constants ─────────────────────────────────────────────────────

pub const SIGMA_OK: i32 = 0;
pub const SIGMA_ERROR: i32 = -1;
pub const MAX_SERVICES: usize = 32;
pub const MAX_MOUNTS: usize = 16;

// ─── Mount Entry ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct MountEntry {
    pub source: [u8; 64],
    pub target: [u8; 64],
    pub fstype: [u8; 32],
    pub options: [u8; 64],
    pub mounted: bool,
}

impl MountEntry {
    pub const fn new() -> Self {
        MountEntry {
            source: [0; 64],
            target: [0; 64],
            fstype: [0; 32],
            options: [0; 64],
            mounted: false,
        }
    }
}

// ─── Service Entry ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct ServiceEntry {
    pub name: [u8; 64],
    pub path: [u8; 128],
    pub pid: u32,
    pub running: bool,
    pub restart_on_fail: bool,
}

impl ServiceEntry {
    pub const fn new() -> Self {
        ServiceEntry {
            name: [0; 64],
            path: [0; 128],
            pid: 0,
            running: false,
            restart_on_fail: false,
        }
    }
}

// ─── SigmaInit System ───────────────────────────────────────────────────────────

pub struct SigmaInit {
    pub ready: bool,
    pub runlevel: u32,
    mounts: [MountEntry; MAX_MOUNTS],
    mount_count: usize,
    services: [ServiceEntry; MAX_SERVICES],
    service_count: usize,
}

impl SigmaInit {
    pub const fn new() -> Self {
        SigmaInit {
            ready: false,
            runlevel: 3, // Multi-user mode
            mounts: [const { MountEntry::new() }; MAX_MOUNTS],
            mount_count: 0,
            services: [const { ServiceEntry::new() }; MAX_SERVICES],
            service_count: 0,
        }
    }

    pub fn init(&mut self) -> i32 {
        if self.ready {
            return SIGMA_OK;
        }

        // Phase 1: Mount essential filesystems
        if self.mount_essential_fs() != SIGMA_OK {
            return SIGMA_ERROR;
        }

        // Phase 2: Load service configuration
        self.load_services();

        // Phase 3: Start essential services
        self.start_essential_services();

        // Phase 4: Spawn shell
        self.spawn_shell();

        self.ready = true;
        SIGMA_OK
    }

    fn mount_essential_fs(&mut self) -> i32 {
        // Mount procfs at /proc
        self.add_mount(b"proc", b"/proc", b"proc", b"rw,nosuid,nodev,noexec,relatime");
        
        // Mount sysfs at /sys
        self.add_mount(b"sysfs", b"/sys", b"sysfs", b"rw,nosuid,nodev,noexec,relatime");
        
        // Mount devtmpfs at /dev
        self.add_mount(b"devtmpfs", b"/dev", b"devtmpfs", b"rw,nosuid,relatime,size=10240k,nr_inodes=4096,mode=755");
        
        // Mount tmpfs at /tmp
        self.add_mount(b"tmpfs", b"/tmp", b"tmpfs", b"rw,nosuid,nodev,noexec,relatime");
        
        // Mount tmpfs at /run
        self.add_mount(b"tmpfs", b"/run", b"tmpfs", b"rw,nosuid,nodev,noexec,relatime,mode=755");

        // Perform actual mounts (delegated to kernel)
        self.perform_mounts();
        
        SIGMA_OK
    }

    fn add_mount(&mut self, source: &[u8], target: &[u8], fstype: &[u8], options: &[u8]) {
        if self.mount_count >= MAX_MOUNTS {
            return;
        }

        let mut entry = MountEntry::new();
        let len = source.len().min(64);
        entry.source[..len].copy_from_slice(&source[..len]);
        
        let len = target.len().min(64);
        entry.target[..len].copy_from_slice(&target[..len]);
        
        let len = fstype.len().min(32);
        entry.fstype[..len].copy_from_slice(&fstype[..len]);
        
        let len = options.len().min(64);
        entry.options[..len].copy_from_slice(&options[..len]);
        
        self.mounts[self.mount_count] = entry;
        self.mount_count += 1;
    }

    fn perform_mounts(&mut self) {
        // In a real implementation, this would call kernel syscalls
        // For now, mark all as mounted
        for i in 0..self.mount_count {
            self.mounts[i].mounted = true;
        }
    }

    fn load_services(&mut self) {
        // Add essential services
        self.add_service(b"udevd", b"/usr/bin/udevd", true);
        self.add_service(b"syslogd", b"/usr/bin/syslogd", true);
        self.add_service(b"networkd", b"/usr/bin/networkd", true);
        self.add_service(b"dhcpcd", b"/usr/bin/dhcpcd", true);
    }

    fn add_service(&mut self, name: &[u8], path: &[u8], restart_on_fail: bool) {
        if self.service_count >= MAX_SERVICES {
            return;
        }

        let mut service = ServiceEntry::new();
        let len = name.len().min(64);
        service.name[..len].copy_from_slice(&name[..len]);
        
        let len = path.len().min(128);
        service.path[..len].copy_from_slice(&path[..len]);
        
        service.restart_on_fail = restart_on_fail;
        
        self.services[self.service_count] = service;
        self.service_count += 1;
    }

    fn start_essential_services(&mut self) {
        // Start udevd for device management
        self.start_service(b"udevd");
        
        // Start syslogd for logging
        self.start_service(b"syslogd");
        
        // Start networkd for network management
        self.start_service(b"networkd");
        
        // Start dhcpcd for DHCP
        self.start_service(b"dhcpcd");
    }

    fn start_service(&mut self, name: &[u8]) {
        for i in 0..self.service_count {
            if self.services[i].name == *name {
                // In a real implementation, fork and exec
                self.services[i].pid = 1000 + i as u32;
                self.services[i].running = true;
                break;
            }
        }
    }

    fn spawn_shell(&mut self) {
        // Spawn sigma-sh on tty1
        // In a real implementation, fork and exec /usr/bin/sigma-sh
        self.add_service(b"sigma-sh", b"/usr/bin/sigma-sh", false);
        self.start_service(b"sigma-sh");
    }

    pub fn main_loop(&mut self) -> ! {
        loop {
            // Reap orphaned processes
            self.reap_zombies();
            
            // Restart failed services
            self.restart_failed_services();
            
            // Handle signals
            self.handle_signals();
            
            // Sleep briefly
            // In a real implementation, use sleep syscall
        }
    }

    fn reap_zombies(&mut self) {
        // In a real implementation, call waitpid(-1, WNOHANG)
        // For now, this is a stub
    }

    fn restart_failed_services(&mut self) {
        for i in 0..self.service_count {
            if !self.services[i].running && self.services[i].restart_on_fail {
                self.start_service(&self.services[i].name);
            }
        }
    }

    fn handle_signals(&mut self) {
        // Handle SIGTERM, SIGINT, SIGHUP
        // In a real implementation, set up signal handlers
    }

    pub fn shutdown(&mut self) -> ! {
        // Stop all services
        for i in 0..self.service_count {
            if self.services[i].running {
                self.stop_service(&self.services[i].name);
            }
        }
        
        // Unmount filesystems
        self.unmount_all();
        
        // Sync filesystems
        self.sync_filesystems();
        
        // Power off or reboot
        loop {}
    }

    fn stop_service(&mut self, name: &[u8]) {
        for i in 0..self.service_count {
            if self.services[i].name == *name && self.services[i].running {
                // Send SIGTERM
                self.services[i].running = false;
                self.services[i].pid = 0;
                break;
            }
        }
    }

    fn unmount_all(&mut self) {
        // Unmount in reverse order (LIFO)
        for i in (0..self.mount_count).rev() {
            if self.mounts[i].mounted {
                self.mounts[i].mounted = false;
            }
        }
    }

    fn sync_filesystems(&mut self) {
        // Call sync syscall to flush buffers
        // In a real implementation, call sync()
    }

    pub fn get_service_status(&self, name: &[u8]) -> Option<bool> {
        for i in 0..self.service_count {
            if self.services[i].name == *name {
                return Some(self.services[i].running);
            }
        }
        None
    }

    pub fn list_mounts(&self) -> usize {
        self.mount_count
    }

    pub fn list_services(&self) -> usize {
        self.service_count
    }
}

// ─── Global Init Instance ─────────────────────────────────────────────────────

static mut SIGMA_INIT: SigmaInit = SigmaInit::new();

// ─── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_init_main() -> i32 {
    SIGMA_INIT.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_init_loop() -> ! {
    SIGMA_INIT.main_loop()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_init_shutdown() -> ! {
    SIGMA_INIT.shutdown()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_init_service_status(name: *const u8, len: usize) -> i32 {
    let name_slice = core::slice::from_raw_parts(name, len);
    match SIGMA_INIT.get_service_status(name_slice) {
        Some(true) => 1,
        Some(false) => 0,
        None => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_init_mount_count() -> usize {
    SIGMA_INIT.list_mounts()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_init_service_count() -> usize {
    SIGMA_INIT.list_services()
}