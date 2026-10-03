//! eBPF (Extended Berkeley Packet Filter) and XDP (eXpress Data Path)
//! Programmable kernel packet filtering inspired by Linux eBPF subsystem
//! Reference: Linux kernel/bpf/ and net/core/filter.c

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

/// eBPF instruction format (inspired by Linux struct bpf_insn)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EbpfInsn {
    pub opcode: u8,  // Instruction opcode
    pub dst_reg: u8, // Destination register (0-10)
    pub src_reg: u8, // Source register (0-10)
    pub off: i16,    // Signed offset
    pub imm: i32,    // Immediate value
}

/// eBPF register set (11 registers: R0-R10)
#[derive(Debug, Clone)]
pub struct EbpfRegisters {
    pub r: [u64; 11], // R0-R10 (R10 = frame pointer)
}

impl EbpfRegisters {
    pub fn new() -> Self {
        Self { r: [0; 11] }
    }
}

/// eBPF program types (inspired by Linux enum bpf_prog_type)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EbpfProgType {
    SocketFilter = 1,
    Kprobe = 2,
    SchedCls = 3,
    SchedAct = 4,
    Tracepoint = 5,
    Xdp = 6, // XDP packet processing
    PerfEvent = 7,
    CgroupSkb = 8,
    CgroupSock = 9,
    LwtIn = 10,
    LwtOut = 11,
    LwtXmit = 12,
    SockOps = 13,
}

/// XDP action codes (inspired by Linux enum xdp_action)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpAction {
    Aborted = 0,  // Drop with trace
    Drop = 1,     // Drop packet
    Pass = 2,     // Pass to network stack
    Tx = 3,       // Transmit from same interface
    Redirect = 4, // Redirect to another interface
}

/// eBPF map types (inspired by Linux enum bpf_map_type)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EbpfMapType {
    Hash = 1,
    Array = 2,
    ProgArray = 3,
    PerfEventArray = 4,
    PercpuHash = 5,
    PercpuArray = 6,
    StackTrace = 7,
    CgroupArray = 8,
    LruHash = 9,
    LruPercpuHash = 10,
}

/// eBPF map structure
#[derive(Debug)]
pub struct EbpfMap {
    pub map_type: EbpfMapType,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl EbpfMap {
    pub fn new(map_type: EbpfMapType, key_size: u32, value_size: u32, max_entries: u32) -> Self {
        Self {
            map_type,
            key_size,
            value_size,
            max_entries,
            data: BTreeMap::new(),
        }
    }

    /// Lookup element in map
    pub fn lookup(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.data.get(key)
    }

    /// Update element in map
    pub fn update(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), EbpfError> {
        if self.data.len() >= self.max_entries as usize {
            return Err(EbpfError::MapFull);
        }
        self.data.insert(key, value);
        Ok(())
    }

    /// Delete element from map
    pub fn delete(&mut self, key: &[u8]) -> Result<(), EbpfError> {
        self.data.remove(key).ok_or(EbpfError::KeyNotFound)?;
        Ok(())
    }
}

/// eBPF program structure
pub struct EbpfProgram {
    pub prog_type: EbpfProgType,
    pub instructions: Vec<EbpfInsn>,
    pub verified: bool,
    pub jit_compiled: bool,
    pub maps: Vec<u32>, // Map file descriptors
}

impl EbpfProgram {
    pub fn new(prog_type: EbpfProgType, instructions: Vec<EbpfInsn>) -> Self {
        Self {
            prog_type,
            instructions,
            verified: false,
            jit_compiled: false,
            maps: Vec::new(),
        }
    }

    /// Verify eBPF program (safety checks)
    pub fn verify(&mut self) -> Result<(), EbpfError> {
        // Check instruction count limit (Linux: 1 million instructions)
        if self.instructions.len() > 1_000_000 {
            return Err(EbpfError::ProgramTooLarge);
        }

        // Verify all instructions are valid
        for insn in &self.instructions {
            if insn.dst_reg > 10 || insn.src_reg > 10 {
                return Err(EbpfError::InvalidRegister);
            }
        }

        // Verify no infinite loops (requires control flow analysis)
        // In real implementation: build CFG and check for back edges

        self.verified = true;
        Ok(())
    }

    /// JIT compile program (stub - real implementation uses platform-specific JIT)
    pub fn jit_compile(&mut self) -> Result<(), EbpfError> {
        if !self.verified {
            return Err(EbpfError::NotVerified);
        }
        // In real implementation: translate eBPF bytecode to native code
        self.jit_compiled = true;
        Ok(())
    }
}

/// XDP program context (packet metadata)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XdpMd {
    pub data: u64,            // Packet data start pointer
    pub data_end: u64,        // Packet data end pointer
    pub data_meta: u64,       // Metadata area
    pub ingress_ifindex: u32, // Ingress interface index
    pub rx_queue_index: u32,  // RX queue index
}

/// eBPF virtual machine (interpreter)
pub struct EbpfVm {
    regs: EbpfRegisters,
    stack: [u8; 512], // eBPF stack (512 bytes)
    maps: BTreeMap<u32, EbpfMap>,
    programs: BTreeMap<u32, EbpfProgram>,
    next_fd: AtomicU64,
}

impl EbpfVm {
    pub fn new() -> Self {
        Self {
            regs: EbpfRegisters::new(),
            stack: [0; 512],
            maps: BTreeMap::new(),
            programs: BTreeMap::new(),
            next_fd: AtomicU64::new(1),
        }
    }

