/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN KERNEL LINKER SCRIPT (v1.0)
 * =============================================================================
 * Target: x86_64 ELF — bare metal (no OS ABI)
 * Entry : _start (boot.asm)
 * Layout:
 *   0x0010_0000  .multiboot2  — Multiboot2 header (must be < 32KB from start)
 *   0x0010_0060  .text        — kernel code
 *              .rodata       — read-only data (strings, tables)
 *              .data         — initialized data (BSS follows)
 *              .bss          — zero-initialized (page-aligned)
 * NOTE: all sections identity-mapped at load address = virtual address.
 *       Higher-half mapping is established after vmm_init() at runtime.
 * =============================================================================
 */

ENTRY(_start)
OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)

SECTIONS {
    /* Load at 1MB — standard bare-metal x86 convention (avoids BIOS reserved) */
    . = 0x0000000000100000;

    /* Multiboot2 header — must appear in first 32 768 bytes of image */
    .multiboot2 ALIGN(8) : {
        *(.multiboot2)
    }

    /* Kernel text (code) */
    .text ALIGN(4096) : {
        *(.text)
        *(.text.*)
    }

    /* Read-only data: strings, vtable-analogues, const arrays */
    .rodata ALIGN(4096) : {
        *(.rodata)
        *(.rodata.*)
    }

    /* Initialized data */
    .data ALIGN(4096) : {
        *(.data)
        *(.data.*)
    }

    /* Zero-initialized BSS — boot.asm must zero this before C entry */
    .bss ALIGN(4096) : {
        __bss_start = .;
        *(.bss)
        *(.bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /* Discard unwanted ELF sections */
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
        *(.gcc_except_table*)
    }

    /* Exported symbols for kernel use */
    __kernel_start = 0x100000;
    __kernel_end   = .;
    __kernel_size  = __kernel_end - __kernel_start;
}
