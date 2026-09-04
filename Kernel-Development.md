# SigmaOS Kernel Development

## Setting Up Dev Environment

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
rustup toolchain install nightly
rustup target add x86_64-unknown-none
rustup component add rust-src llvm-tools-preview
```

## Kernel Source Layout

```
src/kernel/
├── mod.rs           ← Module entry point
├── scheduler.rs     ← BORE+EEVDF process scheduler
├── memory.rs        ← Physical/virtual memory manager
├── ipc.rs           ← IPC channels
├── capabilities.rs  ← Capability tokens
├── interrupts.rs    ← APIC, IRQ handling
├── process.rs       ← Process/thread management
└── init.rs          ← Kernel initialization
```

## Adding a New Syscall

1. Add syscall number to `src/syscall/numbers.rs`
2. Write the handler in `src/syscall/handlers.rs`
3. Register in `src/syscall/dispatcher.rs`
4. Write tests in `tests/`
5. Document in `docs/api-reference.md`

## Kernel Debugging

```bash
# Run with QEMU GDB stub
make run-debug

# Attach GDB
gdb -ex "target remote :1234" target/x86_64/sigma-kernel
(gdb) break sigma_start64
(gdb) continue
```

## Testing

```bash
# Unit tests
cargo +nightly test --package sigma-kernel

# All tests
cargo +nightly test --all

# Integration tests  
bash run_sigma_tests.sh
```

## Coding Guidelines

- Document every `unsafe` block with a `// SAFETY:` comment
- Use `checked_*` arithmetic for kernel calculations
- Prefer `Result<>` over panics in kernel code
- Zero-initialize all security-sensitive structs
