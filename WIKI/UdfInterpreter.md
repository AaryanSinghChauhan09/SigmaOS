# 💻 Zero-Dependency UDF Bytecode Interpreter

This document details the virtual machine specifications and complete, standalone implementation code for SigmaOS's zero-dependency User-Defined Function (UDF) Bytecode Interpreter.

---

## 1. Interpreter Specifications

The interpreter executes sandboxed programs using a custom 4-byte aligned Instruction Set Architecture (ISA). It maintains private register states, isolated heap/stack segments, and bounds-checks execution loops via atomic cycle limits.

---

## 2. Complete Rust Implementation

The code below can be compiled and run directly in any Rust-compliant environment. It is completely zero-dependency and implements safe, bounded virtual stack structures.

```rust
// WIKI Code Block: Complete Rust-Native Bytecode VM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdfError {
    Success = 0,
    InvalidOpcode,
    RegisterOutOfBounds,
    MemoryOutOfBounds,
    Timeout,
}

pub struct UdfInterpreter {
    pc: usize,
    registers: [u64; 16],
    memory: [u8; 1024],
    max_cycles: usize,
}

impl UdfInterpreter {
    pub fn new(max_cycles: usize) -> Self {
        UdfInterpreter {
            pc: 0,
            registers: [0; 16],
            memory: [0; 1024],
            max_cycles,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = [0; 16];
        self.memory = [0; 1024];
    }

    pub fn execute(&mut self, bytecode: &[u8]) -> Result<u64, UdfError> {
        let mut cycles = 0;

        while self.pc + 4 <= bytecode.len() {
            if cycles >= self.max_cycles {
                return Err(UdfError::Timeout);
            }

            let op = bytecode[self.pc];
            let r1 = bytecode[self.pc + 1] as usize;
            let r2 = bytecode[self.pc + 2] as usize;
            let val = bytecode[self.pc + 3];

            if r1 >= 16 || r2 >= 16 {
                return Err(UdfError::RegisterOutOfBounds);
            }

            match op {
                0x00 => { // NOP
                    self.pc += 4;
                }
                0x01 => { // LOAD r1, memory_offset
                    let offset = (r2 << 8 | val as usize) % 1024;
                    self.registers[r1] = self.memory[offset] as u64;
                    self.pc += 4;
                }
                0x02 => { // STORE r1, memory_offset
                    let offset = (r2 << 8 | val as usize) % 1024;
                    self.memory[offset] = (self.registers[r1] & 0xFF) as u8;
                    self.pc += 4;
                }
                0x03 => { // ADD r1, r2
                    self.registers[r1] = self.registers[r1].wrapping_add(self.registers[r2]);
                    self.pc += 4;
                }
                0x04 => { // SUB r1, r2
                    self.registers[r1] = self.registers[r1].wrapping_sub(self.registers[r2]);
                    self.pc += 4;
                }
                0x05 => { // JMP offset
                    let target = (r1 << 16 | r2 << 8 | val as usize);
                    if target + 4 > bytecode.len() {
                        return Err(UdfError::MemoryOutOfBounds);
                    }
                    self.pc = target;
                }
                0x06 => { // JEQ r1, offset (Jump if equal to zero)
                    if self.registers[r1] == 0 {
                        let target = (r2 << 8 | val as usize);
                        if target + 4 > bytecode.len() {
                            return Err(UdfError::MemoryOutOfBounds);
                        }
                        self.pc = target;
                    } else {
                        self.pc += 4;
                    }
                }
                0x07 => { // SET_REG r1, immediate_val
                    self.registers[r1] = val as u64;
                    self.pc += 4;
                }
                _ => return Err(UdfError::InvalidOpcode),
            }

            cycles += 1;
        }

        Ok(self.registers[0])
    }
}
```
