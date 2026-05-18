.global _start
_start:
    mov fp, #0
    ldr sp, =0x80000000
    bl hal_init
    b .
