/* SigmaOS ELF32 user program: find — substring-match cwd entries.
 * Usage: exec FIND.ELF <substr>
 * SYS_READDIRX=20 ebx=buf ecx=len -> "d|f name size\n" records
 * SYS_WRITE_LEN=8 ebx=ptr ecx=len ; SYS_EXIT=0 */
typedef unsigned int u32;
static int sc3(u32 n, u32 a, u32 b, u32 c) {
    int ret;
    __asm__ volatile("int $0x80" : "=a"(ret) : "a"(n), "b"(a), "c"(b), "d"(c) : "memory");
    return ret;
}
static int lower(int c) { return (c >= 'A' && c <= 'Z') ? c + 32 : c; }
static int contains(const char* h, int hlen, const char* n) {
    if (!*n) return 1;
    for (int i = 0; i <= hlen; i++) {
        int j = 0;
        while (n[j] && i + j < hlen && lower(h[i+j]) == lower(n[j])) j++;
        if (!n[j]) return 1;
    }
    return 0;
}
void _start(void) {
    u32* sp;
    __asm__ volatile("mov %%esp, %0" : "=r"(sp));
    u32 argc = sp[0];
    const char* pat = (argc > 1) ? (const char*)sp[2] : 0;
    if (!pat) {
        sc3(1, (u32)"usage: find <substr>\n", 0, 0);
        sc3(0, 0, 0, 0);
        __builtin_unreachable();
    }
    static char buf[4096];
    int total = sc3(20, (u32)buf, sizeof buf, 0);
    if (total < 0) { sc3(1, (u32)"find: failed\n", 0, 0); sc3(0,0,0,0); __builtin_unreachable(); }
    int matched = 0, pos = 0;
    while (pos < total) {
        int ls = pos;
        while (pos < total && buf[pos] != '\n') pos++;
        int le = pos; if (pos < total) pos++;
        /* name field: +2 .. ' ' or '\n' */
        int ns = ls + 2, ne = ns;
        while (ne < le && buf[ne] != ' ' && buf[ne] != '\n') ne++;
        if (contains(buf + ns, ne - ns, pat)) {
            matched++;
            sc3(8, (u32)(buf + ls), le - ls + 1, 0);
        }
    }
    if (!matched) sc3(1, (u32)"(no matches)\n", 0, 0);
    sc3(0, 0, 0, 0);
    __builtin_unreachable();
}
