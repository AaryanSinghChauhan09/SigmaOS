/// SigmaOS — modules/core/kernel/shard.rs
/// Shard Lifecycle Management: spawn, suspend, kill, capability inheritance.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaI64   = i64;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Configuration ────────────────────────────────────────────────────────────

pub const SHARD_MAX_COUNT: SigmaUsize = 1024;
pub const SHARD_NAME_LEN:  SigmaUsize = 32;

// ─── Shard State ──────────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ShardState {
    Empty       = 0,
    Spawning    = 1,
    Runnable    = 2,
    Blocked     = 3,
    Suspended   = 4,
    Zombie      = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShardContext {
    pub rip: SigmaU64,
    pub rsp: SigmaU64,
    pub rbp: SigmaU64,
    pub rbx: SigmaU64,
    pub r12: SigmaU64,
    pub r13: SigmaU64,
    pub r14: SigmaU64,
    pub r15: SigmaU64,
    pub cr3: SigmaU64,  // Page table root
}

impl ShardContext {
    pub const fn empty() -> Self {
        ShardContext {
            rip: 0, rsp: 0, rbp: 0, rbx: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            cr3: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShardControlBlock {
    pub id:           SigmaU32,
    pub parent_id:    SigmaU32,
    pub state:        ShardState,
    pub name:         [u8; SHARD_NAME_LEN],
    pub capabilities: SigmaU64, // Bitmask (from capability.rs)
    pub context:      ShardContext,
    pub cpu_affinity: SigmaU64, // Bitmask of allowed CPUs
    pub exit_code:    SigmaI32,
}

impl ShardControlBlock {
    pub const fn empty() -> Self {
        ShardControlBlock {
            id:           0,
            parent_id:    0,
            state:        ShardState::Empty,
            name:         [0; SHARD_NAME_LEN],
            capabilities: 0,
            context:      ShardContext::empty(),
            cpu_affinity: !0, // All CPUs
            exit_code:    0,
        }
    }
}

static mut SHARD_TABLE: [ShardControlBlock; SHARD_MAX_COUNT] = [ShardControlBlock::empty(); SHARD_MAX_COUNT];
static mut NEXT_SHARD_ID: SigmaU32 = 1;

// ─── External Hooks ───────────────────────────────────────────────────────────

extern "C" {
    fn kernel_inc_shard_count();
    fn kernel_dec_shard_count();
    fn sigma_alloc_pages(order: SigmaU32) -> SigmaU64;
    fn sigma_free_pages(addr: SigmaU64, order: SigmaU32);
}

// ─── Implementation ───────────────────────────────────────────────────────────

/// Allocate a new empty shard slot.
unsafe fn alloc_shard_slot() -> Option<usize> {
    for i in 0..SHARD_MAX_COUNT {
        if SHARD_TABLE[i].state == ShardState::Empty {
            return Some(i);
        }
    }
    None
}

/// Spawn a new shard. Arguments passed via array to match syscall interface.
/// args[0] = parent_id
/// args[1] = entry_point (RIP)
/// args[2] = stack_pointer (RSP)
/// args[3] = page_table (CR3)
/// args[4] = requested_caps
#[no_mangle]
pub unsafe extern "C" fn shard_spawn(args: *const SigmaU64) -> SigmaI64 {
    if args.is_null() { return -1; }
    
    let parent_id      = *args.add(0) as SigmaU32;
    let entry_point    = *args.add(1);
    let stack_pointer  = *args.add(2);
    let page_table     = *args.add(3);
    let requested_caps = *args.add(4);

    let slot = match alloc_shard_slot() {
        Some(s) => s,
        None => return -2, // ENOMEM
    };

    let id = NEXT_SHARD_ID;
    NEXT_SHARD_ID = NEXT_SHARD_ID.wrapping_add(1);

    let scb = &mut SHARD_TABLE[slot];
    scb.id           = id;
    scb.parent_id    = parent_id;
    scb.state        = ShardState::Spawning;
    scb.capabilities = requested_caps; // In reality, intersect with parent caps
    
    scb.context.rip = entry_point;
    scb.context.rsp = stack_pointer;
    scb.context.cr3 = page_table;

    // Mark ready to run
    scb.state = ShardState::Runnable;
    kernel_inc_shard_count();

    id as SigmaI64
}

/// Kill a shard by ID.
#[no_mangle]
pub unsafe extern "C" fn shard_kill(id: SigmaU32) -> SigmaI64 {
    if id == 0 { return -1; } // Cannot kill kernel idle shard
    
    for i in 0..SHARD_MAX_COUNT {
        if SHARD_TABLE[i].id == id && SHARD_TABLE[i].state != ShardState::Empty {
            SHARD_TABLE[i].state = ShardState::Zombie;
            // Memory cleanup (page tables, stacks) would be triggered here
            // or deferred to a reaper thread.
            return 0;
        }
    }
    -4 // ENOENT
}

/// Wait for a child shard to exit and collect its status.
#[no_mangle]
pub unsafe extern "C" fn shard_wait(id: SigmaU32, exit_code_out: *mut SigmaI32) -> SigmaI64 {
    for i in 0..SHARD_MAX_COUNT {
        if SHARD_TABLE[i].id == id {
            if SHARD_TABLE[i].state == ShardState::Zombie {
                if !exit_code_out.is_null() {
                    *exit_code_out = SHARD_TABLE[i].exit_code;
                }
                // Fully reap
                SHARD_TABLE[i].state = ShardState::Empty;
                kernel_dec_shard_count();
                return 0;
            } else {
                return -16; // EBUSY (still running)
            }
        }
    }
    -4 // ENOENT
}

/// Suspend execution of a shard.
#[no_mangle]
pub unsafe extern "C" fn shard_suspend(id: SigmaU32) -> SigmaI64 {
    for i in 0..SHARD_MAX_COUNT {
        if SHARD_TABLE[i].id == id {
            if SHARD_TABLE[i].state == ShardState::Runnable {
                SHARD_TABLE[i].state = ShardState::Suspended;
                return 0;
            }
            return -1; // Wrong state
        }
    }
    -4
}

/// Resume a suspended shard.
#[no_mangle]
pub unsafe extern "C" fn shard_resume(id: SigmaU32) -> SigmaI64 {
    for i in 0..SHARD_MAX_COUNT {
        if SHARD_TABLE[i].id == id {
            if SHARD_TABLE[i].state == ShardState::Suspended {
                SHARD_TABLE[i].state = ShardState::Runnable;
                return 0;
            }
            return -1; // Wrong state
        }
    }
    -4
}
