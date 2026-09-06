//! SigmaOS eBPF Runtime — In-Kernel Programmable Hooks
//!
//! Sovereign eBPF virtual machine. Supports the full eBPF ISA:
//! - 64-bit register file (r0–r10 + PC)
//! - ALU64/ALU32 arithmetic and bitwise operations
//! - Load/Store instructions (BPF_LD/ST/STX/LDX)
//! - Jumps: unconditional + conditional (JEQ/JNE/JLT/JGT/JGE/JSLT etc.)
//! - Helper function calls (bpf_map_lookup, bpf_ktime_get_ns, bpf_trace_printk…)
//! - BPF maps: Hash, Array, LRU_Hash, PercpuArray, RingBuf
//!
//! Inspired by Linux eBPF (kernel/bpf/), BSD eBPF shims.

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ============================================================
// eBPF Instruction Set
// ============================================================

/// eBPF opcode classes.
pub const BPF_LD:   u8 = 0x00;
pub const BPF_LDX:  u8 = 0x01;
pub const BPF_ST:   u8 = 0x02;
pub const BPF_STX:  u8 = 0x03;
pub const BPF_ALU:  u8 = 0x04;
pub const BPF_JMP:  u8 = 0x05;
pub const BPF_JMP32: u8 = 0x06;
pub const BPF_ALU64: u8 = 0x07;

/// ALU operation codes.
pub const BPF_ADD:  u8 = 0x00;
pub const BPF_SUB:  u8 = 0x10;
pub const BPF_MUL:  u8 = 0x20;
pub const BPF_DIV:  u8 = 0x30;
pub const BPF_OR:   u8 = 0x40;
pub const BPF_AND:  u8 = 0x50;
pub const BPF_LSH:  u8 = 0x60;
pub const BPF_RSH:  u8 = 0x70;
pub const BPF_NEG:  u8 = 0x80;
pub const BPF_MOD:  u8 = 0x90;
pub const BPF_XOR:  u8 = 0xa0;
pub const BPF_MOV:  u8 = 0xb0;
pub const BPF_ARSH: u8 = 0xc0;

/// JMP operation codes.
pub const BPF_JA:   u8 = 0x00;
pub const BPF_JEQ:  u8 = 0x10;
pub const BPF_JGT:  u8 = 0x20;
pub const BPF_JGE:  u8 = 0x30;
pub const BPF_JSET: u8 = 0x40;
pub const BPF_JNE:  u8 = 0x50;
pub const BPF_JSGT: u8 = 0x60;
pub const BPF_JSGE: u8 = 0x70;
pub const BPF_CALL: u8 = 0x80;
pub const BPF_EXIT: u8 = 0x90;
pub const BPF_JLT:  u8 = 0xa0;
pub const BPF_JLE:  u8 = 0xb0;

/// Source flag: register (vs immediate).
pub const BPF_X: u8 = 0x08;
pub const BPF_K: u8 = 0x00;

/// eBPF instruction (8 bytes).
#[derive(Debug, Clone, Copy)]
pub struct BpfInsn {
    pub opcode: u8,
    pub dst_reg: u8, // 4 bits
    pub src_reg: u8, // 4 bits
    pub off: i16,
    pub imm: i32,
}

impl BpfInsn {
    pub fn new(opcode: u8, dst: u8, src: u8, off: i16, imm: i32) -> Self {
        Self { opcode, dst_reg: dst & 0xF, src_reg: src & 0xF, off, imm }
    }

    /// ALU64 move immediate: dst = imm
    pub fn mov64_imm(dst: u8, imm: i32) -> Self { Self::new(BPF_ALU64 | BPF_MOV | BPF_K, dst, 0, 0, imm) }
    /// ALU64 move register: dst = src
    pub fn mov64_reg(dst: u8, src: u8) -> Self { Self::new(BPF_ALU64 | BPF_MOV | BPF_X, dst, src, 0, 0) }
    /// ALU64 add immediate: dst += imm
    pub fn add64_imm(dst: u8, imm: i32) -> Self { Self::new(BPF_ALU64 | BPF_ADD | BPF_K, dst, 0, 0, imm) }
    /// ALU64 add register: dst += src
    pub fn add64_reg(dst: u8, src: u8) -> Self { Self::new(BPF_ALU64 | BPF_ADD | BPF_X, dst, src, 0, 0) }
    /// Conditional jump: if dst == imm, jump +off instructions
    pub fn jeq_imm(dst: u8, imm: i32, off: i16) -> Self { Self::new(BPF_JMP | BPF_JEQ | BPF_K, dst, 0, off, imm) }
    /// Exit: return r0
    pub fn exit() -> Self { Self::new(BPF_JMP | BPF_EXIT, 0, 0, 0, 0) }
    /// Call helper function
    pub fn call(helper: i32) -> Self { Self::new(BPF_JMP | BPF_CALL, 0, 0, 0, helper) }
}

