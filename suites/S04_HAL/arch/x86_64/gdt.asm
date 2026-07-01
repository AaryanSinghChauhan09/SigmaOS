; S04_HAL: x86_64 GDT Shard
[BITS 64]

section .data
align 8
gdt64:
    .null: dq 0
    .code: equ $ - gdt64
        dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; Code segment
    .data: equ $ - gdt64
        dq (1<<44) | (1<<47) | (1<<41)           ; Data segment
    .pointer:
        dw $ - gdt64 - 1
        dq gdt64

section .text
global sigma_hal_load_gdt

sigma_hal_load_gdt:
    lgdt [gdt64.pointer]
    ret
