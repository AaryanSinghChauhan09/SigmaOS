/* SigmaOS ELF32 user program: tree — recursive cwd listing.
 * Usage: exec TREE.ELF
 * SYS_READDIRX=20 "d|f name size\n" records ; SYS_CHDIR=18 ; SYS_GETCWD=17 ; SYS_EXIT=0 */
typedef unsigned int u32;
static int sc3(u32 n, u32 a, u32 b, u32 c) {
    int ret;
    __asm__ volatile("int $0x80" : "=a"(ret) : "a"(n), "b"(a), "c"(b), "d"(c) : "memory");
    return ret;
}
static void puts_(const char* s) {
    int n = 0; while (s[n]) n++;
    sc3(8, (u32)s, n, 0);
}
static void putn(int v) {
    char b[12]; int i = 11; b[i] = 0;
    if (v <= 0) { puts_("0"); return; }
    while (v) { b[--i] = '0' + v % 10; v /= 10; }
    puts_(b + i);
}
static char buf[8192];
static char cwd[256];

static void walk(int depth) {
    if (depth > 6) { puts_("  ...\n"); return; }
    int total = sc3(20, (u32)buf, sizeof buf, 0);
    if (total <= 0) return;
    int pos = 0;
    /* first pass: files */
    while (pos < total) {
        int ls = pos;
        while (pos < total && buf[pos] != '\n') pos++;
        int le = pos; if (pos < total) pos++;
        if (buf[ls] != 'f') continue;
        int ns = ls + 2, ne = ns;
        while (ne < le && buf[ne] != ' ') ne++;
        for (int d = 0; d < depth; d++) puts_("  ");
        sc3(8, (u32)(buf + ns), ne - ns, 0);
        puts_("\n");
    }
    /* second pass: dirs */
    pos = 0;
    while (pos < total) {
        int ls = pos;
        while (pos < total && buf[pos] != '\n') pos++;
        int le = pos; if (pos < total) pos++;
        if (buf[ls] != 'd') continue;
        int ns = ls + 2, ne = ns;
        while (ne < le && buf[ne] != ' ') ne++;
        /* copy name to cwd-relative buffer */
        static char nm[128];
        int nl = ne - ns; if (nl > 120) nl = 120;
        for (int i = 0; i < nl; i++) nm[i] = buf[ns + i];
        nm[nl] = 0;
        for (int d = 0; d < depth; d++) puts_("  ");
        puts_(nm); puts_("/\n");
        /* descend */
        sc3(18, (u32)nm, 0, 0);          /* SYS_CHDIR nm */
        walk(depth + 1);
        sc3(18, (u32)"..", 0, 0);        /* back up */
    }
}

void _start(void) {
    int cl = sc3(17, (u32)cwd, sizeof cwd, 0);   /* SYS_GETCWD */
    if (cl > 0) { cwd[cl < 250 ? cl : 250] = 0; puts_(cwd); }
    puts_("\n");
    walk(0);
    sc3(0, 0, 0, 0);
    __builtin_unreachable();
}
