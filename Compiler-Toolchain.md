# 🛠️ SigmaOS Compiler Toolchain

> **Self-Hosting Sovereignty: SigmaOS builds itself.**

The sovereign compiler toolchain ensures SigmaOS can compile, assemble, and link its own binaries without any external GNU or LLVM dependency.

---

## Components

### `sigma_cc` — Sovereign C-Subset Compiler

**Absorbs**: Fabrice Bellard's TCC, c4 compiler, SubC philosophy.

| Feature | Status |
|---------|--------|
| Lexer (tokenizer) | ✅ Implemented |
| Keyword recognition (`int`, `return`, `if`, `while`) | ✅ Implemented |
| Integer literals | ✅ Implemented |
| Operators (`+`, `-`, `*`, `/`, `==`, `<`, `>`) | ✅ Implemented |
| Recursive descent parser | 🚧 Stubbed |
| AST generation | 🚧 Stubbed |
| x86_64 code emission | 📅 Planned |

**Usage:**
```
[sigma-sh]# cc myfile.c
SigmaCC v0.1 [Sovereign C-Subset Compiler]
Compiling myfile.c...
[CC] Lexer -> Parser -> AST -> x86_64 emit
```

---

### `sigma_assembler` — Sovereign Assembler

Translates Sigma Assembly syntax directly to x86_64 machine code bytes.

**Usage:**
```
[sigma-sh]# asm boot.s
SigmaAssembler v1.0 [Self-Hosting Initializer]
Assembling boot.s...
Output written to a.out
```

---

### `sigma_linker` — Sovereign Linker

Resolves symbols from sovereign object files, replacing GNU `ld`.

- Two-pass architecture: collect symbols, then relocate
- Symbol table with up to 1024 entries
- Default load address: `0x400000`

**Usage:**
```
[sigma-sh]# ld kernel.o drivers.o
SigmaLinker v1.0 [Sovereign Symbol Resolver]
Linking 2 objects...
Link complete. Output: sigma_app.bin
```

---

## Self-Hosting Roadmap

1. **Phase A**: `sigma_cc` compiles simple programs (arithmetic, variables)
2. **Phase B**: `sigma_cc` compiles control flow (`if`, `while`, functions)
3. **Phase C**: `sigma_cc` compiles itself → **self-hosting achieved**
