# Bootloader & Kernel Entry Comparison Matrix

| Project | Boot Protocol | Memory Map Source | Early Log Channel | Early Page Table |
|---|---|---|---|---|
| **xv6** | Multiboot / BIOS | E820 | VGA / Serial | Identity mapped low 4MB |
| **Linux** | x86 Boot Protocol | efi_memmap / E820 | earlycon / COM1 | Swapper page directory |
| **FreeBSD** | BTX loader / EFI | EFI memory map | syscons / uart | pmap initial tables |
| **SigmaOS** | UEFI / BIOS dual-boot | EFI memory map / E820 | Serial (COM1 115200) | 4-level PML4 identity & kernel high-half |
