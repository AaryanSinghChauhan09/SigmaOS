/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN SHELL (sigma-sh v1.0)
 * =============================================================================
 * Mission: Minimal kernel-mode interactive shell for debugging, recovery, and
 *          system administration with built-in commands and pipe support.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_SHELL_H
#define SIGMA_SHELL_H

#include "../sigma_kernel_types.h"

#define SHELL_MAX_COMMANDS    64
#define SHELL_CMD_NAME_LEN    24
#define SHELL_CMD_DESC_LEN    64
#define SHELL_INPUT_BUF_LEN  256
#define SHELL_MAX_ARGS        16
#define SHELL_HISTORY_SIZE     8
#define SHELL_MAX_PIPES        4

/* Command handler: receives argc, argv; returns 0 on success */
typedef int (*sigma_shell_handler_t)(int argc, const char* argv[]);

typedef struct {
    char                   name[SHELL_CMD_NAME_LEN];
    char                   description[SHELL_CMD_DESC_LEN];
    sigma_shell_handler_t  handler;
    sigma_bool             kernel_only;   /* requires kernel privilege */
} sigma_shell_cmd_t;

#ifdef __cplusplus
extern "C" {
#endif

void  shell_init(void);
int   shell_register_cmd(const char* name, const char* desc,
                         sigma_shell_handler_t handler, sigma_bool kernel_only);
int   shell_execute(const char* input_line);
void  shell_run_interactive(void);
void  shell_print_help(void);
void  shell_print_prompt(void);
sigma_u32 shell_get_command_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SHELL_H */
