/* SigmaOS ELF32 user program: pid — print real process IDs.
 * Usage: exec PID.ELF [ppid]
 * SYS_GETPID=31, SYS_GETPPID=32. */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    u32 nr = (argc > 1) ? SYS_GETPPID : SYS_GETPID;
    u32 v = sc3(nr, 0, 0, 0);
    uputn(v); outs("\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
