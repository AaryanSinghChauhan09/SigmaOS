/* SigmaOS ELF32 user program: sysinfo — SYS_SYSINFO(43) structured dump.
 * Prints uptime, process count, memory, ticks, own pid. */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static void lab(const char* s, u32 v) { outs(s); uputn(v); outs("\n"); }
void _start(void) {
    u32 si[6];
    if (sc3(SYS_SYSINFO, (u32)si, 0, 0) != 0) { outs("sysinfo: failed\n"); sc3(SYS_EXIT, 1, 0, 0); }
    lab("uptime_s\t", si[0]);
    lab("processes\t", si[1]);
    lab("mem_total_kb\t", si[2]);
    lab("mem_free_kb\t", si[3]);
    lab("ticks\t", si[4]);
    lab("pid\t", si[5]);
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
