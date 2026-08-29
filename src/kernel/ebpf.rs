//! eBPF (Extended Berkeley Packet Filter) Virtual Machine, Hook Engine, & Map Registry
//! Provides safe, sandboxed bytecode execution and Linux-parity State-Sharing Maps inside microkernel hooks.
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]


// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

// =========================================================================
// EBPF INSTRUCTION DECODER & OPCODES
// =========================================================================

pub const BPF_LD: u8 = 0x00;
pub const BPF_ALU: u8 = 0x07;
pub const BPF_JMP: u8 = 0x05;

pub const BPF_ADD: u8 = 0x00;
pub const BPF_SUB: u8 = 0x10;
pub const BPF_MUL: u8 = 0x20;
pub const BPF_XOR: u8 = 0xa0;

#[derive(Debug, Clone, Copy)]
pub struct EbpfInstruction {
    pub opcode: u8,
    pub dst_reg: u8,
    pub src_reg: u8,
    pub offset: i16,
    pub imm: i32,
}

// =========================================================================
// EBPF MAP REGISTRY (Linux Parity State-Sharing)
// =========================================================================

/// eBPF Map Types (Linux Parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EbpfMapType {
    Hash,
    Array,
    RingBuffer,
}

/// Represents an individual eBPF Map (Linux Parity)
#[derive(Debug, Clone)]
pub struct EbpfMap {
    pub map_id: usize,
    pub map_type: EbpfMapType,
    pub key_size: usize,
    pub value_size: usize,
    pub max_entries: usize,
    pub storage: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl EbpfMap {
    pub fn new(
        id: usize,
        map_type: EbpfMapType,
        key_size: usize,
        value_size: usize,
        max_entries: usize,
    ) -> Self {
        Self {
            map_id: id,
            map_type,
            key_size,
            value_size,
            max_entries,
            storage: BTreeMap::new(),
        }
    }

    pub fn lookup_elem(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.storage.get(key)
    }

    pub fn update_elem(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), &'static str> {
        if key.len() != self.key_size || value.len() != self.value_size {
            return Err("eBPF Map: Element size mismatch constraint");
        }
        if self.storage.len() >= self.max_entries && !self.storage.contains_key(&key) {
            return Err("eBPF Map: Maximum entry bounds exceeded");
        }
        self.storage.insert(key, value);
        Ok(())
    }

    pub fn delete_elem(&mut self, key: &[u8]) -> Result<(), &'static str> {
        self.storage
            .remove(key)
            .ok_or("eBPF Map: Element not found")
            .map(|_| ())
    }
}

/// Global eBPF Map Registry (Linux Parity)
pub struct EbpfMapRegistry {
    pub maps: BTreeMap<usize, EbpfMap>,
}

impl EbpfMapRegistry {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            maps: BTreeMap::new(),
        }
    }

    pub fn create_map(
        &mut self,
        id: usize,
        map_type: EbpfMapType,
        key_size: usize,
        value_size: usize,
        max_entries: usize,
    ) -> Result<(), &'static str> {
        if self.maps.contains_key(&id) {
            return Err("eBPF Map: Map ID already registered");
        }
        let map = EbpfMap::new(id, map_type, key_size, value_size, max_entries);
        self.maps.insert(id, map);
        Ok(())
    }

    pub fn get_map(&self, id: usize) -> Option<&EbpfMap> {
        self.maps.get(&id)
    }

    pub fn get_map_mut(&mut self, id: usize) -> Option<&mut EbpfMap> {
        self.maps.get_mut(&id)
    }
}

impl Default for EbpfMapRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// EBPF VIRTUAL MACHINE EXECUTION
// =========================================================================

pub struct EbpfVm {
    pub registers: [u64; 11], // R0 (return) to R10 (frame pointer)
    pub bytecode: Vec<EbpfInstruction>,
}

impl EbpfVm {
    pub fn new(bytecode: Vec<EbpfInstruction>) -> Self {
        Self {
            registers: [0u64; 11],
            bytecode,
        }
    }

