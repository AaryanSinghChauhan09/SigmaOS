# Σ SIGMAOS ZENITH: GETTING STARTED GUIDE 🚀

Welcome to **SigmaOS Sovereign**, an industrial-grade, zero-dependency operating system built on C11 and Assembly.

## 1. Quick Launch ⚡

SigmaOS is designed for both bare-metal and web-based execution.

### Web Deployment

1. Open `index.html` in any modern browser (Chrome, Firefox, Safari).
2. The **Sovereign Dashboard** will initialize.
3. Access the **Delta Terminal** via the taskbar.

### Bare-Metal (Silicon)

1. Ensure you have `nasm` and `gcc-x86_64-elf` installed.
2. Run `make build` to generate the ISO.
3. Launch with `qemu-system-x86_64 -cdrom SigmaOS.iso`.

## 2. Interface Navigation 📁

- **Zenith Dashboard**: Central hub for all system shards.
- **Windows**: Use the 20px snap-to-grid manager for industrial workspace layout.
- **Themes**: Click the **System Theme** icon to cycle through Kali, Zenith, Nord, and Dracula (persistent via localStorage).

## 3. Developer Framework (Issue #1) 🛠️

To register a custom user function:

```c
#include "kernel/SovereignUserShard.c"

void my_logic(void* args) { /* ... */ }

void init() {
    user_register_func("IndustrialTask", my_logic);
}
```

## 4. Stability Checks 🛡️

SigmaOS includes built-in diagnostics (Shift+F1 in UI):

- **B6**: Stack Integrity Check.
- **F1**: Memory Pressure Monitoring.
- **B7**: Zombie Process Reaper.

---
*Visit the Wiki for full API documentation.*
