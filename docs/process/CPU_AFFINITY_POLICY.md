# SigmaOS CPU Affinity Policy Specification

## Representation & Masks
- CPU affinity is represented as a 64-bit bitmask (`u64`), supporting up to 64 CPU cores.
- Bit $i$ set to 1 permits task execution on CPU $i$.

## Migration & Placement Rules
- **Enqueue Validation**: Enqueuing task on core $i$ requires `(cpu_affinity & (1 << i)) != 0`.
- **Affinity Exhaustion**: If mask is 0, defaults to all cores allowed (`0xFFFFFFFF_FFFFFFFF`).
- **Core Migration**: Task migration across CPU run queues must verify target core affinity and update `cpu_id` in process control block.
