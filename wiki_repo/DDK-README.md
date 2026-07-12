# Driver Development Kit (DDK)

The **DDK** is SigmaOS's sovereign framework for authoring, testing, and
formally verifying hardware drivers.

## Why a Sovereign DDK?

Linux drivers land in a monolithic kernel where a single bug can crash the
whole system. SigmaOS isolates every driver in its own capability-gated shard
so a faulty NIC driver can't corrupt the filesystem or scheduler.

## Key APIs

| Symbol | Purpose |
| --- | --- |
| `sigma_register_driver(name)` | Register a driver shard with the HAL registry |
| `sigma_alloc_dma_region(size)` | Allocate physically contiguous, cache-coherent memory |
| `sigma_irq_install(vector, handler)` | Bind an interrupt vector with formal priority checking |

## Directory Layout

```
drivers/ddk/
  ddk_stub.c        ← Minimal compilable template
  ddk_api.h         ← Public DDK header (TODO)
  tests/            ← Formal property tests (TODO)
```

## Roadmap

- [ ] DMA management API

- [ ] IRQ arbitration layer

- [ ] Formal verification harness (CBMC / Frama-C integration)
