// SigmaOS eBPF-inspired Extended Berkeley Packet Filter
// Inspired by Linux kernel eBPF - safe, efficient kernel-space programming
// Zero-dependency, #![no_std] compliant

use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};

/// eBPF program type classification
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfProgramType {
    SocketFilter = 1,
    Kprobe = 2,
    SchedClassifier = 3,
    Xdp = 4,
    PerfEvent = 5,
    CgroupSock = 6,
    CgroupSockAddr = 7,
    LwtIn = 8,
    LwtOut = 9,
    LwtXmit = 10,
    SocketMap = 11,
    SkMsg = 12,
    RawTracepoint = 13,
    CgroupSockOps = 14,
}

/// eBPF instruction set (64-bit instruction encoding)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BpfInstruction {
    pub opcode: u8,
    pub dst_reg: u8,
    pub src_reg: u8,
    pub offset: i16,
    pub imm: i32,
}

/// eBPF register set (10 general-purpose registers)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BpfRegisters {
    pub r0: u64,  // return value
    pub r1: u64,  // argument 1
    pub r2: u64,  // argument 2
    pub r3: u64,  // argument 3
    pub r4: u64,  // argument 4
    pub r5: u64,  // argument 5
    pub r6: u64,  // callee-saved
    pub r7: u64,  // callee-saved
    pub r8: u64,  // callee-saved
    pub r9: u64,  // callee-saved
    pub r10: u64, // read-only frame pointer
}

/// eBPF virtual machine state
pub struct BpfVm {
    pub registers: BpfRegisters,
    pub program: RefCell<[BpfInstruction; 4096]>, // max 4096 instructions
    pub program_len: AtomicUsize,
    pub stack: RefCell<[u8; 512]>, // 512-byte stack
}

impl BpfVm {
    pub fn new() -> Self {
        BpfVm {
            registers: BpfRegisters {
                r0: 0, r1: 0, r2: 0, r3: 0, r4: 0, r5: 0,
                r6: 0, r7: 0, r8: 0, r9: 0, r10: 512,
            },
            program: RefCell::new([BpfInstruction {
                opcode: 0, dst_reg: 0, src_reg: 0, offset: 0, imm: 0
            }; 4096]),
            program_len: AtomicUsize::new(0),
            stack: RefCell::new([0u8; 512]),
        }
    }

    /// Load eBPF program
    pub fn load_program(&self, instructions: &[BpfInstruction]) -> Result<(), BpfError> {
        if instructions.len() > 4096 {
            return Err(BpfError::ProgramTooLarge);
        }

        let mut prog = self.program.borrow_mut();
        for (i, &inst) in instructions.iter().enumerate() {
            prog[i] = inst;
        }
        self.program_len.store(instructions.len(), Ordering::SeqCst);
        Ok(())
    }

    /// Execute eBPF program
    pub fn execute(&self, packet_data: &[u8]) -> Result<u64, BpfError> {
        let prog_len = self.program_len.load(Ordering::SeqCst);
        let prog = self.program.borrow();
        
        let mut pc: i32 = 0; // program counter
        let mut regs = self.registers;
        
        // Set packet data pointer
        regs.r1 = packet_data.as_ptr() as u64;
        regs.r2 = packet_data.len() as u64;

        while (pc as usize) < prog_len {
            let inst = prog[pc as usize];
            
            match inst.opcode {
                0x95 => { // EXIT
                    return Ok(regs.r0);
                }
                0xb7 => { // MOV64_IMM
                    match inst.dst_reg {
                        0 => regs.r0 = inst.imm as u64,
                        1 => regs.r1 = inst.imm as u64,
                        2 => regs.r2 = inst.imm as u64,
                        3 => regs.r3 = inst.imm as u64,
                        4 => regs.r4 = inst.imm as u64,
                        5 => regs.r5 = inst.imm as u64,
                        6 => regs.r6 = inst.imm as u64,
                        7 => regs.r7 = inst.imm as u64,
                        8 => regs.r8 = inst.imm as u64,
                        9 => regs.r9 = inst.imm as u64,
                        _ => return Err(BpfError::InvalidRegister),
                    }
                }
                0x07 => { // ADD64_IMM
                    match inst.dst_reg {
                        0 => regs.r0 = regs.r0.wrapping_add(inst.imm as u64),
                        1 => regs.r1 = regs.r1.wrapping_add(inst.imm as u64),
                        2 => regs.r2 = regs.r2.wrapping_add(inst.imm as u64),
                        3 => regs.r3 = regs.r3.wrapping_add(inst.imm as u64),
                        4 => regs.r4 = regs.r4.wrapping_add(inst.imm as u64),
                        5 => regs.r5 = regs.r5.wrapping_add(inst.imm as u64),
                        6 => regs.r6 = regs.r6.wrapping_add(inst.imm as u64),
                        7 => regs.r7 = regs.r7.wrapping_add(inst.imm as u64),
                        8 => regs.r8 = regs.r8.wrapping_add(inst.imm as u64),
                        9 => regs.r9 = regs.r9.wrapping_add(inst.imm as u64),
                        _ => return Err(BpfError::InvalidRegister),
                    }
                }
                0x20 => { // LD_ABS_B (load byte from packet)
                    let offset = inst.imm as u32 as usize;
                    if offset >= packet_data.len() {
                        regs.r0 = 0;
                    } else {
                        regs.r0 = packet_data[offset] as u64;
                    }
                }
                0x61 => { // LDXW (load word from memory)
                    let offset = inst.offset as u32 as usize;
                    if offset + 4 <= 512 {
                        let stack = self.stack.borrow();
                        let val = u32::from_le_bytes([
                            stack[offset], stack[offset+1], 
                            stack[offset+2], stack[offset+3]
                        ]);
                        match inst.dst_reg {
                            0 => regs.r0 = val as u64,
                            1 => regs.r1 = val as u64,
                            2 => regs.r2 = val as u64,
                            3 => regs.r3 = val as u64,
                            4 => regs.r4 = val as u64,
                            5 => regs.r5 = val as u64,
                            6 => regs.r6 = val as u64,
                            7 => regs.r7 = val as u64,
                            8 => regs.r8 = val as u64,
                            9 => regs.r9 = val as u64,
                            _ => return Err(BpfError::InvalidRegister),
                        }
                    }
                }
                0x05 => { // JMP_IMM (conditional jump)
                    let cond = match inst.dst_reg {
                        0 => regs.r0,
                        1 => regs.r1,
                        2 => regs.r2,
                        3 => regs.r3,
                        4 => regs.r4,
                        5 => regs.r5,
                        6 => regs.r6,
                        7 => regs.r7,
                        8 => regs.r8,
                        9 => regs.r9,
                        _ => return Err(BpfError::InvalidRegister),
                    };
                    
                    if cond == inst.imm as u64 {
                        pc += inst.offset;
                    }
                }
                _ => return Err(BpfError::InvalidOpcode),
            }
            
            pc += 1;
        }
        
        Ok(regs.r0)
    }
}

