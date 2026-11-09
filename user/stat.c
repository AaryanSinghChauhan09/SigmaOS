/* SigmaOS ELF32 user program: stat — file metadata.
 * Usage: exec STAT.ELF <path>
 * SYS_STAT=16 size ; SYS_CHDIR probe for dir ; SYS_GETCWD=17 ; SYS_WRITE_LEN=8 ; SYS_EXIT=0 */
#include "lib.h"

static int is_dir(const char* path) {
    static char cwd[256];
    sc3(SYS_GETCWD, (u32)cwd, sizeof cwd - 1, 0);
    int r = sc3(SYS_CHDIR, (u32)path, 0, 0);
    if (r == 0) { sc3(SYS_CHDIR, (u32)cwd, 0, 0); return 1; }
    return 0;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* p = (argc > 1) ? (const char*)sp[2] : 0;
    if (!p) { uputs("usage: stat <path>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int sz = sc3(SYS_STAT, (u32)p, 0, 0);
    int d = is_dir(p);
    if (sz < 0 && !d) { uputs("stat: not found\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    uputs("  File: "); uputs(p); uputs("\n");
    uputs("  Type: "); uputs(d ? "directory" : "regular file"); uputs("\n");
    uputs("  Size: "); uputn(sz > 0 ? (u32)sz : 0); uputs(" bytes\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
