# SigmaOS Zenith: User-Space Ecosystem & Booting (v15.2)

Moving beyond ring-0 isolation, the SigmaOS ecosystem has expanded to feature hardware-accelerated user-space applications and a robust, UEFI-compliant bootloader chain.

---

## User-Space Applications

**Implementation Directory:** `usr/bin/`

To validate the SigmaOS IPC, Compositor, and VFS implementations, two flagship user-space environments have been engineered:

1. **SigmaTerm (`sigma_term.c`)**
- A high-performance terminal emulator modeled after `st` (suckless).
- Interfaces directly with the Wayland-style kernel compositor to request a shared-memory back-buffer window.
- Parses incoming TTY escape sequences to render hardware-accelerated fonts and colors.

1. **OmniWeb Browser Stub (`sigma_browser.c`)**
- The conceptual foundation for a sovereign rendering engine.
- Implements a lightweight, isolated DOM tree allocator and parser.
- Maps elements (like `html`, `body`, `h1`) into absolute bounding boxes that are passed to the kernel compositor for frame swapping.

## Bootloader & ISO Generation

**Implementation Directory:** `tools/`

To transition SigmaOS from a simulated test-bed into bare-metal execution, we have integrated the **Limine** boot protocol.

- **`limine.cfg`:** Configures the initial hardware states. Bypasses legacy BIOS bottlenecks by requesting direct 1080p 32-bit framebuffers (`RESOLUTION=1920x1080x32`) directly from the UEFI GOP before transferring control to `sigmaos.elf`.

- **`build_iso.sh`:** A CI/CD-ready compilation pipeline utilizing `xorriso` to pack the compiled kernel shards and Limine binaries into a hybrid ISO format, bootable on both legacy MBR/BIOS systems and modern GPT/UEFI hardware.
