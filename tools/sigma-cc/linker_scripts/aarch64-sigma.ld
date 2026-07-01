/* SigmaOS ARM64 (AArch64) Linker Script */
ENTRY(_start)

SECTIONS
{
    /* ARM64 embedded entry point typically higher or strictly defined by the bootloader */
    . = 0x80000;

    .text : 
    {
        KEEP(*(.text.boot))
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
        . = ALIGN(16);
        __bss_start = .;
        *(.bss)
        *(COMMON)
        __bss_end = .;
    }

    /* Stack section defined manually for AArch64 isolation */
    . = ALIGN(4096);
    . += 0x10000; /* 64KB stack */
    __stack_top = .;
}
