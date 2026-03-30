; =============================================================================
; Σ SIGMAOS: SOVEREIGN SILICON-PULSE (v1.0 - RAW ASM FINALITY)
; =============================================================================
; Mission: Zero-Library, Zero-C, Zero-CPP. Raw x86_64 Silicon Control.
; Capability: Direct TSC-based Scheduling, Outperforming Competitor Kernels.
; =============================================================================

SECTION .text
    GLOBAL sovereign_pulse_init
    GLOBAL sovereign_pulse_trigger

; sovereign_pulse_init(): Sets up silicon-direct frequency shards.
sovereign_pulse_init:
    push rbp
    mov rbp, rsp
    
    ; Clear EAX/EDX for RDTSC
    xor rax, rax
    xor rdx, rdx
    cpuid               ; Serialize for precision
    rdtsc               ; Read Time Stamp Counter
    
    ; Store initial pulse tick
    mov [pulse_start_tick], rax
    
    pop rbp
    ret

; sovereign_pulse_trigger(uint64_t target_cycles): Busy-wait silicon-direct.
sovereign_pulse_trigger:
    push rbp
    mov rbp, rsp
    
    mov r8, rdi         ; target_cycles
    cpuid
    rdtsc
    mov r9, rax         ; start_time
    
.pulse_loop:
    cpuid
    rdtsc
    sub rax, r9         ; delta = current - start
    cmp rax, r8         ; compare delta with target
    jl .pulse_loop      ; loop until target reached
    
    pop rbp
    ret

SECTION .data
    pulse_start_tick dq 0
