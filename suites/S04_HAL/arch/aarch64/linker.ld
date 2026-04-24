/*
 * =============================================================================
 * Σ SIGMAOS: AARCH64 LINKER SCRIPT
 * =============================================================================
 * Defines the memory layout for the Raspberry Pi 3 (BCM2837) boot payload.
 *
 * Ensures that the .text, .data, and .bss sections are strictly bounded,
 * and explicitly reserves a physical memory region for the DMA Coherent 
 * Allocator to prevent overlapping crashes.
 * =============================================================================
 */

ENTRY(_start)

/* Raspberry Pi loads the kernel at 0x80000 */
SECTIONS
{
    . = 0x80000;
    
    __kernel_start = .;

    .text : {
        KEEP(*(.text.boot))
        *(.text .text.* .gnu.linkonce.t*)
    }

    .rodata : {
        *(.rodata .rodata.* .gnu.linkonce.r*)
    }

    .data : {
        *(.data .data.* .gnu.linkonce.d*)
    }

    .bss (NOLOAD) : {
        . = ALIGN(16);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    . = ALIGN(4096);
    __kernel_end = .;

    /* 
     * =====================================================================
     * Σ HARDWARE-NATIVE DMA REGION
     * =====================================================================
     * Reserve 16MB of contiguous physical memory strictly for the NPU/GPU
     * DMA coherent allocator. The PMM must never hand out pages from this 
     * region for standard OS heaps.
     */
    . = ALIGN(4096);
    __dma_region_start = .;
    . += 16M;
    __dma_region_end = .;

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
