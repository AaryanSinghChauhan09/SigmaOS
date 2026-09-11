// Linux-inspired eBPF (Extended Berkeley Packet Filter) Engine and Instruction Verifier
// Features static bytecode validation (bounds, division-by-zero, stack alignment, backward jump loop-prevention)
// and execution over standard in-kernel maps.


#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap as HashMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap as HashMap;
#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EbpfInstruction {
    pub opcode: u8,
    pub dst: u8,
    pub src: u8,
    pub offset: i16,
    pub imm: i32,
}

// Opcode constants
pub const EBPF_OP_ADD: u8 = 0x01;
pub const EBPF_OP_ADDI: u8 = 0x02;
pub const EBPF_OP_SUB: u8 = 0x03;
pub const EBPF_OP_LD: u8 = 0x04;
pub const EBPF_OP_ST: u8 = 0x05;
pub const EBPF_OP_JEQ: u8 = 0x06;
pub const EBPF_OP_JNE: u8 = 0x07;
pub const EBPF_OP_MAP_LOOKUP: u8 = 0x08;
pub const EBPF_OP_EXIT: u8 = 0x09;
pub const EBPF_OP_DIV: u8 = 0x0A;

pub struct EbpfVerifier;

impl EbpfVerifier {
    /// Validates the eBPF bytecode statically before kernel execution.
    pub fn verify(program: &[EbpfInstruction]) -> Result<(), &'static str> {
        if program.is_empty() {
            return Err("Empty eBPF program!");
        }

        let mut has_exit = false;

        for (idx, inst) in program.iter().enumerate() {
            // 1. Verify register ranges (R0 - R9 are valid)
            if inst.dst >= 10 || inst.src >= 10 {
                return Err("Register index out of bounds! Valid registers: R0-R9");
            }

            // 2. Prevent division-by-zero statically
            if inst.opcode == EBPF_OP_DIV && inst.imm == 0 && inst.src == 0 {
                return Err("Static validation error: Division by zero immediate!");
            }

            // 3. Verify stack bounds (standard eBPF 512-byte stack limit)
            if inst.opcode == EBPF_OP_LD || inst.opcode == EBPF_OP_ST {
                if inst.offset < 0 || inst.offset > 504 {
                    return Err("Stack offset out of bounds! Must be between 0 and 504 (aligned)");
                }
                if inst.offset % 4 != 0 {
                    return Err("Stack offset must be 4-byte aligned!");
                }
            }

            // 4. Verify jump targets & prevent infinite loops (no backward jumps)
            if inst.opcode == EBPF_OP_JEQ || inst.opcode == EBPF_OP_JNE {
                if inst.offset <= 0 {
                    return Err("Infinite loop prevention: Backward or self-referential jumps are rejected!");
                }
                let target_idx = idx as i32 + 1 + inst.offset as i32;
                if target_idx < 0 || target_idx >= program.len() as i32 {
                    return Err("Jump instruction targets index out of bounds!");
                }
            }

            if inst.opcode == EBPF_OP_EXIT {
                has_exit = true;
            }
        }

        if !has_exit {
            return Err("Program missing standard EBPF_OP_EXIT opcode!");
        }

        Ok(())
    }
}

pub struct EbpfEngine {
    pub registers: [i64; 10], // R0..R9
    pub stack: [u8; 512],
    pub map: HashMap<i64, i64>, // Simulated eBPF kernel map
}

impl EbpfEngine {
    pub fn new() -> Self {
        Self {
            registers: [0; 10],
            stack: [0; 512],
            map: HashMap::new(),
        }
    }

    pub fn execute(&mut self, program: &[EbpfInstruction]) -> Result<i64, &'static str> {
        let mut pc = 0;
        while pc < program.len() {
            let inst = &program[pc];
            match inst.opcode {
                EBPF_OP_ADD => {
                    self.registers[inst.dst as usize] = self.registers[inst.dst as usize]
                        .wrapping_add(self.registers[inst.src as usize]);
                }
                EBPF_OP_ADDI => {
                    self.registers[inst.dst as usize] =
                        self.registers[inst.dst as usize].wrapping_add(inst.imm as i64);
                }
                EBPF_OP_SUB => {
                    self.registers[inst.dst as usize] = self.registers[inst.dst as usize]
                        .wrapping_sub(self.registers[inst.src as usize]);
                }
                EBPF_OP_LD => {
                    let offset = inst.offset as usize;
                    if offset + 8 <= self.stack.len() {
                        let mut bytes = [0u8; 8];
                        bytes.copy_from_slice(&self.stack[offset..offset + 8]);
                        self.registers[inst.dst as usize] = i64::from_le_bytes(bytes);
                    }
                }
                EBPF_OP_ST => {
                    let offset = inst.offset as usize;
                    if offset + 8 <= self.stack.len() {
                        let bytes = self.registers[inst.src as usize].to_le_bytes();
                        self.stack[offset..offset + 8].copy_from_slice(&bytes);
                    }
                }
                EBPF_OP_JEQ => {
                    if self.registers[inst.dst as usize] == self.registers[inst.src as usize] {
                        pc += inst.offset as usize;
                    }
                }
                EBPF_OP_JNE => {
                    if self.registers[inst.dst as usize] != self.registers[inst.src as usize] {
                        pc += inst.offset as usize;
                    }
                }
                EBPF_OP_MAP_LOOKUP => {
                    let key = self.registers[inst.src as usize];
                    self.registers[inst.dst as usize] = self.map.get(&key).copied().unwrap_or(0);
                }
                EBPF_OP_DIV => {
                    let divisor = if inst.src != 0 {
                        self.registers[inst.src as usize]
                    } else {
                        inst.imm as i64
                    };
                    if divisor == 0 {
                        return Err("Division by zero at runtime!");
                    }
                    self.registers[inst.dst as usize] /= divisor;
                }
                EBPF_OP_EXIT => {
                    return Ok(self.registers[0]);
                }
                _ => return Err("Invalid opcode during execution!"),
            }
            pc += 1;
        }
        Ok(self.registers[0])
    }
}

