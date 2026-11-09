/* SigmaOS ELF32 user program: test — POSIX-style condition tests.
 * Usage: exec TEST.ELF <expr>
 *   -e <f> -f <f> -d <f> -z <s> -n <s> <a> = <b> <a> != <b>
 *   <a> -eq|-ne|-lt|-le|-gt|-ge <b>
 * Exit 0 = true, 1 = false.  SYS_STAT=16 SYS_READDIRX=20 SYS_EXIT=0 */
#include "lib.h"

static int atoi_(const char* s) {
    int neg = 0, v = 0;
    if (*s == '-') { neg = 1; s++; }
    while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; }
    return neg ? -v : v;
}
static int streq(const char* a, const char* b) {
    while (*a && *a == *b) { a++; b++; }
    return *a == *b;
}
/* is_dir via READDIRX on parent? simpler: STAT returns size; dirs size ambiguous.
   Use CHDIR probe: chdir(path) succeeds only on dirs, then chdir("..") back. */
static int is_dir(const char* path) {
    /* save cwd, try chdir */
    static char cwd[256];
    sc3(SYS_GETCWD, (u32)cwd, sizeof cwd, 0);
    int r = sc3(SYS_CHDIR, (u32)path, 0, 0);
    if (r == 0) {
        sc3(SYS_CHDIR, (u32)cwd, 0, 0);
        return 1;
    }
    return 0;
}
static int file_exists(const char* path) {
    /* try STAT; dirs also have dirents so STAT may work — use OPEN? */
    return sc3(SYS_STAT, (u32)path, 0, 0) >= 0 || is_dir(path);
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    int ok = 0;
    if (argc == 3) {
        const char* op = (const char*)sp[2];
        const char* a = (const char*)sp[3];
        if (op[0] == '-' && op[1] == 'e' && !op[2]) ok = file_exists(a);
        else if (op[0] == '-' && op[1] == 'f' && !op[2]) ok = (sc3(SYS_STAT,(u32)a,0,0) >= 0 && !is_dir(a));
        else if (op[0] == '-' && op[1] == 'd' && !op[2]) ok = is_dir(a);
        else if (op[0] == '-' && op[1] == 'z' && !op[2]) ok = (a[0] == 0);
        else if (op[0] == '-' && op[1] == 'n' && !op[2]) ok = (a[0] != 0);
    } else if (argc == 4) {
        const char* a = (const char*)sp[2];
        const char* op = (const char*)sp[3];
        const char* b = (const char*)sp[4];
        if (op[0] == '=' && !op[1]) ok = streq(a, b);
        else if (op[0] == '!' && op[1] == '=') ok = !streq(a, b);
        else {
            int x = atoi_(a), y = atoi_(b);
            if (op[0]=='-'&&op[1]=='e'&&op[2]=='q') ok = (x==y);
            else if (op[0]=='-'&&op[1]=='n'&&op[2]=='e') ok = (x!=y);
            else if (op[0]=='-'&&op[1]=='l'&&op[2]=='t') ok = (x<y);
            else if (op[0]=='-'&&op[1]=='l'&&op[2]=='e') ok = (x<=y);
            else if (op[0]=='-'&&op[1]=='g'&&op[2]=='t') ok = (x>y);
            else if (op[0]=='-'&&op[1]=='g'&&op[2]=='e') ok = (x>=y);
        }
    }
    sc3(SYS_EXIT, ok ? 0 : 1, 0, 0);
    __builtin_unreachable();
}
