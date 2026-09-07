# 🚀 AI Agents Loader Management Specification (`docs/AI_AGENTS_LOADER_MANAGEMENT.md`)

This specification defines bootloader operations, dynamic module loader workflows, and security verification policies for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) in SigmaOS.

---

## 1. Bootloader Architecture & Interfaces (`src/tools/bootloader.rs`)

AI agents interact with the multi-stage bootloader:
- **Stage 1 Bootloader**: Real Mode / UEFI entry point initializing CPU GDT, IDT, and A20 line.
- **Stage 2 Bootloader**: Configures 64-bit Long Mode, loads PML4 page directories, and locates the kernel ELF payload.
- **Bootloader Configuration (`Config.sigma`)**: Declarative kernel parameters, initramfs pathing, and video modes.

---

## 2. Dynamic Kernel Module Loader (`src/kernel/module_loader.rs`)

Modules are loaded dynamically into kernel memory:
- **Symbol Relocation**: ELF symbol lookup and relocation mapping (`.text`, `.rodata`, `.data`, `.bss`).
- **Dependency Resolution**: Automated checking of exported kernel symbols before execution.
- **Module Lifecycle**: States (`Unloaded`, `Resolving`, `Loaded`, `Active`, `Error`).

---

## 3. Secure Boot & Cryptographic Verification

- **Dilithium-5 Signature Enforcement**: Before loading any kernel module or third-party extension, Sentinel verifies its Post-Quantum Dilithium-5 digital signature.
- **A/B Slot Update & Fallback**: `Firmitas` A/B atomic boot slots ensure automatic fallback to Slot A if Stage 2 boot fails on Slot B.

---

## 4. AI Agent Loader Responsibilities

- **⚡ Bolt**: Profiles boot phase execution times, optimizes initramfs decompression speed, and tunes module load ordering.
- **🎨 Palette**: Styles bootloader menu interfaces, boot splash graphics, and console progress bars.
- **🛡️ Sentinel**: Validates module signatures, verifies kernel integrity hashes, and enforces Secure Boot keys.
