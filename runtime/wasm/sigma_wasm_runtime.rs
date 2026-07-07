//! SigmaOS — WASM/WASI Capability-Limited Runtime
//! Provides sandboxed execution of WebAssembly modules inside the OS.
//! No std, no allocator — fixed-size module table and stack.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type I64 = i64;
type Usize = usize;

// ── WASM Binary Format Constants ────────────────────────────────────────────
const WASM_MAGIC:   U32 = 0x6D73_6100; // \0asm
const WASM_VERSION: U32 = 1;

// Section IDs
const SEC_CUSTOM:   U8 = 0;
const SEC_TYPE:     U8 = 1;
const SEC_IMPORT:   U8 = 2;
const SEC_FUNCTION: U8 = 3;
const SEC_TABLE:    U8 = 4;
const SEC_MEMORY:   U8 = 5;
const SEC_GLOBAL:   U8 = 6;
const SEC_EXPORT:   U8 = 7;
const SEC_START:    U8 = 8;
const SEC_ELEMENT:  U8 = 9;
const SEC_CODE:     U8 = 10;
const SEC_DATA:     U8 = 11;

// Value types
const TYPE_I32: U8 = 0x7F;
const TYPE_I64: U8 = 0x7E;
const TYPE_F32: U8 = 0x7D;
const TYPE_F64: U8 = 0x7C;

// ── Capability Tokens for WASI Sandboxing ───────────────────────────────────
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WasiCapability {
    None         = 0,
    FdRead       = 1 << 0,
    FdWrite      = 1 << 1,
    PathOpen     = 1 << 2,
    ClockGetTime = 1 << 3,
    ProcExit     = 1 << 4,
    RandomGet    = 1 << 5,
    ArgsGet      = 1 << 6,
    EnvironGet   = 1 << 7,
    SockAccept   = 1 << 8,
    SockConnect  = 1 << 9,
}

// ── WASM Value ──────────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub enum WasmValue {
    I32(I32),
    I64(I64),
    F32(u32), // IEEE 754 bits
    F64(u64), // IEEE 754 bits
}

// ── WASM Function Signature ─────────────────────────────────────────────────
const MAX_PARAMS: usize = 8;
const MAX_RETURNS: usize = 4;

#[derive(Copy, Clone)]
pub struct FuncType {
    pub param_types: [U8; MAX_PARAMS],
    pub param_count: usize,
    pub return_types: [U8; MAX_RETURNS],
    pub return_count: usize,
}

impl FuncType {
    pub const fn empty() -> Self {
        FuncType {
            param_types: [0; MAX_PARAMS],
            param_count: 0,
            return_types: [0; MAX_RETURNS],
            return_count: 0,
        }
    }
}

// ── WASM Module ─────────────────────────────────────────────────────────────
const MAX_FUNCTIONS: usize = 256;
const MAX_EXPORTS:   usize = 64;
const MAX_TYPES:     usize = 64;
const MAX_GLOBALS:   usize = 32;
const STACK_SIZE:    usize = 1024;
const LINEAR_MEM_PAGES: usize = 16; // 16 * 64KB = 1MB
const PAGE_SIZE:     usize = 65536;

#[derive(Copy, Clone)]
pub struct WasmExport {
    pub name: [U8; 64],
    pub name_len: usize,
    pub kind: U8,        // 0=func, 1=table, 2=memory, 3=global
    pub index: U32,
}

