// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// runtime/wasm/sigma_wasm.rs — WASM/WASI Runtime (no_std, cleanroom)
// Language: Rust #![no_std]
// Pattern: OOP via WasmRuntime struct + Module/Instance

#![no_std]

// ── WASM Binary Constants ─────────────────────────────────────────────────────
const WASM_MAGIC:   [u8; 4] = [0x00, 0x61, 0x73, 0x6D]; // \0asm
const WASM_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

// ── Section IDs ───────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SectionId {
    Custom   = 0, Type   = 1, Import  = 2, Function = 3,
    Table    = 4, Memory = 5, Global  = 6, Export   = 7,
    Start    = 8, Element = 9, Code   = 10, Data    = 11,
}

// ── Value Types ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ValType { I32, I64, F32, F64 }

impl ValType {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b { 0x7F => Some(Self::I32), 0x7E => Some(Self::I64),
                  0x7D => Some(Self::F32), 0x7C => Some(Self::F64), _ => None }
    }
}

// ── Value ─────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug)]
pub enum Value { I32(i32), I64(i64), F32(u32), F64(u64) }

impl Value {
    pub fn default_for(t: ValType) -> Self {
        match t { ValType::I32 => Self::I32(0), ValType::I64 => Self::I64(0),
                  ValType::F32 => Self::F32(0), ValType::F64 => Self::F64(0) }
    }
    pub fn as_i32(&self) -> Option<i32> { if let Self::I32(v) = self { Some(*v) } else { None } }
    pub fn as_i64(&self) -> Option<i64> { if let Self::I64(v) = self { Some(*v) } else { None } }
}

// ── Validation ────────────────────────────────────────────────────────────────
#[derive(Debug)]
pub enum WasmError {
    BadMagic, BadVersion, UnexpectedEof,
    InvalidSection, InvalidType, TrapUnreachable,
    StackOverflow, StackUnderflow, OutOfMemory,
    DivisionByZero, OutOfBounds,
}

pub type WasmResult<T> = Result<T, WasmError>;

// ── LEB128 Decoder ────────────────────────────────────────────────────────────
pub struct Reader<'a> { data: &'a [u8], pos: usize }

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, pos: 0 } }
    pub fn remaining(&self) -> usize { self.data.len() - self.pos }

    pub fn read_u8(&mut self) -> WasmResult<u8> {
        if self.pos >= self.data.len() { return Err(WasmError::UnexpectedEof); }
        let b = self.data[self.pos]; self.pos += 1; Ok(b)
    }

    pub fn read_bytes(&mut self, n: usize) -> WasmResult<&'a [u8]> {
        if self.pos + n > self.data.len() { return Err(WasmError::UnexpectedEof); }
        let s = &self.data[self.pos..self.pos+n]; self.pos += n; Ok(s)
    }

    pub fn read_u32_leb(&mut self) -> WasmResult<u32> {
        let mut result = 0u32; let mut shift = 0u32;
        loop {
            let b = self.read_u8()?;
            result |= ((b & 0x7F) as u32) << shift;
            shift  += 7;
            if b & 0x80 == 0 { return Ok(result); }
            if shift >= 35  { return Err(WasmError::InvalidType); }
        }
    }

    pub fn read_i32_leb(&mut self) -> WasmResult<i32> {
        let mut result = 0i32; let mut shift = 0u32;
        loop {
            let b = self.read_u8()?;
            result |= ((b & 0x7F) as i32) << shift;
            shift  += 7;
            if b & 0x80 == 0 {
                if shift < 32 && (b & 0x40) != 0 { result |= -(1i32 << shift); }
                return Ok(result);
            }
            if shift >= 35 { return Err(WasmError::InvalidType); }
        }
    }
}

// ── Function Type ─────────────────────────────────────────────────────────────
pub const MAX_PARAMS:  usize = 8;
pub const MAX_RETURNS: usize = 4;

#[derive(Clone, Copy)]
pub struct FuncType {
    pub params:   [ValType; MAX_PARAMS],
    pub n_params: usize,
    pub results:  [ValType; MAX_RETURNS],
    pub n_results: usize,
}

// ── Wasm Module (parsed header only, function bodies stored as byte offsets) ──
pub const MAX_FUNCS:  usize = 256;
pub const MAX_TYPES:  usize = 64;
pub const MAX_MEMORY: usize = 1024 * 1024; // 1MB linear memory cap

