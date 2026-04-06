[BITS 64]
extern sigma_syscall_handler
global sigma_syscall_entry

sigma_syscall_entry:
    swapgs
    mov [gs:0x10], rsp         ; Save user stack
    mov rsp, [gs:0x00]         ; Load kernel stack
    
    ; Save caller regs
    push r11
    push rcx
    push rbp
    
    ; Dispatch to C handler (rax=num, rdi=arg1, rsi=arg2, rdx=arg3)
    mov rdi, rax
    mov rsi, rdi
    mov rdx, rsi
    call sigma_syscall_handler
    
    pop rbp
    pop rcx
    pop r11
    
    mov rsp, [gs:0x10]         ; Restore user stack
    swapgs
    sysretq
