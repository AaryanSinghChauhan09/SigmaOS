.global _start
_start:
    mv s0, zero
    la sp, 0x80000000
    call hal_init
    j .
