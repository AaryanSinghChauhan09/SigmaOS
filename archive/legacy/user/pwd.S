/*
 * SigmaOS user program: pwd — print current working directory via int 0x80.
 *
 * Usage:  exec PWD.BIN
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_GETCWD=17 ebx=buf, ecx=len -> eax=len or -1
 *   SYS_WRITE_LEN=8 ebx=ptr ecx=len
 *   SYS_WRITE=1   ebx=ptr NUL-string
 *   SYS_EXIT=0
 */
    .section .text
    .global _start
    .intel_syntax noprefix

_start:
    lea    ebx, [esp - 160]      /* cwd buffer */
    mov    ecx, 160
    mov    eax, 17               /* SYS_GETCWD */
    int    0x80
    cmp    eax, -1
    je     fail
    mov    ecx, eax              /* len */
    lea    ebx, [esp - 160]
    mov    eax, 8                /* SYS_WRITE_LEN */
    int    0x80
    /* newline */
    call   h1
h1:
    pop    esi
    mov    al, [esi + nl_char - h1]
    mov    [esp - 8], al
    lea    ebx, [esp - 8]
    mov    ecx, 1
    mov    eax, 8
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
nl_char:   .ascii "\n"
fail_msg:  .ascii "pwd: failed\n\0"
