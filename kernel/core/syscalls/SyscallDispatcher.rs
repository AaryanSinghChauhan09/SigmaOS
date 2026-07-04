/// SigmaOS: Modular Syscall Dispatcher
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Syscall Numbers ───────────────────────────────────────────────────────

pub const SYS_READ: SigmaU64 = 0;
pub const SYS_WRITE: SigmaU64 = 1;
pub const SYS_OPEN: SigmaU64 = 2;
pub const SYS_CLOSE: SigmaU64 = 3;
pub const SYS_MMAP: SigmaU64 = 9;
pub const SYS_MUNMAP: SigmaU64 = 11;
pub const SYS_EXIT: SigmaU64 = 60;
pub const SYS_GETPID: SigmaU64 = 39;
pub const SYS_CLONE: SigmaU64 = 56;
pub const SYS_EXECVE: SigmaU64 = 59;

// ─── Syscall Return Codes ────────────────────────────────────────────────────

pub const SYS_SUCCESS: SigmaI64 = 0;
pub const SYS_ERROR: SigmaI64 = -1;
pub const SYS_EINVAL: SigmaI64 = -22;
pub const SYS_ENOMEM: SigmaI64 = -12;
pub const SYS_EBADF: SigmaI64 = -9;

// ─── Syscall Context ─────────────────────────────────────────────────────────

#[repr(C)]
pub struct SyscallContext {
    pub rax: SigmaU64,  // Syscall number
    pub rdi: SigmaU64,  // Arg 1
    pub rsi: SigmaU64,  // Arg 2
    pub rdx: SigmaU64,  // Arg 3
    pub r10: SigmaU64,  // Arg 4
    pub r8: SigmaU64,   // Arg 5
    pub r9: SigmaU64,   // Arg 6
}

// ─── Syscall Handler Trait ─────────────────────────────────────────────────

pub trait SyscallHandler {
    unsafe fn handle(&self, ctx: &mut SyscallContext) -> SigmaI64;
}

// ─── Basic Syscall Implementations ──────────────────────────────────────────

struct BasicSyscalls;

impl SyscallHandler for BasicSyscalls {
    unsafe fn handle(&self, ctx: &mut SyscallContext) -> SigmaI64 {
        match ctx.rax {
            SYS_EXIT => {
                // Exit process - for now, halt
                core::arch::asm!("hlt");
                0
            }
            SYS_GETPID => {
                // Return fake PID for now
                1
            }
            SYS_MMAP => {
                // Placeholder for mmap - will integrate with VMM
                let _addr = ctx.rdi;
                let _length = ctx.rsi;
                let _prot = ctx.rdx;
                let _flags = ctx.r10;
                let _fd = ctx.r8 as i32;
                let _offset = ctx.r9;
                
                // For now, return error - will integrate with VMM
                SYS_ENOMEM
            }
            SYS_MUNMAP => {
                // Placeholder for munmap
                let _addr = ctx.rdi;
                let _length = ctx.rsi;
                SYS_SUCCESS
            }
            _ => {
                // Unknown syscall
                SYS_EINVAL
            }
        }
    }
}

// ─── Syscall Dispatcher ─────────────────────────────────────────────────────

static BASIC_SYSCALLS: BasicSyscalls = BasicSyscalls;

#[no_mangle]
pub unsafe extern "C" fn syscall_dispatcher(ctx: *mut SyscallContext) -> SigmaI64 {
    if ctx.is_null() {
        return SYS_EINVAL;
    }
    
    let context = &mut *ctx;
    let syscall_num = context.rax;
    
    // Dispatch based on syscall number
    let result = match syscall_num {
        SYS_EXIT | SYS_GETPID | SYS_MMAP | SYS_MUNMAP => {
            BASIC_SYSCALLS.handle(context)
        }
        _ => {
            // Unimplemented syscalls
            SYS_EINVAL
        }
    };
    
    result
}

// ─── Entry Point from Assembly ─────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn syscall_entry() {
    // This is called from assembly stub which saves registers
    // and loads them into a SyscallContext
    // For now, we'll handle the basic dispatch
    
    // In a complete implementation, this would:
    // 1. Save all registers
    // 2. Load syscall context from stack
    // 3. Call syscall_dispatcher
    // 4. Restore registers
    // 5. Return to userland with syscall return value
    
    core::arch::asm!("hlt");
}

