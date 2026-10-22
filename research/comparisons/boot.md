# Comparative Analysis: Kernel Boot Path

| Metric / Feature | Linux | xv6 | Redox | SigmaOS Target |
|---|---|---|---|---|
| Protocol | UEFI / Multiboot2 | Custom bootblock | Multiboot2 / UEFI | UEFI / Multiboot2 |
| Early Output | COM1 / VGA console | UART | Serial / Orbital | Dual Serial + GOP framebuffer |
| Memory Map | E820 / EFI MemMap | Hardcoded RAM limits | EFI MemMap | EFI MemMap parser |
| Init Process | Systemd / SysV init | Minimal `init.c` | Redox `init` | `sigma-init` / systemd supervisor |
