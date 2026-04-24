/*
 * =============================================================================
 * Σ SIGMAOS: RISC-V 64 LINKER SCRIPT
 * =============================================================================
 * Defines the memory layout for QEMU 'virt' and SiFive boards.
 *
 * Ensures that the boot payload is safely loaded via OpenSBI, and explicitly
 * reserves physical memory for the DMA Coherent Allocator.
 * =============================================================================
 */

ENTRY(_start)

/* OpenSBI typically jumps to 0x80200000 */
SECTIONS
{
    . = 0x80200000;

    __kernel_start = .;

    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.sdata .sdata.*) *(.data .data.*)
    }

    .bss (NOLOAD) : {
        . = ALIGN(16);
        __bss_start = .;
        *(.sbss .sbss.*) *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    . = ALIGN(4096);
    __kernel_end = .;

    /* 
     * =====================================================================
     * Σ HARDWARE-NATIVE DMA REGION
     * =====================================================================
     * Reserve 16MB of contiguous physical memory for hardware tensor 
     * operations via RISC-V AI extensions or external NPUs.
     */
    . = ALIGN(4096);
    __dma_region_start = .;
    . += 16M;
    __dma_region_end = .;

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
