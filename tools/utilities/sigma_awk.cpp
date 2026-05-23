/*
 * Σ SigmaOS — sigma_awk: Sovereign Pattern-Action Text Processor
 * Absorbs: GNU awk (gawk), mawk, BusyBox awk — pattern/action engine
 * Features: /pattern/ { action }, $N field splitting, print, NR/NF/FS builtins
 * Zero-Dependency: No libc. Sovereign tokenizer + interpreter.
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
static u32 aw_strlen(const char* s) { u32 n=0; while(s[n]) n++; return n; }
static bool aw_streq(const char* a, const char* b) {
    while(*a&&*b&&*a==*b){a++;b++;} return *a==*b;
}
static void aw_strcpy(char* d, const char* s, u32 max) {
    u32 i=0; while(i<max-1&&s[i]){d[i]=s[i];i++;} d[i]='\0';
}
static void aw_puts(const char* s) { sigma_vga_puts(s); }
static void aw_putln(const char* s) { sigma_vga_puts(s); sigma_vga_putchar('\n'); }

/* ─────────────── Pattern Matcher (same as sigma_sed) ─────────────── */
static bool aw_match_here(const char* pat, const char* txt);
static bool aw_match_star(char c, const char* pat, const char* txt) {
    do {
        if (aw_match_here(pat, txt)) return true;
        if (!*txt) return false;
        if (c!='.' && *txt!=c) return false;
        txt++;
    } while(true);
}
static bool aw_match_here(const char* pat, const char* txt) {
    if (!*pat) return true;
    if (pat[0]=='$'&&!pat[1]) return !*txt;
    if (pat[1]=='*') return aw_match_star(pat[0], pat+2, txt);
    if (pat[0]=='.'||pat[0]==*txt) return *txt&&aw_match_here(pat+1,txt+1);
    return false;
}
static bool aw_match(const char* pat, const char* txt) {
    if (pat[0]=='^') return aw_match_here(pat+1, txt);
    do { if (aw_match_here(pat, txt)) return true; } while(*txt++);
    return false;
}

/* ─────────────── Field Splitting ─────────────── */
#define AWK_MAX_FIELDS 64
#define AWK_FIELD_LEN  256
#define AWK_LINE_LEN   1024

static char  fields[AWK_MAX_FIELDS][AWK_FIELD_LEN];
static u32   NF = 0;

static void split_fields(const char* line, char fs) {
    NF = 0;
    const char* p = line;
    while (*p && NF < AWK_MAX_FIELDS) {
        /* Skip leading FS */
        while (*p == fs && fs == ' ') p++; /* Squash multiple spaces for FS=' ' */
        if (!*p) break;

        u32 fi = 0;
        if (fs == ' ') {
            while (*p && *p != ' ' && *p != '\t' && fi < AWK_FIELD_LEN-1)
                fields[NF][fi++] = *p++;
        } else {
            while (*p && *p != fs && fi < AWK_FIELD_LEN-1)
                fields[NF][fi++] = *p++;
            if (*p == fs) p++;
        }
        fields[NF][fi] = '\0';
        NF++;
    }
}

/* ─────────────── Program Storage ─────────────── */
#define AWK_MAX_RULES  32
#define AWK_PAT_LEN    128
#define AWK_ACT_LEN    512

struct AwkRule {
    char pattern[AWK_PAT_LEN]; /* Empty = match all */
    char action[AWK_ACT_LEN];  /* Commands to execute */
    bool is_begin;
    bool is_end;
};

static AwkRule rules[AWK_MAX_RULES];
static u32     rule_count = 0;

/* ─────────────── Micro-Interpreter ─────────────── */
/* Handles: print $N, print $0, print "literal", print NR, print NF */

static char output_buf[AWK_LINE_LEN];
static u32  NR = 0; /* Number of records processed */

/* Evaluate $N or NR/NF/literal */
static void eval_print_arg(const char* arg, u32 nr) {
    if (arg[0] == '$') {
        if (arg[1] == '0') {
            /* $0 = whole line — reconstruct from fields */
            for (u32 i = 0; i < NF; i++) {
                if (i > 0) sigma_vga_putchar(' ');
                sigma_vga_puts(fields[i]);
            }
            return;
        }
        u32 idx = 0;
        for (u32 i = 1; arg[i] >= '0' && arg[i] <= '9'; i++)
            idx = idx * 10 + (arg[i] - '0');
        if (idx >= 1 && idx <= NF)
            sigma_vga_puts(fields[idx - 1]);
        return;
    }
    if (aw_streq(arg, "NR")) { sigma_vga_printf("%u", nr); return; }
    if (aw_streq(arg, "NF")) { sigma_vga_printf("%u", NF); return; }
    /* String literal (strip quotes if present) */
    if (arg[0] == '"') {
        const char* p = arg + 1;
        while (*p && *p != '"') sigma_vga_putchar(*p++);
        return;
    }
    sigma_vga_puts(arg); /* Bare identifier/value */
}

