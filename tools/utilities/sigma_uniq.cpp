/*
 * Σ SigmaOS — sigma_uniq: Sovereign Duplicate-Line Filter
 * Absorbs: GNU coreutils uniq(1), BusyBox uniq
 * Features: -c count, -d duplicates only, -u unique only, -i ignore case
 * Zero-Dependency: No libc, raw kernel I/O only.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" u32  sigma_fat32_read(const char* name, u8* buf, u32 max);

/* ─────────────── String Helpers ─────────────── */
static bool uq_streq_nocase(const char* a, const char* b) {
    while (*a && *b) {
        char ca = (*a >= 'A' && *a <= 'Z') ? (*a + 32) : *a;
        char cb = (*b >= 'A' && *b <= 'Z') ? (*b + 32) : *b;
        if (ca != cb) return false;
        a++; b++;
    }
    return *a == *b;
}
static bool uq_streq(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return *a == *b;
}
static u32 uq_strlen(const char* s) { u32 n=0; while(s[n]) n++; return n; }

/* ─────────────── Line Buffer ─────────────── */
#define UQ_BUF  (512 * 1024)
#define UQ_MAXL 4096
#define UQ_LL   512

static u8    file_buf[UQ_BUF];
static char  line_store[UQ_BUF];
static char* lptr[UQ_MAXL];
static u32   lcount = 0;

static u32 uniq_extract(u32 len) {
    u32 count = 0, pos = 0, spos = 0;
    for (u32 i = 0; i < len && i < UQ_BUF - 1; i++)
        line_store[i] = (char)file_buf[i];

    while (pos < len && count < UQ_MAXL) {
        lptr[count] = line_store + spos;
        u32 end = pos;
        while (end < len && line_store[end] != '\n') end++;
        line_store[end] = '\0';
        spos = end + 1;
        count++;
        pos = end + 1;
    }
    return count;
}

/* ─────────────── Print Helpers ─────────────── */
static void print_u32(u32 n) {
    if (n >= 10) print_u32(n / 10);
    sigma_vga_putchar('0' + (n % 10));
}

/* ─────────────── Main ─────────────── */
extern "C" int sigma_uniq_main(int argc, char** argv) {
    bool opt_count      = false; /* -c: prefix each line with count */
    bool opt_dup_only   = false; /* -d: only print duplicate lines */
    bool opt_uniq_only  = false; /* -u: only print unique lines */
    bool opt_ignore_case= false; /* -i: ignore case comparisons */
    const char* filename= nullptr;

    for (int i = 1; i < argc; i++) {
        char* a = argv[i];
        if (a[0] == '-') {
            for (int j = 1; a[j]; j++) {
                switch (a[j]) {
                    case 'c': opt_count       = true; break;
                    case 'd': opt_dup_only     = true; break;
                    case 'u': opt_uniq_only    = true; break;
                    case 'i': opt_ignore_case  = true; break;
                }
            }
        } else {
            filename = a;
        }
    }

    if (!filename) {
        sigma_vga_puts("uniq: no input file specified\n");
        return 1;
    }

    u32 len = sigma_fat32_read(filename, file_buf, UQ_BUF - 1);
    if (!len) { sigma_vga_puts("uniq: cannot read file\n"); return 1; }
    file_buf[len] = '\0';
    lcount = uniq_extract(len);
    if (!lcount) return 0;

    /* Process runs of identical lines */
    u32 i = 0;
    while (i < lcount) {
        u32 run = 1;
        while (i + run < lcount) {
            bool eq = opt_ignore_case
                ? uq_streq_nocase(lptr[i], lptr[i + run])
                : uq_streq(lptr[i], lptr[i + run]);
            if (!eq) break;
            run++;
        }

        bool should_print = true;
        if (opt_dup_only  && run == 1) should_print = false;
        if (opt_uniq_only && run >  1) should_print = false;

        if (should_print) {
            if (opt_count) {
                /* Pad count field to 7 chars (coreutils style) */
                u32 pad = (run < 1000000) ? (run < 100000 ? (run < 10000 ?
                    (run < 1000 ? (run < 100 ? (run < 10 ? 6 : 5) : 4) : 3) : 2) : 1) : 0;
                for (u32 p = 0; p < pad; p++) sigma_vga_putchar(' ');
                print_u32(run);
                sigma_vga_putchar(' ');
            }
            sigma_vga_puts(lptr[i]);
            sigma_vga_putchar('\n');
        }
        i += run;
    }
    return 0;
}
