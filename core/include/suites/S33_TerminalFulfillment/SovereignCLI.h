/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN UNIFIED CLI DISPATCHER (v1.0 — PURE C11)
 * =========================================================================
 * Mission: Single-entry-point CLI dispatcher for all sigma-* commands.
 *
 * Built-in commands (inspired by coreutils, util-linux, iproute2, procps):
 *   sigma-sh      — POSIX-compatible interactive shell
 *   sigma-ls      — list directory contents (ls / dir)
 *   sigma-cat     — concatenate / print files
 *   sigma-cp      — copy files & directories
 *   sigma-mv      — move / rename
 *   sigma-rm      — remove files
 *   sigma-mkdir   — create directories
 *   sigma-stat    — file/inode statistics
 *   sigma-find    — recursive file search
 *   sigma-echo    — print arguments
 *   sigma-env     — print/set/unset environment variables (printenv/export)
 *   sigma-ps      — process list (ps aux parity)
 *   sigma-kill    — send signal to process
 *   sigma-top     — live process monitor (top/htop parity)
 *   sigma-uname   — kernel / system information (uname -a)
 *   sigma-dmesg   — print kernel ring buffer
 *   sigma-pkg     — package manager (apt/pacman/nix parity)
 *   sigma-net     — network configuration (ip/ifconfig/netstat parity)
 *   sigma-user    — user/group management (useradd/usermod parity)
 *   sigma-svc     — service management (systemctl/rc-service parity)
 *   sigma-df      — disk free space (df -h parity)
 *   sigma-du      — disk usage (du -sh parity)
 *   sigma-mount   — mount/unmount filesystems
 *   sigma-ctl     — kernel parameter control (sysctl parity)
 *   sigma-hash    — cryptographic hash utilities (sha256sum / md5sum)
 *   sigma-help    — usage information
 * =========================================================================
 */

#ifndef SOVEREIGN_CLI_H
#define SOVEREIGN_CLI_H

#include "sigma_types.h"

/* -------------------------------------------------------------------------
 * Command handler function pointer type
 * ---------------------------------------------------------------------- */
typedef sigma_err_t (*SigmaCLIHandler_t)(int argc, char *argv[]);

/* -------------------------------------------------------------------------
 * Command registration entry
 * ---------------------------------------------------------------------- */
#define SIGMA_CLI_NAME_MAX 32
#define SIGMA_CLI_DESC_MAX 96

typedef struct {
    char               name[SIGMA_CLI_NAME_MAX];
    char               description[SIGMA_CLI_DESC_MAX];
    SigmaCLIHandler_t  handler;
} SigmaCLICmd_t;

/* -------------------------------------------------------------------------
 * CLI context
 * ---------------------------------------------------------------------- */
#define SIGMA_CLI_MAX_COMMANDS 128
#define SIGMA_CLI_HASH_SIZE    256

typedef struct {
    SigmaCLICmd_t cmds[SIGMA_CLI_MAX_COMMANDS];
    sigma_u32     cmd_count;
    
    // Hash Table for O(1) lookup
    sigma_u16     hash_map[SIGMA_CLI_HASH_SIZE]; // Index into cmds[]
    sigma_bool    hash_occupied[SIGMA_CLI_HASH_SIZE];
} SigmaCLICtx_t;

/* -------------------------------------------------------------------------
 * Public API
 * ---------------------------------------------------------------------- */
void        sigma_cli_init      (SigmaCLICtx_t *ctx);
sigma_err_t sigma_cli_register  (SigmaCLICtx_t *ctx,
                                  const char *name,
                                  const char *desc,
                                  SigmaCLIHandler_t handler);
sigma_err_t sigma_cli_dispatch  (SigmaCLICtx_t *ctx,
                                  const char *cmdline);    /* tokenises & dispatches */
void        sigma_cli_help      (const SigmaCLICtx_t *ctx);

/* Global CLI context */
extern SigmaCLICtx_t g_sigma_cli;

/* -------------------------------------------------------------------------
 * Individual command handler declarations
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_cmd_ls    (int argc, char *argv[]);
sigma_err_t sigma_cmd_cat   (int argc, char *argv[]);
sigma_err_t sigma_cmd_cp    (int argc, char *argv[]);
sigma_err_t sigma_cmd_mv    (int argc, char *argv[]);
sigma_err_t sigma_cmd_rm    (int argc, char *argv[]);
sigma_err_t sigma_cmd_mkdir (int argc, char *argv[]);
sigma_err_t sigma_cmd_stat  (int argc, char *argv[]);
sigma_err_t sigma_cmd_find  (int argc, char *argv[]);
sigma_err_t sigma_cmd_echo  (int argc, char *argv[]);
sigma_err_t sigma_cmd_env   (int argc, char *argv[]);
sigma_err_t sigma_cmd_ps    (int argc, char *argv[]);
sigma_err_t sigma_cmd_kill  (int argc, char *argv[]);
sigma_err_t sigma_cmd_top   (int argc, char *argv[]);
sigma_err_t sigma_cmd_uname (int argc, char *argv[]);
sigma_err_t sigma_cmd_dmesg (int argc, char *argv[]);
sigma_err_t sigma_cmd_pkg   (int argc, char *argv[]);
sigma_err_t sigma_cmd_net   (int argc, char *argv[]);
sigma_err_t sigma_cmd_user  (int argc, char *argv[]);
sigma_err_t sigma_cmd_svc   (int argc, char *argv[]);
sigma_err_t sigma_cmd_df    (int argc, char *argv[]);
sigma_err_t sigma_cmd_du    (int argc, char *argv[]);
sigma_err_t sigma_cmd_mount (int argc, char *argv[]);
sigma_err_t sigma_cmd_ctl   (int argc, char *argv[]);
sigma_err_t sigma_cmd_hash  (int argc, char *argv[]);
sigma_err_t sigma_cmd_help  (int argc, char *argv[]);

void SovereignCLI_Init(void);

#endif /* SOVEREIGN_CLI_H */
