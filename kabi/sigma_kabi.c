/* sigma_kabi.c
 * SigmaOS Kernel ABI Checker — replaces legacy kabi/check.py
 * Uses only POSIX libc (stdio.h, string.h) — zero Python dependency.
 * Checks that exported kernel symbols match an approved ABI manifest.
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define MAX_SYMBOLS 512
#define SYM_LEN     128

/* ── Approved ABI Symbol Table ────────────────────────────────────────────
 * Populated from the KABI manifest. In a full implementation this is
 * loaded from a signed binary manifest file.
 */
static const char *APPROVED_SYMBOLS[] = {
    "sigma_kmalloc",
    "sigma_kfree",
    "sigma_printk",
    "sigma_schedule",
    "sigma_alloc_pages",
    "sigma_free_pages",
    "sigma_mmap_region",
    "sigma_ipc_send",
    "sigma_ipc_recv",
    "sigma_sandbox_enter",
    "sigma_sandbox_exit",
    NULL
};

/* ── Symbol Check ────────────────────────────────────────────────────────── */
static int is_approved(const char *sym) {
    for (int i = 0; APPROVED_SYMBOLS[i] != NULL; i++) {
        if (strcmp(sym, APPROVED_SYMBOLS[i]) == 0) return 1;
    }
    return 0;
}

/* ── Read exported symbols from a text file ─────────────────────────────── */
static int check_kabi(const char *symfile) {
    FILE *f = fopen(symfile, "r");
    if (!f) {
        fprintf(stderr, "sigma_kabi: cannot open symbol file: %s\n", symfile);
        return 1;
    }

    char line[SYM_LEN];
    int violations = 0;
    int checked = 0;

    while (fgets(line, sizeof(line), f)) {
        /* Strip newline */
        size_t len = strlen(line);
        if (len > 0 && line[len-1] == '\n') line[len-1] = '\0';
        if (len == 0 || line[0] == '#') continue;

        checked++;
        if (!is_approved(line)) {
            fprintf(stderr, "[KABI VIOLATION] Unknown symbol: %s\n", line);
            violations++;
        }
    }
    fclose(f);

    printf("[sigma_kabi] Checked %d symbols. Violations: %d\n", checked, violations);
    return violations > 0 ? 2 : 0;
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: sigma_kabi <symbol_list.txt>\n");
        return 1;
    }
    return check_kabi(argv[1]);
}