/* Execute one statement like: print $1, $2 */
static void exec_stmt(const char* stmt, u32 nr) {
    /* Skip leading whitespace */
    while (*stmt == ' ' || *stmt == '\t') stmt++;

    /* print ... */
    if (stmt[0]=='p'&&stmt[1]=='r'&&stmt[2]=='i'&&stmt[3]=='n'&&stmt[4]=='t') {
        const char* args = stmt + 5;
        while (*args == ' ') args++;

        /* Split comma-separated args */
        char arg[AWK_FIELD_LEN];
        u32 ai = 0;
        bool first = true;
        while (*args) {
            if (*args == ',' || *args == '\0') {
                arg[ai] = '\0'; ai = 0;
                /* Trim trailing spaces */
                while (ai > 0 && arg[ai-1]==' ') { arg[ai-1]='\0'; ai--; }
                if (!first) sigma_vga_putchar('\t');
                eval_print_arg(arg, nr);
                first = false;
                if (*args == ',') args++;
                while (*args == ' ') args++;
            } else {
                if (ai < AWK_FIELD_LEN-1) arg[ai++] = *args++;
                else args++;
            }
        }
        if (ai > 0) {
            arg[ai] = '\0';
            if (!first) sigma_vga_putchar('\t');
            eval_print_arg(arg, nr);
        }
        sigma_vga_putchar('\n');
        return;
    }

    /* next: skip to next record (no-op in this interpreter stub) */
    /* exit: terminate processing (stub) */
}

/* Execute an action block */
static void exec_action(const char* action, u32 nr) {
    /* Remove braces if present */
    const char* p = action;
    while (*p == ' ' || *p == '{') p++;
    
    char stmt[AWK_ACT_LEN];
    u32 si = 0;

    while (*p && *p != '}') {
        if (*p == ';' || *p == '\n') {
            stmt[si] = '\0'; si = 0;
            if (aw_strlen(stmt) > 0) exec_stmt(stmt, nr);
        } else {
            if (si < AWK_ACT_LEN-1) stmt[si++] = *p;
        }
        p++;
    }
    if (si > 0) { stmt[si]='\0'; exec_stmt(stmt, nr); }
}

/* ─────────────── Program Parser ─────────────── */
/* Parse "/pattern/ { action }" or "{ action }" */
static bool parse_program(const char* prog) {
    const char* p = prog;
    while (*p && rule_count < AWK_MAX_RULES) {
        while (*p == ' ' || *p == '\n' || *p == '\t') p++;
        if (!*p) break;

        AwkRule* rule = &rules[rule_count];
        rule->pattern[0]  = '\0';
        rule->is_begin    = false;
        rule->is_end      = false;

        /* BEGIN block */
        if (p[0]=='B'&&p[1]=='E'&&p[2]=='G'&&p[3]=='I'&&p[4]=='N') {
            rule->is_begin = true; p += 5;
            while (*p == ' ') p++;
        }
        /* END block */
        else if (p[0]=='E'&&p[1]=='N'&&p[2]=='D') {
            rule->is_end = true; p += 3;
            while (*p == ' ') p++;
        }
        /* /pattern/ */
        else if (*p == '/') {
            p++;
            u32 pi = 0;
            while (*p && *p != '/' && pi < AWK_PAT_LEN-1)
                rule->pattern[pi++] = *p++;
            rule->pattern[pi] = '\0';
            if (*p == '/') p++;
            while (*p == ' ') p++;
        }

        /* { action } */
        if (*p == '{') {
            u32 depth = 1, ai = 0;
            p++; /* skip opening brace */
            while (*p && depth > 0 && ai < AWK_ACT_LEN-1) {
                if (*p == '{') depth++;
                else if (*p == '}') { if (--depth == 0) { p++; break; } }
                rule->action[ai++] = *p++;
            }
            rule->action[ai] = '\0';
            rule_count++;
        } else {
            break; /* Syntax error */
        }
    }
    return true;
}

/* ─────────────── Main ─────────────── */
#define AWK_FILE_MAX (256 * 1024)
static u8 file_buf[AWK_FILE_MAX];

extern "C" int sigma_awk_main(int argc, char** argv) {
    rule_count = 0;
    NR         = 0;
    char fs    = ' '; /* Default field separator */
    const char* program  = nullptr;
    const char* filename = nullptr;

    for (int i = 1; i < argc; i++) {
        if (aw_streq(argv[i], "-F") && i + 1 < argc) {
            fs = argv[++i][0];
        } else if (aw_streq(argv[i], "-f") && i + 1 < argc) {
            /* -f progfile not supported (no stdin in bare-metal) */
            sigma_vga_puts("awk: -f progfile not supported\n");
            return 1;
        } else if (!program) {
            program = argv[i];
        } else if (!filename) {
            filename = argv[i];
        }
    }

    if (!program || !filename) {
        sigma_vga_puts("Usage: awk 'program' file\n");
        return 1;
    }

    if (!parse_program(program)) {
        sigma_vga_puts("awk: syntax error in program\n");
        return 1;
    }

    /* Run BEGIN rules */
    for (u32 r = 0; r < rule_count; r++)
        if (rules[r].is_begin) exec_action(rules[r].action, 0);

    /* Read file */
    u32 len = sigma_fat32_read(filename, file_buf, AWK_FILE_MAX - 1);
    if (!len) { sigma_vga_puts("awk: cannot read file\n"); return 1; }
    file_buf[len] = '\0';

    /* Process line by line */
    char line[AWK_LINE_LEN];
    u32 pos = 0;
    while (pos < len) {
        u32 li = 0;
        while (pos < len && file_buf[pos] != '\n' && li < AWK_LINE_LEN-1)
            line[li++] = (char)file_buf[pos++];
        line[li] = '\0';
        if (pos < len && file_buf[pos] == '\n') pos++;

        NR++;
        split_fields(line, fs);

        for (u32 r = 0; r < rule_count; r++) {
            if (rules[r].is_begin || rules[r].is_end) continue;
            bool match = (rules[r].pattern[0] == '\0') ||
                         aw_match(rules[r].pattern, line);
            if (match) exec_action(rules[r].action, NR);
        }
    }

    /* Run END rules */
    for (u32 r = 0; r < rule_count; r++)
        if (rules[r].is_end) exec_action(rules[r].action, NR);

    return 0;
}