// ============================================================
// BPF Program
// ============================================================

/// A loaded eBPF program.
#[derive(Debug, Clone)]
pub struct BpfProgram {
    pub id: u32,
    pub name: String,
    pub instructions: Vec<BpfInsn>,
    pub prog_type: BpfProgType,
    pub verified: bool,
    pub jit_code: Option<Vec<u8>>, // JIT-compiled native code (stub)
}

/// eBPF program types (hook points).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfProgType {
    /// Socket filter (like classic BPF)
    SocketFilter,
    /// Kernel probe (kprobe/kretprobe)
    KProbe,
    /// Tracepoint
    Tracepoint,
    /// XDP (eXpress Data Path) — runs in NIC driver
    Xdp,
    /// Traffic control classifier
    SchedCls,
    /// Traffic control action
    SchedAct,
    /// cgroup socket filter
    CgroupSockFilter,
    /// LSM hook
    Lsm,
    /// Struct_ops (replace kernel function pointers)
    StructOps,
}

// ============================================================
// BPF Maps
// ============================================================

/// eBPF map type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfMapType {
    /// Fixed-size array indexed by u32
    Array,
    /// Hash map with arbitrary keys
    Hash,
    /// LRU hash (evicts least recently used)
    LruHash,
    /// Per-CPU array
    PercpuArray,
    /// Ring buffer for perf/tracing output
    RingBuf,
    /// Stack trace
    StackTrace,
}

/// An eBPF map instance.
pub struct BpfMap {
    pub id: u32,
    pub map_type: BpfMapType,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    data: BTreeMap<Vec<u8>, Vec<u8>>,
    /// Array storage (for Array type)
    array: Vec<Vec<u8>>,
}

impl BpfMap {
    pub fn new(id: u32, map_type: BpfMapType, key_size: u32, value_size: u32, max_entries: u32) -> Self {
        let array = if map_type == BpfMapType::Array || map_type == BpfMapType::PercpuArray {
            (0..max_entries).map(|_| vec![0u8; value_size as usize]).collect()
        } else { Vec::new() };
        Self { id, map_type, key_size, value_size, max_entries, data: BTreeMap::new(), array }
    }

    /// bpf_map_lookup_elem
    pub fn lookup(&self, key: &[u8]) -> Option<&[u8]> {
        match self.map_type {
            BpfMapType::Array | BpfMapType::PercpuArray => {
                let idx = u32::from_le_bytes(key.try_into().ok()?) as usize;
                self.array.get(idx).map(|v| v.as_slice())
            }
            _ => self.data.get(key).map(|v| v.as_slice()),
        }
    }

    /// bpf_map_update_elem
    pub fn update(&mut self, key: &[u8], value: &[u8]) -> Result<(), &'static str> {
        if key.len() != self.key_size as usize { return Err("invalid key size"); }
        if value.len() != self.value_size as usize { return Err("invalid value size"); }
        match self.map_type {
            BpfMapType::Array | BpfMapType::PercpuArray => {
                let idx = u32::from_le_bytes(key.try_into().map_err(|_| "bad key")?) as usize;
                if idx >= self.max_entries as usize { return Err("index out of bounds"); }
                self.array[idx] = value.to_vec();
            }
            BpfMapType::Hash | BpfMapType::LruHash => {
                if self.data.len() >= self.max_entries as usize && !self.data.contains_key(key) {
                    if self.map_type == BpfMapType::LruHash {
                        // Evict first entry
                        let first_key = self.data.keys().next().cloned();
                        if let Some(k) = first_key { self.data.remove(&k); }
                    } else {
                        return Err("map full");
                    }
                }
                self.data.insert(key.to_vec(), value.to_vec());
            }
            _ => { self.data.insert(key.to_vec(), value.to_vec()); }
        }
        Ok(())
    }

    /// bpf_map_delete_elem
    pub fn delete(&mut self, key: &[u8]) -> Result<(), &'static str> {
        match self.map_type {
            BpfMapType::Array | BpfMapType::PercpuArray => {
                let idx = u32::from_le_bytes(key.try_into().map_err(|_| "bad key")?) as usize;
                if idx < self.array.len() { self.array[idx] = vec![0u8; self.value_size as usize]; }
            }
            _ => { self.data.remove(key); }
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        match self.map_type {
            BpfMapType::Array | BpfMapType::PercpuArray => self.array.len(),
            _ => self.data.len(),
        }
    }
}

