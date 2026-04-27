/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SHELL-ENGINE (Atomic Shard)
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef struct OmniShell {
    char        history[100][256];
    u32         hist_count;
    char        cwd[256];
    char        user[32];
} OmniShell;

static OmniShell g_shell;

void shell_init(void) {
    for(int i=0; i<256; i++) g_shell.cwd[i] = 0;
    g_shell.cwd[0] = '/';
    // kprintf("[SHELL-ENGINE]: Atomic Shard Initialized.\n");
}

void shell_exec(const char* cmd) {
    /* Dispatch logic moved to parser shard */
}
