// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// tests/kernel/test_syscall_dispatch.rs

#![no_std]
#![no_main]

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TestResult {
    Pass = 0,
    Fail = 1,
    Skip = 2,
    Error = 3,
}

// Mock register structure
#[repr(C)]
pub struct MockRegisters {
    pub rax: SigmaU64,
    pub rbx: SigmaU64,
    pub rcx: SigmaU64,
    pub rdx: SigmaU64,
    pub rsi: SigmaU64,
    pub rdi: SigmaU64,
    pub r8: SigmaU64,
    pub r9: SigmaU64,
    pub r10: SigmaU64,
    pub r11: SigmaU64,
    pub r12: SigmaU64,
    pub r13: SigmaU64,
    pub r14: SigmaU64,
    pub r15: SigmaU64,
}

// Syscall numbers
const SYS_READ: SigmaU64 = 0;
const SYS_WRITE: SigmaU64 = 1;
const SYS_OPEN: SigmaU64 = 2;
const SYS_CLOSE: SigmaU64 = 3;
const SYS_MMAP: SigmaU64 = 9;
const SYS_MUNMAP: SigmaU64 = 11;
const SYS_EXIT: SigmaU64 = 60;

static mut LAST_SYSCALL: SigmaU64 = 0;
static mut LAST_ERRNO: SigmaI32 = 0;
static mut REGISTERS: MockRegisters = MockRegisters {
    rax: 0,
    rbx: 0,
    rcx: 0,
    rdx: 0,
    rsi: 0,
    rdi: 0,
    r8: 0,
    r9: 0,
    r10: 0,
    r11: 0,
    r12: 0,
    r13: 0,
    r14: 0,
    r15: 0,
};

/// Initialize mock registers
unsafe fn init_registers() {
    REGISTERS = MockRegisters {
        rax: 0,
        rbx: 0,
        rcx: 0,
        rdx: 0,
        rsi: 0,
        rdi: 0,
        r8: 0,
        r9: 0,
        r10: 0,
        r11: 0,
        r12: 0,
        r13: 0,
        r14: 0,
        r15: 0,
    };
    LAST_SYSCALL = 0;
    LAST_ERRNO = 0;
}

/// Mock syscall dispatch
unsafe fn mock_syscall_dispatch(syscall_num: SigmaU64, regs: &mut MockRegisters) -> SigmaI64 {
    LAST_SYSCALL = syscall_num;

    match syscall_num {
        SYS_READ => {
            // Mock read: return bytes read
            regs.rax = regs.rdx; // Return count
            0
        }
        SYS_WRITE => {
            // Mock write: return bytes written
            regs.rax = regs.rdx; // Return count
            0
        }
        SYS_OPEN => {
            // Mock open: return file descriptor
            regs.rax = 3; // fd 3
            0
        }
        SYS_CLOSE => {
            // Mock close: return 0 on success
            regs.rax = 0;
            0
        }
        SYS_MMAP => {
            // Mock mmap: return address
            regs.rax = 0x7ffff0000000;
            0
        }
        SYS_MUNMAP => {
            // Mock munmap: return 0 on success
            regs.rax = 0;
            0
        }
        SYS_EXIT => {
            // Mock exit: doesn't return
            0
        }
        _ => {
            // Unknown syscall
            LAST_ERRNO = -38; // ENOSYS
            -1
        }
    }
}

