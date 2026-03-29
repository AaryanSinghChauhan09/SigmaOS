# Σ SIGMAOS: SOVEREIGN DEVELOPMENT & BUILD GUIDE

Welcome, Industrial OS Architect. This guide outlines how to build and expand the **SigmaOS Zenith Supreme** industrial sharding.

## 🛠️ PREREQUISITES

To build the kernel and userland shards, you require:
- **GCC / G++**: For C and C++ shards.
- **NASM**: For x86_64 Assembly logic.
- **Make**: For the industrial build system.
- **Rustc / Cargo**: For critical safety sharding (optional).
- **Bash**: For automation shell shards.

## 🚀 BUILDING THE SOVEREIGN MASTER

### 1. Build the Kernel (Level-0)
The kernel is the bare-metal foundation:
```bash
make kernel
```

### 2. Build the Userland (Level-1)
The userland contains the Master Industrial Matrix and Distro Mirroring:
```bash
make userland
```

### 3. Build the Zenith Master
To build the complete project, including any WASM/JS components:
```bash
make zenith
```

## 📂 DIRECTORY SHARDING

- `/kernel`: Bare-metal assembly and C logic (Slab/Paging/Audit).
- `/userland/apps`: Master utility shards (Pkg Manager, Personalizer, Matrix, Runner).
- `/libc`: Proprietary **No-GLIBC** sharding.
- `/arch`: Architecture-specific silicon control (x86_64).
- `/scripts`: Automation playbooks and sovereignty audits.

## 🧪 TESTING & AUDITING

- **SigmaAuditTool**: Run `make audit` to conduct a deep system scan for memory leaks and sharding integrity.
- **Terminal Emulator**: Use the `sigma_shell` for local testing is `userland/sigma_shell.c`.

---

Σ SIGMAOS: EVOLVING INDUSTRIAL SOVEREIGNTY.
