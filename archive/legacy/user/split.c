/* SigmaOS ELF32 user program: split — split file into N-byte chunks.
 * Usage: exec SPLIT.ELF <file> <chunk_bytes> <prefix>
 * Writes prefix.aa, prefix.ab, ... via SYS_CREAT + SYS_APPEND.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_CREAT=11 SYS_APPEND=14 SYS_UNLINK=9 ; EXIT=0 */
#include "lib.h"
static unsigned char buf[8192];
static char name[64];
static void out(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 4) { uputs("usage: split <file> <bytes> <prefix>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    const char* path = (const char*)sp[2];
    u32 chunk = 0;
    for (const char* p = (const char*)sp[4]; *p >= '0' && *p <= '9'; p++) chunk = chunk * 10 + *p - '0';
    const char* prefix = (const char*)sp[6];
    if (!chunk || chunk > 65536) { uputs("split: bad size\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("split: cannot open\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    int part = 0;
    u32 in_part = 0;
    int have_name = 0;
    for (;;) {
        if (in_part == 0) {
            /* new part name: prefix + '.' + aa.. */
            int pl = 0;
            for (const char* p = prefix; *p && pl < 55; p++) name[pl++] = *p;
            name[pl++] = '.';
            name[pl++] = 'a' + (part / 26);
            name[pl++] = 'a' + (part % 26);
            name[pl] = 0;
            sc3(SYS_CREAT, (u32)name, 0, 0);
            have_name = 1;
        }
        u32 want = chunk - in_part;
        if (want > sizeof buf) want = sizeof buf;
        int r = sc3(SYS_READ, fd, (u32)buf, want);
        if (r <= 0) break;
        if (sc3(SYS_APPEND, (u32)name, (u32)buf, (u32)r) != 0) {
            uputs("split: write failed\n"); sc3(SYS_CLOSE,fd,0,0); sc3(SYS_EXIT,1,0,0); __builtin_unreachable();
        }
        in_part += (u32)r;
        if (in_part >= chunk) { in_part = 0; part++; }
        if ((u32)r < want) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    out("split: "); uputn((u32)part + (in_part ? 1 : 0)); out(" part(s)\n");
    (void)have_name;
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
