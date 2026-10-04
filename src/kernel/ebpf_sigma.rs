#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfProgType {
    SocketFilter,
    Kprobe,
    Tracepoint,
    Xdp,
    PerfEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfMapType {
    HashMap,
    Array,
    ProgArray,
    PerfEventArray,
    RingBuf,
}

pub struct BpfInstruction {
    pub opcode: u8,
    pub dst_reg: u8,
    pub src_reg: u8,
    pub off: i16,
    pub imm: i32,
}

pub struct BpfMap {
    pub map_type: BpfMapType,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub id: usize,
}

static MAP_ID_GEN: AtomicUsize = AtomicUsize::new(1);

impl BpfMap {
    pub fn new(map_type: BpfMapType, key_size: u32, value_size: u32, max_entries: u32) -> Self {
        Self {
            map_type,
            key_size,
            value_size,
            max_entries,
            id: MAP_ID_GEN.fetch_add(1, Ordering::SeqCst),
        }
    }
}

pub struct BpfProgram {
    pub prog_type: BpfProgType,
    pub instructions: Vec<BpfInstruction>,
    pub maps: Vec<BpfMap>,
}

impl BpfProgram {
    pub fn new(prog_type: BpfProgType) -> Self {
        Self {
            prog_type,
            instructions: Vec::new(),
            maps: Vec::new(),
        }
    }
}

pub struct BpfVerifier;

impl BpfVerifier {
    pub fn verify(prog: &BpfProgram) -> Result<(), &'static str> {
        if prog.instructions.is_empty() {
            return Err("Program is empty");
        }
        Ok(())
    }
}

pub trait BpfJit {
    fn compile(&self, prog: &BpfProgram) -> Result<Vec<u8>, &'static str>;
}

pub struct X86_64Jit;

impl BpfJit for X86_64Jit {
    fn compile(&self, _prog: &BpfProgram) -> Result<Vec<u8>, &'static str> {
        Ok(alloc::vec![0x90, 0xc3])
    }
}

pub struct BpfHelpers;

impl BpfHelpers {
    pub fn map_lookup_elem() -> usize { 1 }
    pub fn map_update_elem() -> usize { 2 }
    pub fn map_delete_elem() -> usize { 3 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bpf_map() {
        let map = BpfMap::new(BpfMapType::HashMap, 4, 8, 1024);
        assert_eq!(map.map_type, BpfMapType::HashMap);
        assert_eq!(map.key_size, 4);
        assert_eq!(map.value_size, 8);
        assert_eq!(map.max_entries, 1024);
    }

    #[test]
    fn test_create_bpf_prog() {
        let prog = BpfProgram::new(BpfProgType::Xdp);
        assert_eq!(prog.prog_type, BpfProgType::Xdp);
        assert_eq!(prog.instructions.len(), 0);
    }

    #[test]
    fn test_bpf_verifier_empty() {
        let prog = BpfProgram::new(BpfProgType::Xdp);
        let res = BpfVerifier::verify(&prog);
        assert!(res.is_err());
    }

    #[test]
    fn test_bpf_verifier_ok() {
        let mut prog = BpfProgram::new(BpfProgType::Xdp);
        prog.instructions.push(BpfInstruction { opcode: 0, dst_reg: 0, src_reg: 0, off: 0, imm: 0 });
        let res = BpfVerifier::verify(&prog);
        assert!(res.is_ok());
    }

    #[test]
    fn test_bpf_jit() {
        let prog = BpfProgram::new(BpfProgType::Xdp);
        let jit = X86_64Jit;
        let machine_code = jit.compile(&prog).unwrap();
        assert_eq!(machine_code, alloc::vec![0x90, 0xc3]);
    }
}
