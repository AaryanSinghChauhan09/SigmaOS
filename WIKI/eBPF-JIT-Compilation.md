# eBPF JIT Compilation

SigmaOS implements eBPF (Extended Berkeley Packet Filter) JIT compilation to transform eBPF bytecode into native machine code for high-performance packet filtering and tracing.

## Overview

eBPF is a technology that allows safe, sandboxed programs to run in kernel space. JIT compilation improves performance by:
- Translating eBPF bytecode to native machine code
- Eliminating interpreter overhead
- Enabling zero-copy packet processing
- Supporting complex packet filtering and tracing logic

## Architecture

### eBPF Execution Pipeline
1. **Load**: eBPF program loaded into kernel
2. **Verify**: Program passes safety verification
3. **JIT Compile**: Bytecode compiled to native code
4. **Execute**: Native code runs for each packet/event
5. **Unload**: Program unloaded when no longer needed

### JIT Compiler Components
- **Frontend**: Parses eBPF bytecode
- **Middle-end**: Performs optimizations (constant folding, dead code elimination)
- **Backend**: Generates native machine code
- **Runtime**: Manages compiled code lifecycle

## Implementation

### eBPF Verifier
```rust
// src/kernel/ebpf/verifier.rs
pub struct EbpfVerifier {
    max_instructions: usize,
    max_stack_depth: usize,
    allowed_helpers: BTreeSet<u32>,
}

impl EbpfVerifier {
    pub fn verify(&self, program: &[u8]) -> Result<(), EbpfError> {
        let mut ctx = VerificationContext::new(program);
        
        for (pc, insn) in ctx.instructions.iter().enumerate() {
            // Check instruction bounds
            if pc >= self.max_instructions {
                return Err(EbpfError::ProgramTooLong);
            }
            
            // Validate register usage
            self.validate_registers(&ctx, insn)?;
            
            // Validate memory access
            self.validate_memory_access(&ctx, insn)?;
            
            // Validate helper calls
            if insn.opcode == EbpfOpcode::Call {
                self.validate_helper_call(&ctx, insn)?;
            }
            
            // Check for loops and recursion
            self.check_control_flow(&ctx, pc)?;
        }
        
        Ok(())
    }
    
    fn validate_helper_call(&self, ctx: &VerificationContext, insn: &EbpfInstruction) -> Result<(), EbpfError> {
        let helper_id = insn.imm as u32;
        if !self.allowed_helpers.contains(&helper_id) {
            return Err(EbpfError::UnauthorizedHelper);
        }
        Ok(())
    }
}
```

### JIT Compiler
```rust
// src/kernel/ebpf/jit.rs
pub struct EbpfJitCompiler {
    target_arch: TargetArch,
    code_buffer: Vec<u8>,
    register_map: BTreeMap<u8, NativeRegister>,
}

impl EbpfJitCompiler {
    pub fn compile(&mut self, program: &[u8]) -> Result<JitCode, JitError> {
        let mut ctx = CompilationContext::new(program);
        
        for insn in ctx.instructions.iter() {
            match insn.opcode {
                EbpfOpcode::Add => self.compile_add(&mut ctx, insn)?,
                EbpfOpcode::Sub => self.compile_sub(&mut ctx, insn)?,
                EbpfOpcode::Mul => self.compile_mul(&mut ctx, insn)?,
                EbpfOpcode::LoadMem => self.compile_load(&mut ctx, insn)?,
                EbpfOpcode::StoreMem => self.compile_store(&mut ctx, insn)?,
                EbpfOpcode::JumpIf => self.compile_jump(&mut ctx, insn)?,
                EbpfOpcode::Call => self.compile_call(&mut ctx, insn)?,
                EbpfOpcode::Return => self.compile_return(&mut ctx, insn)?,
                _ => return Err(JitError::UnsupportedOpcode),
            }
        }
        
        Ok(JitCode {
            code: self.code_buffer.clone(),
            size: self.code_buffer.len(),
        })
    }
    
    fn compile_add(&mut self, ctx: &mut CompilationContext, insn: &EbpfInstruction) -> Result<(), JitError> {
        let dst = self.map_register(ctx, insn.dst)?;
        let src = self.map_register(ctx, insn.src)?;
        
        match self.target_arch {
            TargetArch::X86_64 => {
                self.emit_instruction(&[0x48, 0x01, 0xC2 | (src << 3) | (dst << 3)]); // add rdx, rax
            }
            TargetArch::Aarch64 => {
                self.emit_instruction(&[0x8B, 0x00, 0x00, 0x00]); // add x0, x1
            }
        }
        
        Ok(())
    }
}
```

