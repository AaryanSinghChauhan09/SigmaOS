/*
 * SigmaOS user program: stat — print file size via int 0x80.
 *
 * Usage:  exec STAT.BIN <name>    (single-component, cwd-relative)
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_STAT=16 ebx=name -> eax=size or -1
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
    mov    ebx, [esp + 4]        /* argv[1] = name */
    test   ebx, ebx
    jz     usage

    mov    eax, 16               /* SYS_STAT */
    int    0x80
    cmp    eax, -1
    je     fail

    /* print "size: <decimal>\n" — itoa into stack buf */
    mov    esi, eax
    lea    edi, [esp - 16]
    mov    byte ptr [edi], 0
    mov    ecx, 10
    xor    edx, edx
itoa:
    xor    edx, edx
    div    ecx
    add    al, '0'
    dec    edi
    mov    [edi], al
    test   eax, eax
    jnz    itoa
    /* edi -> digits; print prefix then digits then newline */
    call   h1
h1:
    pop    ebx
    lea    eax, [ebx + prefix - h1]
    mov    ebx, eax
    /* SYS_WRITE wants NUL-terminated; prefix is fixed len — print via len? use WRITE_LEN */
    mov    ecx, prefix_len
    mov    eax, 8               /* SYS_WRITE_LEN ebx=ptr ecx=len */
    int    0x80
    mov    ebx, edi
    /* strlen digits */
    xor    ecx, ecx
1:  cmp    byte ptr [ebx + ecx], 0
    je     2f
    inc    ecx
    jmp    1b
2:  mov    eax, 8
    int    0x80
    call   h2
h2:
    pop    ebx
    mov    al, [ebx + nl_char - h2]
    mov    [esp - 4], al
    lea    ebx, [esp - 4]
    mov    ecx, 1
    mov    eax, 8
    int    0x80
    jmp    bye

fail:
    call   h3
h3:
    pop    ebx
    lea    eax, [ebx + fail_msg - h3]
    mov    ebx, eax
    mov    ecx, fail_len
    mov    eax, 8
    int    0x80
    jmp    bye

usage:
    call   h4
h4:
    pop    ebx
    lea    eax, [ebx + usage_msg - h4]
    mov    ebx, eax
    mov    ecx, usage_len
    mov    eax, 8
    int    0x80
    jmp    bye

bye:
    xor    eax, eax
    int    0x80

    .align 4
prefix:     .ascii "size: "
prefix_len = . - prefix
nl_char:    .ascii "\n"
fail_msg:   .ascii "stat: not found\n"
fail_len    = . - fail_msg
usage_msg:  .ascii "usage: exec STAT.BIN <name>\n"
usage_len   = . - usage_msg