    /// Executes compiled eBPF instructions inside a strict, safe sandbox environment
    pub fn run(&mut self, context_buffer: &[u8]) -> Result<u64, &'static str> {
        // R1 holds the pointer to the input context buffer
        self.registers[1] = context_buffer.as_ptr() as u64;
        self.registers[10] = 512; // Simulated Stack Frame Pointer

        let mut pc = 0;
        while pc < self.bytecode.len() {
            let inst = self.bytecode[pc];
            let class = inst.opcode & 0x07;

            match class {
                BPF_ALU => {
                    let op = inst.opcode & 0xf0;
                    let dst = inst.dst_reg as usize;
                    let src = inst.src_reg as usize;

                    if dst >= 10 {
                        return Err("eBPF: Access violation - destination register out of bounds");
                    }

                    match op {
                        BPF_ADD => {
                            if inst.opcode & 0x08 == 0 {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_add(inst.imm as u64);
                            } else {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_add(self.registers[src]);
                            }
                        }
                        BPF_SUB => {
                            if inst.opcode & 0x08 == 0 {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_sub(inst.imm as u64);
                            } else {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_sub(self.registers[src]);
                            }
                        }
                        BPF_MUL => {
                            if inst.opcode & 0x08 == 0 {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_mul(inst.imm as u64);
                            } else {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_mul(self.registers[src]);
                            }
                        }
                        BPF_XOR => {
                            if inst.opcode & 0x08 == 0 {
                                self.registers[dst] ^= inst.imm as u64;
                            } else {
                                self.registers[dst] ^= self.registers[src];
                            }
                        }
                        _ => return Err("eBPF VM: Unknown ALU operation opcode"),
                    }
                }
                BPF_JMP => {
                    let dst = inst.dst_reg as usize;
                    let imm = inst.imm as u64;

                    if dst >= 10 {
                        return Err("eBPF: Access violation - JMP evaluation out of bounds");
                    }

                    // Jump instruction: if Register[dst] matches immediate, jump by instruction offset
                    if self.registers[dst] == imm {
                        let new_pc = (pc as i32 + inst.offset as i32) as usize;
                        if new_pc >= self.bytecode.len() {
                            return Err("eBPF VM: Jump out of instruction segment bounds");
                        }
                        pc = new_pc;
                        continue;
                    }
                }
                BPF_LD => {
                    let dst = inst.dst_reg as usize;
                    if dst >= 10 {
                        return Err("eBPF: Access violation - Load target out of bounds");
                    }
                    self.registers[dst] = inst.imm as u64;
                }
                _ => return Err("eBPF VM: Unknown instruction class"),
            }
            pc += 1;
        }

        // Return register is R0
        Ok(self.registers[0])
    }
}

// =========================================================================
// IN-KERNEL TRACEPROBES & PERF EVENT RING BUFFER (DTrace/ftrace Style)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProbeType {
    Kprobe,
    Kretprobe,
    Tracepoint,
    PerfEvent,
}

#[derive(Debug, Clone, Copy)]
pub struct PerfEvent {
    pub timestamp_nanos: u64,
    pub cpu_id: u32,
    pub pid: u32,
    pub probe_type: ProbeType,
    pub sample_value: u64,
}

pub struct PerfEventRingBuffer {
    events: Vec<PerfEvent>,
    max_capacity: usize,
}

