/* SigmaOS ELF32 user program: heaptest — exercises malloc/free on SYS_BRK. */
#include "lib.h"
#include "malloc.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static void outd(u32 v) {
    char t[12]; int i = 11; t[i] = 0;
    if (!v) { outs("0"); return; }
    while (v && i) { t[--i] = (char)('0' + v % 10); v /= 10; }
    outs(t + i);
}
void _start(void) {
    u32* a = (u32*)malloc(64);
    u32* b = (u32*)malloc(128);
    u32* c = (u32*)malloc(32);
    if (!a || !b || !c) { outs("heaptest: alloc failed\n"); sc3(SYS_EXIT, 1, 0, 0); }
    for (int i = 0; i < 16; i++) a[i] = (u32)i * 7;
    for (int i = 0; i < 32; i++) b[i] = (u32)i * 3;
    int ok = 1;
    for (int i = 0; i < 16; i++) if (a[i] != (u32)i * 7) ok = 0;
    for (int i = 0; i < 32; i++) if (b[i] != (u32)i * 3) ok = 0;
    free(b);
    u32* d = (u32*)malloc(100);   /* should reuse freed space */
    if (!d) ok = 0;
    outs(ok ? "heaptest: ok\n" : "heaptest: FAIL\n");
    sc3(SYS_EXIT, ok ? 0 : 1, 0, 0); __builtin_unreachable();
}
