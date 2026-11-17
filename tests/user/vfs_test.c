typedef unsigned int uint32_t;
static uint32_t sc_ret(uint32_t nr, uint32_t a, uint32_t b, uint32_t c) {
    uint32_t ret;
    __asm__ volatile("int $0x80" : "=a"(ret) : "a"(nr), "b"(a), "c"(b), "d"(c) : "memory");
    return ret;
}
static char buf[256];
void _start(void) {
    sc_ret(1, (uint32_t)"open data.txt\n", 0, 0);
    uint32_t fd = sc_ret(4, (uint32_t)"data.txt", 0, 0);
    if (fd == 0xFFFFFFFFu) { sc_ret(1, (uint32_t)"open failed\n",0,0); sc_ret(0,0,0,0); }
    uint32_t n = sc_ret(5, fd, (uint32_t)buf, 200);
    sc_ret(1, (uint32_t)"read ", 0, 0);
    sc_ret(8, (uint32_t)buf, n, 0);
    sc_ret(1, (uint32_t)"\n", 0, 0);
    sc_ret(6, fd, 0, 0);
    sc_ret(0, 0, 0, 0);
    __builtin_unreachable();
}
