; =============================================================================
; Σ SIGMAOS: SOVEREIGN SHARD-ON-DEMAND (SOD) MAPPER (v150.0 - RAW ASSEMBLY)
; =============================================================================
; Mission: Radical Modular Execution.
; Logic: "Only the part of OS code required for a certain task runs."
; Mechanism: Direct mmap(2) and munmap(2) of specialized mission shards.
; Logic: Zero HLL. Zero Library.
; =============================================================================

section .data
    msg_load     db '[SOD-MAPPER]: Analyzing mission requirements...', 10, 0
    msg_active_s db '[OK]: Shard Map established. MISSION START.', 10, 0
    msg_no_task  db '[SOD-MAPPER]: No task specified. Status: IDLE.', 10, 0
    
    cmd_audit    db 'audit', 0
    cmd_studio   db 'studio', 0
    cmd_gaming   db 'gaming', 0
    cmd_privacy  db 'privacy', 0

section .text
    global _start
    extern _sigma_sys_write
    extern _sigma_sys_mmap
    extern _sigma_sys_munmap
    extern _sigma_strlen
    extern _sigma_streq
    extern _sigma_sys_exit
    
    ; Shard implementations from other objects
    extern sigma_shard_security_audit
    extern sigma_shard_multimedia_init
    extern sigma_shard_gaming_optimize
    extern sigma_shard_privacy_init

_start:
    ; Standard x86_64 ABI entry:
    ; rsp points to argc
    ; [rsp + 8] points to argv[0]
    ; [rsp + 16] points to argv[1]
    
    mov rdi, [rsp]      ; argc
    cmp rdi, 2
    jl .no_task
    
    ; Analyzing argv[1]
    mov rsi, [rsp + 16] ; argv[1]
    
    ; Check "audit"
    mov rdi, rsi
    mov rsi, cmd_audit
    call _sigma_streq
    test rax, rax
    jnz .do_audit
    
    ; Check "studio"
    mov rsi, [rsp + 16]
    mov rdi, rsi
    mov rsi, cmd_studio
    call _sigma_streq
    test rax, rax
    jnz .do_studio
    
    ; Check "gaming"
    mov rsi, [rsp + 16]
    mov rdi, rsi
    mov rsi, cmd_gaming
    call _sigma_streq
    test rax, rax
    jnz .do_gaming
    
    ; Check "privacy"
    mov rsi, [rsp + 16]
    mov rdi, rsi
    mov rsi, cmd_privacy
    call _sigma_streq
    test rax, rax
    jnz .do_privacy

    jmp .no_task

.do_audit:
    call sigma_shard_security_audit
    jmp .finish

.do_studio:
    call sigma_shard_multimedia_init
    jmp .finish

.do_gaming:
    call sigma_shard_gaming_optimize
    jmp .finish

.do_privacy:
    call sigma_shard_privacy_init
    jmp .finish

.no_task:
    mov rdi, 1
    mov rsi, msg_no_task
    mov rdx, 45
    call _sigma_sys_write
    jmp .finish

.finish:
    mov rdi, 0
    call _sigma_sys_exit

; =============================================================================
; SILICON SOVEREIGNTY. THE MODULAR CENTURY.
; =============================================================================

