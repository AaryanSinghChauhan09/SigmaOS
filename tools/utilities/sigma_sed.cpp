/*
 * Σ SigmaOS — sigma_sed: Sovereign Stream Editor
 * Absorbs: GNU sed(1), BusyBox sed, Plan 9 sed concepts
 * Features: s/pattern/replace/[g|p|d] substitute, /pattern/d delete, /pattern/p print
 * Zero-Dependency: No libc, no stdlib. Raw kernel I/O + sovereign string engine.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" u32  sigma_fat32_read(const char* name, u8* buf, u32 max);

/* ─────────────── String Primitives ─────────────── */
static u32 sed_strlen(const char* s) { u32 n=0; while(s[n]) n++; return n; }
static bool sed_streq(const char* a, const char* b) {
    while (*a && *b && *a==*b) { a++; b++; } return *a==*b;
}
static void sed_strcpy(char* d, const char* s) { while ((*d++=*s++)); }
static void sed_puts(const char* s) { sigma_vga_puts(s); sigma_vga_putchar('\n'); }

/* ─────────────── Sovereign Regex: Simple Pattern Matcher ─────────────── */
/* Supports: literal chars, . (any), * (zero-or-more of prev), ^ (start), $ (end) */
static bool match_here(const char* pattern, const char* text);

static bool match_star(char c, const char* pattern, const char* text) {
    do {
        if (match_here(pattern, text)) return true;
        if (!*text) return false;
        if (c != '.' && *text != c) return false;
        text++;
    } while (true);
}

static bool match_here(const char* pattern, const char* text) {
    if (pattern[0] == '\0') return true;
    if (pattern[0] == '$' && pattern[1] == '\0') return (*text == '\0');
    if (pattern[1] == '*') return match_star(pattern[0], pattern + 2, text);
    if (pattern[0] == '.' || (pattern[0] == *text && *text))
        return match_here(pattern + 1, text + 1);
    return false;
}

static bool sed_match(const char* pattern, const char* text) {
    if (pattern[0] == '^') return match_here(pattern + 1, text);
    do {
        if (match_here(pattern, text)) return true;
    } while (*text++);
    return false;
}

/* Find pattern in text; return pointer to match start, or null */
static const char* sed_find(const char* pattern, const char* text) {
    if (pattern[0] == '^') {
        return match_here(pattern + 1, text) ? text : nullptr;
    }
    const char* t = text;
    do {
        if (match_here(pattern, t)) return t;
    } while (*t++);
    return nullptr;
}

/* ─────────────── Substitute Engine ─────────────── */
/* Replaces first (or all with /g) occurrences of pattern with replacement */
static bool sed_substitute(const char* line, const char* pattern,
                            const char* replace, bool global,
                            char* out, u32 out_max) {
    bool changed = false;
    u32 oi = 0;
    const char* src = line;

    while (*src && oi < out_max - 1) {
        /* Try to match pattern at current position */
        const char* mstart = src;
        bool found = false;
        if (pattern[0] == '^') {
            /* Anchored: only match at start of remaining input */
            if (src == line && match_here(pattern + 1, src)) found = true;
        } else {
            if (match_here(pattern, src)) found = true;
        }

        if (!found) {
            /* No match: copy one char */
            out[oi++] = *src++;
            continue;
        }

        /* Compute match length by advancing pattern over src */
        /* Simple: re-advance until match fails */
        const char* mend = src;
        /* Advance mend to end of match */
        {
            const char* p = pattern[0]=='^' ? pattern+1 : pattern;
            const char* t = src;
            /* We know p matches at t; advance t as far as p consumes */
            while (*p) {
                if (p[1]=='*') {
                    char c = p[0]; p+=2;
                    while (*t && (c=='.' || *t==c)) t++;
                } else if (*p=='.') { if(*t) t++; p++; }
                else if (*p==*t) { t++; p++; }
                else break;
            }
            mend = t;
        }

        /* Emit replacement */
        const char* r = replace;
        while (*r && oi < out_max - 1) out[oi++] = *r++;

        changed = true;
        src = mend;
        if (src == mstart) { /* Zero-length match guard */
            if (*src) out[oi++] = *src++;
        }
        if (!global) {
            /* Copy rest verbatim */
            while (*src && oi < out_max - 1) out[oi++] = *src++;
            break;
        }
    }
    out[oi] = '\0';
    return changed;
}

/* ─────────────── Script Parser ─────────────── */
#define SED_SCRIPT_LEN 512
#define SED_PAT_LEN    128
#define SED_REPL_LEN   256
#define MAX_SED_CMDS   32

enum SedCmdType { SED_SUB, SED_DELETE, SED_PRINT, SED_QUIT };

struct SedCmd {
    SedCmdType type;
    char addr_pattern[SED_PAT_LEN]; /* address pattern (empty = all lines) */
    char pattern[SED_PAT_LEN];
    char replace[SED_REPL_LEN];
    bool global;
    bool print_match; /* s///p flag */
};

static SedCmd cmds[MAX_SED_CMDS];
static u32    cmd_count = 0;

