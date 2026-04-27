[BITS 64]
global vmm_fast_copy

; void vmm_fast_copy(void* dest, void* src)
vmm_fast_copy:
    mov rcx, 512    ; 512 * 8 bytes = 4096 bytes
    rep movsq       ; Fast string copy (RSI to RDI)
    ret
