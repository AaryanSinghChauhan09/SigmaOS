// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SigmaDistroStreamer (Rust, no_std)
//! Simulates virtual syscall translation and container orchestration from legacy Linux distros.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

pub const MAX_DISTROS: usize = 8;
pub const PATH_MAX: usize = 128;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LinuxDistroType {
    Ubuntu,
    Fedora,
    Arch,
    Debian,
}

#[derive(Copy, Clone)]
pub struct DistroStreamConfig {
    pub distro_type: LinuxDistroType,
    pub rootfs_path: [u8; PATH_MAX],
    pub rootfs_len: usize,
    pub emulate_kernel_version: [u8; 16],
    pub emulation_len: usize,
}

pub struct LinuxSyscallMapping {
    pub linux_nr: u32,
    pub sigma_nr: u32,
    pub name: &'static str,
}

// Map key common Linux syscall numbers to SigmaOS capability calls
pub static LINUX_SYSCALL_TABLE: [LinuxSyscallMapping; 12] = [
    LinuxSyscallMapping { linux_nr: 0, sigma_nr: 10, name: "sys_read" },
    LinuxSyscallMapping { linux_nr: 1, sigma_nr: 11, name: "sys_write" },
    LinuxSyscallMapping { linux_nr: 2, sigma_nr: 12, name: "sys_open" },
    LinuxSyscallMapping { linux_nr: 3, sigma_nr: 13, name: "sys_close" },
    LinuxSyscallMapping { linux_nr: 41, sigma_nr: 35, name: "sys_socket" },
    LinuxSyscallMapping { linux_nr: 42, sigma_nr: 36, name: "sys_connect" },
    LinuxSyscallMapping { linux_nr: 43, sigma_nr: 37, name: "sys_accept" },
    LinuxSyscallMapping { linux_nr: 44, sigma_nr: 38, name: "sys_sendto" },
    LinuxSyscallMapping { linux_nr: 56, sigma_nr: 25, name: "sys_clone" },
    LinuxSyscallMapping { linux_nr: 57, sigma_nr: 26, name: "sys_fork" },
    LinuxSyscallMapping { linux_nr: 59, sigma_nr: 28, name: "sys_execve" },
    LinuxSyscallMapping { linux_nr: 60, sigma_nr: 30, name: "sys_exit" },
];

pub struct SigmaDistroStreamer {
    active: bool,
    configs: [Option<DistroStreamConfig>; MAX_DISTROS],
}

impl SigmaDistroStreamer {
    pub const fn new() -> Self {
        SigmaDistroStreamer {
            active: false,
            configs: [None; MAX_DISTROS],
        }
    }

    pub fn init(&mut self) -> SigmaStatus {
        self.active = true;
        SIGMA_OK
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn register_distro(&mut self, config: DistroStreamConfig) -> SigmaStatus {
        for i in 0..MAX_DISTROS {
            if self.configs[i].is_none() {
                self.configs[i] = Some(config);
                return SIGMA_OK;
            }
        }
        SIGMA_ERROR
    }

    pub fn translate_syscall(&self, linux_nr: u32) -> Option<u32> {
        for mapping in LINUX_SYSCALL_TABLE.iter() {
            if mapping.linux_nr == linux_nr {
                return Some(mapping.sigma_nr);
            }
        }
        None
    }
}

static mut G_INSTANCE: SigmaDistroStreamer = SigmaDistroStreamer::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_distro_init() -> SigmaStatus {
    G_INSTANCE.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_distro_active() -> u8 {
    G_INSTANCE.is_active() as u8
}

#[no_mangle]
pub unsafe extern "C" fn sigma_distro_translate_syscall(linux_nr: u32) -> i32 {
    match G_INSTANCE.translate_syscall(linux_nr) {
        Some(sigma_nr) => sigma_nr as i32,
        None => -1,
    }
}