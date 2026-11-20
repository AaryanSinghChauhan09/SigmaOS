/* SigmaOS ELF32 user program: free — physical memory report.
 * Usage: exec FREE.ELF
 * SYS_MEMINFO=29 fills u32[3]: total_pages, free_pages, alloc_count.
 * Prints KiB/MiB human-readable. */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static void print_kb(unsigned int pages, const char* label) {
    unsigned int kb = pages * 4; /* 4 KiB pages */
    outs(label); outs(": ");
    if (kb >= 1024) { uputn(kb / 1024); outs(" MiB"); }
    else { uputn(kb); outs(" KiB"); }
    outs(" ("); uputn(pages); outs(" pages)\n");
}
void _start(void) {
    static u32 info[3];
    int r = sc3(SYS_MEMINFO, (u32)info, 0, 0);
    if (r != 0) { outs("free: meminfo failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    outs("SigmaOS memory:\n");
    print_kb(info[0], "total");
    print_kb(info[1], "free ");
    print_kb(info[0] - info[1] < info[0] ? info[0] - info[1] : 0, "used ");
    outs("allocs: "); uputn(info[2]); outs("\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
