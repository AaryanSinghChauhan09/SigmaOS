/*
 * SigmaOS user program: rm — delete a file via int 0x80 syscalls.
 *
 * Usage:  exec RM.BIN <name>     (argv[1] = file in cwd, 8.3 or LFN)
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_UNLINK=9  ebx=name -> 0 ok / -1 err
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
    mov    ebx, [esp + 8]        /* argv[1] */
    test   ebx, ebx
    jz     usage

    /* ret = unlink(argv[1]) */
    mov    eax, 9
    int    0x80
    test   eax, eax
    jns    ok
    /* fail: print "rm: failed\n" */
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
ok_msg:     .ascii "rm: deleted\n\0"
fail_msg:   .ascii "rm: failed\n\0"
usage_msg:  .ascii "usage: exec RM.BIN <name>\n\0"
