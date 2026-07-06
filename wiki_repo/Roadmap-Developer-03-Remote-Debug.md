# SigmaOS Roadmap: Remote Kernel Debugger
Debug a running SigmaOS kernel remotely over a serial or network stub.
## Goals
- GDB remote protocol server in kernel
- Hardware breakpoints via x86 DR registers
## Key Milestones
- [ ] GDB RSP packet parser
- [ ] Memory read/write stub
- [ ] Breakpoint insertion via INT3