# AI Agent Dynamic Linking Management Guidelines

## 1. Overview & Architecture
This document specifies AI agent protocols for managing ELF dynamic loaders (`ld-linux.so` equivalent), shared library (`.so`) resolution, symbol export tables, and dynamic memory mapping in SigmaOS (`src/process/elf_loader.rs`).

---

## 2. Operational Directives for AI Agents

### 2.1 ELF Dynamic Symbol Resolution
- **Shared Library Loading**: AI agents updating the ELF loader must support parsing `.dynamic` sections, `DT_NEEDED` dependencies, and `DT_SYMBOLIC` export tables.
- **Symbol Relocation Processing**: Implement GOT/PLT (Global Offset Table / Procedure Linkage Table) relocations (`R_X86_64_GLOB_DAT`, `R_X86_64_JUMP_SLOT`, `R_AARCH64_GLOB_DAT`) safely in memory.

### 2.2 Security & Isolation
- **RPATH & LD_LIBRARY_PATH Sanity**: Validate library lookup paths to prevent dynamic library injection attacks or path traversal exploits.
- **ASLR Memory Randomization**: Ensure dynamic shared objects (`ET_DYN`) are loaded at randomized virtual address offsets.

---

## 3. Related Files
- `src/process/elf_loader.rs`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`
