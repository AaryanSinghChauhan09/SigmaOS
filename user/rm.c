/* SigmaOS ELF32 user program: rm — delete file(s).
 * Usage: exec RM.ELF <file> [file...]
 * SYS_UNLINK=9 ; SYS_EXIT=0 ; SYS_WRITE_LEN=8 */
#include "lib.h"
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { uputs("usage: rm <file>...\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fail = 0;
    for (u32 i = 2; i < argc; i++) {
        if (sc3(SYS_UNLINK, sp[i], 0, 0) != 0) { uputs("rm: failed "); uputs((const char*)sp[i]); uputs("\n"); fail = 1; }
    }
    sc3(SYS_EXIT, fail, 0, 0); __builtin_unreachable();
}