    /// Load eBPF program
    pub fn load_program(&mut self, prog: EbpfProgram) -> Result<u32, EbpfError> {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst) as u32;
        self.programs.insert(fd, prog);
        Ok(fd)
    }

    /// Create eBPF map
    pub fn create_map(&mut self, map: EbpfMap) -> Result<u32, EbpfError> {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst) as u32;
        self.maps.insert(fd, map);
        Ok(fd)
    }

    /// Run eBPF program in interpreter mode
    pub fn run(&mut self, prog_fd: u32, ctx: &XdpMd) -> Result<XdpAction, EbpfError> {
        let prog = self
            .programs
            .get(&prog_fd)
            .ok_or(EbpfError::InvalidProgram)?;

        if !prog.verified {
            return Err(EbpfError::NotVerified);
        }

        // Initialize R1 with context pointer
        self.regs.r[1] = ctx as *const XdpMd as u64;

        // Execute instructions
        let mut pc = 0usize;
        while pc < prog.instructions.len() {
            let insn = &prog.instructions[pc];

            match insn.opcode {
                // ALU operations
                0x04 => self.regs.r[insn.dst_reg as usize] += insn.imm as u64, // ADD_IMM
                0x05 => self.regs.r[insn.dst_reg as usize] += self.regs.r[insn.src_reg as usize], // ADD_REG
                0x14 => self.regs.r[insn.dst_reg as usize] -= insn.imm as u64, // SUB_IMM
                0x24 => self.regs.r[insn.dst_reg as usize] *= insn.imm as u64, // MUL_IMM
                0x54 => self.regs.r[insn.dst_reg as usize] &= insn.imm as u64, // AND_IMM
                0x44 => self.regs.r[insn.dst_reg as usize] |= insn.imm as u64, // OR_IMM
                0x64 => self.regs.r[insn.dst_reg as usize] <<= insn.imm,       // LSH_IMM
                0x74 => self.regs.r[insn.dst_reg as usize] >>= insn.imm,       // RSH_IMM

                // Load/Store operations
                0x18 => {
                    // LD_IMM64 (double-wide instruction)
                    self.regs.r[insn.dst_reg as usize] = insn.imm as u64;
                    pc += 1; // Skip next instruction (upper 32 bits)
                }

                // Jump operations
                0x05 => pc = (pc as i32 + insn.off as i32) as usize, // JA (unconditional jump)
                0x15 => {
                    // JEQ_IMM
                    if self.regs.r[insn.dst_reg as usize] == insn.imm as u64 {
                        pc = (pc as i32 + insn.off as i32) as usize;
                    }
                }

                // Exit
                0x95 => {
                    // Return value in R0
                    let ret = self.regs.r[0] as u32;
                    return Ok(match ret {
                        0 => XdpAction::Aborted,
                        1 => XdpAction::Drop,
                        2 => XdpAction::Pass,
                        3 => XdpAction::Tx,
                        4 => XdpAction::Redirect,
                        _ => XdpAction::Aborted,
                    });
                }

                _ => return Err(EbpfError::InvalidOpcode),
            }

            pc += 1;
        }

        Ok(XdpAction::Pass)
    }

    /// Attach XDP program to network interface
    pub fn attach_xdp(&mut self, prog_fd: u32, ifindex: u32) -> Result<(), EbpfError> {
        let prog = self
            .programs
            .get(&prog_fd)
            .ok_or(EbpfError::InvalidProgram)?;

        if prog.prog_type != EbpfProgType::Xdp {
            return Err(EbpfError::InvalidProgType);
        }

        // In real implementation: attach to network interface driver
        Ok(())
    }
}

/// eBPF helper function IDs (subset from Linux)
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum EbpfHelper {
    MapLookupElem = 1,
    MapUpdateElem = 2,
    MapDeleteElem = 3,
    ProbeRead = 4,
    KtimeGetNs = 5,
    TracePrintk = 6,
    GetCurrentPidTgid = 14,
    GetCurrentUidGid = 15,
    XdpAdjustHead = 44,
    XdpAdjustMeta = 54,
    XdpAdjustTail = 65,
}

/// eBPF error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EbpfError {
    InvalidOpcode,
    InvalidRegister,
    InvalidProgram,
    InvalidProgType,
    ProgramTooLarge,
    NotVerified,
    MapFull,
    KeyNotFound,
    VerificationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebpf_vm_create() {
        let vm = EbpfVm::new();
        assert_eq!(vm.regs.r[0], 0);
    }

    #[test]
    fn test_ebpf_map_operations() {
        let mut map = EbpfMap::new(EbpfMapType::Hash, 4, 4, 1024);
        map.update(vec![1, 2, 3, 4], vec![5, 6, 7, 8]).unwrap();
        assert_eq!(map.lookup(&[1, 2, 3, 4]), Some(&vec![5, 6, 7, 8]));
    }

    #[test]
    fn test_ebpf_program_verify() {
        let instructions = vec![
            EbpfInsn {
                opcode: 0x18,
                dst_reg: 0,
                src_reg: 0,
                off: 0,
                imm: 2,
            }, // Load 2 into R0
            EbpfInsn {
                opcode: 0x95,
                dst_reg: 0,
                src_reg: 0,
                off: 0,
                imm: 0,
            }, // Exit
        ];
        let mut prog = EbpfProgram::new(EbpfProgType::Xdp, instructions);
        assert!(prog.verify().is_ok());
        assert!(prog.verified);
    }
}
