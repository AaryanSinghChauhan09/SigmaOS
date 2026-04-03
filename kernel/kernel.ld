/*
 Σ SIGMAOS ZENITH: BARE-METAL LINKER SCRIPT (v1500.0)
 Hardware Target: x86_64 (Multiboot2 Compatible)
*/

ENTRY(start)

SECTIONS
{
    /* The kernel will be loaded at 1MB by the bootloader */
    . = 1M;

    .boot :
    {
        /* Ensure multiboot header is at the very beginning */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }

    .rodata :
    {
        *(.rodata)
    }

    .data :
    {
        *(.data)
    }

    .bss :
    {
        *(COMMON)
        *(.bss)
    }
}
