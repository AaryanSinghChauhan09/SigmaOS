/* SigmaOS ELF32 user program: mv — rename/move file.
 * Usage: exec MV.ELF <old> <new>
 * SYS_RENAME=13 ebx=old ecx=new ; SYS_EXIT=0 ; SYS_WRITE_LEN=8 */
#include "lib.h"
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 3) { uputs("usage: mv <old> <new>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    if (sc3(SYS_RENAME, sp[2], sp[3], 0) != 0) { uputs("mv: failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
