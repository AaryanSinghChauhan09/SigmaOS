/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN USERLAND (Suite S21)
 * =========================================================================
 * Shard: Sovereign Shell Core
 * Parity: POSIX sh / Windows CMD / macOS Zsh
 * Design: High-performance tokenizer and job orchestration.
 * =========================================================================
 */

#ifndef SOVEREIGN_SHELL_H
#define SOVEREIGN_SHELL_H

#include "SovereignCommon.h"

#define SHELL_MAX_ARGS    16
#define SHELL_MAX_LINE    1024
#define SHELL_HISTORY_MAX 50

typedef struct {
    char* args[SHELL_MAX_ARGS];
    sigma_u32 arg_count;
    sigma_bool background;
} shell_cmd_t;

/* Public API */
void        sigma_shell_init(void);

/* Command loop */
void        sigma_shell_run(void);
sigma_err_t sigma_shell_execute(const char* line);

/* Job control */
void        sigma_shell_list_jobs(void);

/* Builtins */
void        builtin_cd(shell_cmd_t* cmd);
void        builtin_ps(shell_cmd_t* cmd);
void        builtin_kill(shell_cmd_t* cmd);

#endif /* SOVEREIGN_SHELL_H */
