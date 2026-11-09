/* SigmaOS ELF32 user program: ln — hardlink.
 * Usage: exec LN.ELF <old> <new>
 * SYS_LINK=24 ebx=old ecx=new ; SYS_EXIT=0 ; SYS_WRITE_LEN=8 */
#include "lib.h"
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 3) { uputs("usage: ln <old> <new>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    if (sc3(SYS_LINK, sp[2], sp[3], 0) != 0) { uputs("ln: failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
