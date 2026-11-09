/*
 * SigmaOS user program: mv — rename a file via int 0x80.
 *
 * Usage:  exec MV.BIN <old> <new>    (argv[1], argv[2] single-component)
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_RENAME=13 ebx=old, ecx=new -> 0 ok / -1 err
 *   SYS_WRITE=1   ebx=ptr NUL-string
 *   SYS_EXIT=0
 */
    .section .text
    .global _start
    .intel_syntax noprefix

_start:
    mov    eax, [esp]
    cmp    eax, 3
    jl     usage
    mov    ebx, [esp + 8]        /* argv[1] = old */
    mov    ecx, [esp + 12]       /* argv[2] = new */
    test   ebx, ebx
    jz     usage
    test   ecx, ecx
    jz     usage

    mov    eax, 13               /* SYS_RENAME */
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
ok_msg:     .ascii "mv: renamed\n\0"
fail_msg:   .ascii "mv: failed\n\0"
usage_msg:  .ascii "usage: exec MV.BIN <old> <new>\n\0"
