/* SigmaOS ELF32 user program: stat — file metadata.
 * Usage: exec STAT.ELF <path>
 * SYS_STATX=23 packed (is_dir<<31)|size ; SYS_WRITE_LEN=8 ; SYS_EXIT=0 */
#include "lib.h"

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* p = (argc > 1) ? (const char*)sp[2] : 0;
    if (!p) { uputs("usage: stat <path>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int sx = sc3(SYS_STATX, (u32)p, 0, 0);
    if (sx < 0) { uputs("stat: not found\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    int sz = sx & 0x7FFFFFFF;
    int d = sx & 0x80000000;
    uputs("  File: "); uputs(p); uputs("\n");
    uputs("  Type: "); uputs(d ? "directory" : "regular file"); uputs("\n");
    uputs("  Size: "); uputn((u32)sz); uputs(" bytes\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}