pub struct WasmModule<'a> {
    raw:        &'a [u8],
    pub types:  [FuncType; MAX_TYPES],
    pub n_types: usize,
    pub func_type_idx: [u32; MAX_FUNCS],
    pub n_funcs:       usize,
    pub code_offsets:  [usize; MAX_FUNCS], // byte offset into `raw` for each func body
    pub memory_pages:  u32,
    pub start_func:    Option<u32>,
}

impl<'a> WasmModule<'a> {
    /// Parse a WASM binary
    pub fn parse(raw: &'a [u8]) -> WasmResult<Self> {
        let mut r = Reader::new(raw);
        // Magic + version
        let magic = r.read_bytes(4)?;
        if magic != &WASM_MAGIC { return Err(WasmError::BadMagic); }
        let ver = r.read_bytes(4)?;
        if ver != &WASM_VERSION { return Err(WasmError::BadVersion); }

        let mut m = WasmModule {
            raw,
            types:          [FuncType { params: [ValType::I32;MAX_PARAMS], n_params: 0,
                                        results: [ValType::I32;MAX_RETURNS], n_results: 0 };
                             MAX_TYPES],
            n_types:        0,
            func_type_idx:  [0u32; MAX_FUNCS],
            n_funcs:        0,
            code_offsets:   [0usize; MAX_FUNCS],
            memory_pages:   1,
            start_func:     None,
        };

        while r.remaining() > 0 {
            let sec_id = r.read_u8()?;
            let sec_len = r.read_u32_leb()? as usize;
            let sec_data = r.read_bytes(sec_len)?;
            let mut sr = Reader::new(sec_data);

            match sec_id {
                1 => m.parse_type_section(&mut sr)?,
                3 => m.parse_function_section(&mut sr)?,
                5 => m.parse_memory_section(&mut sr)?,
                8 => {
                    m.start_func = Some(sr.read_u32_leb()?);
                }
                10 => m.parse_code_section(&mut sr, r.pos - sec_len)?,
                _  => {} // skip unknown sections
            }
        }
        Ok(m)
    }

    fn parse_type_section(&mut self, r: &mut Reader<'_>) -> WasmResult<()> {
        let count = r.read_u32_leb()? as usize;
        for i in 0..count.min(MAX_TYPES) {
            let tag = r.read_u8()?;
            if tag != 0x60 { return Err(WasmError::InvalidType); }
            let np = r.read_u32_leb()? as usize;
            for j in 0..np.min(MAX_PARAMS) {
                self.types[i].params[j] = ValType::from_byte(r.read_u8()?)
                    .ok_or(WasmError::InvalidType)?;
            }
            self.types[i].n_params = np.min(MAX_PARAMS);
            let nr = r.read_u32_leb()? as usize;
            for j in 0..nr.min(MAX_RETURNS) {
                self.types[i].results[j] = ValType::from_byte(r.read_u8()?)
                    .ok_or(WasmError::InvalidType)?;
            }
            self.types[i].n_results = nr.min(MAX_RETURNS);
        }
        self.n_types = count.min(MAX_TYPES);
        Ok(())
    }

    fn parse_function_section(&mut self, r: &mut Reader<'_>) -> WasmResult<()> {
        let count = r.read_u32_leb()? as usize;
        for i in 0..count.min(MAX_FUNCS) {
            self.func_type_idx[i] = r.read_u32_leb()?;
        }
        self.n_funcs = count.min(MAX_FUNCS);
        Ok(())
    }

    fn parse_memory_section(&mut self, r: &mut Reader<'_>) -> WasmResult<()> {
        let count = r.read_u32_leb()?;
        if count > 0 {
            let limits_type = r.read_u8()?;
            self.memory_pages = r.read_u32_leb()?;
            if limits_type == 1 { let _ = r.read_u32_leb()?; } // max
        }
        Ok(())
    }

    fn parse_code_section(&mut self, r: &mut Reader<'_>, base_off: usize) -> WasmResult<()> {
        let count = r.read_u32_leb()? as usize;
        for i in 0..count.min(MAX_FUNCS) {
            let body_size = r.read_u32_leb()? as usize;
            self.code_offsets[i] = base_off + r.pos;
            r.read_bytes(body_size)?;
        }
        Ok(())
    }
}