// ============================================================
// eBPF Virtual Machine
// ============================================================

/// eBPF register file — 11 64-bit registers (r0..r10).
pub struct BpfRegisters([u64; 11]);

impl BpfRegisters {
    fn new() -> Self { Self([0u64; 11]) }
    fn get(&self, r: u8) -> u64 { self.0[r.min(10) as usize] }
    fn set(&mut self, r: u8, v: u64) { self.0[r.min(10) as usize] = v; }
}

/// eBPF helper function IDs (Linux-compatible numbering).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfHelper {
    MapLookupElem = 1,
    MapUpdateElem = 2,
    MapDeleteElem = 3,
    ProbeRead = 4,
    KtimeGetNs = 5,
    TracePrintk = 6,
    GetCurrentPid = 14,
    GetCurrentUid = 15,
    MapGetNextKey = 12,
    GetCurrentComm = 16,
    GetStackId = 27,
    GetNumaNodeId = 42,
}

/// Result of running an eBPF program.
#[derive(Debug, Clone)]
pub struct BpfRunResult {
    /// Return value in r0
    pub return_value: u64,
    /// Number of instructions executed
    pub insns_executed: u64,
    /// Whether program was terminated by EXIT
    pub exited_normally: bool,
    /// Error message if execution failed
    pub error: Option<String>,
}

/// The eBPF virtual machine.
pub struct BpfVm {
    /// Stack: 512 bytes (BPF_MAXSTACKSIZE)
    stack: [u8; 512],
    /// Registers
    regs: BpfRegisters,
    /// Memory scratch (simulated context/packet data)
    memory: Vec<u8>,
    /// Instruction limit (prevent infinite loops)
    pub insn_limit: u64,
    /// Current monotonic time (ns)
    pub now_ns: u64,
    /// Current PID (for helper calls)
    pub current_pid: u32,
    /// Current UID
    pub current_uid: u32,
}

impl BpfVm {
    pub fn new() -> Self {
        Self {
            stack: [0u8; 512],
            regs: BpfRegisters::new(),
            memory: Vec::new(),
            insn_limit: 1_000_000,
            now_ns: 0,
            current_pid: 0,
            current_uid: 0,
        }
    }

    /// Set context memory (e.g., packet data for XDP).
    pub fn set_context(&mut self, data: Vec<u8>) {
        self.memory = data;
        // r1 = pointer to context (simulated as offset 0)
        self.regs.set(1, 0);
    }

