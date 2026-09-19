# AI Agent ABI Operation Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                            AI ABI Operations Manager                            |
|     (LinuxBsdAbiBridge, ElfDynamicLinker, CosmopolitanApeBridge)                |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                       Syscall Number & Struct Translator                        |
|       (System V AMD64 RAX/RDI/RSI, AAPCS64 X8/X0, RISC-V A7/A0 Translator)       |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
| Linux ELF Syscall Map |   | FreeBSD/OpenBSD Map   |   | Cosmopolitan APE Stub |
| (sys_mmap, clone, etc)|   | (pledge, unveil, jail)|   | (MZqFpD PE/ELF Bridge)|
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                    SigmaOS Native Kernel Syscall Handler                        |
|          (Fast Syscall MSRs [STAR/LSTAR/FMASK], Ring-0 Context Dispatch)        |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Multi-Arch Syscall & Register Translator**:
   - Intercepts fast syscall MSR dispatches (`IA32_LSTAR_MSR`) and extracts register arguments according to guest ABI calling conventions.
   - Translates Linux (`0..450`), FreeBSD (`1..580`), and OpenBSD (`1..330`) system call numbers into native SigmaOS kernel handlers.

2. **Cosmopolitan APE Header Engine**:
   - Parses Cosmopolitan Actually Portable Executable (APE) binary stubs (`MZqFpD`), extracting ELF `.text`, `.rodata`, and `.data` headers for direct execution on any host architecture.

3. **ELF Dynamic Linker & Relocator**:
   - Parses ELF `.dynamic` section tags (`DT_NEEDED`, `DT_SYMTAB`, `DT_STRTAB`, `DT_PLTRELSZ`).
   - Executes lazy PLT/GOT symbol binding on first function call.

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
