/* SigmaOS ELF32 user program: tsort — topological sort.
 * Usage: exec TSORT.ELF            (stdin)  OR  exec TSORT.ELF <file>
 * Input lines "a b" mean a must precede b. Kahn's algorithm.
 * Nodes: up to 64 distinct tokens, edges up to 256.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_GETCHAR=2 SYS_WRITE_LEN=8 ; EXIT=0 */
#include "lib.h"

static char nodes[64][32];
static int nnodes = 0;
static int indeg[64];
static char adj[64][64];       /* adj[i][j]=1 if i->j */
static int emitted[64];

static int find_node(const char* s, int len) {
    for (int i = 0; i < nnodes; i++)
        if (u32strlen(nodes[i]) == (u32)len) {
            int k = 0; while (k < len && nodes[i][k] == s[k]) k++;
            if (k == len) return i;
        }
    if (nnodes >= 64 || len > 30) return -1;
    for (int k = 0; k < len; k++) nodes[nnodes][k] = s[k];
    nodes[nnodes][len] = 0;
    indeg[nnodes] = 0; emitted[nnodes] = 0;
    for (int k = 0; k < 64; k++) adj[nnodes][k] = 0;
    return nnodes++;
}

static int getchar_all(void) {
    /* feed via SYS_GETCHAR or file buffer handled by caller */
    return sc3(SYS_GETCHAR, 0, 0, 0);
}

static unsigned char fbuf[16384];
static int fpos = 0, flen = 0, fdone = 0, use_file = 0;
static int g_fd;
static int next_ch(void) {
    if (use_file) {
        if (fpos >= flen) {
            if (fdone) return -1;
            int r = sc3(SYS_READ, g_fd, (u32)fbuf, sizeof fbuf);
            if (r <= 0) { fdone = 1; return -1; }
            fpos = 0; flen = r;
            if ((u32)r < sizeof fbuf) fdone = 1;
        }
        return fbuf[fpos++];
    }
    int c = sc3(SYS_GETCHAR, 0, 0, 0);
    return (c < 0 || c == 0xFF) ? -1 : c;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc > 1) {
        g_fd = sc3(SYS_OPEN, sp[2], 0, 0);
        if (g_fd < 0) { uputs("tsort: cannot open\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
        use_file = 1;
    }
    /* tokenize: whitespace-separated tokens; pairs per line */
    char tok[32]; int tl = 0;
    int line_has = 0;
    char first_tok_node = -1; /* not used; simpler: collect pairs per line */
    /* approach: read token pairs separated by newline boundaries */
    int c;
    /* simpler model: each line "a b" — parse per line */
    char line[128]; int ll = 0;
    while ((c = next_ch()) >= 0) {
        if (c == '\n') {
            /* process line */
            if (ll > 0) {
                line[ll] = 0;
                /* split into two tokens */
                int sp1 = -1;
                for (int i = 0; i < ll; i++) if (line[i] == ' ' || line[i] == '\t') { sp1 = i; break; }
                if (sp1 > 0) {
                    int a = find_node(line, sp1);
                    /* skip spaces */
                    int b0 = sp1; while (b0 < ll && (line[b0]==' '||line[b0]=='\t')) b0++;
                    int bl = ll - b0;
                    int b = find_node(line + b0, bl);
                    if (a >= 0 && b >= 0 && a != b && !adj[a][b]) {
                        adj[a][b] = 1; indeg[b]++;
                    }
                } else {
                    find_node(line, ll);  /* lone node */
                }
            }
            ll = 0;
        } else if (ll < 120) line[ll++] = (char)c;
    }
    if (ll > 0) {
        line[ll] = 0;
        int sp1 = -1;
        for (int i = 0; i < ll; i++) if (line[i] == ' ' || line[i] == '\t') { sp1 = i; break; }
        if (sp1 > 0) {
            int a = find_node(line, sp1);
            int b0 = sp1; while (b0 < ll && (line[b0]==' '||line[b0]=='\t')) b0++;
            int b = find_node(line + b0, ll - b0);
            if (a >= 0 && b >= 0 && a != b && !adj[a][b]) { adj[a][b] = 1; indeg[b]++; }
        } else find_node(line, ll);
    }
    if (use_file) sc3(SYS_CLOSE, g_fd, 0, 0);
    /* Kahn's algorithm */
    int remaining = nnodes, out = 0;
    while (remaining > 0) {
        int pick = -1;
        for (int i = 0; i < nnodes; i++)
            if (!emitted[i] && indeg[i] == 0) { pick = i; break; }
        if (pick < 0) { uputs("tsort: cycle detected\n"); break; }
        emitted[pick] = 1; remaining--;
        uputs(nodes[pick]); uputs("\n");
        for (int j = 0; j < nnodes; j++)
            if (adj[pick][j]) indeg[j]--;
    }
    (void)out;
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}
