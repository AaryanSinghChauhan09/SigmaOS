/* SigmaOS ELF32 user program: sleep — delay for N seconds.
 * Usage: exec SLEEP.ELF <seconds> — ticks at 100 Hz (PIT_HZ). */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { outs("usage: sleep <seconds>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    const char* p = (const char*)sp[2];
    u32 n = 0;
    while (*p >= '0' && *p <= '9') { n = n * 10 + (u32)(*p - '0'); p++; }
    if (!n) n = 1;
    sc3(SYS_SLEEP, n * 100u, 0, 0); /* 100 ticks/sec */
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
