# SigmaOS Roadmap: Live Kernel Hotpatching
Apply security patches to a running kernel without reboot.
## Goals
- Function-level text patching via trampolines
- Atomic patch apply with rollback on failure
## Key Milestones
- [ ] Symbol resolution from DWARF debug info
- [ ] Trampoline injection at function entry
- [ ] Patch verification signature check