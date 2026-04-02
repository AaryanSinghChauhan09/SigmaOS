# SigmaOS Suggestions & Feature Restorations

## Features Not Working as Intended (To Fix / Upgrade)
1. **Omni-CLI Argument Parsing:** High-level abstractions need to be completely removed. We must rewrite argument parsing using raw pointer arithmetic in C11.
2. **Library Interference:** Too many integrations still lean on standard libc or external dependencies. Must enforce purely `SovereignLibC.asm` across the system.
3. **Memory Allocation:** Dynamic allocation is still occurring outside the supervised Sovereign Quantum Shard limits. Needs absolute RAII/OOPs style encapsulation (custom macros) for memory bounds.
4. **Python Domination in Build:** Build scripts (`append_*.py`) are violating the low-level constraints. Must completely port automation scripts to native C/C++ or Assembly utilities.

## Restored Features (Mistakenly Deleted)
- **Sovereign Quantum Memory De-allocator (RAII Compliant in C)**: Accidental removal of the pointer tracking matrix. Restored concept to trace all dynamically allocated shards.
- **Hardware-level Native Debugger Hook**: The exception interrupt handler (INT 3) setup which was bypassed for higher-level debugging tools.

## Planned Custom Features (To Excel Over Competitors)
- OOPS implemented natively in C via function pointer tables (`vtable` implementation in C structs).
- Absolute automation: Sovereign automation tools capable of compiling, hot-swapping, and testing shards in runtime without system reboot.
- GUI/Browser based capability: A WebAssembly projection shard, streaming bare-metal framebuffer directly to a local HTML5 canvas.
