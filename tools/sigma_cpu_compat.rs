//! SigmaOS Micro-Architectural, Firmware & Instruction Set Abstraction Engine
//! Simulates and abstracts low-level ISA concepts for both x86_64 (CISC) and ARM AArch64/AArch32 (RISC) architectures.
//! Includes instruction modeling, flag arithmetic, cache operations, JIT safety, and sync primitives.
//! Zero external dependencies.

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaSize = usize;

/// CPU Architecture Mode
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum CpuArchMode {
    CiscX86_64,
    RiscArm32,
    RiscArm64,
}

/// CPU State Register Set (combining standard x86 and ARM concepts)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CpuState {
    pub r: [SigmaU32; 16],        // ARM general registers (R0 to R15). R13=SP, R14=LR, R15=PC
    pub rip: SigmaU64,            // x86_64 Instruction Pointer (EIP/PC)
    pub lr: SigmaU32,             // Link Register (BX LR, branch link return target)
    pub sp: SigmaU32,             // Stack Pointer
    pub cpsr: SigmaU32,           // Current Program Status Register (NZCV flags)
    pub mode: CpuStateMode,       // Processor Execution Mode (USR, SVC, etc.)
    pub thumb_state: SigmaBool,   // Active Thumb state (AArch32 Thumb state branch target)
    pub icache_dirty: SigmaBool,  // Instruction Cache consistency state
    pub dcache_dirty: SigmaBool,  // Data Cache consistency state
}

/// Processor Execution Privilege Modes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum CpuStateMode {
    UserMode,       // USR Mode (unprivileged Ring 3)
    SupervisorMode, // SVC Mode (privileged Ring 0)
}

/// ARM Multiple Register Addressing Modes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum AddressingMode {
    IncrementAfter,  // IA
    IncrementBefore, // IB
    DecrementAfter,  // DA
    DecrementBefore, // DB
}

/// ARM Condition Codes (NZCV flags)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum ConditionCode {
    EQ, // Equal (Z=1)
    NE, // Not Equal (Z=0)
    MI, // Minus / Negative (N=1)
    PL, // Plus / Positive (N=0)
    VS, // Overflow set (V=1)
    VC, // Overflow clear (V=0)
    HI, // Unsigned higher (C=1 and Z=0)
    LS, // Unsigned lower or same (C=0 or Z=1)
    GE, // Signed greater than or equal (N=V)
    LT, // Signed less than (N!=V)
    GT, // Signed greater than (Z=0 and N=V)
    LE, // Signed less than or equal (Z=1 or N!=V)
    AL, // Always (unconditional execution)
}

/// Active global CPU state
static mut CPU_STATE: CpuState = CpuState {
    r: [0; 16],
    rip: 0,
    lr: 0,
    sp: 0x20000000, // Top of memory stack
    cpsr: 0,
    mode: CpuStateMode::SupervisorMode,
    thumb_state: false,
    icache_dirty: false,
    dcache_dirty: false,
};

/// Initialize CPU Abstraction State
#[no_mangle]
pub unsafe extern "C" fn cpu_init(arch: CpuArchMode) -> SigmaI32 {
    CPU_STATE.rip = 0x100000; // Boot memory entry
    CPU_STATE.sp = 0x20000000;
    CPU_STATE.cpsr = 0; // Clear all N, Z, C, V flags
    CPU_STATE.mode = CpuStateMode::SupervisorMode;
    CPU_STATE.thumb_state = if arch == CpuArchMode::RiscArm32 { false } else { false };
    CPU_STATE.icache_dirty = false;
    CPU_STATE.dcache_dirty = false;
    for i in 0..16 {
        CPU_STATE.r[i] = 0;
    }
    0 // Success
}

