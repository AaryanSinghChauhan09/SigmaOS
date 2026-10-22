# Upstream Study: xv6 Boot Process

1. **Upstream Problem**: Educational x86/RISC-V bootloader handoff and minimal kernel initialization.
2. **Data Structure & Control Flow**: `bootmain` -> `entry.S` -> `main()` -> `userinit()` -> `scheduler()`.
3. **Security & Failure Assumptions**: Single-tenant hardware assumption; panic on unexpected hardware state.
4. **Direct Adoption**: Clean boot stage progression and explicit kernel entry trace logging.
5. **Rust/SigmaOS Adaptation**: Type-safe x86_64 long mode UEFI bootloader handoff with COM1 serial logging.
6. **Exit Test Criteria**: QEMU x86_64 boots Stage 1-4 and launches init shell cleanly.
7. **Licensing & Attribution**: MIT License (xv6 / MIT OS Group).
