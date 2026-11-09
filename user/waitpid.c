/* SigmaOS ELF32 user program: waitpid — block until a pid exits.
 * Polls SYS_WAITPID(39) with SYS_SLEEP between checks.
 * Usage: exec WAITPID.ELF <pid>   (0 = any child) */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static int digits(const char* s) { int v = 0; while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; } return v; }
void _start(void) {
    u32* sp = uargs();
    u32 pid = (sp[0] > 1) ? (u32)digits((const char*)sp[2]) : 0;
    u32 status = 0;
    for (;;) {
        u32 r = sc3(SYS_WAITPID, pid, (u32)&status, 0);
        if (r != (u32)-2) {
            if (r == (u32)-1) { outs("waitpid: no such process\n"); sc3(SYS_EXIT, 1, 0, 0); }
            outs("waitpid: reaped pid "); uputn(r);
            outs(" status "); uputn(status); outs("\n");
            sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
        }
        sc3(SYS_SLEEP, 1, 0, 0);   /* yield ~10ms between polls */
    }
}