impl Default for EbpfEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// eBPF XDP Fast Data-Plane Programmable Networking Hooks
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpHookType {
    Ingress,
    Egress,
    DriverNative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpAction {
    Pass = 1,
    Drop = 2,
    Tx = 3,
    Redirect = 4,
}

#[derive(Debug, Clone)]
pub struct XdpPacketContext {
    pub ingress_ifindex: u32,
    pub payload: Vec<u8>,
    pub rx_timestamp_ns: u64,
}

pub struct EbpfXdpFilterEngine {
    pub hook_type: XdpHookType,
    pub loaded_program: Vec<EbpfInstruction>,
    pub engine: EbpfEngine,
}

impl EbpfXdpFilterEngine {
    pub fn new(hook_type: XdpHookType) -> Self {
        Self {
            hook_type,
            loaded_program: Vec::new(),
            engine: EbpfEngine::new(),
        }
    }

    pub fn attach_program(&mut self, program: Vec<EbpfInstruction>) -> Result<(), &'static str> {
        EbpfVerifier::verify(&program)?;
        self.loaded_program = program;
        Ok(())
    }

    pub fn process_xdp_packet_hook(
        &mut self,
        ctx: &mut XdpPacketContext,
    ) -> Result<XdpAction, &'static str> {
        if self.loaded_program.is_empty() {
            return Ok(XdpAction::Pass);
        }

        // Pass packet length in R1
        self.engine.registers[1] = ctx.payload.len() as i64;
        let return_code = self.engine.execute(&self.loaded_program)?;

        match return_code {
            1 => Ok(XdpAction::Pass),
            2 => Ok(XdpAction::Drop),
            3 => Ok(XdpAction::Tx),
            4 => Ok(XdpAction::Redirect),
            _ => Ok(XdpAction::Pass),
        }
    }
}

impl Default for EbpfXdpFilterEngine {
    fn default() -> Self {
        Self::new(XdpHookType::Ingress)
    }
}

// =========================================================================
// Linux BPF_MAP_TYPE_RINGBUF Lock-Free Kernel-to-Userland Event Ring Buffer
// =========================================================================

