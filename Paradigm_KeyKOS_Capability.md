# Sovereign Capability Lattice: KeyKOS & EROS Paradigm Absorption

> **Status**: ✅ Absorbed | **Target Shard**: `SovereignKeyKOS` | **Source Paradigm**: KeyKOS / EROS (Object Capability Security)

---

## 1. Executive Summary

Traditional Unix-like operating systems rely on Access Control Lists (ACLs) and coarse-grained User/Group IDs (UID/GID), which are vulnerable to privilege escalation (e.g., Confused Deputy Problem). The **KeyKOS** and **EROS** paradigms establish security by structuring all references as **Capabilities**—unforgeable tokens containing both a pointer to an object and a set of access rights.

In **SigmaOS Zenith**, the `SovereignKeyKOS` shard integrates this paradigm directly into the microkernel's page table and address translation layers, providing silicon-enforced object capability security and amnesic memory persistence.

---

## 2. Strategic Features & USPs

### 2.1 Object Capability Security (OCap)
- **KeyKOS Concept**: A process can only reference resources (memory, IPC channels, files) for which it has been explicitly handed a "Key" (capability). Access rights are tied to the key itself, not the subject identity.
- **Sovereign Implementation**: All kernel objects are referenced through unforgeable, kernel-managed capability registers (`CapRef`). Passing a capability to another process is performed via a zero-copy capability exchange system call.

### 2.2 Amnesic Persistent States
- **KeyKOS Concept**: Single-level store. The system automatically takes periodic, atomic snapshots of the entire memory state. If a power loss occurs, the system restarts from the last valid snapshot as if it never went down.
- **Sovereign Implementation**: The `SovereignKeyKOS` filesystem coordinates with the memory manager to commit copy-on-write page frames atomically to persistent flash blocks. This creates an amnesic, crash-resilient runtime state.

### 2.3 Eliminating the Confused Deputy
- **KeyKOS Concept**: By requiring all authority to be passed explicitly along with a message, deputies cannot be tricked into using their ambient authority maliciously.
- **Sovereign Implementation**: SigmaOS drivers and system servers run with zero ambient authority. All file reads, socket accesses, and hardware configurations require explicit token presentation per request.

---

## 3. Shard Architecture

The `SovereignKeyKOS` capability management structure is layered directly below user-level runtimes:

```
┌─────────────────────────────────────────────────────────┐
│               SOVEREIGN KEYKOS SHARD                    │
├─────────────────────────────────────────────────────────┤
│  ┌───────────────────────┐   ┌───────────────────────┐  │
│  │   Capability Table    │   │  Single-Level Store   │  │
│  │   (Kernel Keyspace)   │   │  (COW Memory States)  │  │
│  └───────────┬───────────┘   └───────────┬───────────┘  │
│              └─────────────┬─────────────┘              │
│              ┌─────────────▼─────────────┐              │
│              │      Silicon-Enforced     │              │
│              │   Address Space Sharding  │              │
│              └───────────────────────────┘              │
└─────────────────────────────────────────────────────────┘
```

---

## 4. Integration & Usage

### 4.1 CLI Deployment
You can deploy and initialize the capability security environment using the `sigma` tool suite:

```powershell
$ sigma absorb paradigm capability
Σ [INFO] Deploying advanced OS paradigm: 'capability'...
Σ [INFO]   -> Activating SovereignKeyKOS shard...
Σ [INFO]   -> Loading silicon-enforced object capabilities...
Σ [SUCCESS] KeyKOS/EROS capability security lattice deployed successfully!
```

---

## 5. References & Standards
- KeyKOS Principles of Operation (Tymshare, 1985)
- "EROS: A Fast Capability System" by Jonathan S. Shapiro et al.
- Object Capability Model (OCap) Design Specifications
