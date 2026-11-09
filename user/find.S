/*
 * SigmaOS user program: find — substring-match cwd entries via int 0x80.
 *
 * Usage:  exec FIND.BIN <substr>
 *
 * ABI: eax=nr, ebx/ecx/edx=args, ret in eax.
 *   SYS_READDIRX=20 ebx=buf, ecx=len -> "d|f name size\n" records
 *   SYS_WRITE_LEN=8 ebx=ptr ecx=len
 *   SYS_EXIT=0
 */
    .section .text
    .global _start
    .intel_syntax noprefix

_start:
    mov    eax, [esp]
    cmp    eax, 2
    jl     usage
    mov    edi, [esp + 8]        /* argv[1] = pattern */
    test   edi, edi
    jz     usage

    call   here
here:
    pop    esi                  /* PIC base */

    lea    ebx, [esi + offset buf - offset here]
    mov    ecx, 4096
    mov    eax, 20              /* SYS_READDIRX */
    int    0x80
    test   eax, eax
    js     fail
    mov    r8d, eax             /* record bytes */
    test   eax, eax
    jz     empty

    lea    edx, [esi + offset buf - offset here]
    xor    ecx, ecx             /* record offset */
    xor    ebp, ebp             /* matched count */
rec_loop:
    cmp    ecx, r8d
    jae    done
    /* record length -> r9d (bytes through '\n') */
    lea    ebx, [edx + ecx]
    xor    r9d, r9d
reclen:
    mov    al, [ebx + r9]
    inc    r9d
    cmp    al, 10
    jne    reclen
    /* name field at +2 until ' ' or '\n' -> r10d=len */
    lea    ebx, [edx + ecx + 2]
    xor    r10d, r10d
name_scan:
    mov    al, [ebx + r10]
    cmp    al, ' '
    je     name_done
    cmp    al, 10
    je     name_done
    inc    r10d
    cmp    r10d, 120
    jb     name_scan
name_done:
    /* substring search: pattern edi in name[0..r10) */
    xor    r11d, r11d           /* name idx */
    xor    r13d, r13d           /* pat idx */
pat_check:
    cmp    r11d, r10d
    ja     no_match
    mov    al, [edi + r13]
    test   al, al
    jz     print_rec            /* pattern exhausted -> match */
    mov    dl, [ebx + r11]
    cmp    al, 'A'
    jb     c1
    cmp    al, 'Z'
    ja     c1
    or     al, 32
c1:
    cmp    dl, 'A'
    jb     c2
    cmp    dl, 'Z'
    ja     c2
    or     dl, 32
c2:
    cmp    al, dl
    jne    shift
    inc    r11d
    inc    r13d
    jmp    pat_check
shift:
    inc    r11d
    xor    r13d, r13d
    jmp    pat_check
no_match:
    add    ecx, r9d
    jmp    rec_loop
print_rec:
    inc    ebp
    push   ecx                 /* save record offset */
    lea    ebx, [edx + ecx]
    mov    ecx, r9d
    mov    eax, 8
    int    0x80
    pop    ecx
    add    ecx, r9d
    jmp    rec_loop

done:
    test   ebp, ebp
    jnz    bye
    call   he
he:
    pop    esi
    lea    ebx, [esi + empty_msg - he]
    mov    eax, 1
    int    0x80
    jmp    bye

usage:
    call   hu
hu:
    pop    esi
    lea    ebx, [esi + usage_msg - hu]
    mov    eax, 1
    int    0x80
    jmp    bye

fail:
    call   hf
hf:
    pop    esi
    lea    ebx, [esi + fail_msg - hf]
    mov    eax, 1
    int    0x80

bye:
    xor    eax, eax
    int    0x80

buf:
    .skip 4096
usage_msg: .ascii "usage: find <substr>\n\0"
empty_msg: .ascii "(no matches)\n\0"
fail_msg:  .ascii "find: failed\n\0"
