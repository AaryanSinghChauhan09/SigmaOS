/*
 * SigmaOS user program: cat — read a file via int 0x80 syscalls.
 *
 * Usage:  exec CAT.BIN <path>     (argv[1] = file path on FAT volume)
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_OPEN=4  ebx=path -> fd
 *   SYS_READ=5  ebx=fd, ecx=buf, edx=len -> nbytes
 *   SYS_WRITE_LEN=8  ebx=buf, ecx=len
 *   SYS_CLOSE=6 ebx=fd
 *   SYS_EXIT=0
 *
 * PIC: call/pop EIP base; buffer lives in .text (pages are RW).
 */
    .section .text
    .global _start
    .intel_syntax noprefix

_start:
    /* stack: [esp]=argc [esp+4]=argv0 [esp+8]=argv1 [esp+12]=NULL */
    mov    eax, [esp]
    cmp    eax, 2
    jl     usage
    mov    ebx, [esp + 8]        /* argv[1] */
    test   ebx, ebx
    jz     usage

    /* fd = open(argv[1]) */
    mov    eax, 4
    int    0x80
    test   eax, eax
    js     open_fail
    mov    edi, eax              /* fd */

    call   here
here:
    pop    esi                  /* PIC base */

read_loop:
    /* n = read(fd, buf, 4096) */
    mov    eax, 5
    mov    ebx, edi
    lea    ecx, [esi + offset buf - offset here]
    mov    edx, 4096
    int    0x80
    test   eax, eax
    jle    done
    /* write(buf, n) */
    mov    edx, eax             /* len -> ecx below */
    mov    eax, 8
    lea    ebx, [esi + offset buf - offset here]
    mov    ecx, edx
    int    0x80
    jmp    read_loop

done:
    /* close(fd) */
    mov    eax, 6
    mov    ebx, edi
    int    0x80
    xor    eax, eax             /* SYS_EXIT */
    int    0x80
    int3
    jmp    _start

usage:
    call   here2
here2:
    pop    esi
    mov    eax, 1
    lea    ebx, [esi + offset msg_usage - offset here2]
    int    0x80
    jmp    done_exit

open_fail:
    call   here3
here3:
    pop    esi
    mov    eax, 1
    lea    ebx, [esi + offset msg_fail - offset here3]
    int    0x80
done_exit:
    xor    eax, eax
    int    0x80
    int3
    jmp    _start

    .align 16
msg_usage: .ascii "usage: exec CAT.BIN <file>\n\0"
msg_fail:  .ascii "cat: open failed\n\0"
buf:       .skip 4096
