/* SigmaOS ELF32 user program: kill — terminate a process by pid.
 * Usage: exec KILL.ELF <pid>   (SYS_KILL=33) */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static int digits(const char* s) { int v = 0; while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; } return v; }
void _start(void) {
    u32* sp = uargs();
    if (sp[0] < 2) { outs("usage: kill <pid>\n"); sc3(SYS_EXIT, 1, 0, 0); }
    u32 pid = (u32)digits((const char*)sp[2]);
    u32 r = sc3(SYS_KILL, pid, 0, 0);
    if (r == (u32)-1) { outs("kill: no such pid\n"); sc3(SYS_EXIT, 1, 0, 0); }
    outs("kill: sent\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