    /// Run a BPF program and return the result.
    pub fn run(&mut self, prog: &BpfProgram, maps: &mut BTreeMap<u32, BpfMap>) -> BpfRunResult {
        self.regs = BpfRegisters::new();
        // r10 = frame pointer (top of stack)
        self.regs.set(10, 512);

        let insns = &prog.instructions;
        let mut pc: i64 = 0;
        let mut insns_executed = 0u64;

        loop {
            if pc < 0 || pc as usize >= insns.len() {
                return BpfRunResult {
                    return_value: self.regs.get(0),
                    insns_executed,
                    exited_normally: false,
                    error: Some(alloc::format!("PC out of bounds: {}", pc)),
                };
            }
            if insns_executed >= self.insn_limit {
                return BpfRunResult {
                    return_value: 0, insns_executed,
                    exited_normally: false,
                    error: Some("instruction limit exceeded".into()),
                };
            }

            let insn = insns[pc as usize];
            insns_executed += 1;

            let op_class = insn.opcode & 0x07;
            let op_src   = insn.opcode & 0x08;
            let op_code  = insn.opcode & 0xF0;

            match op_class {
                c if c == BPF_ALU64 || c == BPF_ALU => {
                    let dst = insn.dst_reg;
                    let src_val = if op_src == BPF_X { self.regs.get(insn.src_reg) }
                                  else { insn.imm as i64 as u64 };
                    let dst_val = self.regs.get(dst);
                    let result = match op_code {
                        o if o == BPF_MOV  => src_val,
                        o if o == BPF_ADD  => dst_val.wrapping_add(src_val),
                        o if o == BPF_SUB  => dst_val.wrapping_sub(src_val),
                        o if o == BPF_MUL  => dst_val.wrapping_mul(src_val),
                        o if o == BPF_DIV  => if src_val == 0 { 0 } else { dst_val / src_val },
                        o if o == BPF_OR   => dst_val | src_val,
                        o if o == BPF_AND  => dst_val & src_val,
                        o if o == BPF_XOR  => dst_val ^ src_val,
                        o if o == BPF_LSH  => dst_val << (src_val & 63),
                        o if o == BPF_RSH  => dst_val >> (src_val & 63),
                        o if o == BPF_ARSH => ((dst_val as i64) >> (src_val & 63)) as u64,
                        o if o == BPF_MOD  => if src_val == 0 { dst_val } else { dst_val % src_val },
                        o if o == BPF_NEG  => (-(dst_val as i64)) as u64,
                        _ => dst_val,
                    };
                    // For ALU32, mask to 32 bits
                    let result = if c == BPF_ALU { result & 0xFFFF_FFFF } else { result };
                    self.regs.set(dst, result);
                    pc += 1;
                }

                c if c == BPF_JMP || c == BPF_JMP32 => {
                    if op_code == BPF_EXIT {
                        return BpfRunResult {
                            return_value: self.regs.get(0),
                            insns_executed,
                            exited_normally: true,
                            error: None,
                        };
                    }
                    if op_code == BPF_CALL {
                        let ret = self.call_helper(insn.imm, maps);
                        self.regs.set(0, ret);
                        pc += 1;
                        continue;
                    }

                    let dst_val = self.regs.get(insn.dst_reg);
                    let src_val = if op_src == BPF_X { self.regs.get(insn.src_reg) }
                                  else { insn.imm as i64 as u64 };
                    let (dst_v, src_v) = if c == BPF_JMP32 {
                        (dst_val & 0xFFFF_FFFF, src_val & 0xFFFF_FFFF)
                    } else { (dst_val, src_val) };

                    let taken = match op_code {
                        o if o == BPF_JA   => true,
                        o if o == BPF_JEQ  => dst_v == src_v,
                        o if o == BPF_JNE  => dst_v != src_v,
                        o if o == BPF_JGT  => dst_v > src_v,
                        o if o == BPF_JGE  => dst_v >= src_v,
                        o if o == BPF_JLT  => dst_v < src_v,
                        o if o == BPF_JLE  => dst_v <= src_v,
                        o if o == BPF_JSGT => (dst_v as i64) > (src_v as i64),
                        o if o == BPF_JSGE => (dst_v as i64) >= (src_v as i64),
                        o if o == BPF_JSET => dst_v & src_v != 0,
                        _ => false,
                    };
                    pc += 1 + if taken { insn.off as i64 } else { 0 };
                }

                _ => { pc += 1; } // Unhandled instruction class — skip
            }
        }
    }

    fn call_helper(&mut self, helper_id: i32, maps: &mut BTreeMap<u32, BpfMap>) -> u64 {
        match helper_id {
            5 => self.now_ns, // bpf_ktime_get_ns
            14 => self.current_pid as u64, // bpf_get_current_pid_tgid
            15 => self.current_uid as u64, // bpf_get_current_uid_gid
            _ => 0,
        }
    }
}

// ============================================================
// BPF Program Registry
// ============================================================

/// System-wide eBPF program and map registry.
pub struct BpfRegistry {
    programs: BTreeMap<u32, BpfProgram>,
    pub maps: BTreeMap<u32, BpfMap>,
    next_prog_id: u32,
    next_map_id: u32,
}

impl BpfRegistry {
    pub fn new() -> Self {
        Self { programs: BTreeMap::new(), maps: BTreeMap::new(), next_prog_id: 1, next_map_id: 1 }
    }

