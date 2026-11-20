/* SigmaOS ELF32 user program: mkdir — create directory.
 * Usage: exec MKDIR.ELF <name> [name...]
 * SYS_MKDIR=10 ; SYS_EXIT=0 ; SYS_WRITE_LEN=8 */
#include "lib.h"
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { uputs("usage: mkdir <name>...\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fail = 0;
    for (u32 i = 2; i < argc; i++) {
        if (sc3(SYS_MKDIR, sp[i], 0, 0) != 0) { uputs("mkdir: failed "); uputs((const char*)sp[i]); uputs("\n"); fail = 1; }
    }
    sc3(SYS_EXIT, fail, 0, 0); __builtin_unreachable();
}
