/* SigmaOS ELF32 user program: rmdir — remove empty directory.
 * Usage: exec RMDIR.ELF <dir>
 * SYS_RMDIR=25 ; SYS_EXIT=0 ; SYS_WRITE_LEN=8 */
#include "lib.h"
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { uputs("usage: rmdir <dir>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    if (sc3(SYS_RMDIR, sp[2], 0, 0) != 0) { uputs("rmdir: failed (not empty or missing)\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
