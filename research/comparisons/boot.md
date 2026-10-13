# Boot Subsystem Comparison Matrix

| System | Entry Point | Memory Map Source | Console | Verdict for SigmaOS |
| :--- | :--- | :--- | :--- | :--- |
| **xv6** | Multiboot | BIOS E820 | VGA / Serial | Adopt multiboot + UART serial |
| **Linux** | EFI Stub / Setup | e820 / EFI MemMap | ttyS0 / DRM | Adopt EFI memmap parser |
| **Redox** | Bootloader | Memory map struct | Framebuffer / Serial | Adopt zero-copy memory map pass |