pub const BPF_RINGBUF_BUSY_BIT: u32 = 1 << 31;
pub const BPF_RINGBUF_DISCARD_BIT: u32 = 1 << 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BpfRingBufferHeader {
    pub len: u32,
    pub pgoff: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BpfRingBufferSample {
    pub producer_offset: u32,
    pub reserved_len: usize,
    pub payload: Vec<u8>,
    pub is_discarded: bool,
}

/// Linux eBPF Ring Buffer (BPF_MAP_TYPE_RINGBUF) parity engine
pub struct BpfRingBufferEngine {
    pub capacity: usize,
    pub producer_pos: usize,
    pub consumer_pos: usize,
    pub next_handle: usize,
    pub samples: BTreeMap<usize, BpfRingBufferSample>,
    pub sample_order: Vec<usize>,
    pub dropped_samples_count: u64,
}

impl BpfRingBufferEngine {
    pub fn new(capacity_bytes: usize) -> Self {
        let cap = if capacity_bytes.is_power_of_two() && capacity_bytes >= 4096 {
            capacity_bytes
        } else {
            capacity_bytes.next_power_of_two().max(4096)
        };

        Self {
            capacity: cap,
            producer_pos: 0,
            consumer_pos: 0,
            next_handle: 1,
            samples: BTreeMap::new(),
            sample_order: Vec::new(),
            dropped_samples_count: 0,
        }
    }

    pub fn reserve_sample(&mut self, payload_len: usize) -> Result<usize, &'static str> {
        let total_size = payload_len + 8; // 8 bytes header
        if self.producer_pos + total_size - self.consumer_pos > self.capacity {
            self.dropped_samples_count += 1;
            return Err("BPF_MAP_TYPE_RINGBUF: Buffer overflow");
        }

        let handle = self.next_handle;
        self.next_handle += 1;

        self.samples.insert(
            handle,
            BpfRingBufferSample {
                producer_offset: (self.producer_pos % self.capacity) as u32,
                reserved_len: payload_len,
                payload: vec![0u8; payload_len],
                is_discarded: false,
            },
        );
        self.sample_order.push(handle);

        self.producer_pos += total_size;
        Ok(handle)
    }

    pub fn submit_sample(&mut self, handle: usize, data: &[u8]) -> Result<(), &'static str> {
        let sample = self
            .samples
            .get_mut(&handle)
            .ok_or("BPF_MAP_TYPE_RINGBUF: Invalid sample handle")?;

        if sample.payload.len() != data.len() {
            return Err("BPF_MAP_TYPE_RINGBUF: Payload size mismatch");
        }

        sample.payload.copy_from_slice(data);
        sample.is_discarded = false;
        Ok(())
    }

    pub fn discard_sample(&mut self, handle: usize) -> Result<(), &'static str> {
        let sample = self
            .samples
            .get_mut(&handle)
            .ok_or("BPF_MAP_TYPE_RINGBUF: Invalid sample handle")?;

        sample.is_discarded = true;
        sample.payload.clear();
        Ok(())
    }

    pub fn consume_next_sample(&mut self) -> Option<BpfRingBufferSample> {
        if self.sample_order.is_empty() {
            None
        } else {
            let handle = self.sample_order.remove(0);
            if let Some(sample) = self.samples.remove(&handle) {
                let total_size = sample.reserved_len + 8;
                self.consumer_pos += total_size;
                if sample.is_discarded {
                    self.consume_next_sample()
                } else {
                    Some(sample)
                }
            } else {
                self.consume_next_sample()
            }
        }
    }
}

impl Default for BpfRingBufferEngine {
    fn default() -> Self {
        Self::new(4096)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebpf_verification_and_execution() {
        let program = [
            EbpfInstruction {
                opcode: EBPF_OP_ADDI,
                dst: 0,
                src: 0,
                offset: 0,
                imm: 42,
            },
            EbpfInstruction {
                opcode: EBPF_OP_EXIT,
                dst: 0,
                src: 0,
                offset: 0,
                imm: 0,
            },
        ];

        assert!(EbpfVerifier::verify(&program).is_ok());

        let mut engine = EbpfEngine::new();
        let result = engine.execute(&program).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_ebpf_xdp_fast_path_hook() {
        let mut xdp_engine = EbpfXdpFilterEngine::new(XdpHookType::Ingress);
        let mut ctx = XdpPacketContext {
            ingress_ifindex: 1,
            payload: vec![1, 2, 3, 4, 5],
            rx_timestamp_ns: 1000,
        };

        // Unloaded program defaults to Pass
        assert_eq!(
            xdp_engine.process_xdp_packet_hook(&mut ctx).unwrap(),
            XdpAction::Pass
        );

        // eBPF program returning 2 (XdpAction::Drop)
        let drop_program = vec![
            EbpfInstruction {
                opcode: EBPF_OP_ADDI,
                dst: 0,
                src: 0,
                offset: 0,
                imm: 2,
            },
            EbpfInstruction {
                opcode: EBPF_OP_EXIT,
                dst: 0,
                src: 0,
                offset: 0,
                imm: 0,
            },
        ];

        xdp_engine.attach_program(drop_program).unwrap();
        assert_eq!(
            xdp_engine.process_xdp_packet_hook(&mut ctx).unwrap(),
            XdpAction::Drop
        );
    }

    #[test]
    fn test_bpf_ringbuf_reserve_submit_and_consume() {
        let mut ringbuf = BpfRingBufferEngine::new(4096);
        let sample_id = ringbuf.reserve_sample(16).unwrap();

        let event_data = b"EVENT_DATA_12345";
        assert!(ringbuf.submit_sample(sample_id, event_data).is_ok());

        let consumed = ringbuf.consume_next_sample().unwrap();
        assert_eq!(consumed.payload, event_data);
    }

    #[test]
    fn test_bpf_ringbuf_discard_sample() {
        let mut ringbuf = BpfRingBufferEngine::new(4096);
        let id1 = ringbuf.reserve_sample(8).unwrap();
        let id2 = ringbuf.reserve_sample(8).unwrap();

        assert!(ringbuf.discard_sample(id1).is_ok());
        assert!(ringbuf.submit_sample(id2, b"12345678").is_ok());

        // Consuming should skip discarded sample id1 and yield sample id2
        let consumed = ringbuf.consume_next_sample().unwrap();
        assert_eq!(consumed.payload, b"12345678");
    }
}
