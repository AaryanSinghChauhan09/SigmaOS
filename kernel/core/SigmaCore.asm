; =========================================================================
; Σ SIGMAOS ZENITH SUPREME: CORE ASSEMBLY SHARD (v1.0)
; =========================================================================
; Mission: Low-level Silicon Control & Industrial Interrupt Sharding.
; Based on: torvalds/linux & LupusOS silicon logic.
; =========================================================================

[BITS 64]

global sigma_silicon_halt
global sigma_silicon_reboot
global sigma_lpm_shard_enter

section .text

; --- Halt Silicon Shard ---
sigma_silicon_halt:
    cli         ; Clear Interrupts
    hlt         ; Halt Processor core
    ret

; --- Reboot Silicon Shard ---
sigma_silicon_reboot:
    ; Standard x86 industrial reboot through 8042 controller
    mov al, 0xFE
    out 0x64, al
    jmp $       ; Infinite loop if reboot fails

; --- Low Power Mode (LPM) Shard ---
sigma_lpm_shard_enter:
    ; Industrial LPM logic for sovereign energy management
    cli
    hlt
    ret