/// Parse Condition Codes from CPSR NZCV flags (Bit 31: N, Bit 30: Z, Bit 29: C, Bit 28: V)
#[no_mangle]
pub unsafe extern "C" fn cpu_check_condition(cond: ConditionCode) -> SigmaBool {
    let n = (CPU_STATE.cpsr >> 31) & 1 == 1;
    let z = (CPU_STATE.cpsr >> 30) & 1 == 1;
    let c = (CPU_STATE.cpsr >> 29) & 1 == 1;
    let v = (CPU_STATE.cpsr >> 28) & 1 == 1;

    match cond {
        ConditionCode::EQ => z,
        ConditionCode::NE => !z,
        ConditionCode::MI => n,
        ConditionCode::PL => !n,
        ConditionCode::VS => v,
        ConditionCode::VC => !v,
        ConditionCode::HI => c && !z,
        ConditionCode::LS => !c || z,
        ConditionCode::GE => n == v,
        ConditionCode::LT => n != v,
        ConditionCode::GT => !z && (n == v),
        ConditionCode::LE => z || (n != v),
        ConditionCode::AL => true,
    }
}

/// Simulate LDR (Load Register) instruction with ARM Addressing Mode offset and index
#[no_mangle]
pub unsafe extern "C" fn cpu_ldr(reg_idx: SigmaU32, base_addr: SigmaU32, offset: SigmaI32, mode: AddressingMode) -> SigmaI32 {
    if reg_idx >= 16 {
        return -1;
    }

    let mut target_addr = base_addr;
    match mode {
        AddressingMode::IncrementAfter  => { /* offset applied post-load */ },
        AddressingMode::IncrementBefore => { target_addr = (base_addr as i64 + offset as i64) as SigmaU32; },
        AddressingMode::DecrementAfter  => { /* offset applied post-load */ },
        AddressingMode::DecrementBefore => { target_addr = (base_addr as i64 - offset as i64) as SigmaU32; },
    }

    // Simulate MMIO or memory register read
    let loaded_value = if target_addr % 4 == 0 {
        // Read aligned memory (simulated offset value)
        target_addr.wrapping_add(0xABCDEF)
    } else {
        target_addr
    };

    CPU_STATE.r[reg_idx as usize] = loaded_value;
    0 // Success
}

/// Simulate STR (Store Register) instruction
#[no_mangle]
pub unsafe extern "C" fn cpu_str(reg_idx: SigmaU32, base_addr: SigmaU32, offset: SigmaI32, mode: AddressingMode) -> SigmaI32 {
    if reg_idx >= 16 {
        return -1;
    }

    let mut target_addr = base_addr;
    match mode {
        AddressingMode::IncrementAfter  => { /* post-write offset */ },
        AddressingMode::IncrementBefore => { target_addr = (base_addr as i64 + offset as i64) as SigmaU32; },
        AddressingMode::DecrementAfter  => { /* post-write offset */ },
        AddressingMode::DecrementBefore => { target_addr = (base_addr as i64 - offset as i64) as SigmaU32; },
    }

    let value_to_store = CPU_STATE.r[reg_idx as usize];
    // In production, this writes volatile MMIO to the computed target_addr
    let _ = target_addr;
    let _ = value_to_store;

    0 // Success
}

/// Simulate LDM (Load Multiple) instruction
#[no_mangle]
pub unsafe extern "C" fn cpu_ldm(base_reg_idx: SigmaU32, reg_mask: SigmaU16, mode: AddressingMode) -> SigmaI32 {
    if base_reg_idx >= 16 {
        return -1;
    }

    let mut addr = CPU_STATE.r[base_reg_idx as usize];

    for i in 0..16 {
        if (reg_mask & (1 << i)) != 0 {
            match mode {
                AddressingMode::IncrementBefore => { addr += 4; },
                AddressingMode::DecrementBefore => { addr -= 4; },
                _ => {}
            }

            // Perform simulated memory read
            CPU_STATE.r[i] = addr.wrapping_add(0x42);

            match mode {
                AddressingMode::IncrementAfter => { addr += 4; },
                AddressingMode::DecrementAfter => { addr -= 4; },
                _ => {}
            }
        }
    }

    CPU_STATE.r[base_reg_idx as usize] = addr; // Writeback
    0 // Success
}

/// Simulate STM (Store Multiple) instruction
#[no_mangle]
pub unsafe extern "C" fn cpu_stm(base_reg_idx: SigmaU32, reg_mask: SigmaU16, mode: AddressingMode) -> SigmaI32 {
    if base_reg_idx >= 16 {
        return -1;
    }

    let mut addr = CPU_STATE.r[base_reg_idx as usize];

    for i in 0..16 {
        if (reg_mask & (1 << i)) != 0 {
            match mode {
                AddressingMode::IncrementBefore => { addr += 4; },
                AddressingMode::DecrementBefore => { addr -= 4; },
                _ => {}
            }

            let value_to_write = CPU_STATE.r[i];
            let _ = value_to_write;

            match mode {
                AddressingMode::IncrementAfter => { addr += 4; },
                AddressingMode::DecrementAfter => { addr -= 4; },
                _ => {}
            }
        }
    }

    CPU_STATE.r[base_reg_idx as usize] = addr; // Writeback
    0 // Success
}

/// Simulate PUSH registers to stack (DB Addressing Mode)
#[no_mangle]
pub unsafe extern "C" fn cpu_push(reg_mask: SigmaU16) -> SigmaI32 {
    let mut addr = CPU_STATE.sp;
    for i in (0..16).rev() {
        if (reg_mask & (1 << i)) != 0 {
            addr -= 4;
            let val = CPU_STATE.r[i];
            let _ = val; // Store to stack memory [addr]
        }
    }
    CPU_STATE.sp = addr;
    0 // Success
}

/// Simulate POP registers from stack (IA Addressing Mode)
#[no_mangle]
pub unsafe extern "C" fn cpu_pop(reg_mask: SigmaU16) -> SigmaI32 {
    let mut addr = CPU_STATE.sp;
    for i in 0..16 {
        if (reg_mask & (1 << i)) != 0 {
            CPU_STATE.r[i] = addr.wrapping_add(0x77); // Load from stack memory [addr]
            addr += 4;
        }
    }
    CPU_STATE.sp = addr;
    0 // Success
}

/// Simulate ARM Register Shift / Logical operations (LSL, LSR, ASR, ROR, RRX)
#[no_mangle]
pub unsafe extern "C" fn cpu_shift(reg_idx: SigmaU32, val: SigmaU32, shift_amount: SigmaU32, shift_type: SigmaU8) -> SigmaU32 {
    if reg_idx >= 16 {
        return 0;
    }

    let result = match shift_type {
        0 => val << shift_amount, // LSL
        1 => val >> shift_amount, // LSR
        2 => (val as i32 >> shift_amount) as SigmaU32, // ASR
        3 => val.rotate_right(shift_amount), // ROR
        4 => { // RRX (Rotate Right with Extend using CPSR C-flag)
            let carry = (CPU_STATE.cpsr >> 29) & 1;
            (val >> 1) | (carry << 31)
        },
        _ => val,
    };

    CPU_STATE.r[reg_idx as usize] = result;
    result
}

/// Simulate ARM AArch32 Thumb inter-working branch (`BX LR` / `BLX`)
#[no_mangle]
pub unsafe extern "C" fn cpu_bx_lr() -> SigmaI32 {
    // If bit 0 of Link Register is 1, switch to THUMB state, else ARM state
    CPU_STATE.thumb_state = (CPU_STATE.lr & 1) == 1;
    CPU_STATE.rip = (CPU_STATE.lr & 0xFFFFFFFE) as SigmaU64; // Clear LSB
    0 // Success
}

/// Simulate CBZ (Compare and Branch on Zero)
#[no_mangle]
pub unsafe extern "C" fn cpu_cbz(reg_idx: SigmaU32, target_offset: SigmaI32) -> SigmaBool {
    if reg_idx < 16 && CPU_STATE.r[reg_idx as usize] == 0 {
        CPU_STATE.rip = (CPU_STATE.rip as i64 + target_offset as i64) as SigmaU64;
        return true;
    }
    false
}

