/*
 * SigmaOS user program: cd — change working directory via int 0x80.
 *
 * Usage:  exec CD.BIN <path>    ("." / ".." / subdir; cwd is kernel-global)
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_CHDIR=18 ebx=name -> eax=0 ok / -1 err
 *   SYS_WRITE=1  ebx=ptr NUL-string
 *   SYS_EXIT=0
 */
    .section .text
    .global _start
    .intel_syntax noprefix

_start:
    mov    eax, [esp]
    cmp    eax, 2
    jl     usage
    mov    ebx, [esp + 4]        /* argv[1] = path */
    test   ebx, ebx
    jz     usage

    mov    eax, 18               /* SYS_CHDIR */
    int    0x80
    test   eax, eax
    jns    ok
    call   h1
h1:
    pop    esi
    lea    ebx, [esi + fail_msg - h1]
    mov    eax, 1
    int    0x80
    jmp    bye

ok:
    call   h2
h2:
    pop    esi
    lea    ebx, [esi + ok_msg - h2]
    mov    eax, 1
    int    0x80

bye:
    xor    eax, eax
    int    0x80

usage:
    call   h3
h3:
    pop    esi
    lea    ebx, [esi + usage_msg - h3]
    mov    eax, 1
    int    0x80
    jmp    bye

    .align 4
ok_msg:      .ascii "cd: ok\n\0"
fail_msg:    .ascii "cd: failed\n\0"
usage_msg:   .ascii "usage: exec CD.BIN <path>\n\0"
