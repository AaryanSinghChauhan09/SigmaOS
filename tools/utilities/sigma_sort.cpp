/*
 * Σ SigmaOS — sigma_sort: Sovereign Line Sorter
 * Absorbs: GNU coreutils sort(1), Alpine BusyBox sort
 * Features: lexicographic sort, -r reverse, -n numeric, -u unique
 * Zero-Dependency: No libc, no stdlib, raw kernel I/O only.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" u32  sigma_fat32_read(const char* name, u8* buf, u32 max);

/* ─────────────── Sovereign String Helpers ─────────────── */
static u32 sv_strlen(const char* s) {
    u32 n = 0; while (s[n]) n++; return n;
}
static bool sv_streq(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return *a == *b;
}
static void sv_putline(const char* s) {
    sigma_vga_puts(s); sigma_vga_putchar('\n');
}

/* ─────────────── Numeric Parse (no atoi/strtol) ─────────────── */
static long sv_atol(const char* s) {
    long result = 0; bool neg = false;
    if (*s == '-') { neg = true; s++; }
    while (*s >= '0' && *s <= '9') { result = result * 10 + (*s - '0'); s++; }
    return neg ? -result : result;
}

/* ─────────────── Line Extraction ─────────────── */
#define MAX_LINES  4096
#define MAX_LINE_LEN 512
#define FILE_BUF_SIZE (MAX_LINES * MAX_LINE_LEN)

static char file_storage[FILE_BUF_SIZE];
static char* lines[MAX_LINES];
static u32   line_count = 0;

static u32 extract_lines(const char* buf, u32 len) {
    u32 count = 0;
    u32 start = 0;
    /* Copy buf into file_storage and split */
    for (u32 i = 0; i < len && i < FILE_BUF_SIZE - 1; i++)
        file_storage[i] = buf[i];

    u32 pos = 0;
    while (pos < len && count < MAX_LINES) {
        lines[count] = file_storage + pos;
        u32 end = pos;
        while (end < len && file_storage[end] != '\n') end++;
        file_storage[end] = '\0';
        count++;
        pos = end + 1;
    }
    return count;
}

/* ─────────────── Comparison Functions ─────────────── */
static int cmp_lex(const char* a, const char* b) {
    while (*a && *b) {
        if (*a < *b) return -1;
        if (*a > *b) return  1;
        a++; b++;
    }
    if (!*a && !*b) return 0;
    return (!*a) ? -1 : 1;
}

static int cmp_num(const char* a, const char* b) {
    long la = sv_atol(a), lb = sv_atol(b);
    return (la < lb) ? -1 : (la > lb) ? 1 : 0;
}

/* ─────────────── QuickSort (Hoare partition) ─────────────── */
static bool opt_reverse = false;
static bool opt_numeric = false;
static bool opt_unique  = false;

static int do_cmp(const char* a, const char* b) {
    int r = opt_numeric ? cmp_num(a, b) : cmp_lex(a, b);
    return opt_reverse ? -r : r;
}

static void qs_sort(u32 lo, u32 hi) {
    if (lo >= hi) return;
    const char* pivot = lines[(lo + hi) / 2];
    u32 i = lo, j = hi;
    while (true) {
        while (do_cmp(lines[i], pivot) < 0) i++;
        while (do_cmp(lines[j], pivot) > 0) j--;
        if (i >= j) break;
        const char* tmp = lines[i]; lines[i] = lines[j]; lines[j] = tmp;
        i++; if (j > 0) j--;
    }
    if (lo < j) qs_sort(lo, j);
    qs_sort(j + 1, hi);
}

/* ─────────────── Main ─────────────── */
extern "C" int sigma_sort_main(int argc, char** argv) {
    opt_reverse = opt_numeric = opt_unique = false;
    const char* filename = nullptr;

    for (int i = 1; i < argc; i++) {
        if (sv_streq(argv[i], "-r")) opt_reverse = true;
        else if (sv_streq(argv[i], "-n")) opt_numeric = true;
        else if (sv_streq(argv[i], "-u")) opt_unique  = true;
        else filename = argv[i];
    }

    static u8 fbuf[FILE_BUF_SIZE];
    u32 len = 0;

    if (filename) {
        len = sigma_fat32_read(filename, fbuf, FILE_BUF_SIZE - 1);
        if (!len) { sigma_vga_puts("sort: cannot read file\n"); return 1; }
        fbuf[len] = '\0';
    } else {
        sigma_vga_puts("sort: no input (stdin not supported in bare-metal mode)\n");
        return 1;
    }

    line_count = extract_lines((const char*)fbuf, len);
    if (!line_count) return 0;

    qs_sort(0, line_count - 1);

    for (u32 i = 0; i < line_count; i++) {
        if (opt_unique && i > 0 && sv_streq(lines[i], lines[i-1])) continue;
        sv_putline(lines[i]);
    }
    return 0;
}
