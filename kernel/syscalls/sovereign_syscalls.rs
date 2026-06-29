// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Syscall Translation Layer (Rust, no_std)
//! Replaces: kernel/syscalls/sigma_syscalls.cpp
//! =========================================================================

#![no_std]

pub const SIGMA_SYS_DEBUG_PRINT: u64 = 0x01;
pub const SIGMA_SYS_ALLOC_MEM: u64 = 0x02;
pub const SIGMA_SYS_FREE_MEM: u64 = 0x03;
pub const SIGMA_SYS_SEND_MSG: u64 = 0x04;
pub const SIGMA_SYS_RECV_MSG: u64 = 0x05;
pub const SIGMA_SYS_HW_IO: u64 = 0x06;
pub const SIGMA_SYS_SPAWN_TASK: u64 = 0x07;
pub const SIGMA_SYS_YIELD: u64 = 0x08;

pub const POSIX_SYS_READ: u64 = 0;
pub const POSIX_SYS_WRITE: u64 = 1;
pub const POSIX_SYS_OPEN: u64 = 2;
pub const POSIX_SYS_CLOSE: u64 = 3;
pub const POSIX_SYS_FORK: u64 = 57;
pub const POSIX_SYS_EXECVE: u64 = 59;
pub const POSIX_SYS_EXIT: u64 = 60;

extern "C" {
    fn sched_add_task(pid: u32, policy: i32, priority: u8, deadline_us: u64) -> u32;
    fn sched_yield();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_syscall_handler(
    mut syscall_num: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
) -> u64 {
    // POSIX translation matching legacy cpp behaviour
    match syscall_num {
        POSIX_SYS_READ => syscall_num = SIGMA_SYS_RECV_MSG,
        POSIX_SYS_WRITE => syscall_num = SIGMA_SYS_DEBUG_PRINT,
        POSIX_SYS_FORK => syscall_num = SIGMA_SYS_SPAWN_TASK,
        POSIX_SYS_EXIT => syscall_num = SIGMA_SYS_YIELD,
        _ => {}
    }

    match syscall_num {
        SIGMA_SYS_DEBUG_PRINT => 0,
        SIGMA_SYS_ALLOC_MEM => 0,
        SIGMA_SYS_FREE_MEM => 0,
        SIGMA_SYS_SEND_MSG => 0,
        SIGMA_SYS_RECV_MSG => 0,
        SIGMA_SYS_HW_IO => 0,
        SIGMA_SYS_SPAWN_TASK => {
            sched_add_task(arg1 as u32, arg2 as i32, arg3 as u8, 0) as u64
        }
        SIGMA_SYS_YIELD => {
            sched_yield();
            0
        }
        _ => !0, // ENOSYS/error equivalent
    }
}