/// eBPF error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfError {
    ProgramTooLarge,
    InvalidRegister,
    InvalidOpcode,
    StackOverflow,
    InvalidMemoryAccess,
}

/// eBPF map type (kernel-space data structures)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfMapType {
    Hash = 1,
    Array = 2,
    PerCpuHash = 3,
    PerCpuArray = 4,
    RingBuf = 5,
}

/// eBPF map definition
pub struct BpfMap {
    pub map_type: BpfMapType,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub data: RefCell<[u8; 65536]>, // 64KB max map size
}

impl BpfMap {
    pub fn new(map_type: BpfMapType, key_size: u32, value_size: u32, max_entries: u32) -> Self {
        BpfMap {
            map_type,
            key_size,
            value_size,
            max_entries,
            data: RefCell::new([0u8; 65536]),
        }
    }

    /// Simple hash map lookup
    pub fn lookup(&self, key: &[u8]) -> Option<Vec<u8>> {
        // Simplified hash map implementation
        let data = self.data.borrow();
        let hash = self.simple_hash(key);
        let offset = (hash % self.max_entries as u64) as usize;
        
        if offset + self.value_size as usize <= data.len() {
            let mut value = vec![0u8; self.value_size as usize];
            value.copy_from_slice(&data[offset..offset + self.value_size as usize]);
            Some(value)
        } else {
            None
        }
    }

    /// Simple hash map update
    pub fn update(&self, key: &[u8], value: &[u8]) -> Result<(), BpfError> {
        if value.len() != self.value_size as usize {
            return Err(BpfError::InvalidMemoryAccess);
        }

        let mut data = self.data.borrow_mut();
        let hash = self.simple_hash(key);
        let offset = (hash % self.max_entries as u64) as usize;
        
        if offset + self.value_size as usize <= data.len() {
            data[offset..offset + self.value_size as usize].copy_from_slice(value);
            Ok(())
        } else {
            Err(BpfError::InvalidMemoryAccess)
        }
    }

    fn simple_hash(&self, key: &[u8]) -> u64 {
        let mut hash: u64 = 5381;
        for &byte in key {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpf_vm_simple_program() {
        let vm = BpfVm::new();
        
        // Simple program: return packet length
        let program = [
            BpfInstruction { opcode: 0xb7, dst_reg: 0, src_reg: 0, offset: 0, imm: 0 }, // r0 = 0
            BpfInstruction { opcode: 0x07, dst_reg: 0, src_reg: 0, offset: 0, imm: 10 }, // r0 += 10
            BpfInstruction { opcode: 0x95, dst_reg: 0, src_reg: 0, offset: 0, imm: 0 }, // exit
        ];
        
        vm.load_program(&program).unwrap();
        let result = vm.execute(&[1, 2, 3, 4, 5]).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_bpf_map_hash() {
        let map = BpfMap::new(BpfMapType::Hash, 4, 8, 100);
        
        // Generate test key using timestamp-based approach
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let mut key = [0u8; 4];
        for (i, byte) in key.iter_mut().enumerate() {
            *byte = ((timestamp >> (i * 8)) & 0xFF) as u8;
        }
        let mut value = [0u8; 8];
        let value_timestamp = timestamp.wrapping_add(1);
        for (i, byte) in value.iter_mut().enumerate() {
            *byte = ((value_timestamp >> (i * 8)) & 0xFF) as u8;
        }
        
        map.update(&key, &value).unwrap();
        let retrieved = map.lookup(&key).unwrap();
        assert_eq!(retrieved, value.to_vec());
    }
}
