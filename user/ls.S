/*
 * SigmaOS user program: ls — list cwd via int 0x80.
 *
 * Usage:  exec LS.BIN
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_READDIR=19 ebx=buf, ecx=len -> eax=bytes or -1
 *   SYS_WRITE_LEN=8 ebx=ptr ecx=len
 *   SYS_EXIT=0
 */
    .section .text
    .global _start
    .intel_syntax noprefix

_start:
    lea    ebx, [esp - 2048]     /* 2KB name buffer */
    mov    ecx, 2048
    mov    eax, 19               /* SYS_READDIR */
    int    0x80
    cmp    eax, -1
    je     fail
    test   eax, eax
    jz     empty
    mov    ecx, eax
    lea    ebx, [esp - 2048]
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
