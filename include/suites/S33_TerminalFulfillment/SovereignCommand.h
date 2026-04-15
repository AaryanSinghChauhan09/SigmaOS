/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN COMMAND INTERFACE (v1.0)
 * =========================================================================
 * Mission: Pluggable CLI commands for community extension.
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_COMMAND_H
#define SOVEREIGN_COMMAND_H

#include "sigma_types.h"

typedef sigma_err_t (*sigma_cmd_exec_fn)(int argc, char** argv);

typedef struct {
    char name[32];
    char description[64];
    sigma_cmd_exec_fn execute;
} sovereign_command_shard_t;

/* Registry API */
void SovereignCommand_InitRegistry(void);
sigma_err_t SovereignCommand_Register(const char* name, const char* desc, sigma_cmd_exec_fn exec);
sigma_err_t SovereignCommand_Dispatch(const char* name, int argc, char** argv);

#endif /* SOVEREIGN_COMMAND_H */
