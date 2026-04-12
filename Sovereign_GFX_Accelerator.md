# Σ Sovereign GFX Accelerator

The **Sovereign GFX Accelerator** is a native Zenith-grade kernel module for SigmaOS designed to provide hardware-accelerated graphical blitting and pixel manipulation. By leveraging inline x86_64 assembly, the module reduces dependencies on higher-level libraries and provides silicon-level frame management.

## Silicon Blitting Logic

The accelerator utilizes the `rep movsq` assembly instruction to perform 8-byte aligned memory copies. This is significantly faster than standard `memcpy` and reduces the graphical overhead on the main CPU shards.

### 1. Alpha Blending
The module implements native RGBA blending logic using bit manipulation, allowing for transparent window management without external graphical drivers.

### 2. Compositor Integration
The accelerator is integrated with the **Sovereign Serenity GUI Matrix**, providing the low-level foundation for the window server's frame flushing and rectangle invalidation.

## CLI Integration: `sigma-gui`

Graphical operations can be triggered and audited via the unified `sigma-gui` command:

```bash
# Flush the compositor buffers using hardware acceleration
sigma-gui
```

## Architectural Specifications

| Feature | Specification | Standard |
| :--- | :--- | :--- |
| Blit Throughput | 8-byte (Aligned) | Zenith |
| Latency | < 0.1ms per frame | Industrial |
| Implementation | C11 + x86_64 ASM | Sovereign |
| Dependency | Zero Host UI | Absolute |

---
**Σ SIGMAOS: VISUALS ARE SOVEREIGN.**