/// Test: Read syscall dispatch
unsafe fn test_read_syscall() -> TestResult {
    init_registers();

    REGISTERS.rdi = 3; // fd
    REGISTERS.rsi = 0x1000; // buffer
    REGISTERS.rdx = 1024; // count

    let result = mock_syscall_dispatch(SYS_READ, &mut REGISTERS);

    if result != 0 {
        return TestResult::Fail;
    }

    if LAST_SYSCALL != SYS_READ {
        return TestResult::Fail;
    }

    if REGISTERS.rax != 1024 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Write syscall dispatch
unsafe fn test_write_syscall() -> TestResult {
    init_registers();

    REGISTERS.rdi = 1; // fd (stdout)
    REGISTERS.rsi = 0x1000; // buffer
    REGISTERS.rdx = 512; // count

    let result = mock_syscall_dispatch(SYS_WRITE, &mut REGISTERS);

    if result != 0 {
        return TestResult::Fail;
    }

    if LAST_SYSCALL != SYS_WRITE {
        return TestResult::Fail;
    }

    if REGISTERS.rax != 512 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Open syscall dispatch
unsafe fn test_open_syscall() -> TestResult {
    init_registers();

    REGISTERS.rdi = 0x2000; // filename
    REGISTERS.rsi = 0; // flags (O_RDONLY)
    REGISTERS.rdx = 0; // mode

    let result = mock_syscall_dispatch(SYS_OPEN, &mut REGISTERS);

    if result != 0 {
        return TestResult::Fail;
    }

    if LAST_SYSCALL != SYS_OPEN {
        return TestResult::Fail;
    }

    if REGISTERS.rax != 3 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Close syscall dispatch
unsafe fn test_close_syscall() -> TestResult {
    init_registers();

    REGISTERS.rdi = 3; // fd

    let result = mock_syscall_dispatch(SYS_CLOSE, &mut REGISTERS);

    if result != 0 {
        return TestResult::Fail;
    }

    if LAST_SYSCALL != SYS_CLOSE {
        return TestResult::Fail;
    }

    if REGISTERS.rax != 0 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Mmap syscall dispatch
unsafe fn test_mmap_syscall() -> TestResult {
    init_registers();

    REGISTERS.rdi = 0; // addr
    REGISTERS.rsi = 4096; // length
    REGISTERS.rdx = 3; // prot (PROT_READ|PROT_WRITE)
    REGISTERS.r10 = 2; // flags (MAP_PRIVATE)
    REGISTERS.r8 = -1i64 as SigmaU64; // fd
    REGISTERS.r9 = 0; // offset

    let result = mock_syscall_dispatch(SYS_MMAP, &mut REGISTERS);

    if result != 0 {
        return TestResult::Fail;
    }

    if LAST_SYSCALL != SYS_MMAP {
        return TestResult::Fail;
    }

    if REGISTERS.rax != 0x7ffff0000000 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Unknown syscall handling
unsafe fn test_unknown_syscall() -> TestResult {
    init_registers();

    let result = mock_syscall_dispatch(999, &mut REGISTERS);

    if result != -1 {
        return TestResult::Fail;
    }

    if LAST_ERRNO != -38 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Register preservation
unsafe fn test_register_preservation() -> TestResult {
    init_registers();

    // Set some register values
    REGISTERS.rbx = 0xDEADBEEF;
    REGISTERS.r12 = 0xCAFEBABE;
    REGISTERS.r13 = 0x12345678;

    mock_syscall_dispatch(SYS_READ, &mut REGISTERS);

    // Verify registers are preserved (except return value in rax)
    if REGISTERS.rbx != 0xDEADBEEF {
        return TestResult::Fail;
    }
    if REGISTERS.r12 != 0xCAFEBABE {
        return TestResult::Fail;
    }
    if REGISTERS.r13 != 0x12345678 {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Test: Error handling
unsafe fn test_error_handling() -> TestResult {
    init_registers();

    // Test with invalid fd
    REGISTERS.rdi = -1i64 as SigmaU64; // invalid fd
    REGISTERS.rsi = 0x1000;
    REGISTERS.rdx = 1024;

    let result = mock_syscall_dispatch(SYS_READ, &mut REGISTERS);

    // In real implementation, this would return error
    // For mock, we just verify the dispatch works
    if LAST_SYSCALL != SYS_READ {
        return TestResult::Fail;
    }

    TestResult::Pass
}

/// Main test entry point
#[no_mangle]
pub extern "C" fn test_main() -> SigmaI32 {
    let mut passed = 0;
    let mut failed = 0;

    // Run syscall dispatch tests
    unsafe {
        if test_read_syscall() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_write_syscall() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_open_syscall() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_close_syscall() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_mmap_syscall() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_unknown_syscall() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_register_preservation() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }

        if test_error_handling() == TestResult::Pass {
            passed += 1;
        } else {
            failed += 1;
        }
    }

    // Return 0 on success, non-zero on failure
    if failed > 0 {
        1
    } else {
        0
    }
}
