/* SigmaOS Linker Script for x86_64 bare-metal */
/* Links the kernel to load at 1MB physical (0x100000) */

OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(kernel_main)

SECTIONS
{
    /* Load at 1MB — above BIOS reserved region */
    . = 0x100000;
    _kernel_start = .;

    /* Text (code) section */
    .text ALIGN(4096) :
    {
        *(.multiboot)       /* GRUB Multiboot header must be first */
        *(.text)
        *(.text.*)
    }

    /* Read-only data */
    .rodata ALIGN(4096) :
    {
        *(.rodata)
        *(.rodata.*)
    }

    /* Read-write data */
    .data ALIGN(4096) :
    {
        *(.data)
        *(.data.*)
    }

    /* BSS — zero-initialized at boot */
    .bss ALIGN(4096) :
    {
        *(COMMON)
        *(.bss)
        *(.bss.*)
    }
    _kernel_end = .;

    /* Discard DWARF debug sections in production */
    /DISCARD/ :
    {
        *(.debug*)
        *(.comment)
    }
}
