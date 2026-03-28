/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

; =========================================================================
; Σ SIGMAOS: SOVEREIGN HARDWARE MMU PAGE-WALKER (x86_64)
; =========================================================================
; Purpose: Replaces standard library abstractions with high-speed assembler.
; Implements absolute translation from Virtual to Physical Memory using
; Intel 4-level paging structure (PML4 -> PDPT -> PD -> PT).
; USP Absorbed: Linux fast-path TLB resolution, zero dependencies.
; =========================================================================

section .text
global sigma_mmu_virt_to_phys_fast
global sigma_flush_tlb_fast

; -------------------------------------------------------------------------
; uint64_t sigma_mmu_virt_to_phys_fast(uint64_t cr3_pml4, uint64_t virt_addr)
; Input:
;   rdi: Physical address of active PML4 root (CR3)
;   rsi: Virtual address to resolve
; Output:
;   rax: Resolved Physical address, or 0 if unmapped/invalid
; -------------------------------------------------------------------------
align 16
sigma_mmu_virt_to_phys_fast:
    ; 1. Canonical check (top 16 bits must match bit 47)
    mov rcx, rsi
    shr rcx, 47         ; Shift out lower 47 bits
    cmp rcx, 0          ; Is top 0?
    je .canonical_ok
    cmp rcx, 0x1FFFF    ; Is top all 1s (sign extended)?
    jne .invalid        ; If neither, not canonical

.canonical_ok:
    ; 2. Extract PML4 index (bits 39-47) -> Shift right 39, mask 0x1FF
    mov rcx, rsi
    shr rcx, 39
    and rcx, 0x1FF
    ; Multiply by 8 (sizeof(uint64_t))
    lea rdx, [rdi + rcx*8]
    mov r8, qword [rdx] ; Read PML4E
    test r8, 1          ; Check Present flag (bit 0)
    jz .invalid
    
    ; Mask out flags to get physical address of PDPT
    mov r9, 0x000FFFFFFFFFF000
    and r8, r9          

    ; 3. Extract PDPT index (bits 30-38)
    mov rcx, rsi
    shr rcx, 30
    and rcx, 0x1FF
    lea rdx, [r8 + rcx*8]
    mov r10, qword [rdx] ; Read PDPTE
    test r10, 1
    jz .invalid

    ; Check if 1GB huge page (bit 7)
    test r10, 0x80
    jnz .huge_1gb
    
    ; Mask PD physical address
    and r10, r9

    ; 4. Extract PD index (bits 21-29)
    mov rcx, rsi
    shr rcx, 21
    and rcx, 0x1FF
    lea rdx, [r10 + rcx*8]
    mov r8, qword [rdx] ; Read PDE
    test r8, 1
    jz .invalid

    ; Check if 2MB huge page (bit 7)
    test r8, 0x80
    jnz .huge_2mb

    ; Mask PT physical address
    and r8, r9

    ; 5. Extract PT index (bits 12-20)
    mov rcx, rsi
    shr rcx, 12
    and rcx, 0x1FF
    lea rdx, [r8 + rcx*8]
    mov r10, qword [rdx] ; Read PTE
    test r10, 1
    jz .invalid

    ; Calculate explicit physical page frame + 4K offset
    and r10, r9         ; Physical Base 4K Frame
    mov rcx, rsi
    and rcx, 0xFFF      ; Page offset
    add r10, rcx
    mov rax, r10        ; Return final physical address
    ret

.huge_2mb:
    ; Extract 2MB aligned physical page and add offset
    mov r9, 0x000FFFFFFFE00000
    and r8, r9
    mov rcx, rsi
    ; 2MB mask: 0x1FFFFF
    and rcx, 0x1FFFFF
    add r8, rcx
    mov rax, r8
    ret

.huge_1gb:
    ; Extract 1GB aligned physical page and add offset
    mov r9, 0x000FFFFFC0000000
    and r10, r9
    mov rcx, rsi
    ; 1GB mask: 0x3FFFFFFF
    and rcx, 0x3FFFFFFF
    add r10, rcx
    mov rax, r10
    ret

.invalid:
    xor rax, rax        ; Return 0 for invalid mapping
    ret

; -------------------------------------------------------------------------
; void sigma_flush_tlb_fast(uint64_t cr3_val)
; -------------------------------------------------------------------------
align 16
sigma_flush_tlb_fast:
    mov cr3, rdi        ; Reload CR3 to rapidly flush complete TLB cache
    ret

