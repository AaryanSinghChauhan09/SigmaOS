# 💻 Zero-Dependency UDF Bytecode Interpreter Plan

This design blueprint describes the architecture for SigmaOS’s User-Defined Function (UDF) Bytecode Interpreter. Operating inside both the microkernel and sandboxed user-space processes, this interpreter executes arbitrary custom scripts (for drivers, package resolution hooks, and custom scheduling policies) safely with zero external dependencies and a minimal memory footprint.

---

## 1. Core Interpreter Architecture

The interpreter operates on a custom, simplified instruction set designed for speed and safety. It relies on private virtual registers and strictly limits execution cycle counts to prevent infinite loop exploits.

```
       +---------------------------------------------+
       |           UdfInterpreter Context            |
       +---------------------------------------------+
       | - pc : usize (Program Counter)              |
       | - registers : [u64; 16] (Private Registers) |
       | - memory : [u8; 1024] (Isolated Stack/Heap) |
       | - max_cycles : usize (Cycle Bounding)       |
       +---------------------------------------------+
                              |
                              v
       +---------------------------------------------+
       |             Instruction Decoder             |
       +---------------------------------------------+
       | Reads 4-byte aligned instruction opcodes   |
       +---------------------------------------------+
                              |
            +-----------------+-----------------+
            |                 |                 |
            v                 v                 v
       [LOAD / STORE]     [ADD / SUB]       [JMP / JEQ]
```

### 1.1 Encapsulated Interpreter State
To prevent arbitrary memory corruption, the entire virtual machine context is strictly encapsulated:
*   **Virtual Program Counter (`pc`):** Tracked privately to prevent external jump manipulations.
*   **Registers Pool:** 16 private general-purpose virtual registers (`R0` through `R15`), initialized to `0`.
*   **Isolated Heap/Stack:** A pre-allocated, isolated memory buffer (typically 1KB to 4KB), completely separate from the host process's physical heap or stack.

### 1.2 Bounded Cycle Execution (DoS Protection)
To prevent custom functions from hanging the kernel or driver managers, every execution loop enforces a strict cycle boundary:
```rust
pub fn run(&mut self, bytecode: &[u8]) -> Result<(), UdfError> {
    let mut cycles = 0;
    while self.pc < bytecode.len() {
        if cycles >= self.max_cycles {
            return Err(UdfError::Timeout);
        }
        self.execute_instruction(bytecode)?;
        cycles += 1;
    }
    Ok(())
}
```

---

## 2. Instruction Set Architecture (ISA)

Opcodes are represented as 4-byte integers (32-bit aligned structures):
*   `0x00`: `NOP` — No operation.
*   `0x01`: `LOAD r_dest, offset` — Loads value from private memory buffer into register.
*   `0x02`: `STORE r_src, offset` — Stores register value into private memory buffer.
*   `0x03`: `ADD r_dest, r_src` — Adds values of two registers.
*   `0x04`: `SUB r_dest, r_src` — Subtracts source register from destination.
*   `0x05`: `JMP offset` — Unconditional jump to instructions offset.
*   `0x06`: `JEQ r_cmp, offset` — Jumps to offset if register value is equal to zero.
*   `0x07`: `SYS r_num` — Executes a restricted kernel capability-gated syscall.

---

## 3. Implementation Plan

1.  **Phase 1: Standardize Instruction Parser (Milestone 1)**
    *   Expose the `UdfInterpreter` struct in `src/driver/device.rs` or a dedicated module.
    *   Write safe, bounds-checked instruction decoding loops.
2.  **Phase 2: Add Capability-Gated Syscalls (Milestone 2)**
    *   Bind the `SYS` opcode to validated security Capability Tokens (`CapabilityToken`).
    *   Ensure that unprivileged bytecode attempting to access forbidden resources immediately halts.
3.  **Phase 3: Sandbox Validation (Milestone 3)**
    *   Write extensive test suites executing recursive and infinite loops to verify timeout safety.
    *   Ensure that memory read/write boundaries are verified with offset wrap-around logic.