impl WasmExport {
    pub const fn empty() -> Self {
        WasmExport {
            name: [0; 64],
            name_len: 0,
            kind: 0,
            index: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct WasmFunction {
    pub type_idx: U32,
    pub code_offset: U32,  // Byte offset into the module binary
    pub code_size: U32,
    pub local_count: U32,
}

impl WasmFunction {
    pub const fn empty() -> Self {
        WasmFunction {
            type_idx: 0,
            code_offset: 0,
            code_size: 0,
            local_count: 0,
        }
    }
}

pub struct WasmModule {
    pub valid: bool,
    pub types: [FuncType; MAX_TYPES],
    pub type_count: usize,
    pub functions: [WasmFunction; MAX_FUNCTIONS],
    pub func_count: usize,
    pub exports: [WasmExport; MAX_EXPORTS],
    pub export_count: usize,
    pub globals: [WasmValue; MAX_GLOBALS],
    pub global_count: usize,
    pub start_func: Option<U32>,
    pub memory_pages: U32,
    pub memory_max_pages: U32,
    pub capabilities: U32,   // Bitmask of WasiCapability
    // Execution state
    pub stack: [WasmValue; STACK_SIZE],
    pub sp: usize,
    pub linear_memory: [U8; LINEAR_MEM_PAGES * PAGE_SIZE],
}

// ── Module Table ────────────────────────────────────────────────────────────
const MAX_MODULES: usize = 4;

static mut MODULES: [Option<usize>; MAX_MODULES] = [None; MAX_MODULES]; // placeholder
static mut MODULE_COUNT: usize = 0;

// We use a single global module for simplicity in no_std
static mut CURRENT_MODULE: WasmModule = WasmModule {
    valid: false,
    types: [FuncType::empty(); MAX_TYPES],
    type_count: 0,
    functions: [WasmFunction::empty(); MAX_FUNCTIONS],
    func_count: 0,
    exports: [WasmExport::empty(); MAX_EXPORTS],
    export_count: 0,
    globals: [WasmValue::I32(0); MAX_GLOBALS],
    global_count: 0,
    start_func: None,
    memory_pages: 1,
    memory_max_pages: LINEAR_MEM_PAGES as U32,
    capabilities: 0,
    stack: [WasmValue::I32(0); STACK_SIZE],
    sp: 0,
    linear_memory: [0u8; LINEAR_MEM_PAGES * PAGE_SIZE],
};

// ── LEB128 Decoder ──────────────────────────────────────────────────────────
fn decode_leb128_u32(bytes: &[U8], pos: &mut usize) -> U32 {
    let mut result: U32 = 0;
    let mut shift = 0u32;
    loop {
        if *pos >= bytes.len() { break; }
        let byte = bytes[*pos];
        *pos += 1;
        result |= ((byte & 0x7F) as U32) << shift;
        if byte & 0x80 == 0 { break; }
        shift += 7;
        if shift >= 35 { break; }
    }
    result
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Validate a WASM binary's magic number and version.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_validate(binary: *const U8, len: U32) -> i32 {
    if binary.is_null() || len < 8 { return -1; }
    let bytes = core::slice::from_raw_parts(binary, len as usize);

    // Check magic: \0asm
    if bytes[0] != 0x00 || bytes[1] != 0x61 || bytes[2] != 0x73 || bytes[3] != 0x6D {
        return -2; // Bad magic
    }
    // Check version
    let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    if version != WASM_VERSION {
        return -3; // Unsupported version
    }
    0
}

/// Load a WASM module from a binary buffer with given capabilities.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_load(
    binary: *const U8,
    len: U32,
    capabilities: U32,
) -> i32 {
    let rc = sigma_wasm_validate(binary, len);
    if rc != 0 { return rc; }

    let bytes = core::slice::from_raw_parts(binary, len as usize);
    let mut pos: usize = 8; // Skip magic + version

    CURRENT_MODULE.valid = false;
    CURRENT_MODULE.capabilities = capabilities;
    CURRENT_MODULE.type_count = 0;
    CURRENT_MODULE.func_count = 0;
    CURRENT_MODULE.export_count = 0;
    CURRENT_MODULE.global_count = 0;
    CURRENT_MODULE.start_func = None;
    CURRENT_MODULE.sp = 0;

    // Parse sections
    while pos < bytes.len() {
        let sec_id = bytes[pos];
        pos += 1;
        let sec_len = decode_leb128_u32(bytes, &mut pos) as usize;
        let sec_end = pos + sec_len;

        match sec_id {
            SEC_TYPE => {
                let count = decode_leb128_u32(bytes, &mut pos) as usize;
                for i in 0..count {
                    if i >= MAX_TYPES { pos = sec_end; break; }
                    let _form = bytes[pos]; pos += 1; // 0x60 = functype
                    let pcount = decode_leb128_u32(bytes, &mut pos) as usize;
                    let mut ft = FuncType::empty();
                    ft.param_count = pcount.min(MAX_PARAMS);
                    for p in 0..pcount {
                        if p < MAX_PARAMS { ft.param_types[p] = bytes[pos]; }
                        pos += 1;
                    }
                    let rcount = decode_leb128_u32(bytes, &mut pos) as usize;
                    ft.return_count = rcount.min(MAX_RETURNS);
                    for r in 0..rcount {
                        if r < MAX_RETURNS { ft.return_types[r] = bytes[pos]; }
                        pos += 1;
                    }
                    CURRENT_MODULE.types[i] = ft;
                    CURRENT_MODULE.type_count = i + 1;
                }
            }
            SEC_FUNCTION => {
                let count = decode_leb128_u32(bytes, &mut pos) as usize;
                for i in 0..count {
                    if i >= MAX_FUNCTIONS { break; }
                    let tidx = decode_leb128_u32(bytes, &mut pos);
                    CURRENT_MODULE.functions[i].type_idx = tidx;
                    CURRENT_MODULE.func_count = i + 1;
                }
            }
            SEC_EXPORT => {
                let count = decode_leb128_u32(bytes, &mut pos) as usize;
                for i in 0..count {
                    if i >= MAX_EXPORTS { pos = sec_end; break; }
                    let name_len = decode_leb128_u32(bytes, &mut pos) as usize;
                    let mut exp = WasmExport::empty();
                    let copy_len = name_len.min(64);
                    for j in 0..copy_len {
                        exp.name[j] = bytes[pos + j];
                    }
                    exp.name_len = copy_len;
                    pos += name_len;
                    exp.kind = bytes[pos]; pos += 1;
                    exp.index = decode_leb128_u32(bytes, &mut pos);
                    CURRENT_MODULE.exports[i] = exp;
                    CURRENT_MODULE.export_count = i + 1;
                }
            }
            SEC_MEMORY => {
                let _count = decode_leb128_u32(bytes, &mut pos);
                let flags = decode_leb128_u32(bytes, &mut pos);
                CURRENT_MODULE.memory_pages = decode_leb128_u32(bytes, &mut pos);
                if flags & 1 != 0 {
                    CURRENT_MODULE.memory_max_pages = decode_leb128_u32(bytes, &mut pos);
                }
            }
            SEC_START => {
                let func_idx = decode_leb128_u32(bytes, &mut pos);
                CURRENT_MODULE.start_func = Some(func_idx);
            }
            SEC_CODE => {
                let count = decode_leb128_u32(bytes, &mut pos) as usize;
                for i in 0..count {
                    if i >= MAX_FUNCTIONS { pos = sec_end; break; }
                    let body_size = decode_leb128_u32(bytes, &mut pos);
                    CURRENT_MODULE.functions[i].code_offset = pos as U32;
                    CURRENT_MODULE.functions[i].code_size = body_size;
                    // Parse local declarations
                    let local_decl_count = decode_leb128_u32(bytes, &mut pos) as usize;
                    let mut total_locals: U32 = 0;
                    for _ in 0..local_decl_count {
                        let n = decode_leb128_u32(bytes, &mut pos);
                        let _type = bytes[pos]; pos += 1;
                        total_locals += n;
                    }
                    CURRENT_MODULE.functions[i].local_count = total_locals;
                    // Skip remaining code bytes
                    let code_start = CURRENT_MODULE.functions[i].code_offset as usize;
                    pos = code_start + body_size as usize;
                }
            }
            _ => {
                // Skip unknown section
                pos = sec_end;
            }
        }
        pos = sec_end;
    }

    CURRENT_MODULE.valid = true;
    0
}

/// Push a value onto the WASM stack.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_push_i32(val: I32) -> i32 {
    if CURRENT_MODULE.sp >= STACK_SIZE { return -1; }
    CURRENT_MODULE.stack[CURRENT_MODULE.sp] = WasmValue::I32(val);
    CURRENT_MODULE.sp += 1;
    0
}

/// Pop a value from the WASM stack.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_pop_i32() -> I32 {
    if CURRENT_MODULE.sp == 0 { return 0; }
    CURRENT_MODULE.sp -= 1;
    match CURRENT_MODULE.stack[CURRENT_MODULE.sp] {
        WasmValue::I32(v) => v,
        _ => 0,
    }
}

