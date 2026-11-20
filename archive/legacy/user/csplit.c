/* SigmaOS ELF32 user program: csplit — split file every N lines.
 * Usage: exec CSPLIT.ELF <file> <lines_per_part> <prefix>
 * Writes prefix00, prefix01, ... each with N lines.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_CREAT=11 SYS_APPEND=14 ; EXIT=0 */
#include "lib.h"
static unsigned char buf[8192];
static char name[64];
static void out(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 4) { uputs("usage: csplit <file> <lines> <prefix>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    const char* path = (const char*)sp[2];
    int nlines = 0;
    for (const char* p = (const char*)sp[4]; *p >= '0' && *p <= '9'; p++) nlines = nlines * 10 + *p - '0';
    const char* prefix = (const char*)sp[6];
    if (nlines <= 0) { uputs("csplit: bad count\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("csplit: cannot open\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    int part = 0, in_part = 0, have = 0;
    for (;;) {
        if (in_part == 0) {
            int pl = 0;
            for (const char* p = prefix; *p && pl < 56; p++) name[pl++] = *p;
            name[pl++] = '0' + (part / 10) % 10;
            name[pl++] = '0' + part % 10;
            name[pl] = 0;
            sc3(SYS_CREAT, (u32)name, 0, 0);
            have = 1;
        }
        int r = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (r <= 0) break;
        /* write whole chunk; count newlines to track lines */
        if (sc3(SYS_APPEND, (u32)name, (u32)buf, (u32)r) != 0) {
            uputs("csplit: write failed\n"); sc3(SYS_CLOSE,fd,0,0); sc3(SYS_EXIT,1,0,0); __builtin_unreachable();
        }
        for (int i = 0; i < r; i++) {
            if (buf[i] == '\n') {
                in_part++;
                if (in_part >= nlines) { in_part = 0; part++; break; }
            }
        }
        if ((u32)r < sizeof buf) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    out("csplit: "); uputn((u32)part + (have && in_part ? 1 : 0) + (have ? 1 : 0)); out(" part(s)\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
