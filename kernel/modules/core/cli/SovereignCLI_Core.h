#ifndef SOVEREIGN_CLI_CORE_H
#define SOVEREIGN_CLI_CORE_H

#include "../../../../include/sigma_kernel.h"

#define SIGMA_CLI_NAME_MAX 32
#define SIGMA_CLI_DESC_MAX 128
#define SIGMA_CLI_MAX_COMMANDS 512
#define SIGMA_CLI_HASH_SIZE 1024

typedef sigma_err_t (*SigmaCLIHandler_t)(int argc, char *argv[]);

typedef struct {
    char name[SIGMA_CLI_NAME_MAX];
    char description[SIGMA_CLI_DESC_MAX];
    SigmaCLIHandler_t handler;
} SigmaCLICmd_t;

typedef struct {
    SigmaCLICmd_t cmds[SIGMA_CLI_MAX_COMMANDS];
    sigma_u32     cmd_count;
    sigma_u16     hash_map[SIGMA_CLI_HASH_SIZE];
    sigma_bool    hash_occupied[SIGMA_CLI_HASH_SIZE];
} SigmaCLICtx_t;

extern SigmaCLICtx_t g_sigma_cli;

void sigma_cli_init(SigmaCLICtx_t *ctx);
sigma_err_t sigma_cli_register(SigmaCLICtx_t *ctx, const char *name, const char *desc, SigmaCLIHandler_t handler);
sigma_err_t sigma_cli_dispatch(SigmaCLICtx_t *ctx, const char *cmdline);

#endif /* SOVEREIGN_CLI_CORE_H */
