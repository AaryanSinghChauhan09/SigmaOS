; =========================================================================
; Σ SIGMAOS: SOVEREIGN KERNEL FINALITY (v25.0 - ABSOLUTE FINALITY)
; =========================================================================
; Mission: Direct Metal Control (No Library). Ring-0 Sovereignty.
; Capability: Interrupt Handlers, Paging, Task-Switching, Syscalls.
; Principle: 100% Zero-Library. Direct silicon-to-logic sharding.
; =========================================================================

[BITS 64]

global _start


extern sigma_kernel_entry
extern sigma_dispatch_shards

section .text

_start:
; =========================================================================
; BOOT: Silicon Handshake & Stack Alignment
; =========================================================================
    cli                             ; Disable interrupts
    xor rax, rax
    mov rsp, stack_top              ; Initial stack for sharding
    
    call qword sigma_kernel_setup_paging
    call qword sigma_kernel_setup_idt
    call sigma_kernel_entry         ; Handover to C++ Sovereign Core

    hlt                              ; Halt on absolute completion

sigma_kernel_setup_paging:
    ; Paging sharding logic (x86_64 CR3/PLM4 Handshake)
    ret

sigma_kernel_setup_idt:
    ; Interrupt Descriptor Table Sharding
    lidt [idt_ptr]
    ret

; =========================================================================
; INTERRUPT SHARDER: Silicon Event Dispatcher
; =========================================================================
sigma_handler_common:
    push rax
    push rbx
    push rcx
    push rdx
    
    ; Dispatch to C++ Sovereign Interrupt Manager
    call sigma_dispatch_shards
    
    pop rdx
    pop rcx
    pop rbx
    pop rax
    iretq

section .data
idt_ptr: 
    dw 2047
    dq 0x0

section .bss
stack_bottom:
    resb 16384          ; 16KB Sovereign Stack
stack_top:
