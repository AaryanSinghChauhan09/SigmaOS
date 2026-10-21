/* SigmaOS Stub Kernel Linker Script
 * Links the minimal Multiboot2 stub kernel.
 * GALL'S LAW: Simple linker for a simple stub.
 *
 * Load address: 1 MiB (0x100000) — safe, avoids BIOS reserved regions.
 * The Multiboot2 header must appear in the first 32768 bytes.
 */

ENTRY(_start)

SECTIONS {
    /* Load at 1 MiB physical address */
    . = 1M;

    /* Multiboot2 header first (must be within first 32 KiB of file) */
    .multiboot2_header ALIGN(8) : {
        *(.multiboot2_header)
    }

    /* Executable code */
    .text ALIGN(4K) : {
        *(.text)
        *(.text.*)
    }

    /* Read-only data */
    .rodata ALIGN(4K) : {
        *(.rodata)
        *(.rodata.*)
    }

    /* Initialized data */
    .data ALIGN(4K) : {
        *(.data)
        *(.data.*)
    }

    /* BSS (zero-initialized — stack lives here) */
    .bss ALIGN(4K) : {
        *(COMMON)
        *(.bss)
        *(.bss.*)
    }

    /* Discard debug / comment sections to keep the ELF lean */
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}
