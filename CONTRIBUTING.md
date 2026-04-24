
# Contributing to SigmaOS


Welcome to the **SigmaOS Sovereign Lattice**. We are building a bare-metal, component-driven microkernel designed for absolute hardware sovereignty, deterministic intelligence, and infinite modularity. 

We are thrilled you want to contribute! This document outlines our architectural philosophy and how you can get started.

---


## 🏗️ Architectural Philosophy


SigmaOS is not a traditional monolithic kernel. It is a **Sovereign Lattice**. When contributing, please adhere to these core tenets:

1. **Absolute Modularization:** Every feature must be an isolated "Shard." Code belongs in `suites/`, never hard-linked into a monolithic core.
2. **Contract-Based Interfaces:** Modules must communicate via the `hal_contract.h` or the Microkernel IPC layer. Direct hardware manipulation outside of `S04_HAL` is strictly forbidden.
3. **Zero-Trust Memory:** Use the per-module memory pools (`sigma_pool_alloc`). Global heap allocations (`kmalloc`) are considered a security risk and will be rejected.
4. **Declarative Features:** All new features must be toggleable via `sigma_features.json`. The OS must be able to compile without your feature.

---


## 🛠️ Getting Started



### 1. Set Up Your Environment

Run the automated setup script to install all required cross-compilers and QEMU emulators for x86_64, aarch64, and riscv64:
```bash
./setup_dev_env.sh
```


### 2. Verify the Lattice

Ensure your environment is correctly configured by running the automated test suite, which includes isolated driver compilation and mock HAL integration:
```bash
./run_sigma_tests.sh
```


### 3. Build & Emulate

Test your build using the Sovereign Orchestrator:
```bash

# Build for x86_64

python3 scripts/sovereign_builder.py x86_64


# Build for RISC-V and boot in QEMU

./deploy_edge.sh --target riscv
qemu-system-riscv64 -machine virt -bios default -kernel build/sigmaos_riscv64.bin -nographic
```

---


## 🎯 Current High-Priority Areas


If you're looking for something to work on, we are actively focusing on:


### 1. Hardware-Native Intelligence (NPU Offload)

We are expanding `suites/S09_Intelligence` to offload tensor operations from the CPU to dedicated Neural Processing Units (NPUs) and hardware accelerators. 
* **Needed:** HAL contracts for matrix multiplication engines, vendor-specific driver stubs (e.g., Google Coral TPU, Rockchip NPU).


### 2. Edge / IoT Hardware Support

We want SigmaOS booting on as many physical boards as possible.
* **Needed:** Device tree parsing, additional UART drivers (e.g., Pine64, BeagleBone), and minimal footprint optimizations.


### 3. Rust Integration

We are slowly migrating memory-critical paths to Rust (`suites/S05_Memory/sigma_safety.rs`).
* **Needed:** Safe Rust wrappers for IPC channels, capability verification, and scheduler queues.

---


## 📥 Submitting Changes


1. Create a feature branch: `git checkout -b feat/your-feature-name`
2. Ensure you have added a `module.json` manifest if creating a new suite.
3. Ensure `./run_sigma_tests.sh` passes completely.
4. Push your branch and open a Pull Request! We will review your code for architectural parity with the Sovereign Lattice guidelines.
