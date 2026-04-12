#include "SovereignCLI_Core.h"

SigmaCLICtx_t g_sigma_cli;

#define CLI_ARGV_MAX 32
#define CLI_TOKEN_BUF 256

static sigma_u32 cli_tokenise(const char *line, char argv_buf[][CLI_TOKEN_BUF], char *argv[], sigma_u32 max) {
    sigma_u32 argc = 0;
    const char *p  = line;
    while (*p && argc < max) {
        while (*p == ' ' || *p == '\t') p++;
        if (!*p) break;
        char *out = argv_buf[argc];
        sigma_u32 len = 0;
        char q = 0;
        while (*p && len < CLI_TOKEN_BUF - 1) {
            if (!q && (*p == '\'' || *p == '"')) { q = *p++; continue; }
            if ( q && *p == q)                   { q = 0; p++; continue; }
            if (!q && (*p == ' ' || *p == '\t')) break;
            out[len++] = *p++;
        }
        out[len] = '\0';
        argv[argc] = out;
        argc++;
    }
    argv[argc] = SIGMA_NULL;
    return argc;
}

static sigma_u32 sigma_cli_hash(const char *str) {
    sigma_u32 hash = 5381;
    int c;
    while ((c = *str++)) hash = ((hash << 5) + hash) + c;
    return hash % SIGMA_CLI_HASH_SIZE;
}

void sigma_cli_init(SigmaCLICtx_t *ctx) {
    sigma_memset(ctx, 0, sizeof(*ctx));
}

sigma_err_t sigma_cli_register(SigmaCLICtx_t *ctx, const char *name, const char *desc, SigmaCLIHandler_t handler) {
    if (ctx->cmd_count >= SIGMA_CLI_MAX_COMMANDS) return SIGMA_ENOSPC;
    sigma_u32 h = sigma_cli_hash(name);
    while (ctx->hash_occupied[h]) h = (h + 1) % SIGMA_CLI_HASH_SIZE;
    SigmaCLICmd_t *cmd = &ctx->cmds[ctx->cmd_count];
    sigma_strcpy(cmd->name, name);
    sigma_strcpy(cmd->description, desc);
    cmd->handler = handler;
    ctx->hash_map[h] = (sigma_u16)ctx->cmd_count;
    ctx->hash_occupied[h] = SIGMA_TRUE;
    ctx->cmd_count++;
    return SIGMA_OK;
}

sigma_err_t sigma_cli_dispatch(SigmaCLICtx_t *ctx, const char *cmdline) {
    static char argv_buf[CLI_ARGV_MAX][CLI_TOKEN_BUF];
    char *argv[CLI_ARGV_MAX + 1];
    sigma_u32 argc = cli_tokenise(cmdline, argv_buf, argv, CLI_ARGV_MAX);
    if (argc == 0) return SIGMA_OK;
    sigma_u32 h = sigma_cli_hash(argv[0]);
    sigma_u32 start = h;
    while (ctx->hash_occupied[h]) {
        SigmaCLICmd_t *cmd = &ctx->cmds[ctx->hash_map[h]];
        if (sigma_streq(cmd->name, argv[0])) return cmd->handler((int)argc, argv);
        h = (h + 1) % SIGMA_CLI_HASH_SIZE;
        if (h == start) break;
    }
    sigma_printf("Σ [CLI]: Unknown command: '%s'\n", argv[0]);
    return SIGMA_ENOENT;
}
