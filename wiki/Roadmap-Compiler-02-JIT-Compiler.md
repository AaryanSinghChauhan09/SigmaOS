# SigmaOS Roadmap: JIT Compilation Engine
A lightweight JIT compiler for SigmaLang and WASM hot paths.
## Goals
- SSA IR â†’ x86_64 and aarch64 code generation
- Inline caching for dynamic dispatch
## Key Milestones
- [ ] SSA IR construction from AST
- [ ] x86_64 instruction emitter
- [ ] Simple register allocator (linear scan)