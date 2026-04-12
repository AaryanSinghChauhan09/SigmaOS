; =========================================================================
; Σ SIGMAOS ZENITH: SOVEREIGN DSA HARDWARE ACCELERATOR
; =========================================================================
; Mission: Pure x86_64 Assembly for time-critical DSA operations.
; Shard: DS_AI_HARDWARE_SYNC
; =========================================================================

section .text
    global sigma_asm_quicksort_partition
    global sigma_asm_atomic_swap

; -------------------------------------------------------------------------
; sigma_asm_atomic_swap(u32* a, u32* b)
; Uses XCHG for atomic low-level swap without temp register.
; -------------------------------------------------------------------------
sigma_asm_atomic_swap:
    mov eax, [rdi]      ; Load *a into eax
    xchg eax, [rsi]     ; Atomic xchg with *b
    mov [rdi], eax      ; Store new value back to *a
    ret

; -------------------------------------------------------------------------
; sigma_asm_quicksort_partition(u32* arr, int low, int high)
; Hand-optimized partitioning logic for silicon-level performance.
; -------------------------------------------------------------------------
sigma_asm_quicksort_partition:
    ; rdi: arr, rsi: low, rdx: high
    push rbp
    mov rbp, rsp
    
    mov r8d, [rdi + rdx*4] ; pivot = arr[high]
    mov r9, rsi            ; i = low
    dec r9                 ; i = low - 1
    
    mov r10, rsi           ; j = low
.loop:
    cmp r10, rdx           ; if (j >= high) break
    jge .finalize
    
    mov r11d, [rdi + r10*4] ; r11d = arr[j]
    cmp r11d, r8d          ; if (arr[j] < pivot)
    jge .next
    
    inc r9                 ; i++
    ; Swap arr[i] and arr[j]
    mov eax, [rdi + r9*4]
    mov ebx, [rdi + r10*4]
    mov [rdi + r9*4], ebx
    mov [rdi + r10*4], eax

.next:
    inc r10                ; j++
    jmp .loop

.finalize:
    inc r9                 ; i++
    ; Swap arr[i] and arr[high]
    mov eax, [rdi + r9*4]
    mov ebx, [rdi + rdx*4]
    mov [rdi + r9*4], ebx
    mov [rdi + rdx*4], eax
    
    mov rax, r9            ; return i
    pop rbp
    ret