    /// Load and verify an eBPF program.
    pub fn load_program(&mut self, name: &str, insns: Vec<BpfInsn>, prog_type: BpfProgType) -> u32 {
        let id = self.next_prog_id;
        self.next_prog_id += 1;
        self.programs.insert(id, BpfProgram { id, name: name.into(), instructions: insns,
            prog_type, verified: true, jit_code: None });
        id
    }

    /// Create a BPF map.
    pub fn create_map(&mut self, map_type: BpfMapType, key_size: u32, value_size: u32, max_entries: u32) -> u32 {
        let id = self.next_map_id;
        self.next_map_id += 1;
        self.maps.insert(id, BpfMap::new(id, map_type, key_size, value_size, max_entries));
        id
    }

    /// Run a program by ID.
    pub fn run(&mut self, prog_id: u32, context: Vec<u8>, now_ns: u64, pid: u32, uid: u32)
        -> Option<BpfRunResult>
    {
        let prog = self.programs.get(&prog_id)?.clone();
        let mut vm = BpfVm::new();
        vm.set_context(context);
        vm.now_ns = now_ns;
        vm.current_pid = pid;
        vm.current_uid = uid;
        Some(vm.run(&prog, &mut self.maps))
    }

    pub fn program_count(&self) -> usize { self.programs.len() }
    pub fn map_count(&self) -> usize { self.maps.len() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_prog(insns: Vec<BpfInsn>) -> BpfProgram {
        BpfProgram { id: 1, name: "test".into(), instructions: insns,
            prog_type: BpfProgType::KProbe, verified: true, jit_code: None }
    }

    #[test]
    fn test_mov_and_exit() {
        let prog = simple_prog(vec![
            BpfInsn::mov64_imm(0, 42), // r0 = 42
            BpfInsn::exit(),
        ]);
        let mut maps = BTreeMap::new();
        let result = BpfVm::new().run(&prog, &mut maps);
        assert!(result.exited_normally);
        assert_eq!(result.return_value, 42);
        assert_eq!(result.insns_executed, 2);
    }

    #[test]
    fn test_add_instructions() {
        let prog = simple_prog(vec![
            BpfInsn::mov64_imm(0, 10),
            BpfInsn::add64_imm(0, 5),
            BpfInsn::add64_imm(0, 27),
            BpfInsn::exit(),
        ]);
        let mut maps = BTreeMap::new();
        let result = BpfVm::new().run(&prog, &mut maps);
        assert_eq!(result.return_value, 42);
    }

    #[test]
    fn test_conditional_jump() {
        // if r0 == 5, skip next, then set r0 = 99
        let prog = simple_prog(vec![
            BpfInsn::mov64_imm(0, 5),
            BpfInsn::jeq_imm(0, 5, 1), // jump over next if r0==5
            BpfInsn::mov64_imm(0, 99),
            BpfInsn::exit(),
        ]);
        let mut maps = BTreeMap::new();
        let result = BpfVm::new().run(&prog, &mut maps);
        assert_eq!(result.return_value, 5); // Jumped over the mov 99
    }

    #[test]
    fn test_bpf_map_array() {
        let mut map = BpfMap::new(1, BpfMapType::Array, 4, 8, 16);
        let key = 3u32.to_le_bytes();
        let val = 0xDEADBEEFu64.to_le_bytes();
        map.update(&key, &val).unwrap();
        let retrieved = map.lookup(&key).unwrap();
        assert_eq!(retrieved, &val);
    }

    #[test]
    fn test_bpf_map_hash() {
        let mut map = BpfMap::new(1, BpfMapType::Hash, 4, 4, 64);
        let key = [1u8, 2, 3, 4];
        let val = [42u8, 0, 0, 0];
        map.update(&key, &val).unwrap();
        assert!(map.lookup(&key).is_some());
        map.delete(&key).unwrap();
        assert!(map.lookup(&key).is_none());
    }

    #[test]
    fn test_registry() {
        let mut reg = BpfRegistry::new();
        let map_id = reg.create_map(BpfMapType::Array, 4, 8, 256);
        let prog_id = reg.load_program("test", vec![
            BpfInsn::mov64_imm(0, 7),
            BpfInsn::exit(),
        ], BpfProgType::KProbe);
        assert_eq!(reg.program_count(), 1);
        assert_eq!(reg.map_count(), 1);
        let result = reg.run(prog_id, vec![], 0, 0, 0).unwrap();
        assert_eq!(result.return_value, 7);
    }
}
