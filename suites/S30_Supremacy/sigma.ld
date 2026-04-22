/*
 * =========================================================================
 * SIGMA OS: SOVEREIGN LINKER SCRIPT
 * Positions kernel sections precisely in physical memory.
 * =========================================================================
 */

ENTRY(_start)

SECTIONS {
    . = 1M;  /* Load kernel at 1MB — above BIOS reserved regions */

    .text ALIGN(4K) : {
        *(.multiboot)           /* Multiboot header MUST be first */
        *(.text)
    }

    .rodata ALIGN(4K) : {
        *(.rodata*)
    }

    .data ALIGN(4K) : {
        *(.data)
    }

    .bss ALIGN(4K) : {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    }

    kernel_end = .;             /* Symbol marking end of kernel image */
}
