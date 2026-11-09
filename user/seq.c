/* SigmaOS ELF32 user program: seq — print 1..n (or a..b).
 * Usage: exec SEQ.ELF <n>  |  exec SEQ.ELF <first> <last> [step]
 * SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

static int atoi_(const char* s) {
    int v = 0;
    while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; }
    return v;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { uputs("usage: seq <n> | seq <first> <last> [step]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int first = 1, last, step = 1;
    if (argc == 2) last = atoi_((const char*)sp[2]);
    else {
        first = atoi_((const char*)sp[2]);
        last = atoi_((const char*)sp[3]);
        if (argc > 4) step = atoi_((const char*)sp[4]);
        if (step <= 0) step = 1;
    }
    for (int i = first; step > 0 ? i <= last : i >= last; i += step) {
        uputn((u32)i);
        uputs("\n");
        if (i > 2000000 || i < -2000000) break; /* safety */
    }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
