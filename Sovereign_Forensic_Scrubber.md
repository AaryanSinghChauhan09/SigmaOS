# Σ Sovereign Forensic Scrubber

The **Sovereign Forensic Scrubber** is a native Zenith-grade security module for SigmaOS designed to enforce absolute amnesic privacy through silicon-level memory sanitization. By utilizing volatile-safe wiping algorithms, the scrubber ensures that sensitive data is permanently purged from the system, reducing dependency on external security utilities.

## Amnesic Scrubbing Algorithm

The scrubber implements a multi-pass sanitization workflow to achieve industrial-grade forensic finality.

### 1. Volatile Memory Wiping
The implementation uses `volatile` pointers to ensure that the compiler does not optimize away the wiping operation, guaranteeing that every byte in the target sector is physically zeroed.

### 2. Multi-Pass Sanitization
- **Pass 1**: Zeroing of all silicon shards within the target address range.
- **Pass 2**: Injection of silicon entropy noise to disrupt residual memory patterns.
- **Pass 3**: Final verification and amnesic finality audit.

## CLI Command: `sigma-scrub`

Forensic missions are managed via the unified `sigma-scrub` command:

```bash
# Trigger a system-wide amnesic purge
sigma-scrub all

# Securely wipe a specific memory sector
sigma-scrub sector 0xABCD 4096

# Audit forensic sanitization statistics
sigma-scrub
```

## Architectural Specifications

| Feature | Specification | Standard |
| :--- | :--- | :--- |
| Algorithm | Amnesic Forensic (3-Pass) | Zenith |
| Security Parity | DOD 5220.22-M | Industrial |
| Implementation | C11 Native (Volatile) | Sovereign |
| Dependency | Zero Host Utilities | Absolute |

---
**Σ SIGMAOS: PRIVACY IS SOVEREIGN.**
