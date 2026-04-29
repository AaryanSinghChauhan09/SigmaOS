# CI/CD Pipeline: Real-Time Shard Validation

SigmaOS utilizes a deterministic, multi-architecture CI/CD pipeline to ensure every shard maintains sovereign integrity.

## 🧪 Testing Layers

### 1. Static Analysis (Sovereignty Check)

We use `nm` and `objdump` to verify that no shard links against external standard libraries.

### 2. Modular Hardening (Lattice Alignment)
Automated scripts (`fix_includes.py` and `sovereign_hardener.py`) enforce root-relative includes, ensuring the lattice remains sovereign and modular.

### 3. Modularity Integrity Check
The `check_modularity.py` tool scans for monolith violations, maintaining 100% modular purity across the 600 shards.

### 2. Cross-Compilation

Every commit triggers a parallel build for:

- **x86_64**: Standard server/workstation baseline.
- **aarch64 (ARM64)**: Mobile, SBC (Raspberry Pi), and Cloud-native ARM.
- **riscv64**: The future of open-silicon sovereignty.

### 3. Emulated Boot (QEMU)

We boot the compiled ISO in QEMU environments to verify:

- MMU setup and page table consistency.
- Interrupt routing (GIC for ARM, PLIC for RISC-V).
- Morphic UI frame-buffer initialization.

## 📊 Dashboard Integration

The **Morphic UI** features a live Lattice Sync widget that pulls the latest build results directly from GitHub Actions, providing developers with immediate feedback on shard stability.

---
*Status: [02_CrossArch_Test](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/02_CrossArch_Test.yml) - ACTIVE*


