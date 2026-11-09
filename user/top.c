/* SigmaOS ELF32 user program: top — one-shot scheduler/process snapshot.
 * Reads PROC/SCHED virtual file and prints it. Usage: exec TOP.ELF */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
void _start(void) {
    u32 fd = sc3(SYS_OPEN, (u32)"PROC/SCHED", 0, 0);
    if (fd == (u32)-1) { outs("top: cannot open PROC/SCHED\n"); sc3(SYS_EXIT, 1, 0, 0); }
    char buf[2048];
    u32 n = sc3(SYS_READ, fd, (u32)buf, 2047);
    sc3(SYS_CLOSE, fd, 0, 0);
    if (n) sc3(SYS_WRITE_LEN, (u32)buf, n, 0);
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
