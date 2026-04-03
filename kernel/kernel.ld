/* 
 Σ SIGMAOS ZENITH: SOVEREIGN LINKER SCRIPT (v2900.0)
 Mission: Absolute Silicon Layout Sovereignty.
*/

ENTRY(sigma_kernel_entry)

SECTIONS {
    /* Σ BOOTLOADER HANDOFF: 1MB MARK */
    . = 1M;

    .text : {
        /* Σ MISSION CODE SHARD */
        *(.multiboot)
        *(.text)
    }

    .rodata : {
        /* Σ READ-ONLY SILICON STRINGS */
        *(.rodata)
    }

    .data : {
        /* Σ INITIALIZED DATA SHARD */
        *(.data)
    }

    .bss : {
        /* Σ ZERO-INITIALIZED MEMORY SHARD */
        *(COMMON)
        *(.bss)
    }

    /* Σ ZENITH STACK (64KB) */
    .stack : {
        . = . + 0x10000;
        sigma_stack_top = .;
    }
}