impl PerfEventRingBuffer {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            events: Vec::new(),
            max_capacity,
        }
    }

    pub fn push_event(&mut self, event: PerfEvent) {
        if self.events.len() >= self.max_capacity {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn pop_event(&mut self) -> Option<PerfEvent> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

pub struct TraceprobeManager {
    probes: BTreeMap<ProbeType, EbpfVm>,
    pub perf_ring_buffer: PerfEventRingBuffer,
}

impl TraceprobeManager {
    pub fn new(ring_buffer_capacity: usize) -> Self {
        Self {
            probes: BTreeMap::new(),
            perf_ring_buffer: PerfEventRingBuffer::new(ring_buffer_capacity),
        }
    }

    pub fn attach_probe(&mut self, probe_type: ProbeType, vm: EbpfVm) {
        self.probes.insert(probe_type, vm);
    }

    pub fn trigger_probe(
        &mut self,
        probe_type: ProbeType,
        context: &[u8],
        timestamp: u64,
        cpu_id: u32,
        pid: u32,
    ) -> Result<u64, &'static str> {
        if let Some(vm) = self.probes.get_mut(&probe_type) {
            let result = vm.run(context)?;
            let event = PerfEvent {
                timestamp_nanos: timestamp,
                cpu_id,
                pid,
                probe_type,
                sample_value: result,
            };
            self.perf_ring_buffer.push_event(event);
            Ok(result)
        } else {
            Err("Traceprobe: Probe type not attached")
        }
    }
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebpf_alu_add_immediate() {
        let bytecode = vec![
            // R0 = 5
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 5,
            },
            // R0 += 10
            EbpfInstruction {
                opcode: BPF_ALU | BPF_ADD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 10,
            },
        ];

        let mut vm = EbpfVm::new(bytecode);
        let res = vm.run(&[]).unwrap();
        assert_eq!(res, 15);
    }

    #[test]
    fn test_ebpf_alu_xor_register() {
        let bytecode = vec![
            // R0 = 10
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 10,
            },
            // R2 = 12
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 2,
                src_reg: 0,
                offset: 0,
                imm: 12,
            },
            // R0 ^= R2 (10 ^ 12 = 6)
            EbpfInstruction {
                opcode: BPF_ALU | BPF_XOR | 0x08, // 0x08 signifies register src
                dst_reg: 0,
                src_reg: 2,
                offset: 0,
                imm: 0,
            },
        ];

        let mut vm = EbpfVm::new(bytecode);
        let res = vm.run(&[]).unwrap();
        assert_eq!(res, 6);
    }

    #[test]
    fn test_ebpf_jmp_condition() {
        let bytecode = vec![
            // R2 = 5
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 2,
                src_reg: 0,
                offset: 0,
                imm: 5,
            },
            // if R2 == 5, JMP offset 2 (skip the R0 = 99 load)
            EbpfInstruction {
                opcode: BPF_JMP,
                dst_reg: 2,
                src_reg: 0,
                offset: 2,
                imm: 5,
            },
            // R0 = 99 (skipped)
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 99,
            },
            // R0 = 42
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 42,
            },
        ];

        let mut vm = EbpfVm::new(bytecode);
        let res = vm.run(&[]).unwrap();
        assert_eq!(res, 42);
    }

    #[test]
    fn test_ebpf_map_lifecycle() {
        let mut registry = EbpfMapRegistry::new();
        assert!(registry.create_map(1, EbpfMapType::Hash, 4, 8, 10).is_ok());
        // duplicate map
        assert!(registry.create_map(1, EbpfMapType::Hash, 4, 8, 10).is_err());

        {
            let map = registry.get_map_mut(1).unwrap();
            let key = vec![1, 2, 3, 4];
            let val = vec![10, 20, 30, 40, 50, 60, 70, 80];

            // Success update
            assert!(map.update_elem(key.clone(), val.clone()).is_ok());
            // Lookup success
            assert_eq!(map.lookup_elem(&key).unwrap(), &val);

            // Size mismatch check
            assert!(map.update_elem(vec![1, 2], val.clone()).is_err());

            // Delete success
            assert!(map.delete_elem(&key).is_ok());
            assert!(map.lookup_elem(&key).is_none());
        }
    }

    #[test]
    fn test_traceprobe_and_perf_event_ring_buffer() {
        let bytecode = vec![
            // R0 = 100 (simulated probe metric output)
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 100,
            },
        ];

        let vm = EbpfVm::new(bytecode);
        let mut manager = TraceprobeManager::new(5);

        manager.attach_probe(ProbeType::Kprobe, vm);

        let res = manager.trigger_probe(ProbeType::Kprobe, &[], 1718900000, 0, 1001);
        assert_eq!(res.unwrap(), 100);

        assert_eq!(manager.perf_ring_buffer.len(), 1);
        let event = manager.perf_ring_buffer.pop_event().unwrap();
        assert_eq!(event.sample_value, 100);
        assert_eq!(event.pid, 1001);
        assert_eq!(event.probe_type, ProbeType::Kprobe);
    }
}