### Native Code Execution
```rust
// src/kernel/ebpf/runtime.rs
pub struct EbpfRuntime {
    compiled_programs: BTreeMap<u64, JitCode>,
    next_id: AtomicU64,
}

impl EbpfRuntime {
    pub fn load_program(&mut self, program: &[u8]) -> Result<u64, EbpfError> {
        // Verify program
        let verifier = EbpfVerifier::new();
        verifier.verify(program)?;
        
        // Compile to native code
        let mut compiler = EbpfJitCompiler::new(TargetArch::X86_64);
        let jit_code = compiler.compile(program)?;
        
        // Make code executable
        let code_ptr = unsafe {
            let ptr = mmap_anonymous(jit_code.size)?;
            mprotect_executable(ptr, jit_code.size)?;
            std::ptr::copy_nonoverlapping(jit_code.code.as_ptr(), ptr, jit_code.size);
            ptr
        };
        
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.compiled_programs.insert(id, JitCode {
            code: code_ptr as *mut u8,
            size: jit_code.size,
        });
        
        Ok(id)
    }
    
    pub fn execute(&self, id: u64, context: &mut EbpfContext) -> Result<u64, EbpfError> {
        let code = self.compiled_programs.get(&id).ok_or(EbpfError::NotFound)?;
        let exec_fn: fn(&mut EbpfContext) -> u64 = unsafe {
            std::mem::transmute(code.code)
        };
        Ok(exec_fn(context))
    }
}
```

## Usage

### Loading eBPF Program
```rust
use sigmaos::ebpf::{EbpfRuntime, EbpfContext};

fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = EbpfRuntime::new();
    
    // Load program
    let program = vec![
        0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // add r0, 0
        0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // exit
    ];
    
    let id = runtime.load_program(&program)?;
    
    // Execute
    let mut context = EbpfContext::new();
    let result = runtime.execute(id, &mut context)?;
    
    println!("Result: {}", result);
    Ok(())
}
```

### Network Packet Filtering
```rust
// Attach eBPF program to network interface
fn attach_packet_filter(iface: &str, program: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut runtime = EbpfRuntime::new();
    let id = runtime.load_program(program)?;
    
    sigmaos::net::attach_ebpf_filter(iface, id)?;
    Ok(())
}
```

## Performance

### JIT vs Interpreter
- **Interpreter**: ~10-100 cycles per instruction
- **JIT**: ~1-5 cycles per instruction
- **Speedup**: 10-100x faster for most programs

### Optimization Techniques
- **Constant folding**: Evaluate constant expressions at compile time
- **Dead code elimination**: Remove unused instructions
- **Register allocation**: Map eBPF registers to native registers efficiently
- **Branch prediction**: Optimize conditional jumps

## Configuration

### JIT Settings
```toml
# /etc/sigmaos/ebpf.toml
[jit]
enabled = true
cache_size = "16M"
max_program_size = "1M"

[verifier]
max_instructions = 100000
max_stack_depth = 512
allow_unprivileged = true

[helpers]
# List of allowed helper functions
allowed = ["map_lookup_elem", "map_update_elem", "skb_load_bytes"]
```

### Runtime Control
```bash
# View loaded eBPF programs
sigebpf list

# Load eBPF program
sigebpf load program.o

# Attach to network interface
sigebpf attach eth0 <program_id>

# View program statistics
sigebpf stats <program_id>
```

## Security

### Safety Guarantees
- All programs must pass verification before JIT compilation
- Bounded loops prevent infinite loops
- Memory access bounds checking
- No arbitrary pointer dereferencing

### Helper Function Restrictions
- Only whitelisted helper functions can be called
- Helper functions validate their arguments
- Privileged operations require capabilities

## Troubleshooting

### Program Verification Failed
If a program fails verification:
1. Check program size: `sigebpf verify program.o`
2. Review helper function usage
3. Check for unsupported operations
4. Simplify program logic

### JIT Compilation Failed
If JIT compilation fails:
1. Check target architecture support
2. Review program for unsupported instructions
3. Disable JIT and use interpreter: `echo 0 > /proc/sys/kernel/ebpf_jit_enable`

### Performance Issues
If eBPF programs are slow:
1. Profile program execution: `sigebpf profile <program_id>`
2. Optimize program logic
3. Consider using interpreter for simple programs

---

**[Performance & Kernel](Category-Performance)** | **[eBPF Overview](SIGMA_EBPF)** | **[XDP Networking](XDP-Zero-Copy-Networking)**
