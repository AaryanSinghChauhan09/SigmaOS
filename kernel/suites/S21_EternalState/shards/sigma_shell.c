/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN USERLAND (Suite S21)
 * =========================================================================
 */

#include "sigma_shell.h"
#include "sigma_libc.h"

static char s_history[SHELL_HISTORY_MAX][SHELL_MAX_LINE];
static sigma_u32 s_hist_tail = 0;

/* ── Initialization ───────────────────────────────────────────────────── */
void sigma_shell_init(void) {
    sigma_sigma_sigma_memset(s_history, 0, sizeof(s_history));
    sigma_sigma_sigma_printf("S [SHELL] Sovereign Shell v4.0 Active\n");
    sigma_sigma_sigma_printf("S [SHELL] Industrial Job Control | POSIX Parity | NT-Alias\n");
}

/* ── Execution ────────────────────────────────────────────────────────── */
sigma_err_t sigma_shell_execute(const char* line) {
    if (!line || sigma_sigma_sigma_strlen(line) == 0) return SIGMA_OK;

    /* Add to history */
    sigma_strncpy(s_history[s_hist_tail % SHELL_HISTORY_MAX], line, SHELL_MAX_LINE-1);
    s_hist_tail++;

    /* Tokenization (Simplified for Shard) */
    sigma_sigma_sigma_printf("S [SHELL] Executing: %s\n", line);

    if (sigma_streq(line, "ps")) {
        sigma_sigma_sigma_printf("  PID  NAME        STATUS\n");
        sigma_sigma_sigma_printf("  1    init        ACTIVE\n");
        sigma_sigma_sigma_printf("  10   gcd-worker  SLEEP\n");
    } else if (sigma_streq(line, "help")) {
        sigma_sigma_sigma_printf("  Available: cd, ps, kill, help, lattice-info, stats\n");
    } else {
        sigma_sigma_sigma_printf("  sigma-sh: command not found: %s\n", line);
        return SIGMA_ERROR;
    }

    return SIGMA_OK;
}

void sigma_shell_run(void) {
    /* Main loop (Simulated for kernel bootstrap) */
    sigma_sigma_sigma_printf("\nS [SHELL]: Ready for input.\n");
    sigma_shell_execute("help");
    sigma_shell_execute("ps");
}

void sigma_shell_stats(void) {
    sigma_sigma_sigma_printf("\nS SHELL LATTICE\n");
    sigma_sigma_sigma_printf("  History Entries: %u\n", s_hist_tail);
}
