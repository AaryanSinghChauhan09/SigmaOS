# SigmaOS Architecture Deep Dive

## 1. Kernel Ring Model

SigmaOS uses the standard x86_64 hardware privilege ring model with a strict separation policy:

| Ring | Who runs here | What they can do |
|---|---|---|
| Ring 0 (kernel) | sigma-kernel | Full hardware access, manage page tables, handle IRQs, load/unload shards |
| Ring 1 | (unused) | Reserved |
| Ring 2 | (unused) | Reserved |
| Ring 3 (user) | All shards, userland processes | No direct hardware access; all I/O via sigma-bus IPC |

The kernel never runs any application code in Ring 0. Even device drivers are Ring 3 shards communicating via DMA-safe message queues.

### Exception: Interrupt Handlers

IRQ handlers run in Ring 0 for < 2µs, then immediately defer work to Ring 3 interrupt-bottom-half shards via sigma-bus async messages. This keeps the Ring 0 window minimal.

---

## 2. Shard Isolation Model

A **shard** is SigmaOS's atomic capability unit — equivalent to a process in a conventional OS, but with a formal interface contract and capability-scoped I/O.

### Shard Properties

- **Isolated address space**: each shard gets its own virtual address space (ASLR + guard pages)

- **Capability token**: a 256-bit unforgeable token issued by the kernel at shard registration

- **sigma_pledge**: an allowlist of IPC message types the shard may send/receive

- **sigma_unveil**: a path allowlist for SigmaFS access (additive, irreversible per-call)

### Shard Communication via sigma-bus

```
Shard A                  sigma-bus kernel channel             Shard B
  │                              │                               │
  │── send(BusMsg {              │                               │
  │     dest: ShardId::B,        │                               │
  │     payload: Msg::FileOpen,  │                               │
  │     token: CapToken,         │                               │
  │   }) ──────────────────────► │                               │
  │                              │── verify token + pledge ────► │
  │                              │                               │── recv()
  │                              │◄── BusMsg { reply: FD } ──── │
  │◄──────────────────────────── │                               │
```

The kernel validates the capability token and pledge on **every message**. There is no ambient authority.

---

## 3. Memory Layout

```
Virtual Address Space (x86_64, 48-bit canonical):

  0x0000_0000_0000_0000  Null guard page (unmapped)
  0x0000_0000_0001_0000  Shard text/data (load address)
  ...                    Shard heap (grows up)
  0x0000_7FFF_FFFF_F000  Shard stack (grows down)
  0x0000_8000_0000_0000  Non-canonical gap (hardware reserved)
  ...
  0xFFFF_8000_0000_0000  Kernel space start
  0xFFFF_8000_0001_0000  sigma-kernel text/data (mapped in all address spaces)
  0xFFFF_C000_0000_0000  Direct physical memory map (kernel-only)
  0xFFFF_F800_0000_0000  sigma-bus kernel channel ring buffers (per-CPU)
  0xFFFF_FFFF_0000_0000  Architecture-specific (APIC, GDT, IDT)
  0xFFFF_FFFF_FFFF_F000  Top guard page
```

### Shard Space

Each shard sees only its own virtual space plus the kernel text (read-only). sigma-bus shared memory regions are explicitly mapped by the kernel into both sender and receiver address spaces for zero-copy.

---

## 4. IPC Performance: sigma-bus Benchmark Targets

| Metric | Target | Mechanism |
|---|---|---|
| Round-trip latency (local) | < 500 ns | Shared memory ring + futex wait |
| Round-trip latency (cross-CPU) | < 2 µs | IPI + ring buffer |
| Throughput (small messages, 64B) | > 10M msg/s | Lock-free MPSC ring |
| Throughput (large, 4KB, zero-copy) | > 1M msg/s | mmap region hand-off |
| Message size limit | 4 KB inline / unlimited via shared region | |

Benchmark tool: `sigma-bench-ipc` in `tools/bench/`.

---

## 5. Security Model: Pledge → Unveil → AVC → PQC Chain

```
Process birth
  │
  ▼ sigma_pledge("stdio rpath inet")
    ┌─ syscall allowlist carved into process descriptor
    └─ irreversible: pledge can only shrink, never grow

  ▼ sigma_unveil("/usr/share/fonts", "r")
    ┌─ SigmaFS path added to process unveil map
    └─ irreversible: any access outside unveil → SIGKILL

  ▼ AVC (Access Vector Cache)
    ┌─ O(1) hash table: (source_type, target_type, class) → decision
    ├─ Policy loaded at boot from /etc/sigma/policy.sig (Dilithium-5 signed)
    └─ Cache eviction: LRU, 65536 entries per CPU

  ▼ PQC Attestation
    ┌─ Every shard has an SVID (SPIFFE identity) signed by sigma-ca
    ├─ sigma-ca uses Dilithium-5 for certificate signatures
    ├─ Key exchange: Kyber-1024 KEM for sigma-bus shared memory handshake
    └─ TPM2 PCR sealing: FDE key sealed to PCRs 0, 4, 7, 8
```

Any message that fails pledge, unveil, AVC, or SVID verification is **silently dropped** — no error is returned to the sender (prevents oracle attacks).
