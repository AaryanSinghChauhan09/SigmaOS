# SigmaOS eBPF Runtime

## Overview

SigmaOS implements a complete eBPF virtual machine with the full Linux eBPF ISA. Programs can be attached to kprobes, tracepoints, XDP hooks, LSM hooks, and more.

**Location:** `src/kernel/sigma_ebpf_runtime.rs`

---

## Instruction Set

All 64-bit eBPF instructions are supported:

| Class | Operations |
|-------|-----------|
| ALU64 | MOV, ADD, SUB, MUL, DIV, OR, AND, XOR, LSH, RSH, ARSH, MOD, NEG |
| ALU32 | Same, but truncate to 32 bits |
| JMP | JA, JEQ, JNE, JGT, JGE, JLT, JLE, JSGT, JSGE, JSET, CALL, EXIT |
| LD/ST | Load/store to stack and context memory |

---

## Program Types

| Type | Hook Point |
|------|-----------|
| `SocketFilter` | Socket receive path |
| `KProbe` | Kernel function entry/exit |
| `Tracepoint` | Static tracepoints |
| `Xdp` | NIC driver (zero-copy networking) |
| `SchedCls` | TC classifier |
| `Lsm` | Linux Security Module hooks |

---

## BPF Maps

| Type | Description |
|------|-------------|
| `Array` | Fixed-size indexed by u32 |
| `Hash` | Arbitrary key hash map |
| `LruHash` | LRU eviction hash map |
| `PercpuArray` | Per-CPU array |
| `RingBuf` | Lock-free ring for output |

---

## API Reference

```rust
let mut reg = BpfRegistry::new();

// Create a map
let map_id = reg.create_map(BpfMapType::Array, 4, 8, 256);

// Load a program
let prog_id = reg.load_program("count_calls", vec![
    BpfInsn::mov64_imm(0, 0),       // r0 = 0 (XDP_PASS)
    BpfInsn::exit(),
], BpfProgType::Xdp);

// Run with context
let result = reg.run(prog_id, packet_data, now_ns, pid, uid).unwrap();
println!("return={}", result.return_value);

// Direct VM usage
let mut vm = BpfVm::new();
vm.insn_limit = 100_000; // Override default 1M
let result = vm.run(&program, &mut maps);
```

---

## Comparison

| Feature | Linux eBPF | BSD BPF | SigmaOS eBPF |
|---------|-----------|---------|--------------|
| 64-bit ISA | Yes | No | Yes |
| Maps | Yes | No | Yes |
| JIT | Yes | Limited | Planned |
| Verifier | Yes | No | Basic |
| no_std | No | No | **Yes** |
| Helper calls | ~200 | None | Core subset |
