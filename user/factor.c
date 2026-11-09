/* SigmaOS ELF32 user program: factor — prime factorization.
 * Usage: exec FACTOR.ELF <n>
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
    const char* a = (argc > 1) ? (const char*)sp[2] : 0;
    if (!a) { uputs("usage: factor <n>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    unsigned int n = (unsigned int)atoi_(a);
    if (n < 2) { uputs("factor: n must be >= 2\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    uputn(n); uputs(":");
    unsigned int m = n;
    for (unsigned int p = 2; p * p <= m; p++) {
        while (m % p == 0) {
            uputs(" "); uputn(p);
            m /= p;
        }
    }
    if (m > 1) { uputs(" "); uputn(m); }
    uputs("\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
