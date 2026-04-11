/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN COMMAND INTERFACE (v1.0)
 * =========================================================================
 * Mission: Modular command dispatching for the Omni-CLI.
 * Design: C11 / Zero-Dependency / Extensible.
 * =========================================================================
 */

#ifndef SOVEREIGN_COMMAND_H
#define SOVEREIGN_COMMAND_H

#include "sigma_types.h"

#define MAX_COMMANDS 512
#define CMD_NAME_MAX 32
#define CMD_DESC_MAX 128

typedef void (*sigma_cmd_handler_t)(int argc, char** argv);

typedef struct {
    char name[CMD_NAME_MAX];
    char description[CMD_DESC_MAX];
    sigma_cmd_handler_t handler;
    sigma_u32 security_level;
} sovereign_command_t;

typedef struct {
    sovereign_command_t commands[MAX_COMMANDS];
    sigma_u32 command_count;
} sovereign_command_registry_t;

/* Public API */
void SovereignCommand_Init(void);
sigma_err_t SovereignCommand_Register(const char* name, const char* desc, sigma_cmd_handler_t handler);
void SovereignCommand_Dispatch(int argc, char** argv);
void SovereignCommand_ListAll(void);

#endif /* SOVEREIGN_COMMAND_H */