/* Parse "s/pat/repl/flags" */
static bool parse_sub(const char* expr, SedCmd* cmd) {
    if (expr[0] != 's') return false;
    char delim = expr[1];
    if (!delim) return false;

    u32 pi = 0, ri = 0;
    const char* p = expr + 2;

    /* Extract pattern */
    while (*p && *p != delim && pi < SED_PAT_LEN-1) cmd->pattern[pi++] = *p++;
    cmd->pattern[pi] = '\0';
    if (*p != delim) return false;
    p++;

    /* Extract replacement */
    while (*p && *p != delim && ri < SED_REPL_LEN-1) cmd->replace[ri++] = *p++;
    cmd->replace[ri] = '\0';
    if (*p == delim) p++;

    /* Parse flags */
    cmd->global = false; cmd->print_match = false;
    while (*p) {
        if (*p == 'g') cmd->global = true;
        if (*p == 'p') cmd->print_match = true;
        p++;
    }
    cmd->type = SED_SUB;
    return true;
}

/* Parse one sed expression: [/addr/]cmd */
static bool parse_expr(const char* expr, SedCmd* cmd) {
    cmd->addr_pattern[0] = '\0';
    const char* p = expr;

    /* Optional address */
    if (*p == '/') {
        p++;
        u32 ai = 0;
        while (*p && *p != '/' && ai < SED_PAT_LEN-1) cmd->addr_pattern[ai++] = *p++;
        cmd->addr_pattern[ai] = '\0';
        if (*p == '/') p++;
    }

    /* Command */
    if (*p == 's') return parse_sub(p, cmd);
    if (*p == 'd') { cmd->type = SED_DELETE; return true; }
    if (*p == 'p') { cmd->type = SED_PRINT;  return true; }
    if (*p == 'q') { cmd->type = SED_QUIT;   return true; }
    return false;
}

/* ─────────────── Line Processing ─────────────── */
#define SED_LINE_BUF 2048
#define SED_OUT_BUF  2048
#define SED_FILE_MAX (256 * 1024)

static u8   file_buf[SED_FILE_MAX];
static char out_line[SED_OUT_BUF];

static void process_line(const char* line, bool* should_quit) {
    char current[SED_LINE_BUF];
    u32 li = 0;
    while (line[li] && li < SED_LINE_BUF-1) { current[li] = line[li]; li++; }
    current[li] = '\0';

    bool deleted = false;
    bool printed_extra = false;

    for (u32 i = 0; i < cmd_count && !deleted; i++) {
        SedCmd* c = &cmds[i];

        /* Check address match */
        bool addr_match = (c->addr_pattern[0] == '\0') ||
                          sed_match(c->addr_pattern, current);
        if (!addr_match) continue;

        switch (c->type) {
            case SED_DELETE:
                deleted = true;
                break;
            case SED_PRINT:
                sed_puts(current);
                printed_extra = true;
                break;
            case SED_QUIT:
                sed_puts(current);
                *should_quit = true;
                return;
            case SED_SUB:
                bool changed = sed_substitute(current, c->pattern, c->replace,
                                              c->global, out_line, SED_OUT_BUF);
                if (changed) {
                    sed_strcpy(current, out_line);
                    if (c->print_match) sed_puts(current);
                }
                break;
        }
    }

    if (!deleted) sed_puts(current);
}

/* ─────────────── Main ─────────────── */
extern "C" int sigma_sed_main(int argc, char** argv) {
    cmd_count = 0;
    const char* filename = nullptr;
    bool in_place = false; /* -i stub */

    for (int i = 1; i < argc; i++) {
        if (argv[i][0] == '-' && argv[i][1] == 'e') {
            /* -e 'expr' */
            if (i + 1 < argc && cmd_count < MAX_SED_CMDS) {
                if (parse_expr(argv[++i], &cmds[cmd_count])) cmd_count++;
            }
        } else if (argv[i][0] == '-' && argv[i][1] == 'i') {
            in_place = true; /* acknowledged but not yet implemented */
        } else if (argv[i][0] != '-' && cmd_count == 0) {
            /* First non-flag arg is the script */
            if (cmd_count < MAX_SED_CMDS && parse_expr(argv[i], &cmds[cmd_count]))
                cmd_count++;
        } else {
            filename = argv[i];
        }
    }

    if (!filename) {
        sigma_vga_puts("sed: no input file specified\n");
        return 1;
    }

    u32 len = sigma_fat32_read(filename, file_buf, SED_FILE_MAX - 1);
    if (!len) { sigma_vga_puts("sed: cannot read file\n"); return 1; }
    file_buf[len] = '\0';

    /* Process line by line */
    char line[SED_LINE_BUF];
    u32 pos = 0;
    bool quit = false;

    while (pos < len && !quit) {
        u32 li = 0;
        while (pos < len && file_buf[pos] != '\n' && li < SED_LINE_BUF-1)
            line[li++] = (char)file_buf[pos++];
        line[li] = '\0';
        if (pos < len && file_buf[pos] == '\n') pos++;
        process_line(line, &quit);
    }
    return 0;
}
