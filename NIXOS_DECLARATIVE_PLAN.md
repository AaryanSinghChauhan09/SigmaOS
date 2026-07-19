# ❄️ Pure Declarative State & Reproducibility Blueprint (NIXOS_DECLARATIVE_PLAN)

This document details the high-level software engineering design to absorb and surpass the key strengths of NixOS (declarative system configuration, immutable filesystems, and reproducible environments) inside the zero-dependency, capability-based microkernel architecture of SigmaOS.

---

## 🏛️ OOP System Architecture

The declarative configuration engine of SigmaOS uses a highly structured, zero-dependency parser and polymorphic evaluator model.

```
       [Sovereign Config File: /etc/sigma.conf]
                           |
                           v
        +-------------------------------------+
        |     ZeroDependencyConfigParser      | (Entirely allocation-free parsing)
        +-------------------------------------+
                           |
                           v (AST Representation)
        +-------------------------------------+
        |     PolymorphicProfileEvaluator     | (Strategy Pattern for subsystems)
        +-------------------------------------+
                           |
            +--------------+--------------+
            |                             |
            v                             v
+-----------------------+     +-----------------------+
|  MemoryConfigStrategy |     | NetworkConfigStrategy |
+-----------------------+     +-----------------------+
            |                             |
            +--------------+--------------+
                           |
                           v
        +-------------------------------------+
        |     SovereignGenerationManager      | (Composite state tracker)
        +-------------------------------------+
                           |
                           v
        +-------------------------------------+
        |      Cryptographic Hash Store       | (Immutable physical partitions)
        +-------------------------------------+
```

---

## 📅 Core Design Specifications

### 1. Zero-Dependency Declarative Config Parser
*   **The Problem in NixOS:** The Nix evaluation language is slow, dynamically-typed, and relies on a large interpreter codebase.
*   **The SigmaOS Solution:** SigmaOS introduces a highly optimized, static, zero-dependency declarative parser (`SovereignConfigParser`).
*   **UDF Evaluation Engine:** Subsystem parameters are parsed as User Defined Functions (UDFs). Evaluation is entirely allocation-free, resolving dependencies on stack-allocated structures in $O(N)$ time complexity.

### 2. Immutable Cryptographic Hash Directory Layout
*   **The Problem in NixOS:** Legacy filesystems (ext4) do not natively enforce block-level reproducibility, depending on user-space read-only mounts.
*   **The SigmaOS Solution:** Built-in cryptographic hash-mapped immutable filesystem (integrated with `SigmaFS`).
*   **Merkle Hash Mapping:** Subsystem binaries are addressed by their Kyber-signed Merkle tree root hash. Execution paths are mapped to physical storage sectors. The microkernel's physical memory manager (`S-MM`) enforces block-level read-only hardware pages for system-level configurations.

### 3. Sub-Millisecond Generation-Level Rollbacks
*   **Instant Restores:** System configurations represent immutable generations.
*   **OOP Strategy:** Implements the `State` and `memento` design patterns. Each generation state is a lightweight, read-only capability descriptor. Restoring a previous system state is as simple as re-pointing the microkernel's active configuration capability descriptor to a previous generation's index, executing rollbacks in sub-milliseconds without disk rewrites.

---

## 🛠️ Verification & Test Harness Specifications

*   **Config Parsing Integrity Tests**: Programmatically verify that nested declarative structures parse successfully and match target structures without heap allocations.
*   **State Transition Validation**: Test that switching active generations updates microkernel paging directories and network capabilities correctly under zero-trust validation.
