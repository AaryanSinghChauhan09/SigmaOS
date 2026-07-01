; S04_HAL: x86_64 IDT Shard
[BITS 64]

section .bss
align 16
idt64: resb 4096 ; 256 entries * 16 bytes

section .data
idt_ptr:
    dw 4095
    dq idt64

section .text
global sigma_hal_load_idt

sigma_hal_load_idt:
    lidt [idt_ptr]
    ret

; Placeholder for interrupt handlers
global sigma_hal_dummy_handler
sigma_hal_dummy_handler:
    iretq
