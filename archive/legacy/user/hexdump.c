/* SigmaOS ELF32 user program: hexdump — canonical hex+ASCII dump.
 * Usage: exec HEXDUMP.ELF <file> [skip] [len]
 * SYS_OPEN=4 SYS_READ=5 SYS_SEEK=7 SYS_CLOSE=6 SYS_FSTAT=15 ; EXIT=0 */
#include "lib.h"
static unsigned char buf[4096];
static void hex2(char* o, unsigned v) {
    const char* h = "0123456789abcdef";
    o[0] = h[(v >> 4) & 15]; o[1] = h[v & 15];
}
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { uputs("usage: hexdump <file> [skip] [len]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    const char* path = (const char*)sp[2];
    u32 skip = 0, maxlen = 0xFFFFFFFFu;
    if (argc > 3) { for (const char* p = (const char*)sp[4]; *p >= '0' && *p <= '9'; p++) skip = skip * 10 + *p - '0'; }
    if (argc > 4) { maxlen = 0; for (const char* p = (const char*)sp[6]; *p >= '0' && *p <= '9'; p++) maxlen = maxlen * 10 + *p - '0'; }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("hexdump: cannot open "); uputs(path); uputs("\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    if (skip) sc3(SYS_SEEK, fd, skip, 0);
    u32 off = skip, done = 0;
    for (;;) {
        u32 want = sizeof buf;
        if (maxlen != 0xFFFFFFFFu && done + want > maxlen) want = maxlen - done;
        if (want == 0) break;
        int r = sc3(SYS_READ, fd, (u32)buf, want);
        if (r <= 0) break;
        for (int base = 0; base < r; base += 16) {
            char line[80]; int lp = 0;
            /* offset 8 hex */
            for (int i = 7; i >= 0; i--) { char t[2]; hex2(t, (off + base) >> (i * 4)); line[lp++] = t[0]; line[lp++] = t[1]; }
            line[lp++] = ' ';
            for (int i = 0; i < 16; i++) {
                if (base + i < r) { hex2(&line[lp], buf[base + i]); lp += 2; }
                else { line[lp++]=' '; line[lp++]=' '; }
                line[lp++] = (i == 7) ? ' ' : ' ';
                if (i == 7) line[lp-1] = ' ';
            }
            line[lp++] = ' ';
            for (int i = 0; i < 16 && base + i < r; i++) {
                unsigned char c = buf[base + i];
                line[lp++] = (c >= 32 && c < 127) ? (char)c : '.';
            }
            line[lp++] = '\n';
            sc3(SYS_WRITE_LEN, (u32)line, (u32)lp, 0);
        }
        off += (u32)r; done += (u32)r;
        if ((u32)r < want) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
