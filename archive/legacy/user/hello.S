/*
 * SigmaOS user program: hello — first ring-3 binary (i386, position-independent).
 *
 * ABI (int 0x80): eax = syscall nr, ebx/ecx/edx = args, ret in eax.
 *   SYS_EXIT=0  SYS_WRITE=1 (ebx = NUL-terminated string ptr)
 *   SYS_GETCHAR=2  SYS_TICKS=3 (returns PIT tick count)
 *
 * PIC note: `exec` loads this at a runtime-chosen address, so all data
 * references go through call/pop EIP-relative addressing. Strings live in
 * .text so label differences are assembly-time constants.
 *
 * Build: make -C user   ->  user/hello.bin
 * Run:   kernel shell ->  exec HELLO.BIN   (8.3 name on the FAT volume)
 */
    .section .text
    .global _start
    .intel_syntax noprefix

_start:
    call   here
here:
    pop    esi                      /* esi = runtime address of 'here' */

    /* SYS_WRITE msg1 */
    mov    eax, 1
    lea    ebx, [esi + offset msg1 - offset here]
    int    0x80

    /* SYS_TICKS (prove return path; result unused) */
    mov    eax, 3
    int    0x80

    /* SYS_WRITE msg2 */
    mov    eax, 1
    lea    ebx, [esi + offset msg2 - offset here]
    int    0x80

    /* SYS_EXIT */
    xor    eax, eax
    int    0x80

    int3                             /* visible trap if exit returns */
    jmp    _start

    /* strings in .text: keeps (label - here) a link-time constant */
    .align 4
msg1: .ascii "hello from SigmaOS userland!\n\0"
msg2: .ascii "\nbye from ring3 — syscalls work\n\0"
