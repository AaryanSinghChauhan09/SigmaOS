/*
 * SigmaOS user program: ls — list cwd (type+size) via int 0x80.
 *
 * Usage:  exec LS.BIN
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_READDIRX=20 ebx=buf, ecx=len -> "d|f name size\n" records, eax=bytes
 *   SYS_WRITE_LEN=8 ebx=ptr ecx=len
 *   SYS_EXIT=0
 */
    .section .text
    .global _start
    .intel_syntax noprefix

_start:
    lea    ebx, [esp - 4096]     /* 4KB record buffer */
    mov    ecx, 4096
    mov    eax, 20               /* SYS_READDIRX */
    int    0x80
    cmp    eax, -1
    je     fail
    test   eax, eax
    jz     empty
    mov    ecx, eax
    lea    ebx, [esp - 4096]
    mov    eax, 8                /* SYS_WRITE_LEN */
    int    0x80
    jmp    bye

empty:
    call   h1
h1:
    pop    esi
    lea    ebx, [esi + empty_msg - h1]
    mov    eax, 1
    int    0x80
    jmp    bye

fail:
    call   h2
h2:
    pop    esi
    lea    ebx, [esi + fail_msg - h2]
    mov    eax, 1
    int    0x80
    jmp    bye

bye:
    xor    eax, eax
    int    0x80

    .align 4
empty_msg: .ascii "(empty)\n\0"
fail_msg:  .ascii "ls: failed\n\0"
