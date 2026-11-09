/*
 * SigmaOS user program: appends a fixed line to a file via int 0x80.
 *
 * Usage:  exec APPEND.BIN <name>     (single-component, file must exist)
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_APPEND=14 ebx=name, ecx=buf, edx=len -> 0 ok / -1 err
 *   SYS_WRITE=1   ebx=ptr NUL-string
 *   SYS_EXIT=0
 */
    .section .text
    .global _start
    .intel_syntax noprefix

_start:
    mov    eax, [esp]
    cmp    eax, 2
    jl     usage
    mov    ebx, [esp + 4]        /* argv[1] = name */
    test   ebx, ebx
    jz     usage

    call   get_msg
    pop    esi                   /* esi -> text base (PIC) */
    lea    ecx, [esi + append_text - get_msg]
    mov    edx, append_len
    mov    eax, 14               /* SYS_APPEND */
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

get_msg:
    call   1f
1:
    pop    esi
    ret

    .align 4
ok_msg:      .ascii "append: ok\n\0"
fail_msg:    .ascii "append: failed\n\0"
usage_msg:   .ascii "usage: exec APPEND.BIN <name>\n\0"
append_text: .ascii "appended by APPEND.BIN\n"
append_len   = . - append_text
