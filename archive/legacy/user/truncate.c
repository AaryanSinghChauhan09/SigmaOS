/* SigmaOS ELF32 user program: truncate — shrink file to N bytes.
 * Usage: exec TRUNCATE.ELF <file> <size>
 * SYS_TRUNC=26 ebx=name ecx=size ; SYS_EXIT=0 ; SYS_WRITE_LEN=8 */
#include "lib.h"
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 3) { uputs("usage: truncate <file> <size>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    u32 sz = 0;
    for (const char* p = (const char*)sp[4]; *p >= '0' && *p <= '9'; p++) sz = sz * 10 + *p - '0';
    if (sc3(SYS_TRUNC, sp[2], sz, 0) != 0) { uputs("truncate: failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
