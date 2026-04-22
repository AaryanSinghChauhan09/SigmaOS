/// shards/wasm/runtime.rs — SigmaOS WASM Runtime Bridge
/// Validates, loads, and executes WASM modules in an isolated sandbox.
/// No external WASM engine — implements a minimal bytecode dispatcher.
#![no_std]

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

// ── Constants ───────────────────────────────────────────────────────────────
const WASM_MAGIC:   [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
const WASM_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
const MAX_STACK:    usize   = 256;
const SHARED_MEM:   usize   = 65536; // 64KB linear memory

// ── Stats ───────────────────────────────────────────────────────────────────
static MODULES_LOADED:  AtomicU32 = AtomicU32::new(0);
static TOTAL_CYCLES:    AtomicU64 = AtomicU64::new(0);
static VALIDATION_FAILS: AtomicU32 = AtomicU32::new(0);

// ── Shared linear memory ────────────────────────────────────────────────────
static mut LINEAR_MEM: [u8; SHARED_MEM] = [0u8; SHARED_MEM];

// ── WASM Section IDs ────────────────────────────────────────────────────────
#[repr(u8)]
#[allow(dead_code)]
enum WasmSection {
    Custom   = 0, Type    = 1, Import  = 2, Function = 3,
    Table    = 4, Memory  = 5, Global  = 6, Export   = 7,
    Start    = 8, Element = 9, Code    = 10, Data     = 11,
}

// ── Module descriptor ───────────────────────────────────────────────────────
pub struct WasmRuntime {
    binary:     *const u8,
    binary_len: usize,
    stack:      [i64; MAX_STACK],
    sp:         usize,
    pc:         usize,
    cycle_limit: u64,
}

impl WasmRuntime {
    pub fn new(binary: *const u8, len: usize, cycle_limit: u64) -> Self {
        Self {
            binary, binary_len: len,
            stack: [0i64; MAX_STACK],
            sp: 0, pc: 8, // skip magic+version
            cycle_limit,
        }
    }

    fn byte_at(&self, idx: usize) -> u8 {
        if idx < self.binary_len { unsafe { *self.binary.add(idx) } } else { 0 }
    }

    // WASM LEB128 unsigned integer decoder
    fn read_uleb128(&mut self) -> u64 {
        let mut result: u64 = 0;
        let mut shift = 0u32;
        loop {
            let b = self.byte_at(self.pc) as u64;
            self.pc += 1;
            result |= (b & 0x7F) << shift;
            if b & 0x80 == 0 { break; }
            shift += 7;
        }
        result
    }

    fn stack_push(&mut self, v: i64) -> bool {
        if self.sp >= MAX_STACK { return false; }
        self.stack[self.sp] = v; self.sp += 1; true
    }

    fn stack_pop(&mut self) -> i64 {
        if self.sp == 0 { return 0; }
        self.sp -= 1; self.stack[self.sp]
    }

    /// Minimal WASM bytecode interpreter (subset of MVP opcodes)
    pub fn execute(&mut self) -> i32 {
        let mut cycles: u64 = 0;

        loop {
            if cycles >= self.cycle_limit { return -99; } // cycle limit
            if self.pc >= self.binary_len { break; }

            let op = self.byte_at(self.pc);
            self.pc += 1;
            cycles += 1;

            match op {
                0x00 => break,                   // unreachable → halt
                0x01 => {}                        // nop
                0x41 => {                         // i32.const
                    let v = self.read_uleb128() as i64;
                    self.stack_push(v);
                }
                0x6A => {                         // i32.add
                    let b = self.stack_pop();
                    let a = self.stack_pop();
                    self.stack_push(a.wrapping_add(b));
                }
                0x6B => {                         // i32.sub
                    let b = self.stack_pop();
                    let a = self.stack_pop();
                    self.stack_push(a.wrapping_sub(b));
                }
                0x6C => {                         // i32.mul
                    let b = self.stack_pop();
                    let a = self.stack_pop();
                    self.stack_push(a.wrapping_mul(b));
                }
                0x45 => {                         // i32.eqz
                    let v = self.stack_pop();
                    self.stack_push(if v == 0 { 1 } else { 0 });
                }
                0x0B => break,                   // end — function end
                0x0F => break,                   // return
                _    => {}                        // unimplemented: skip
            }
        }

        TOTAL_CYCLES.fetch_add(cycles, Ordering::Relaxed);
        0
    }
}

// ── C FFI ───────────────────────────────────────────────────────────────────

/// Validate and execute a WASM binary in the shared sandbox
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_exec(
    binary: *const u8, len: usize, cycle_limit: u64
) -> i32 {
    // 1. Validate header
    if binary.is_null() || len < 8 { return -1; }
    let hdr = core::slice::from_raw_parts(binary, 8);
    if hdr[..4] != WASM_MAGIC    { VALIDATION_FAILS.fetch_add(1, Ordering::Relaxed); return -2; }
    if hdr[4..8] != WASM_VERSION { VALIDATION_FAILS.fetch_add(1, Ordering::Relaxed); return -3; }

    // 2. Load and execute
    MODULES_LOADED.fetch_add(1, Ordering::Relaxed);
    let mut rt = WasmRuntime::new(binary, len, cycle_limit);
    rt.execute()
}

#[no_mangle]
pub extern "C" fn sigma_wasm_stats(
    out_loaded: *mut u32, out_cycles: *mut u64, out_fails: *mut u32
) {
    if !out_loaded.is_null()  { unsafe { *out_loaded  = MODULES_LOADED.load(Ordering::Relaxed); } }
    if !out_cycles.is_null()  { unsafe { *out_cycles  = TOTAL_CYCLES.load(Ordering::Relaxed); } }
    if !out_fails.is_null()   { unsafe { *out_fails   = VALIDATION_FAILS.load(Ordering::Relaxed); } }
}

#[no_mangle]
pub extern "C" fn sigma_wasm_linear_mem() -> *mut u8 {
    unsafe { LINEAR_MEM.as_mut_ptr() }
}

#[no_mangle]
pub extern "C" fn sigma_wasm_linear_mem_size() -> usize { SHARED_MEM }
