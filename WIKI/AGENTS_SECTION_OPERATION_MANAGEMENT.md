# AI Agent Guidelines: Binary Section Operation Management in SigmaOS

## 📌 1. Overview & Binary Section Architecture

In **SigmaOS**, binary section operation management governs the parsing, mapping, relocation, and access control enforcement for executable ELF (Linux/BSD) and PE (Windows/Win32) binary sections.

As an AI agent working on executable loaders, kernel linkers, dynamic linkers, or security enforcers, you must enforce strict **W^X (Write XOR Execute)** security invariants and $4\text{ KB}$ page-aligned section boundaries across all loaded modules.

---

## ⚙️ 2. Binary Section Layout & W^X Security Invariants

```
+-----------------------------------------------------------------------------------+
|                         SIGMAOS ELF BINARY SECTION LAYOUT                         |
+-----------------------------------------------------------------------------------+
|  📜 .text         | Code Segment            | Read + Execute (RX)                 |
|  🔒 .rodata       | Read-Only Constants     | Read-Only (R)                       |
|  💾 .data         | Initialized Globals     | Read + Write (RW)                   |
|  🧹 .bss          | Zero-Initialized Data   | Read + Write (RW)                   |
|  🔗 .got / .plt   | Global Offset Table     | RELRO (Read-Only after Relocation) |
|  🚀 .init_array   | Constructor Callbacks   | Read-Only (R)                       |
+-----------------------------------------------------------------------------------+
```

### Section Access Control Rules:

| Section Name | Purpose & Content | Page Table Flags (`PageTableFlags`) | W^X Compliance |
| :--- | :--- | :--- | :--- |
| **`.text`** | Machine instructions & functions | `PRESENT \| USER_ACCESSIBLE` (No `WRITABLE`, No `NO_EXECUTE`) | ✅ **Execute Only** |
| **`.rodata`** | String literals, vtables, const data | `PRESENT \| NO_EXECUTE` (No `WRITABLE`) | ✅ **Read Only** |
| **`.data`** | Initialized global/static variables | `PRESENT \| WRITABLE \| NO_EXECUTE` | ✅ **Write Only** |
| **`.bss`** | Uninitialized static memory ($0$-filled) | `PRESENT \| WRITABLE \| NO_EXECUTE` | ✅ **Write Only** |
| **`.got.plt`** | Dynamic linking GOT/PLT entries | `PRESENT \| NO_EXECUTE` (Write disabled post-relocation) | ✅ **RELRO** |

---

## 🧮 3. Kernel Linker Symbols & Page Boundary Alignment

* **Linker Script Location:** `kernel/arch/x86_64/linker.ld`
* **Section Symbols:**
  ```rust
  extern "C" {
      static _stext: u8;   // Start of .text section
      static _etext: u8;   // End of .text section
      static _srodata: u8; // Start of .rodata section
      static _erodata: u8; // End of .rodata section
      static _sdata: u8;   // Start of .data section
      static _edata: u8;   // End of .data section
      static _sbss: u8;    // Start of .bss section
      static _ebss: u8;    // End of .bss section
  }
  ```
* **Page Alignment Invariant:**
  * Every section start (`_stext`, `_srodata`, `_sdata`, `_sbss`) MUST be aligned to a $4096$-byte ($4\text{ KB}$) virtual address boundary (`. = ALIGN(4096);`).
  * Page alignment prevents security leakage where writable data and executable code share the same physical page table entry.

---

## 🛡️ 4. Dynamic Relocation & RELRO Enforcement

1. **Section Parsing:**
   * Parsers (`src/exec/elf.rs`, `src/open_source_os_gap_closure.rs`) parse section headers (`Elf64_Shdr`) and program headers (`Elf64_Phdr`).
2. **Dynamic Symbol Resolution:**
   * Dynamic linkers populate `.got` entries using `R_X86_64_GLOB_DAT` and `R_X86_64_JUMP_SLOT` relocations while `.got` is temporarily writable (`RW`).
3. **RELRO Lockdown:**
   * Immediately after resolving dynamic symbols, the kernel transitions page table permissions for the GOT page range from `RW` to `R` (`PageTableFlags::PRESENT | PageTableFlags::NO_EXECUTE`).

---

## 🚫 5. AI Agent Rules & Code Patterns

1. **Never Violate W^X:**
   * Reject any ELF/PE binary segment requesting simultaneous write and execute permissions (`PF_W | PF_X`). Return `ElfError::InsecureWPlusXSegment`.
2. **Zero the `.bss` Section:**
   * Memory allocated for `.bss` sections must be explicitly zero-filled before execution to prevent uninitialized memory disclosure attacks.
3. **Validate GOT Boundaries:**
   * Ensure dynamic symbol relocation writes are strictly bounded within the `.got` address range (`_sgot` to `_egot`).

---

## 🧪 6. Standalone Testing Commands

AI agents can verify ELF section loading, relocation, and W^X enforcement via standalone unit compilation:

```bash
# Test ELF dynamic relocation & section parser
rustc --test --edition=2021 src/open_source_os_gap_closure.rs -o build/elf_tests && ./build/elf_tests && rm build/elf_tests

# Test virtual memory page table section protection flags
rustc --test --edition=2021 src/kernel/vmm_paging.rs -o build/vmm_tests && ./build/vmm_tests && rm build/vmm_tests
```
