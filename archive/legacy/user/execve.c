/* SigmaOS ELF32 user program: execve — replace own image via SYS_EXEC(38).
 * Usage: exec EXECVE.ELF <TARGET.ELF>  — loads target with argc=1. */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
void _start(void) {
    u32* sp = uargs();
    if (sp[0] < 2) { outs("usage: execve <target.elf>\n"); sc3(SYS_EXIT, 1, 0, 0); }
    outs("execve: replacing image with ");
    outs((const char*)sp[2]); outs("\n");
    u32 r = sc3(SYS_EXEC, sp[2], 0, 0);
    if (r == (u32)-1) { outs("execve: load failed\n"); sc3(SYS_EXIT, 1, 0, 0); }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
