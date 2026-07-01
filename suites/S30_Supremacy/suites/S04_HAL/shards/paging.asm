[BITS 64]
global load_page_directory
global enable_paging

; void load_page_directory(u64* pd)
load_page_directory:
    mov cr3, rdi    ; Load PD address (RDI) into CR3
    ret

; void enable_paging()
enable_paging:
    mov rax, cr0
    or rax, 0x80000000 ; Set PG bit
    mov cr0, rax
    ret