/// Check if a WASI capability is granted.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_has_capability(cap: U32) -> i32 {
    if (CURRENT_MODULE.capabilities & cap) != 0 { 1 } else { 0 }
}

/// Get the number of exported functions.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_export_count() -> U32 {
    CURRENT_MODULE.export_count as U32
}

/// Get the total function count in the loaded module.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_func_count() -> U32 {
    CURRENT_MODULE.func_count as U32
}

/// Read a byte from linear memory.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_mem_read(addr: U32) -> I32 {
    let a = addr as usize;
    if a >= CURRENT_MODULE.linear_memory.len() { return -1; }
    CURRENT_MODULE.linear_memory[a] as I32
}

/// Write a byte to linear memory.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_mem_write(addr: U32, val: U8) -> i32 {
    let a = addr as usize;
    if a >= CURRENT_MODULE.linear_memory.len() { return -1; }
    CURRENT_MODULE.linear_memory[a] = val;
    0
}

/// Get the current linear memory size in pages (64KB each).
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_mem_pages() -> U32 {
    CURRENT_MODULE.memory_pages
}

/// Returns 1 if a module is currently loaded and valid.
#[no_mangle]
pub unsafe extern "C" fn sigma_wasm_is_loaded() -> i32 {
    if CURRENT_MODULE.valid { 1 } else { 0 }
}