/// Simulate CBNZ (Compare and Branch on Non-Zero)
#[no_mangle]
pub unsafe extern "C" fn cpu_cbnz(reg_idx: SigmaU32, target_offset: SigmaI32) -> SigmaBool {
    if reg_idx < 16 && CPU_STATE.r[reg_idx as usize] != 0 {
        CPU_STATE.rip = (CPU_STATE.rip as i64 + target_offset as i64) as SigmaU64;
        return true;
    }
    false
}

/// Low-Level JIT Cache Flushing / Consistency Protocol (Self-Modifying Code protection)
#[no_mangle]
pub unsafe extern "C" fn cpu_flush_caches(addr: SigmaSize, len: SigmaSize) -> SigmaI32 {
    // 1. Flush Data Cache lines to memory
    CPU_STATE.dcache_dirty = false;

    // 2. Invalidate Instruction Cache lines
    CPU_STATE.icache_dirty = false;

    // Simulate architecture-specific fence commands (ISB / DSB on ARM, CLFLUSH / MFENCE on x86)
    let _ = addr;
    let _ = len;

    0 // Success
}

/// Synchronization Primitives: LDREX (Load Register Exclusive)
#[no_mangle]
pub unsafe extern "C" fn cpu_ldrex(reg_idx: SigmaU32, base_addr: SigmaU32) -> SigmaI32 {
    if reg_idx >= 16 {
        return -1;
    }

    // Set processor physical lock monitor (mocked)
    CPU_STATE.r[reg_idx as usize] = base_addr.wrapping_add(0xF00D);
    0 // Success
}

/// Synchronization Primitives: STREX (Store Register Exclusive - lock-free atomics)
#[no_mangle]
pub unsafe extern "C" fn cpu_strex(dest_reg_idx: SigmaU32, src_reg_idx: SigmaU32, base_addr: SigmaU32) -> SigmaI32 {
    if dest_reg_idx >= 16 || src_reg_idx >= 16 {
        return -1;
    }

    // Attempt exclusive write back (simulating lock monitor success)
    let store_success = true;
    if store_success {
        let value_to_store = CPU_STATE.r[src_reg_idx as usize];
        let _ = value_to_store;
        let _ = base_addr;
        CPU_STATE.r[dest_reg_idx as usize] = 0; // 0 = Store completed successfully
    } else {
        CPU_STATE.r[dest_reg_idx as usize] = 1; // 1 = Store failed / exclusive monitor lost
    }

    0 // Success
}

/// Update CPSR flags based on comparison arithmetic
#[no_mangle]
pub unsafe extern "C" fn cpu_cmp(val1: SigmaU32, val2: SigmaU32) {
    let result = val1.wrapping_sub(val2);
    let n = (result >> 31) & 1;
    let z = if result == 0 { 1 } else { 0 };
    let c = if val1 >= val2 { 1 } else { 0 };
    let v = if ((val1 ^ val2) & (val1 ^ result)) >> 31 == 1 { 1 } else { 0 };

    // Update CPSR flag bits
    CPU_STATE.cpsr = (n << 31) | (z << 30) | (c << 29) | (v << 28);
}

/// Get current Link Register value
#[no_mangle]
pub unsafe extern "C" fn cpu_get_lr() -> SigmaU32 {
    CPU_STATE.lr
}

/// Get current Program Counter / Instruction Pointer
#[no_mangle]
pub unsafe extern "C" fn cpu_get_pc() -> SigmaU64 {
    CPU_STATE.rip
}

/// Get general-purpose register value
#[no_mangle]
pub unsafe extern "C" fn cpu_get_reg(idx: SigmaU32) -> SigmaU32 {
    if idx < 16 {
        CPU_STATE.r[idx as usize]
    } else {
        0
    }
}
