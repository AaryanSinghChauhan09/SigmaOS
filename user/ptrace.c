/* SigmaOS ELF32 user program: ptrace — end-to-end SYS_PTRACE(51) test.
 * Forks a spinning child, GETREGS its context, PEEKTEXT at its eip,
 * verifies the traced word is a plausible instruction, then kills it. */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static u32 ptrace3(u32 req, u32 pid, u32 addr, u32 data) {
    /* ebx=req, ecx=pid, edx=addr, esi=data — sc3 only covers ebx/ecx/edx,
     * so inline asm for the 4-arg form */
    u32 r;
    __asm__ volatile(
        "movl %1, %%ebx\n\tmovl %2, %%ecx\n\tmovl %3, %%edx\n\tmovl %4, %%esi\n\t"
        "movl $51, %%eax\n\tint $0x80\n\tmovl %%eax, %0"
        : "=m"(r) : "m"(req), "m"(pid), "m"(addr), "m"(data)
        : "eax", "ebx", "ecx", "edx", "esi", "memory");
    return r;
}
void _start(void) {
    u32 me = sc3(SYS_GETPID, 0, 0, 0);
    u32 child = sc3(SYS_FORK, 0, 0, 0);
    if (child == 0) {
        /* child: spin so parent can inspect a live context */
        volatile u32 x = 0;
        for (;;) x++;
    }
    if (child == (u32)-1) { outs("ptrace: fork failed\n"); sc3(SYS_EXIT, 1, 0, 0); }
    sc3(SYS_SLEEP, 30, 0, 0);              /* let child get scheduled */
    if (ptrace3(4, child, 0, 0) != 0) {    /* STOP the child */
        outs("ptrace: stop failed\n"); sc3(SYS_KILL, child, 0, 0);
        sc3(SYS_EXIT, 1, 0, 0);
    }
    u32 regs[10];
    if (ptrace3(3, child, (u32)regs, 0) != 0) {
        outs("ptrace: getregs failed\n"); sc3(SYS_KILL, child, 0, 0);
        sc3(SYS_EXIT, 1, 0, 0);
    }
    u32 eip = regs[8];
    u32 word = ptrace3(1, child, eip, 0);  /* PEEKTEXT at child eip */
    ptrace3(5, child, 0, 0);               /* CONT before exit */
    int ok = (eip != 0 && word != 0 && word != 0xFFFFFFFFu);
    outs("ptrace: child eip=0x");
    /* print eip hex */
    { char hx[9]; for (int d = 7; d >= 0; d--) { int nib=(eip>>(d*4))&15; hx[7-d]=(char)(nib<10?'0'+nib:'a'+nib-10); } hx[8]=0; outs(hx); }
    outs(ok ? " word ok\n" : " word BAD\n");
    sc3(SYS_KILL, child, 0, 0);
    sc3(SYS_EXIT, ok ? 0 : 1, 0, 0); __builtin_unreachable();
}
