# SigmaOS Roadmap: Binary Hardening Suite
Apply systematic binary hardening to all SigmaOS executables.
## Goals
- PIE + ASLR for all binaries
- Stack canaries and CFI (Control Flow Integrity)
## Key Milestones
- [ ] CFI forward-edge enforcement
- [ ] Shadow stack for return addresses
- [ ] RELRO + BIND_NOW linker flags