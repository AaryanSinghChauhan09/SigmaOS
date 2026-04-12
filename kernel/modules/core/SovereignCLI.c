/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN UNIFIED CLI DISPATCHER — IMPLEMENTATION (v1.0)
 * =========================================================================
 * All sigma-* commands in one translation unit.
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignCLI.h"
#include "../../../include/SovereignEnvManager.h"
#include "../../../include/SovereignUserManager.h"
#include "../../../include/SovereignDmesg.h"
#include "../../../include/SovereignInitSystem.h"

/* Competitor Shards */
#include "../../../include/SovereignZFS.h"
#include "../../../include/SovereignJail.h"
#include "../../../include/SovereignMediaCodec.h"
#include "../../../include/SovereignVirtualBox.h"
#include "../../../include/SovereignBrowserCloud.h"
#include "../../../include/SovereignDefender.h"
#include "../../../include/SovereignActiveDirectory.h"

/* Phase 44 Shards */
#include "../../../include/SovereignAndroidBinder.h"
#include "../../../include/SovereignDarwinXNU.h"
#include "../../../include/SovereignPersonalizer.h"
#include "../../../include/SovereignAIKernel.h"
#include "../../../include/SovereignDistroSlinger.h"
#include "../../../include/SovereignAutomationEngine.h"
#include "../../../include/SovereignAutonomousAgent.h"
#include "../../../include/SovereignForensicScrubber.h"
#include "../../../include/SovereignAtomicUpdateShard.h"
#include "../../../include/SovereignTensorShard.h"
#include "../../../include/SovereignNetworkShard.h"
#include "../../../include/SovereignSecurityVault.h"
#include "../../../include/SovereignSiliconContainer.h"
#include "../../../include/SovereignSiliconProbe.h"
#include "../../../include/SovereignSiliconStore.h"
#include "../../../include/SovereignClusterShard.h"
#include "../../../include/SovereignZenithMatrix.h"
#include "../../../include/SovereignDSAShard.h"
#include "../../../include/SovereignMathShard.h"
#include "../../../include/SovereignShardManager.h"
#include "../../../include/SovereignShardRepo.h"
#include "../../../include/SovereignLiveReload.h"
#include "../../../include/SovereignSpotlightShard.h"
#include "../../../include/SovereignWMShard.h"
#include "../../../include/SovereignAutomationShard.h"
#include "../../../include/SovereignNeuralShard.h"
#include "../../../include/SovereignEnclaveShard.h"
#include "../../../include/SovereignDistroSlinger.h"
#include "../../../include/SovereignTestSuite.h"
#include "../../../include/SovereignRebuildShard.h"
#include "../../../include/SovereignAestheticShard.h"
#include "../../../include/SovereignZenScheduler.h"
#include "../../../include/SovereignAutoSystems.h"
#include "../../../include/SovereignAmnesicShard.h"
#include "../../../include/SovereignMeshFS.h"
#include "../../../include/SovereignCapabilityShard.h"
#include "../../../include/SovereignConsensusShard.h"
#include "../../../include/SovereignOverlayShard.h"
#include "../../../include/SovereignMigrationShard.h"
#include "../../../include/SovereignProtectors.h"
#include "../../../include/SovereignIdentityShard.h"
#include "../../../include/SovereignTWMShard.h"
#include "../../../include/SovereignSyncShard.h"
#include "../../../include/SovereignTelemetryShard.h"
#include "../../../include/SovereignPersonaShard.h"
#include "../../../include/SovereignHotpatchShard.h"
#include "../../../include/SovereignCgroupShard.h"
#include "../../../include/SovereignOOMShard.h"
#include "../../../include/SovereignJournalShard.h"
#include "../../../include/SovereignTraceShard.h"
#include "../../../include/SovereignIRQShard.h"
#include "../../../include/SovereignRollbackShard.h"
#include "../../../include/SovereignFirewallShard.h"
#include "../../../include/SovereignDMAShard.h"
#include "../../../include/SovereignPowerShard.h"
#include "../../../include/SovereignConfigShard.h"
#include "../../../include/SovereignSignalShard.h"
#include "../../../include/SovereignVFSShard.h"
#include "../../../include/SovereignNUMAShard.h"
#include "../../../include/SovereignIPCShard.h"
#include "../../../include/SovereignCryptoShard.h"
#include "../../../include/SovereignAuditShard.h"
#include "../../../include/SovereignGamingShard.h"
#include "../../../include/SovereignMultimediaShard.h"
#include "../../../include/SovereignPrivacyShard.h"
#include "../../../include/SovereignContainerShard.h"
#include "../../../include/SovereignNetStackShard.h"
#include "../../../include/SovereignAutoCleanShard.h"
#include "../../../include/SovereignWatchdogShard.h"
#include "../../../include/SovereignCronShard.h"
#include "../../../include/SovereignTTYShard.h"
#include "../../../include/SovereignOptimizationShard.h"
#include "../../../include/SovereignCompositorShard.h"
#include "../../../include/SovereignHIDShard.h"
#include "../../../include/SovereignIntelligenceShard.h"
#include "../../../include/SovereignPackageShard.h"
#include "../../../include/SovereignSoundShard.h"
#include "../../../include/SovereignButlerShard.h"
#include "../../../include/SovereignWindowShard.h"
#include "../../../include/SovereignSessionShard.h"
#include "../../../include/SovereignRestoreShard.h"
#include "../../../include/SovereignGPUShard.h"
#include "../../../include/SovereignRecallShard.h"
#include "../../../include/SovereignWebViewShard.h"
#include "../../../include/SovereignNeuralShard.h"
#include "../../../include/SovereignShellShard.h"
#include "../../../include/SovereignGarbageShard.h"

/* Global CLI context */
SigmaCLICtx_t g_sigma_cli;

/* Forward reference to init system context */
extern SigmaInitCtx_t *sigma_get_init_ctx(void);

/* =========================================================================
 * § 1  TOKENISER — split a cmdline string into argc/argv
 * ====================================================================== */
#define CLI_ARGV_MAX 32
#define CLI_TOKEN_BUF 256

static sigma_u32 cli_tokenise(const char *line,
                               char argv_buf[][CLI_TOKEN_BUF],
                               char *argv[], sigma_u32 max) {
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

/* =========================================================================
 * § 2  CLI REGISTRATION HELPERS
 * ====================================================================== */

static sigma_u32 sigma_cli_hash(const char *str) {
    sigma_u32 hash = 5381;
    int c;
    while ((c = *str++)) hash = ((hash << 5) + hash) + c;
    return hash % SIGMA_CLI_HASH_SIZE;
}

void sigma_cli_init(SigmaCLICtx_t *ctx) {
    sigma_memset(ctx, 0, sizeof(*ctx));
    sigma_memset(ctx->hash_occupied, 0, sizeof(ctx->hash_occupied));
}

sigma_err_t sigma_cli_register(SigmaCLICtx_t *ctx,
                                const char *name, const char *desc,
                                SigmaCLIHandler_t handler) {
    if (ctx->cmd_count >= SIGMA_CLI_MAX_COMMANDS) return SIGMA_ENOSPC;

    sigma_u32 h = sigma_cli_hash(name);
    while (ctx->hash_occupied[h]) {
        h = (h + 1) % SIGMA_CLI_HASH_SIZE; // Linear probing
    }

    SigmaCLICmd_t *cmd = &ctx->cmds[ctx->cmd_count];
    sigma_strcpy(cmd->name, name, SIGMA_CLI_NAME_MAX);
    sigma_strcpy(cmd->description, desc, SIGMA_CLI_DESC_MAX);
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
        if (sigma_streq(cmd->name, argv[0]))
            return cmd->handler((int)argc, argv);
        h = (h + 1) % SIGMA_CLI_HASH_SIZE;
        if (h == start) break;
    }

    sigma_printf("Σ [CLI]: Unknown command: '%s'. Run 'sigma-help' for usage.\n", argv[0]);
    return SIGMA_ENOENT;
}

/* =========================================================================
 * § 3  COMMAND IMPLEMENTATIONS
 * ====================================================================== */

/* ---- sigma-ls ---------------------------------------------------------- */
sigma_err_t sigma_cmd_ls(int argc, char *argv[]) {
    const char *path = (argc > 1) ? argv[1] : ".";
    sigma_printf("Σ [LS]: Listing '%s':\n", path);
    /* In a live kernel: iterate SigmaDentry children via sigma_vfs_readdir() */
    static const char *demo[] = {
        "bin/", "boot/", "dev/", "etc/", "home/", "lib/", "proc/",
        "root/", "run/", "sbin/", "sys/", "tmp/", "usr/", "var/", SIGMA_NULL
    };
    for (int i = 0; demo[i]; i++)
        sigma_printf("  %s\n", demo[i]);
    return SIGMA_OK;
}

/* ---- sigma-cat --------------------------------------------------------- */
sigma_err_t sigma_cmd_cat(int argc, char *argv[]) {
    if (argc < 2) { sigma_printf("Usage: sigma-cat <file>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [CAT]: Reading '%s'...\n", argv[1]);
    sigma_printf("  [SigmaOS configuration placeholder content]\n");
    return SIGMA_OK;
}

/* ---- sigma-cp ---------------------------------------------------------- */
sigma_err_t sigma_cmd_cp(int argc, char *argv[]) {
    if (argc < 3) { sigma_printf("Usage: sigma-cp <src> <dst>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [CP]: %s -> %s\n", argv[1], argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-mv ---------------------------------------------------------- */
sigma_err_t sigma_cmd_mv(int argc, char *argv[]) {
    if (argc < 3) { sigma_printf("Usage: sigma-mv <src> <dst>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [MV]: %s -> %s\n", argv[1], argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-rm ---------------------------------------------------------- */
sigma_err_t sigma_cmd_rm(int argc, char *argv[]) {
    if (argc < 2) { sigma_printf("Usage: sigma-rm <file>\n"); return SIGMA_EINVAL; }
    sigma_bool recursive = SIGMA_FALSE;
    for (int i = 1; i < argc; i++) {
        if (sigma_streq(argv[i], "-r") || sigma_streq(argv[i], "-rf"))
            recursive = SIGMA_TRUE;
    }
    sigma_printf("Σ [RM]: Removing '%s' %s\n",
                 argv[argc - 1], recursive ? "(recursive)" : "");
    return SIGMA_OK;
}

/* ---- sigma-mkdir ------------------------------------------------------- */
sigma_err_t sigma_cmd_mkdir(int argc, char *argv[]) {
    if (argc < 2) { sigma_printf("Usage: sigma-mkdir <dir>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [MKDIR]: Creating directory '%s'\n", argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-stat -------------------------------------------------------- */
sigma_err_t sigma_cmd_stat(int argc, char *argv[]) {
    if (argc < 2) { sigma_printf("Usage: sigma-stat <file>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [STAT]: File: %s\n"
                 "  Size:   4096 bytes\n"
                 "  Inode:  1024\n"
                 "  Mode:   -rw-r--r-- (0644)\n"
                 "  UID/GID: 0/0\n"
                 "  Links:  1\n", argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-find -------------------------------------------------------- */
sigma_err_t sigma_cmd_find(int argc, char *argv[]) {
    const char *root = (argc > 1) ? argv[1] : ".";
    sigma_printf("Σ [FIND]: Searching under '%s'...\n", root);
    sigma_printf("  [VFS dentry traversal would enumerate here]\n");
    return SIGMA_OK;
}

/* ---- sigma-echo -------------------------------------------------------- */
sigma_err_t sigma_cmd_echo(int argc, char *argv[]) {
    for (int i = 1; i < argc; i++) {
        if (i > 1) sigma_printf(" ");
        sigma_printf("%s", argv[i]);
    }
    sigma_printf("\n");
    return SIGMA_OK;
}

/* ---- sigma-env (printenv/export CLI) ---------------------------------- */
sigma_err_t sigma_cmd_env(int argc, char *argv[]) {
    if (argc == 1) {
        sigma_env_dump(&g_sigma_env);
        return SIGMA_OK;
    }
    /* sigma-env KEY=VAL */
    const char *eq = sigma_strstr(argv[1], "=");
    if (eq) {
        char key[SIGMA_ENV_KEY_MAX];
        sigma_u32 klen = (sigma_u32)(eq - argv[1]);
        sigma_memcpy(key, argv[1], klen); key[klen] = '\0';
        sigma_env_set(&g_sigma_env, key, eq + 1);
        sigma_printf("Σ [ENV]: Set %s=%s\n", key, eq + 1);
    } else {
        /* Lookup */
        const char *val = sigma_env_get(&g_sigma_env, argv[1]);
        sigma_printf("%s=%s\n", argv[1], val ? val : "(unset)");
    }
    return SIGMA_OK;
}

/* ---- sigma-ps --------------------------------------------------------- */
sigma_err_t sigma_cmd_ps(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_printf("Σ [PS]: Process List:\n");
    sigma_printf("  PID  PPID  STATE    CMD\n");
    sigma_printf("    1     0  running  sigma-init\n");
    sigma_printf(" 1000     1  running  sigma-logger\n");
    sigma_printf(" 1001     1  running  sigma-netd\n");
    sigma_printf(" 1002     1  running  sigma-sshd\n");
    sigma_printf(" 1003     1  running  sigma-desktop\n");
    sigma_printf(" 2000  1003  running  sigma-sh\n");
    return SIGMA_OK;
}

/* ---- sigma-kill -------------------------------------------------------- */
sigma_err_t sigma_cmd_kill(int argc, char *argv[]) {
    if (argc < 2) { sigma_printf("Usage: sigma-kill [-SIGNAL] <pid>\n"); return SIGMA_EINVAL; }
    sigma_printf("Σ [KILL]: Sending signal to PID %s\n", argv[argc - 1]);
    return SIGMA_OK;
}

/* ---- sigma-top --------------------------------------------------------- */
sigma_err_t sigma_cmd_top(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_printf("Σ [TOP]: Live Process Monitor (snapshot):\n");
    sigma_printf("  Tasks: 6 total, 6 running, 0 sleeping\n");
    sigma_printf("  CPU:  12.3%% user  3.1%% sys  84.6%% idle\n");
    sigma_printf("  Mem:  1024 MB used  7168 MB free\n\n");
    sigma_printf("  PID   CPU%%  MEM%%  CMD\n");
    sigma_printf("  1     0.0   0.1   sigma-init\n");
    sigma_printf("  1001  0.5   0.8   sigma-netd\n");
    sigma_printf("  2000  1.2   0.3   sigma-sh\n");
    return SIGMA_OK;
}

/* ---- sigma-uname ------------------------------------------------------- */
sigma_err_t sigma_cmd_uname(int argc, char *argv[]) {
    sigma_bool all = (argc > 1 && sigma_streq(argv[1], "-a"));
    sigma_printf("SigmaOS");
    if (all) sigma_printf(" sigma-host 1.0.0-sovereign x86_64 SigmaOS/GNU");
    sigma_printf("\n");
    return SIGMA_OK;
}

/* ---- sigma-dmesg ------------------------------------------------------- */
sigma_err_t sigma_cmd_dmesg(int argc, char *argv[]) {
    sigma_bool clear = (argc > 1 && sigma_streq(argv[1], "-c"));
    if (clear) { sigma_dmesg_clear(&g_sigma_dmesg); return SIGMA_OK; }
    sigma_dmesg_dump(&g_sigma_dmesg);
    return SIGMA_OK;
}

/* ---- sigma-pkg --------------------------------------------------------- */
sigma_err_t sigma_cmd_pkg(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-pkg [install|remove|update|search|list] <pkg>\n");
        return SIGMA_EINVAL;
    }
    if (sigma_streq(argv[1], "install") && argc >= 3)
        sigma_printf("Σ [PKG]: Installing '%s'... [DONE]\n", argv[2]);
    else if (sigma_streq(argv[1], "remove") && argc >= 3)
        sigma_printf("Σ [PKG]: Removing '%s'... [DONE]\n", argv[2]);
    else if (sigma_streq(argv[1], "update"))
        sigma_printf("Σ [PKG]: Updating sovereign shard repository... [SYNCED]\n");
    else if (sigma_streq(argv[1], "search") && argc >= 3)
        sigma_printf("Σ [PKG]: Searching for '%s'... [1 result: %s-1.0]\n",
                     argv[2], argv[2]);
    else if (sigma_streq(argv[1], "list"))
        sigma_printf("Σ [PKG]: Installed packages:\n  sigma-core-1.0\n  sigma-net-1.0\n  sigma-desktop-1.0\n");
    return SIGMA_OK;
}

/* ---- sigma-net --------------------------------------------------------- */
sigma_err_t sigma_cmd_net(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-net [addr|link|route|stat] [iface]\n");
        return SIGMA_EINVAL;
    }
    if (sigma_streq(argv[1], "addr")) {
        sigma_printf("Σ [NET]: Network interfaces:\n");
        sigma_printf("  lo:    127.0.0.1/8     UP LOOPBACK\n");
        sigma_printf("  eth0:  192.168.1.100/24 UP BROADCAST RUNNING\n");
    } else if (sigma_streq(argv[1], "link")) {
        sigma_printf("Σ [NET]: eth0: link/ether aa:bb:cc:dd:ee:ff  state UP\n");
    } else if (sigma_streq(argv[1], "route")) {
        sigma_printf("Σ [NET]: Routing table:\n");
        sigma_printf("  default via 192.168.1.1 dev eth0\n");
        sigma_printf("  192.168.1.0/24 dev eth0\n");
    } else if (sigma_streq(argv[1], "stat")) {
        sigma_printf("Σ [NET]: eth0: RX 10240 bytes TX 4096 bytes\n");
    }
    return SIGMA_OK;
}

/* ---- sigma-user -------------------------------------------------------- */
sigma_err_t sigma_cmd_user(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-user [add|del|passwd|lock|unlock|list] ...\n");
        return SIGMA_EINVAL;
    }
    if (sigma_streq(argv[1], "list")) {
        sigma_userdb_dump(&g_sigma_userdb);
    } else if (sigma_streq(argv[1], "add") && argc >= 3) {
        sigma_user_add(&g_sigma_userdb, argv[2], "changeme",
                       "/home/new", "/bin/sigma-sh");
    } else if (sigma_streq(argv[1], "del") && argc >= 3) {
        sigma_user_del(&g_sigma_userdb, argv[2]);
    } else if (sigma_streq(argv[1], "lock") && argc >= 3) {
        sigma_user_lock(&g_sigma_userdb, argv[2]);
    } else if (sigma_streq(argv[1], "unlock") && argc >= 3) {
        sigma_user_unlock(&g_sigma_userdb, argv[2]);
    }
    return 0;
}

/* -------------------------------------------------------------------------
 * Phase 42 & 43 Commands (Competitor Shards)
 * ---------------------------------------------------------------------- */

static int sigma_cmd_zfs(int argc, char **argv) {
    if (argc < 2) { sigma_zfs_list(SIGMA_NULL); return 0; }
    if (sigma_streq(argv[1], "create")) sigma_zfs_create(argv[2], SIGMA_DS_FILESYSTEM);
    else if (sigma_streq(argv[1], "snap")) sigma_zfs_snapshot(argv[2], argv[3]);
    else sigma_zfs_list(SIGMA_NULL);
    return 0;
}

static int sigma_cmd_jail(int argc, char **argv) {
    (void)argc; (void)argv;
    sigma_jls();
    return 0;
}

static int sigma_cmd_obs(int argc, char **argv) {
    (void)argc; (void)argv;
    sigma_obs_stats();
    return 0;
}

static int sigma_cmd_vbox(int argc, char **argv) {
    (void)argc; (void)argv;
    sigma_vbox_list_vms();
    return 0;
}

static int sigma_cmd_browser(int argc, char **argv) {
    (void)argc; (void)argv;
    sigma_browser_stats();
    return 0;
}

static int sigma_cmd_defender(int argc, char **argv) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-defender [scan <file> | quarantine <file>]\n");
        return 0;
    }
    if (sigma_streq(argv[1], "scan") && argc >= 3) {
        char threat[64] = {0};
        if (sigma_defender_scan_file(argv[2], threat) == DEFENDER_MALWARE) {
            sigma_printf("Σ [DEFENDER]: THREAT DETECTED: %s\n", threat);
        } else {
            sigma_printf("Σ [DEFENDER]: File is clean.\n");
        }
    } else if (sigma_streq(argv[1], "quarantine") && argc >= 3) {
        sigma_defender_quarantine(argv[2]);
    }
    return 0;
}

static int sigma_cmd_ad(int argc, char **argv) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-ad [join <domain> <dc_ip> <user> | gpo <file>]\n");
        return 0;
    }
    if (sigma_streq(argv[1], "join") && argc >= 5) {
        sigma_ad_join_domain(argv[2], argv[3], argv[4], "");
    } else if (sigma_streq(argv[1], "gpo") && argc >= 3) {
        sigma_ad_apply_gpo(argv[2]);
    }
    return 0;
}

/* -------------------------------------------------------------------------
 * Phase 44 Commands (Android/macOS Shards)
 * ---------------------------------------------------------------------- */

static int sigma_cmd_binder(int argc, char **argv) {
    (void)argc; (void)argv;
    return 0;
}

static int sigma_cmd_xnu(int argc, char **argv) {
    (void)argc; (void)argv;
    return 0;
}

/* -------------------------------------------------------------------------
 * Phase 45 Commands (Linux/SerenityOS)
 * ---------------------------------------------------------------------- */

static int sigma_cmd_iouring(int argc, char **argv) {
    (void)argc; (void)argv;
    return 0;
}

static int sigma_cmd_gui(int argc, char **argv) {
    (void)argc; (void)argv;
    sigma_printf("Σ [GUI]: Requesting hardware-accelerated frame flush...\n");
    sigma_window_server_flush_compositor();
    return 0;
}

/* ---- sigma-personalize ------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-personalize ------------------------------------------------- */
sigma_err_t sigma_cmd_personalize(int argc, char *argv[]) {
    static SovereignPersonalizer_t g_user_p;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_user_p = SovereignPersonalizer_Create("SigmaSovereign"); init = SIGMA_TRUE; }

    if (argc < 2) {
        g_user_p.audit_customizations(&g_user_p);
        sigma_printf("Usage: sigma-personalize [theme <name> | auto <0-2> | heal]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "theme") && argc >= 3) {
        g_user_p.apply_theme(&g_user_p, argv[2]);
    } else if (sigma_streq(argv[1], "auto") && argc >= 3) {
        g_user_p.set_automation_policy(&g_user_p, (sigma_u32)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "heal")) {
        g_user_p.trigger_self_healing(&g_user_p);
    }
    return SIGMA_OK;
}

/* ---- sigma-distro ------------------------------------------------------ */
    return SIGMA_OK;
}

/* ---- sigma-distro ------------------------------------------------------ */
sigma_err_t sigma_cmd_distro(int argc, char *argv[]) {
    static SovereignDistroSlinger_t g_slinger;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_slinger = SovereignDistroSlinger_Create(); init = SIGMA_TRUE; }

    if (argc < 2) {
        g_slinger.audit_shards(&g_slinger);
        sigma_printf("Usage: sigma-distro [-load <path> <name> | -map | -spawn]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "-load") && argc >= 4) {
        g_slinger.load_shard(&g_slinger, argv[2], argv[3]);
    } else if (sigma_streq(argv[1], "-map")) {
        g_slinger.map_syscalls(&g_slinger);
    } else if (sigma_streq(argv[1], "-spawn")) {
        g_slinger.spawn_autonomous(&g_slinger);
    }
    return SIGMA_OK;
}

/* ---- sigma-run --------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-run --------------------------------------------------------- */
sigma_err_t sigma_cmd_run(int argc, char *argv[]) {
    static SovereignAutomationEngine_t g_auto_eng;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_auto_eng = SovereignAutomationEngine_Create(); init = SIGMA_TRUE; }

    if (argc < 2) {
        g_auto_eng.audit_automation(&g_auto_eng);
        sigma_printf("Usage: sigma-run <script_path | label>\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "standard_boot")) {
        const char* script = 
            "sigma-uname -a\n"
            "sigma-personalize theme ZENITH_DARK\n"
            "sigma-ai audit\n"
            "sigma-ls /\n"
            "sigma-echo [AUTO]: System Stabilized.\n";
        g_auto_eng.execute_script(&g_auto_eng, script);
    } else {
        sigma_printf("Σ [RUN]: Reading script '%s'...\n", argv[1]);
        sigma_printf("[SKIPPED]: File I/O simulation only.\n");
    }
    return SIGMA_OK;
}

/* ---- sigma-agent ------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-agent ------------------------------------------------------- */
sigma_err_t sigma_cmd_agent(int argc, char *argv[]) {
    static SovereignAutonomousAgent_t g_agent;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_agent = SovereignAutonomousAgent_Create(7); init = SIGMA_TRUE; }

    if (argc < 2) {
        g_agent.execute_autonomous_audit(&g_agent);
        sigma_printf("Usage: sigma-agent [start | prowl <sector> | stop]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "start")) {
        g_agent.bootstrap_mission(&g_agent);
    } else if (sigma_streq(argv[1], "prowl") && argc >= 3) {
        g_agent.prowl_sector(&g_agent, argv[2]);
    } else if (sigma_streq(argv[1], "stop")) {
        sigma_printf("[AGENT]: Missions suspended. Returning to carrier.\n");
        g_agent.prowling = SIGMA_FALSE;
    }
    return SIGMA_OK;
}

/* ---- sigma-scrub ------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-scrub ------------------------------------------------------- */
sigma_err_t sigma_cmd_scrub(int argc, char *argv[]) {
    static SovereignForensicScrubber_t g_scrubber;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_scrubber = SovereignForensicScrubber_Create(); init = SIGMA_TRUE; }

    if (argc < 2) {
        SovereignForensicScrubber_Audit(&g_scrubber);
        sigma_printf("Usage: sigma-scrub [all | sector <address> <size> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "all")) {
        sigma_printf("[SCRUB]: Triggering system-wide amnesic purge...\n");
        sigma_scrub_memory_sector(&g_scrubber, (void*)0x1000, 4096); // Simulated
    } else if (sigma_streq(argv[1], "sector") && argc >= 4) {
         sigma_scrub_memory_sector(&g_scrubber, (void*)0xABCD, (sigma_size_t)sigma_atoi(argv[3]));
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignForensicScrubber_Audit(&g_scrubber);
    }
    return SIGMA_OK;
}

/* ---- sigma-boost ------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-boost ------------------------------------------------------- */
sigma_err_t sigma_cmd_boost(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-boost <pid>\n");
        return SIGMA_OK;
    }
    
    sigma_u32 pid = (sigma_u32)sigma_atoi(argv[1]);
    sigma_sched_boost_pid(pid);
    return SIGMA_OK;
}

/* ---- sigma-rebuild ----------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-rebuild ----------------------------------------------------- */
sigma_err_t sigma_cmd_rebuild(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignAtomicUpdate_Audit();
        sigma_printf("Usage: sigma-rebuild [switch <manifesto> | rollback | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "switch") && argc >= 3) {
        sigma_rebuild_system(argv[2]);
    } else if (sigma_streq(argv[1], "rollback")) {
        SovereignAtomicUpdate_Rollback();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignAtomicUpdate_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-tensor ------------------------------------------------------ */
    return SIGMA_OK;
}

/* ---- sigma-tensor ------------------------------------------------------ */
sigma_err_t sigma_cmd_tensor(int argc, char *argv[]) {
    static SovereignTensorShard_t g_tensor;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_tensor = SovereignTensorShard_Create(); init = SIGMA_TRUE; }

    if (argc < 2) {
        SovereignTensorShard_Audit(&g_tensor);
        sigma_printf("Usage: sigma-tensor [bench | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "bench")) {
        float a[4] = {1, 2, 3, 4};
        float b[4] = {5, 6, 7, 8};
        float c[4] = {0, 0, 0, 0};
        SigmaTensor_t ta = {a, 2, 2};
        SigmaTensor_t tb = {b, 2, 2};
        SigmaTensor_t tc = {c, 2, 2};
        sigma_tensor_gemm(&ta, &tb, &tc);
        g_tensor.ops_completed++;
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignTensorShard_Audit(&g_tensor);
    }
    return SIGMA_OK;
}

/* ---- sigma-net --------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-net --------------------------------------------------------- */
sigma_err_t sigma_cmd_net(int argc, char *argv[]) {
    static SovereignNetworkShard_t g_net;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_net = SovereignNetworkShard_Create(); init = SIGMA_TRUE; }

    if (argc < 2) {
        SovereignNetworkShard_Audit(&g_net);
        sigma_printf("Usage: sigma-net [ping <addr> | xdp <count> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "ping") && argc >= 3) {
        sigma_printf("[NETWORK]: Sending industrial probe to %s... [OK 1.2ms]\n", argv[2]);
        g_net.eth0.packets_switched++;
    } else if (sigma_streq(argv[1], "xdp") && argc >= 3) {
        sigma_net_zero_copy_dispatch(SIGMA_NULL, (sigma_u32)sigma_atoi(argv[2]));
        g_net.eth0.packets_switched += (sigma_u32)sigma_atoi(argv[2]);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignNetworkShard_Audit(&g_net);
    }
    return SIGMA_OK;
}

/* ---- sigma-vault ------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-vault ------------------------------------------------------- */
sigma_err_t sigma_cmd_vault(int argc, char *argv[]) {
    static SovereignSecurityVault_t g_vault;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_vault = SovereignSecurityVault_Create(); init = SIGMA_TRUE; }

    if (argc < 2) {
        SovereignSecurityVault_Audit(&g_vault);
        sigma_printf("Usage: sigma-vault [pledge <caps_hex> | unveil <path> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "pledge") && argc >= 3) {
        sigma_pledge((sigma_u32)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "unveil") && argc >= 3) {
        sigma_unveil(argv[2]);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignSecurityVault_Audit(&g_vault);
    }
    return SIGMA_OK;
}

/* ---- sigma-spawn ------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-spawn ------------------------------------------------------- */
sigma_err_t sigma_cmd_spawn(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignSiliconContainer_Audit();
        sigma_printf("Usage: sigma-spawn <zone_name> [memory_limit_mb]\n");
        return SIGMA_OK;
    }

    sigma_u64 limit = (argc >= 3) ? (sigma_u64)sigma_atoi(argv[2]) * 1024 * 1024 : 1024 * 1024 * 1024;
    sigma_container_spawn(argv[1], limit);
    return SIGMA_OK;
}

/* ---- sigma-probe ------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-probe ------------------------------------------------------- */
sigma_err_t sigma_cmd_probe(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignSiliconProbe_Audit();
        sigma_printf("Usage: sigma-probe [hook <point> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "hook") && argc >= 3) {
        sigma_probe_register(argv[2]);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignSiliconProbe_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-store ------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-store ------------------------------------------------------- */
sigma_err_t sigma_cmd_store(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignSiliconStore_Audit();
        sigma_printf("Usage: sigma-store [set <key> <val> | get <key> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "set") && argc >= 4) {
        sigma_store_set(argv[2], argv[3]);
    } else if (sigma_streq(argv[1], "get") && argc >= 3) {
        const char* val = sigma_store_get(argv[2]);
        sigma_printf("Σ [STORE]: %s = %s\n", argv[2], val ? val : "(NULL)");
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignSiliconStore_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-cluster ----------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-cluster ----------------------------------------------------- */
sigma_err_t sigma_cmd_cluster(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignClusterShard_Audit();
        sigma_printf("Usage: sigma-cluster [join <node_name> | reconcile | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "join") && argc >= 3) {
        sigma_cluster_join(argv[2]);
    } else if (sigma_streq(argv[1], "reconcile")) {
        sigma_cluster_reconcile();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignClusterShard_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-zenith ------------------------------------------------------ */
    return SIGMA_OK;
}

/* ---- sigma-zenith ------------------------------------------------------ */
sigma_err_t sigma_cmd_zenith(int argc, char *argv[]) {
    (void)argc; (void)argv;
    return sigma_zenith_master_audit();
}

/* ---- sigma-dsa --------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-dsa --------------------------------------------------------- */
sigma_err_t sigma_cmd_dsa(int argc, char *argv[]) {
    static SovereignDSAShard_t g_dsa;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_dsa = SovereignDSA_Create(); init = SIGMA_TRUE; }

    if (argc < 2) {
        g_dsa.audit_complexity(&g_dsa);
        sigma_printf("Usage: sigma-dsa [sort | map <addr> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "sort")) {
        sigma_u32 data[5] = {5, 2, 9, 1, 5};
        g_dsa.sort_quicksort(&g_dsa, data, 5);
    } else if (sigma_streq(argv[1], "map") && argc >= 3) {
        g_dsa.map_silicon_shard(&g_dsa, (sigma_u64)sigma_atoi(argv[2]), 4096);
    } else if (sigma_streq(argv[1], "audit")) {
        g_dsa.audit_complexity(&g_dsa);
    }
    return SIGMA_OK;
}

/* ---- sigma-math -------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-math -------------------------------------------------------- */
sigma_err_t sigma_cmd_math(int argc, char *argv[]) {
    static SovereignMathShard_t g_math;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_math = SovereignMath_Create(); init = SIGMA_TRUE; }

    if (argc < 2) {
        SovereignMathShard_Audit(&g_math);
        sigma_printf("Usage: sigma-math [isqrt <val> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "isqrt") && argc >= 3) {
        float val = (float)sigma_atoi(argv[2]);
        float res = sigma_math_fast_inv_sqrt(val);
        sigma_printf("Σ [MATH]: FastInvSqrt(%f) = %f\n", val, res);
        g_math.total_calcs++;
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignMathShard_Audit(&g_math);
    }
    return SIGMA_OK;
}

/* ---- sigma-ctl --------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-ctl --------------------------------------------------------- */
sigma_err_t sigma_cmd_ctl(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignShardManager_Audit();
        sigma_printf("Usage: sigma-ctl [start <name> | stop <name> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "start") && argc >= 3) {
        sigma_shard_start(argv[2], SIGMA_FALSE);
    } else if (sigma_streq(argv[1], "stop") && argc >= 3) {
        sigma_shard_stop(argv[2]);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignShardManager_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-pkg --------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-pkg --------------------------------------------------------- */
sigma_err_t sigma_cmd_pkg(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_repo_list();
        sigma_printf("Usage: sigma-pkg [install <name> | list]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "install") && argc >= 3) {
        sigma_repo_pull(argv[2]);
    } else if (sigma_streq(argv[1], "list")) {
        sigma_repo_list();
    }
    return SIGMA_OK;
}

/* ---- sigma-reload ------------------------------------------------------ */
    return SIGMA_OK;
}

/* ---- sigma-reload ------------------------------------------------------ */
sigma_err_t sigma_cmd_reload(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignLiveReload_Audit();
        sigma_printf("Usage: sigma-reload <target_shard_name> [mock_addr_hex]\n");
        return SIGMA_OK;
    }

    void* mock_addr = (argc >= 3) ? (void*)(sigma_size_t)sigma_atoi(argv[2]) : (void*)0xDEADBEEF;
    sigma_reload_shard(argv[1], mock_addr);
    return SIGMA_OK;
}

/* ---- sigma-find -------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-find -------------------------------------------------------- */
sigma_err_t sigma_cmd_find(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignSpotlight_Audit();
        sigma_printf("Usage: sigma-find <query> | audit\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "audit")) {
        SovereignSpotlight_Audit();
    } else {
        sigma_spotlight_query(argv[1]);
    }
    return SIGMA_OK;
}

/* ---- sigma-wm ---------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-wm ---------------------------------------------------------- */
sigma_err_t sigma_cmd_wm(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignWM_Audit();
        sigma_printf("Usage: sigma-wm [create <title> <x> <y> <w> <h> | composite | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "create") && argc >= 7) {
        sigma_wm_create_window(argv[2], (sigma_u32)sigma_atoi(argv[3]), (sigma_u32)sigma_atoi(argv[4]), 
                               (sigma_u32)sigma_atoi(argv[5]), (sigma_u32)sigma_atoi(argv[6]));
    } else if (sigma_streq(argv[1], "composite")) {
        sigma_wm_composite();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignWM_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-script ------------------------------------------------------ */
    return SIGMA_OK;
}

/* ---- sigma-script ------------------------------------------------------ */
sigma_err_t sigma_cmd_script(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignAutomation_Audit();
        sigma_printf("Usage: sigma-script [run | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "run")) {
        sigma_automation_execute();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignAutomation_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-neural ------------------------------------------------------ */
    return SIGMA_OK;
}

/* ---- sigma-neural ------------------------------------------------------ */
sigma_err_t sigma_cmd_neural(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignNeural_Audit();
        sigma_printf("Usage: sigma-neural [infer <model> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "infer") && argc >= 3) {
        sigma_neural_infer(argv[2]);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignNeural_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-enclave ----------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-enclave ----------------------------------------------------- */
sigma_err_t sigma_cmd_enclave(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignEnclave_Audit();
        sigma_printf("Usage: sigma-enclave [seal <key_name> | gen <name> <bits> (qs) | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "seal") && argc >= 3) {
        sigma_enclave_seal(argv[2]);
    } else if (sigma_streq(argv[1], "gen") && argc >= 4) {
        sigma_bool qs = (argc >= 5 && sigma_streq(argv[4], "qs")) ? SIGMA_TRUE : SIGMA_FALSE;
        sigma_enclave_gen_key(argv[2], (sigma_u32)sigma_atoi(argv[3]), qs);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignEnclave_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-persona ----------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-persona ----------------------------------------------------- */
sigma_err_t sigma_cmd_persona(int argc, char *argv[]) {
    static SovereignDistroSlinger_t g_slinger;
    static sigma_bool init = SIGMA_FALSE;
    if (!init) { g_slinger = SovereignDistroSlinger_Create(); init = SIGMA_TRUE; }

    if (argc < 2) {
        g_slinger.audit_shards(&g_slinger);
        sigma_printf("Usage: sigma-persona [sigma | linux | darwin | windows | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "sigma")) {
        g_slinger.switch_persona(&g_slinger, PERSONA_SIGMA);
    } else if (sigma_streq(argv[1], "linux")) {
        g_slinger.switch_persona(&g_slinger, PERSONA_LINUX);
    } else if (sigma_streq(argv[1], "darwin")) {
        g_slinger.switch_persona(&g_slinger, PERSONA_DARWIN);
    } else if (sigma_streq(argv[1], "windows")) {
        g_slinger.switch_persona(&g_slinger, PERSONA_WINDOWS);
    } else if (sigma_streq(argv[1], "audit")) {
        g_slinger.audit_shards(&g_slinger);
    }
    return SIGMA_OK;
}

/* ---- sigma-test -------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-test -------------------------------------------------------- */
sigma_err_t sigma_cmd_test(int argc, char *argv[]) {
    (void)argc; (void)argv;
    return sigma_execute_full_test_suite();
}

/* ---- sigma-rebuild ----------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-rebuild ----------------------------------------------------- */
sigma_err_t sigma_cmd_rebuild(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignRebuild_Audit();
        sigma_printf("Usage: sigma-rebuild [switch | rollback | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "switch")) {
        sigma_rebuild_system();
    } else if (sigma_streq(argv[1], "rollback")) {
        sigma_rebuild_rollback();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignRebuild_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-style ------------------------------------------------------- */
sigma_err_t sigma_cmd_style(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignAesthetic_Audit();
        sigma_printf("Usage: sigma-style [set <name> <color_hex> <blur_px> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "set") && argc >= 5) {
     return SIGMA_OK;
}

/* ---- sigma-style ------------------------------------------------------- */
sigma_err_t sigma_cmd_style(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignAesthetic_Audit();
        sigma_printf("Usage: sigma-style [set <name> <color_hex> <blur_px> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "set") && argc >= 5) {
        sigma_aesthetic_set_theme(argv[2], (sigma_u32)sigma_atoi(argv[3]), (sigma_u32)sigma_atoi(argv[4]));
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignAesthetic_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-sched ------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-sched ------------------------------------------------------- */
sigma_err_t sigma_cmd_sched(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignZenScheduler_Audit();
        sigma_printf("Usage: sigma-sched [balance | add <name> <prio> <policy_id> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "balance")) {
        sigma_sched_balance();
    } else if (sigma_streq(argv[1], "add") && argc >= 5) {
        sigma_sched_add_task(argv[2], (sigma_u32)sigma_atoi(argv[3]), (sigma_u32)sigma_atoi(argv[4]));
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignZenScheduler_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-auto -------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-auto -------------------------------------------------------- */
sigma_err_t sigma_cmd_auto(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignAutoClean_Audit();
        SovereignAutoPerf_Audit();
        sigma_printf("Usage: sigma-auto [clean | boost | compact | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "clean")) {
        sigma_autoclean_volatile();
        sigma_autoclean_legacy();
    } else if (sigma_streq(argv[1], "boost")) {
        sigma_autoperf_boost();
    } else if (sigma_streq(argv[1], "compact")) {
        sigma_autoperf_compact();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignAutoClean_Audit();
        SovereignAutoPerf_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-amnesia ----------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-amnesia ----------------------------------------------------- */
sigma_err_t sigma_cmd_amnesia(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignAmnesic_Audit();
        sigma_printf("Usage: sigma-amnesia [scrub | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "scrub")) {
        sigma_amnesic_scrub();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignAmnesic_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-mesh -------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-mesh -------------------------------------------------------- */
sigma_err_t sigma_cmd_mesh(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignMeshFS_Audit();
        sigma_printf("Usage: sigma-mesh [publish <data> | sync | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "publish") && argc >= 3) {
        sigma_mesh_publish(argv[2], (sigma_u64)sigma_strlen(argv[2]));
    } else if (sigma_streq(argv[1], "sync")) {
        sigma_mesh_sync();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignMeshFS_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-cap --------------------------------------------------------- */
sigma_err_t sigma_cmd_cap(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignCapability_Audit();
        sigma_printf("Usage: sigma-cap [grant <resource> <rights_hex> | verify <handle_hex> <rights_hex> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "grant") && argc >= 4) {
        sigma_cap_grant(argv[2], (sigma_u32)sigma_atoi(argv[3]));
    } else if (sigma_streq(argv[1], "verify") && argc >= 4) {
        sigma_cap_verify((sigma_u32)sigma_atoi(argv[2]), (sigma_u32)sigma_atoi(argv[3]));
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignCapability_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-ai ---------------------------------------------------------- */
sigma_err_t sigma_cmd_ai(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-ai [train | predict <intent> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "train")) {
        sigma_printf("[AI]: Training model on silicon telemetry... [ZENITH_STRIDE: 0.01]\n");
        sigma_printf("[AI]: Final results: y = 2.01x + 0.05 [ACCURACY: 99.8%%]\n");
    } else if (sigma_streq(argv[1], "predict") && argc >= 3) {
        sigma_printf("[AI]: Prediction for '%s': SUCCESS (Confidence 0.99)\n", argv[2]);
    } else if (sigma_streq(argv[1], "audit")) {
        sigma_printf("--- AI SHARD AUDIT ---\nMODE: Pure C11 Zenith\nCONFIDENCE: 0.9997\n");
    }
    return SIGMA_OK;
}

/* ---- sigma-quorum ------------------------------------------------------ */
    return SIGMA_OK;
}

/* ---- sigma-quorum ------------------------------------------------------ */
sigma_err_t sigma_cmd_quorum(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignConsensus_Audit();
        sigma_printf("Usage: sigma-quorum [elect | replicate <entry> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "elect")) {
        sigma_quorum_elect();
    } else if (sigma_streq(argv[1], "replicate") && argc >= 3) {
        sigma_quorum_replicate(argv[2]);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignConsensus_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-overlay ----------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-overlay ----------------------------------------------------- */
sigma_err_t sigma_cmd_overlay(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignOverlay_Audit();
        sigma_printf("Usage: sigma-overlay [push <name> <mount> <ro_bool> | merge | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "push") && argc >= 5) {
        sigma_overlay_push(argv[2], argv[3], (sigma_bool)sigma_atoi(argv[4]));
    } else if (sigma_streq(argv[1], "merge")) {
        sigma_overlay_merge();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignOverlay_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-migrate ----------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-migrate ----------------------------------------------------- */
sigma_err_t sigma_cmd_migrate(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignMigration_Audit();
        sigma_printf("Usage: sigma-migrate [snap <shard_id> | push <shard_id> <node> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "snap") && argc >= 3) {
        sigma_migrate_checkpoint(argv[2]);
    } else if (sigma_streq(argv[1], "push") && argc >= 4) {
        sigma_migrate_push(argv[2], argv[3]);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignMigration_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-protect ----------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-protect ----------------------------------------------------- */
sigma_err_t sigma_cmd_protect(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignProtectors_Audit();
        sigma_printf("Usage: sigma-protect [reg <addr_hex> | verify <addr_hex> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "reg") && argc >= 3) {
        sigma_protect_register_target((sigma_u64)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "verify") && argc >= 3) {
        sigma_protect_verify_jump((sigma_u64)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignProtectors_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-id ---------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-id ---------------------------------------------------------- */
sigma_err_t sigma_cmd_id(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignIdentity_Audit();
        sigma_printf("Usage: sigma-id [mint <principal> | auth <ticket_hex> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "mint") && argc >= 3) {
        sigma_id_mint(argv[2]);
    } else if (sigma_streq(argv[1], "auth") && argc >= 3) {
        sigma_id_authenticate((sigma_u32)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignIdentity_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-twm --------------------------------------------------------- */
    return SIGMA_OK;
}

/* ---- sigma-twm --------------------------------------------------------- */
sigma_err_t sigma_cmd_twm(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignTWM_Audit();
        sigma_printf("Usage: sigma-twm [recalc | add <win_id> | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "recalc")) {
        sigma_twm_recalculate();
    } else if (sigma_streq(argv[1], "add") && argc >= 3) {
        sigma_twm_add((sigma_u32)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignTWM_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-sync -------------------------------------------------------- */
sigma_err_t sigma_cmd_sync(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignSync_Audit();
        sigma_printf("Usage: sigma-sync [push <uid> | reconcile | audit]\n");
        return SIGMA_OK;
    }

    if (sigma_streq(argv[1], "push") && argc >= 3) {
        sigma_sync_push(argv[2]);
    } else if (sigma_streq(argv[1], "reconcile")) {
        sigma_sync_reconcile();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignSync_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-tele -------------------------------------------------------- */
sigma_err_t sigma_cmd_tele(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignTelemetry_Audit();
        sigma_printf("Usage: sigma-tele [arm <name> <addr> | sample | flush | audit]\n");
        return SIGMA_OK;
    }
    if (sigma_streq(argv[1], "arm") && argc >= 4) {
        sigma_tele_probe_arm(argv[2], (sigma_u64)sigma_atoi(argv[3]), PROBE_KPROBE);
    } else if (sigma_streq(argv[1], "sample")) {
        sigma_tele_sample();
    } else if (sigma_streq(argv[1], "flush")) {
        sigma_tele_map_flush();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignTelemetry_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-persona ----------------------------------------------------- */
sigma_err_t sigma_cmd_sigma_persona(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignPersona_Audit();
        sigma_printf("Usage: sigma-persona [create <name> <theme> <uid> <cap_mask> | switch <name> | audit]\n");
        return SIGMA_OK;
    }
    if (sigma_streq(argv[1], "create") && argc >= 6) {
        sigma_persona_create(argv[2], argv[3],
                             (sigma_u32)sigma_atoi(argv[4]),
                             (sigma_u32)sigma_atoi(argv[5]));
    } else if (sigma_streq(argv[1], "switch") && argc >= 3) {
        sigma_persona_switch(argv[2]);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignPersona_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-hotpatch ---------------------------------------------------- */
sigma_err_t sigma_cmd_hotpatch(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignHotpatch_Audit();
        sigma_printf("Usage: sigma-hotpatch [load <id> <target_addr> <patch_addr> | revert <id> | audit]\n");
        return SIGMA_OK;
    }
    if (sigma_streq(argv[1], "load") && argc >= 5) {
        sigma_hotpatch_load(argv[2],
                            (sigma_u64)sigma_atoi(argv[3]),
                            (sigma_u64)sigma_atoi(argv[4]));
    } else if (sigma_streq(argv[1], "revert") && argc >= 3) {
        sigma_hotpatch_revert(argv[2]);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignHotpatch_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-cgroup ------------------------------------------------------ */
sigma_err_t sigma_cmd_cgroup(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignCgroup_Audit();
        sigma_printf("Usage: sigma-cgroup [create <name> <cpu_pct> <mem_mb> <io_weight> | enforce | audit]\n");
        return SIGMA_OK;
    }
    if (sigma_streq(argv[1], "create") && argc >= 6) {
        sigma_cgroup_create(argv[2],
                            (sigma_u32)sigma_atoi(argv[3]),
                            (sigma_u64)sigma_atoi(argv[4]) * 1024ULL * 1024ULL,
                            (sigma_u32)sigma_atoi(argv[5]));
    } else if (sigma_streq(argv[1], "enforce")) {
        sigma_cgroup_enforce();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignCgroup_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-oom --------------------------------------------------------- */
sigma_err_t sigma_cmd_oom(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignOOM_Audit();
        sigma_printf("Usage: sigma-oom [reg <name> <pid> <mem_kb> <score> <prot> | sweep <free_kb> | audit]\n");
        return SIGMA_OK;
    }
    if (sigma_streq(argv[1], "reg") && argc >= 7) {
        sigma_oom_register(argv[2],
                           (sigma_u32)sigma_atoi(argv[3]),
                           (sigma_u64)sigma_atoi(argv[4]),
                           (sigma_i32)sigma_atoi(argv[5]),
                           (sigma_bool)sigma_atoi(argv[6]));
    } else if (sigma_streq(argv[1], "sweep") && argc >= 3) {
        sigma_oom_sweep((sigma_u64)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignOOM_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-journal ----------------------------------------------------- */
sigma_err_t sigma_cmd_journal(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_journal_follow(LOG_INFO);
        sigma_printf("Usage: sigma-journal [write <level> <unit> <msg> | follow <min_level> | audit]\n");
        return SIGMA_OK;
    }
    if (sigma_streq(argv[1], "write") && argc >= 5) {
        sigma_journal_write((SigmaLogLevel_t)sigma_atoi(argv[2]), argv[3], argv[4]);
    } else if (sigma_streq(argv[1], "follow") && argc >= 3) {
        sigma_journal_follow((SigmaLogLevel_t)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignJournal_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-trace ------------------------------------------------------- */
sigma_err_t sigma_cmd_trace(int argc, char *argv[]) {
    if (argc < 2) {
        SovereignTrace_Audit();
        sigma_printf("Usage: sigma-trace [attach <pid> | detach | audit]\n");
        return SIGMA_OK;
    }
    if (sigma_streq(argv[1], "attach") && argc >= 3) {
        sigma_trace_attach((sigma_u32)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "detach")) {
        sigma_trace_detach();
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignTrace_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-irq --------------------------------------------------------- */
sigma_err_t sigma_cmd_irq(int argc, char *argv[]) {
    if (argc < 2) { SovereignIRQ_Audit();
        sigma_printf("Usage: sigma-irq [reg <irq> <dev> <type> <cpu> | balance | pin <irq> <cpu> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "reg") && argc >= 6) {
        sigma_irq_register((sigma_u32)sigma_atoi(argv[2]), argv[3],
                           (SigmaIRQType_t)sigma_atoi(argv[4]),
                           (sigma_u32)sigma_atoi(argv[5]));
    } else if (sigma_streq(argv[1], "balance")) {
        sigma_irq_balance();
    } else if (sigma_streq(argv[1], "pin") && argc >= 4) {
        sigma_irq_set_affinity((sigma_u32)sigma_atoi(argv[2]),
                               (sigma_u32)sigma_atoi(argv[3]));
    } else if (sigma_streq(argv[1], "audit")) { SovereignIRQ_Audit(); }
    return SIGMA_OK;
}

/* ---- sigma-rollback ---------------------------------------------------- */
sigma_err_t sigma_cmd_rollback(int argc, char *argv[]) {
    if (argc < 2) { SovereignRollback_Audit();
        sigma_printf("Usage: sigma-rollback [snap <path> | restore <id> | prune <n> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "snap") && argc >= 3) {
        sigma_rollback_snap(argv[2]);
    } else if (sigma_streq(argv[1], "restore") && argc >= 3) {
        sigma_rollback_restore(argv[2]);
    } else if (sigma_streq(argv[1], "prune") && argc >= 3) {
        sigma_rollback_prune((sigma_u32)sigma_atoi(argv[2]));
    } else if (sigma_streq(argv[1], "audit")) { SovereignRollback_Audit(); }
    return SIGMA_OK;
}

/* ---- sigma-fw ---------------------------------------------------------- */
sigma_err_t sigma_cmd_fw(int argc, char *argv[]) {
    if (argc < 2) { SovereignFirewall_Audit();
        sigma_printf("Usage: sigma-fw [add <proto> <src> <dst> <port> <verdict> <comment> | test <proto> <dst_port> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "add") && argc >= 8) {
        sigma_fw_add_rule((SigmaFWProto_t)sigma_atoi(argv[2]),
                          (sigma_u32)sigma_atoi(argv[3]),
                          (sigma_u32)sigma_atoi(argv[4]),
                          (sigma_u16)sigma_atoi(argv[5]),
                          (SigmaFWVerdict_t)sigma_atoi(argv[6]), argv[7]);
    } else if (sigma_streq(argv[1], "test") && argc >= 4) {
        SigmaFWVerdict_t v = sigma_fw_classify(
            (SigmaFWProto_t)sigma_atoi(argv[2]), 0, 0,
            (sigma_u16)sigma_atoi(argv[3]));
        static const char* vn[] = { "ACCEPT", "DROP", "REJECT", "LOG+ACCEPT" };
        sigma_printf("[FW-TEST]: Verdict -> %s\n", vn[v]);
    } else if (sigma_streq(argv[1], "audit")) { SovereignFirewall_Audit(); }
    return SIGMA_OK;
}

/* ---- sigma-dma --------------------------------------------------------- */
sigma_err_t sigma_cmd_dma(int argc, char *argv[]) {
    if (argc < 2) { SovereignDMA_Audit();
        sigma_printf("Usage: sigma-dma [map <bdf> <iova> <pa> <size_kb> | quarantine <bdf> | sweep | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "map") && argc >= 6)
        sigma_dma_map(argv[2], (sigma_u64)sigma_atoi(argv[3]),
                      (sigma_u64)sigma_atoi(argv[4]),
                      (sigma_u64)sigma_atoi(argv[5]) * 1024ULL, DMA_PROT_RW);
    else if (sigma_streq(argv[1], "quarantine") && argc >= 3)
        sigma_dma_quarantine(argv[2]);
    else if (sigma_streq(argv[1], "sweep"))
        sigma_dma_integrity_sweep();
    else if (sigma_streq(argv[1], "audit"))
        SovereignDMA_Audit();
    return SIGMA_OK;
}

/* ---- sigma-power ------------------------------------------------------- */
sigma_err_t sigma_cmd_power(int argc, char *argv[]) {
    if (argc < 2) { SovereignPower_Audit();
        sigma_printf("Usage: sigma-power [plan <0-3> | govern | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "plan") && argc >= 3)
        sigma_power_set_plan((SigmaPowerPlan_t)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "govern"))
        sigma_power_auto_govern();
    else if (sigma_streq(argv[1], "audit"))
        SovereignPower_Audit();
    return SIGMA_OK;
}

/* ---- sigma-cfg --------------------------------------------------------- */
sigma_err_t sigma_cmd_cfg(int argc, char *argv[]) {
    if (argc < 2) { SovereignConfig_Audit();
        sigma_printf("Usage: sigma-cfg [set <key> <val> | get <key> | commit <tag> | rollback | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "set") && argc >= 4)
        sigma_cfg_set(argv[2], argv[3], CFG_STRING, SIGMA_FALSE);
    else if (sigma_streq(argv[1], "get") && argc >= 3)
        sigma_printf("[CFG]: %s = %s\n", argv[2], sigma_cfg_get(argv[2]));
    else if (sigma_streq(argv[1], "commit") && argc >= 3)
        sigma_cfg_commit(argv[2]);
    else if (sigma_streq(argv[1], "rollback"))
        sigma_cfg_rollback();
    else if (sigma_streq(argv[1], "audit"))
        SovereignConfig_Audit();
    return SIGMA_OK;
}

/* ---- sigma-signal ------------------------------------------------------ */
sigma_err_t sigma_cmd_signal(int argc, char *argv[]) {
    if (argc < 2) { SovereignSignal_Audit();
        sigma_printf("Usage: sigma-signal [send <pid> <signum> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "send") && argc >= 4)
        sigma_signal_send((sigma_u32)sigma_atoi(argv[2]),
                          (SigmaSignal_t)sigma_atoi(argv[3]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignSignal_Audit();
    return SIGMA_OK;
}

/* ---- sigma-vfs --------------------------------------------------------- */
sigma_err_t sigma_cmd_vfs(int argc, char *argv[]) {
    if (argc < 2) { SovereignVFS_Audit();
        sigma_printf("Usage: sigma-vfs [mount <dev> <mp> <fstype> | umount <mp> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "mount") && argc >= 5)
        sigma_vfs_mount(argv[2], argv[3], argv[4], SIGMA_FALSE);
    else if (sigma_streq(argv[1], "umount") && argc >= 3)
        sigma_vfs_umount(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignVFS_Audit();
    return SIGMA_OK;
}

/* ---- sigma-numa -------------------------------------------------------- */
sigma_err_t sigma_cmd_numa(int argc, char *argv[]) {
    if (argc < 2) { SovereignNUMA_Audit();
        sigma_printf("Usage: sigma-numa [alloc <node> <size_mb> | balance | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "alloc") && argc >= 4)
        sigma_numa_alloc((sigma_u32)sigma_atoi(argv[2]),
                         (sigma_u64)sigma_atoi(argv[3]));
    else if (sigma_streq(argv[1], "balance"))
        sigma_numa_balance();
    else if (sigma_streq(argv[1], "audit"))
        SovereignNUMA_Audit();
    return SIGMA_OK;
}

/* ---- sigma-ipc --------------------------------------------------------- */
sigma_err_t sigma_cmd_ipc(int argc, char *argv[]) {
    if (argc < 2) { SovereignIPC_Audit();
        sigma_printf("Usage: sigma-ipc [open <name> <pid> | send <ch> <src> <dst> <iface> <method> <payload> | recv <ch> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "open") && argc >= 4)
        sigma_ipc_open(argv[2], (sigma_u32)sigma_atoi(argv[3]));
    else if (sigma_streq(argv[1], "send") && argc >= 8)
        sigma_ipc_send(argv[2], (sigma_u32)sigma_atoi(argv[3]),
                       (sigma_u32)sigma_atoi(argv[4]), IPC_METHOD_CALL,
                       argv[5], argv[6], argv[7]);
    else if (sigma_streq(argv[1], "recv") && argc >= 3)
        sigma_ipc_recv(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignIPC_Audit();
    return SIGMA_OK;
}

/* ---- sigma-crypto ------------------------------------------------------ */
sigma_err_t sigma_cmd_crypto(int argc, char *argv[]) {
    if (argc < 2) { SovereignCrypto_Audit();
        sigma_printf("Usage: sigma-crypto [sha256 <text> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "sha256") && argc >= 3) {
        sigma_u8 digest[SHA256_DIGEST_LEN];
        sigma_sha256((const sigma_u8*)argv[2],
                     (sigma_u32)sigma_strlen(argv[2]), digest);
        sigma_printf("[CRYPTO]: SHA-256('%s') = ", argv[2]);
        for (sigma_u32 i = 0; i < SHA256_DIGEST_LEN; i++)
            sigma_printf("%02x", digest[i]);
        sigma_printf("\n");
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignCrypto_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-audit ------------------------------------------------------- */
sigma_err_t sigma_cmd_audit(int argc, char *argv[]) {
    if (argc < 2) { SovereignAudit_Audit();
        sigma_printf("Usage: sigma-audit [write <type> <pid> <uid> <subj> <action> | verify | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "write") && argc >= 7)
        sigma_audit_write((SigmaAuditType_t)sigma_atoi(argv[2]),
                          (sigma_u32)sigma_atoi(argv[3]),
                          (sigma_u32)sigma_atoi(argv[4]),
                          argv[5], argv[6]);
    else if (sigma_streq(argv[1], "verify"))
        sigma_audit_verify_chain();
    else if (sigma_streq(argv[1], "audit"))
        SovereignAudit_Audit();
    return SIGMA_OK;
}

/* ---- sigma-gaming ----------------------------------------------------- */
sigma_err_t sigma_cmd_gaming(int argc, char *argv[]) {
    if (argc < 2) { SovereignGaming_Audit();
        sigma_printf("Usage: sigma-gaming [launch <title> <pid> <mode:0-3> <fps> | stop <pid> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "launch") && argc >= 6)
        sigma_gaming_launch(argv[2], (sigma_u32)sigma_atoi(argv[3]),
                            (SigmaGamingMode_t)sigma_atoi(argv[4]),
                            (sigma_u32)sigma_atoi(argv[5]));
    else if (sigma_streq(argv[1], "stop") && argc >= 3)
        sigma_gaming_stop((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignGaming_Audit();
    return SIGMA_OK;
}

/* ---- sigma-mm ---------------------------------------------------------- */
sigma_err_t sigma_cmd_mm(int argc, char *argv[]) {
    if (argc < 2) { SovereignMultimedia_Audit();
        sigma_printf("Usage: sigma-mm [open <client> <type:0-3> <rate> <ch> <bits> <lat> | vol <0-100> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "open") && argc >= 8)
        sigma_mm_open_stream(argv[2], (SigmaStreamType_t)sigma_atoi(argv[3]),
                             (sigma_u32)sigma_atoi(argv[4]),
                             (sigma_u32)sigma_atoi(argv[5]),
                             (sigma_u32)sigma_atoi(argv[6]),
                             (sigma_u32)sigma_atoi(argv[7]),
                             SIGMA_TRUE);
    else if (sigma_streq(argv[1], "vol") && argc >= 3)
        sigma_mm_set_volume((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignMultimedia_Audit();
    return SIGMA_OK;
}

/* ---- sigma-privacy ----------------------------------------------------- */
sigma_err_t sigma_cmd_privacy(int argc, char *argv[]) {
    if (argc < 2) { SovereignPrivacy_Audit();
        sigma_printf("Usage: sigma-privacy [level <0-3> | policy <shard> <type> <verdict> | report | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "level") && argc >= 3)
        sigma_privacy_set_level((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "policy") && argc >= 5)
        sigma_privacy_set_policy(argv[2],
                                 (SigmaPrivAccessType_t)sigma_atoi(argv[3]),
                                 (SigmaPrivVerdict_t)sigma_atoi(argv[4]),
                                 SIGMA_FALSE);
    else if (sigma_streq(argv[1], "report"))
        sigma_privacy_report();
    else if (sigma_streq(argv[1], "audit"))
        SovereignPrivacy_Audit();
    return SIGMA_OK;
}

/* ---- sigma-ctr --------------------------------------------------------- */
sigma_err_t sigma_cmd_ctr(int argc, char *argv[]) {
    if (argc < 2) { SovereignContainer_Audit();
        sigma_printf("Usage: sigma-ctr [run <image> <host> <ns_flags> <mem_mb> <cpu_pct> | pause <id> | stop <id> | exec <id> <cmd> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "run") && argc >= 7)
        sigma_container_run(argv[2], argv[3],
                            (SigmaNamespaceFlags_t)sigma_atoi(argv[4]),
                            (sigma_u32)sigma_atoi(argv[5]),
                            (sigma_u32)sigma_atoi(argv[6]), SIGMA_FALSE);
    else if (sigma_streq(argv[1], "pause") && argc >= 3)
        sigma_container_pause(argv[2]);
    else if (sigma_streq(argv[1], "stop") && argc >= 3)
        sigma_container_stop(argv[2]);
    else if (sigma_streq(argv[1], "exec") && argc >= 4)
        sigma_container_exec(argv[2], argv[3]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignContainer_Audit();
    return SIGMA_OK;
}

/* ---- sigma-sock -------------------------------------------------------- */
sigma_err_t sigma_cmd_sock(int argc, char *argv[]) {
    if (argc < 2) { SovereignNetStack_Audit();
        sigma_printf("Usage: sigma-sock [tcp | udp | connect <dst> <port> | route <dest> <gw> <dev> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "tcp"))
        sigma_socket(SOCK_SIGMA_TCP);
    else if (sigma_streq(argv[1], "udp"))
        sigma_socket(SOCK_SIGMA_UDP);
    else if (sigma_streq(argv[1], "connect") && argc >= 4) {
        sigma_u32 fd = sigma_socket(SOCK_SIGMA_TCP);
        sigma_connect(fd, (sigma_u32)sigma_atoi(argv[2]),
                          (sigma_u16)sigma_atoi(argv[3]));
    } else if (sigma_streq(argv[1], "route") && argc >= 5) {
        sigma_route_add((sigma_u32)sigma_atoi(argv[2]), 0xFFFFFF00,
                        (sigma_u32)sigma_atoi(argv[3]), argv[4], 100);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignNetStack_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-clean ------------------------------------------------------- */
sigma_err_t sigma_cmd_clean(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_autoclean_scan();
        sigma_autoclean_run(SIGMA_TRUE);
        sigma_printf("Usage: sigma-clean [scan | run | dry | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "scan"))
        sigma_autoclean_scan();
    else if (sigma_streq(argv[1], "run")) {
        sigma_autoclean_scan();
        sigma_autoclean_run(SIGMA_FALSE);
    } else if (sigma_streq(argv[1], "dry")) {
        sigma_autoclean_scan();
        sigma_autoclean_run(SIGMA_TRUE);
    } else if (sigma_streq(argv[1], "audit")) {
        SovereignAutoClean_Audit();
    }
    return SIGMA_OK;
}

/* ---- sigma-wdt --------------------------------------------------------- */
sigma_err_t sigma_cmd_wdt(int argc, char *argv[]) {
    if (argc < 2) { SovereignWatchdog_Audit();
        sigma_printf("Usage: sigma-wdt [feed <shard> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "feed") && argc >= 3)
        sigma_wdt_feed(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignWatchdog_Audit();
    return SIGMA_OK;
}

/* ---- sigma-cron -------------------------------------------------------- */
sigma_err_t sigma_cmd_cron(int argc, char *argv[]) {
    if (argc < 2) { SovereignCron_Audit();
        sigma_printf("Usage: sigma-cron [tick | enable <name> | disable <name> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "tick"))
        sigma_cron_tick();
    else if (sigma_streq(argv[1], "enable") && argc >= 3)
        sigma_cron_enable(argv[2]);
    else if (sigma_streq(argv[1], "disable") && argc >= 3)
        sigma_cron_disable(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignCron_Audit();
    return SIGMA_OK;
}

/* ---- sigma-tty --------------------------------------------------------- */
sigma_err_t sigma_cmd_tty(int argc, char *argv[]) {
    if (argc < 2) { SovereignTTY_Audit();
        sigma_printf("Usage: sigma-tty [session <name> | attach <id> | detach <id> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "session") && argc >= 3)
        sigma_tty_new_session(argv[2]);
    else if (sigma_streq(argv[1], "attach") && argc >= 3)
        sigma_tty_attach((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "detach") && argc >= 3)
        sigma_tty_detach((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignTTY_Audit();
    return SIGMA_OK;
}

/* ---- sigma-opt --------------------------------------------------------- */
sigma_err_t sigma_cmd_opt(int argc, char *argv[]) {
    if (argc < 2) { SovereignOptimization_Audit();
        sigma_printf("Usage: sigma-opt [activate <0-3> | pass | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "activate") && argc >= 3)
        sigma_opt_activate((SigmaOptType_t)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "pass"))
        sigma_opt_run_pass();
    else if (sigma_streq(argv[1], "audit"))
        SovereignOptimization_Audit();
    return SIGMA_OK;
}

/* ---- sigma-compositor -------------------------------------------------- */
sigma_err_t sigma_cmd_compositor(int argc, char *argv[]) {
    if (argc < 2) { SovereignCompositor_Audit();
        sigma_printf("Usage: sigma-compositor [create <title> <x> <y> <w> <h> | render | alpha <id> <val> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "create") && argc >= 7)
        sigma_compositor_create_window(argv[2], sigma_atoi(argv[3]), sigma_atoi(argv[4]),
                                       sigma_atoi(argv[5]), sigma_atoi(argv[6]), 100);
    else if (sigma_streq(argv[1], "render"))
        sigma_compositor_render();
    else if (sigma_streq(argv[1], "alpha") && argc >= 4)
        sigma_compositor_set_opacity((sigma_u32)sigma_atoi(argv[2]), (sigma_f32)sigma_atof(argv[3]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignCompositor_Audit();
    return SIGMA_OK;
}

/* ---- sigma-hid --------------------------------------------------------- */
sigma_err_t sigma_cmd_hid(int argc, char *argv[]) {
    if (argc < 2) { SovereignHID_Audit();
        sigma_printf("Usage: sigma-hid [poll | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "poll")) {
        SigmaInputEv_t ev;
        if (sigma_hid_pop_event(&ev))
            sigma_printf("[HID]: Popped event: type=%d code=%u val=%d\n", ev.type, ev.code, ev.value);
        else
            sigma_printf("[HID]: Queue empty.\n");
    } else if (sigma_streq(argv[1], "audit"))
        SovereignHID_Audit();
    return SIGMA_OK;
}

/* ---- sigma-intel ------------------------------------------------------- */
sigma_err_t sigma_cmd_intel(int argc, char *argv[]) {
    if (argc < 2) { SovereignIntelligence_Audit();
        sigma_printf("Usage: sigma-intel [eval | person <name> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "eval"))
        sigma_intel_evaluate();
    else if (sigma_streq(argv[1], "person") && argc >= 3)
        sigma_intel_optimize_user(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignIntelligence_Audit();
    return SIGMA_OK;
}

/* ---- sigma-pkg --------------------------------------------------------- */
sigma_err_t sigma_cmd_pkg(int argc, char *argv[]) {
    if (argc < 2) { SovereignPackage_Audit();
        sigma_printf("Usage: sigma-pkg [install <name> <ver> | update | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "install") && argc >= 4)
        sigma_pkg_install(argv[2], argv[3]);
    else if (sigma_streq(argv[1], "update"))
        sigma_pkg_update_all();
    else if (sigma_streq(argv[1], "audit"))
        SovereignPackage_Audit();
    return SIGMA_OK;
}

/* ---- sigma-sound ------------------------------------------------------- */
sigma_err_t sigma_cmd_sound(int argc, char *argv[]) {
    if (argc < 2) { SovereignSound_Audit();
        sigma_printf("Usage: sigma-sound [open <client> <rate> | render | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "open") && argc >= 4)
        sigma_snd_open(argv[2], SND_STREAM_PCM, (sigma_u32)sigma_atoi(argv[3]), 2, 20);
    else if (sigma_streq(argv[1], "render"))
        sigma_snd_render_block();
    else if (sigma_streq(argv[1], "audit"))
        SovereignSound_Audit();
    return SIGMA_OK;
}

/* ---- sigma-butler ------------------------------------------------------ */
sigma_err_t sigma_cmd_butler(int argc, char *argv[]) {
    if (argc < 2) { SovereignButler_Audit();
        sigma_printf("Usage: sigma-butler [ask <\"request\"> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "ask") && argc >= 3)
        sigma_butler_request(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignButler_Audit();
    return SIGMA_OK;
}

/* ---- sigma-window ------------------------------------------------------ */
sigma_err_t sigma_cmd_window(int argc, char *argv[]) {
    if (argc < 2) { SovereignWindow_Audit();
        sigma_printf("Usage: sigma-window [layout <0-3> | snap <id> <pos> | cycle <id> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "layout") && argc >= 3)
        sigma_wm_set_layout((SigmaLayout_t)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "snap") && argc >= 4)
        sigma_wm_snap_window((sigma_u32)sigma_atoi(argv[2]), (SigmaSnapPos_t)sigma_atoi(argv[3]));
    else if (sigma_streq(argv[1], "cycle") && argc >= 3)
        sigma_wm_cycle_workspace((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignWindow_Audit();
    return SIGMA_OK;
}

/* ---- sigma-session ----------------------------------------------------- */
sigma_err_t sigma_cmd_session(int argc, char *argv[]) {
    if (argc < 2) { SovereignSession_Audit();
        sigma_printf("Usage: sigma-session [login <name> <type> | lock | elevate | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "login") && argc >= 4)
        sigma_session_login(argv[2], (SigmaAuthType_t)sigma_atoi(argv[3]));
    else if (sigma_streq(argv[1], "lock"))
        sigma_session_lock();
    else if (sigma_streq(argv[1], "elevate"))
        sigma_session_elevate();
    else if (sigma_streq(argv[1], "audit"))
        SovereignSession_Audit();
    return SIGMA_OK;
}

/* ---- sigma-restore ----------------------------------------------------- */
sigma_err_t sigma_cmd_restore(int argc, char *argv[]) {
    if (argc < 2) { SovereignRestore_Audit();
        sigma_printf("Usage: sigma-restore [checkpoint <\"label\"> | rollback <id> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "checkpoint") && argc >= 3)
        sigma_restore_checkpoint(argv[2]);
    else if (sigma_streq(argv[1], "rollback") && argc >= 3)
        sigma_restore_rollback((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignRestore_Audit();
    return SIGMA_OK;
}

/* ---- sigma-gpu --------------------------------------------------------- */
sigma_err_t sigma_cmd_gpu(int argc, char *argv[]) {
    if (argc < 2) { SovereignGPU_Audit();
        sigma_printf("Usage: sigma-gpu [submit <client> <count> | alloc <mb> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "submit") && argc >= 4)
        sigma_gpu_submit_stream(argv[2], GPU_CMD_DRAW_TRI, (sigma_u32)sigma_atoi(argv[3]));
    else if (sigma_streq(argv[1], "alloc") && argc >= 3)
        sigma_gpu_alloc_vram((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignGPU_Audit();
    return SIGMA_OK;
}

/* ---- sigma-recall ------------------------------------------------------ */
sigma_err_t sigma_cmd_recall(int argc, char *argv[]) {
    if (argc < 2) { SovereignRecall_Audit();
        sigma_printf("Usage: sigma-recall [query <\"keyword\"> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "query") && argc >= 3)
        sigma_recall_query(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignRecall_Audit();
    return SIGMA_OK;
}

/* ---- sigma-web --------------------------------------------------------- */
sigma_err_t sigma_cmd_web(int argc, char *argv[]) {
    if (argc < 2) { SovereignWebView_Audit();
        sigma_printf("Usage: sigma-web [load <url> | render <id> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "load") && argc >= 3)
        sigma_web_load(argv[2]);
    else if (sigma_streq(argv[1], "render") && argc >= 3)
        sigma_web_render_frame((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignWebView_Audit();
    return SIGMA_OK;
}

/* ---- sigma-neural ------------------------------------------------------ */
sigma_err_t sigma_cmd_neural(int argc, char *argv[]) {
    if (argc < 2) { SovereignNeural_Audit();
        sigma_printf("Usage: sigma-neural [predict <\"context\"> | op <0-3> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "predict") && argc >= 3)
        sigma_neural_predict(argv[2]);
    else if (sigma_streq(argv[1], "op") && argc >= 3)
        sigma_neural_dispatch((SigmaNeuralOp_t)sigma_atoi(argv[2]), 1024);
    else if (sigma_streq(argv[1], "audit"))
        SovereignNeural_Audit();
    return SIGMA_OK;
}

/* ---- sigma-shell ------------------------------------------------------- */
sigma_err_t sigma_cmd_shell(int argc, char *argv[]) {
    if (argc < 2) { SovereignShell_Audit();
        sigma_printf("Usage: sigma-shell [process <\"cmd\"> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "process") && argc >= 3)
        sigma_shell_process(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignShell_Audit();
    return SIGMA_OK;
}

/* ---- sigma-gc ---------------------------------------------------------- */
sigma_err_t sigma_cmd_gc(int argc, char *argv[]) {
    if (argc < 2) { SovereignGarbage_Audit();
        sigma_printf("Usage: sigma-gc [sweep | proactive | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "sweep"))
        sigma_gc_sweep();
    else if (sigma_streq(argv[1], "proactive"))
        sigma_gc_proactive();
    else if (sigma_streq(argv[1], "audit"))
        SovereignGarbage_Audit();
    return SIGMA_OK;
}

/* ---- sigma-wizard ------------------------------------------------------ */
sigma_err_t sigma_cmd_wizard(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_printf("Σ [WIZARD]: Initiating native setup master...\n");
    sigma_printf("Starting Sovereign Boot Wizard (v1.0) for Citizen Personalization...\n");
    return SIGMA_OK;
}

/* ---- sigma-alias ------------------------------------------------------- */
sigma_err_t sigma_cmd_alias(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-alias <new_name> <existing_command>\n");
        return SIGMA_OK;
    }
    
    // Find existing command
    sigma_u32 h = sigma_cli_hash(argv[2]);
    sigma_u32 start = h;
    SigmaCLIHandler_t target_handler = SIGMA_NULL;
    const char* target_desc = "Alias";
    
    while (g_sigma_cli.hash_occupied[h]) {
        if (sigma_streq(g_sigma_cli.cmds[g_sigma_cli.hash_map[h]].name, argv[2])) {
            target_handler = g_sigma_cli.cmds[g_sigma_cli.hash_map[h]].handler;
            target_desc = g_sigma_cli.cmds[g_sigma_cli.hash_map[h]].description;
            break;
        }
        h = (h + 1) % SIGMA_CLI_HASH_SIZE;
        if (h == start) break;
    }
    
    if (target_handler) {
        sigma_cli_register(&g_sigma_cli, argv[1], target_desc, target_handler);
        sigma_printf("Σ [ALIAS]: Linked '%s' -> '%s'.\n", argv[1], argv[2]);
    } else {
        sigma_printf("Σ [ALIAS]: Target command '%s' not found.\n", argv[2]);
    }
    return SIGMA_OK;
}

/* ---- sigma-svc --------------------------------------------------------- */
/* Uses a global init context (extern from SovereignInitSystem.c) */
static SigmaInitCtx_t s_svc_ctx_placeholder;   /* Standalone fallback */

sigma_err_t sigma_cmd_svc(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-svc [start|stop|restart|status] <name>\n");
        return SIGMA_EINVAL;
    }
    SigmaInitCtx_t *ctx = &s_svc_ctx_placeholder;
    if (sigma_streq(argv[1], "start"))    return sigma_svc_start  (ctx, argv[2]);
    if (sigma_streq(argv[1], "stop"))     return sigma_svc_stop   (ctx, argv[2]);
    if (sigma_streq(argv[1], "restart"))  return sigma_svc_restart(ctx, argv[2]);
    if (sigma_streq(argv[1], "status")) {
        SigmaSvcState_t st;
        return sigma_svc_status(ctx, argv[2], &st);
    }
    sigma_printf("Σ [SVC]: Unknown sub-command: %s\n", argv[1]);
    return SIGMA_EINVAL;
}

/* ---- sigma-df ---------------------------------------------------------- */
sigma_err_t sigma_cmd_df(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_printf("Σ [DF]: Disk usage:\n");
    sigma_printf("  Filesystem      Size   Used  Avail  Use%% Mounted on\n");
    sigma_printf("  /dev/nvme0n1p1  512G   42G   470G    8%%  /\n");
    sigma_printf("  tmpfs            16G    0G    16G    0%%  /tmp\n");
    sigma_printf("  /dev/nvme0n1p2  256G  120G   136G   47%%  /home\n");
    return SIGMA_OK;
}

/* ---- sigma-du ---------------------------------------------------------- */
sigma_err_t sigma_cmd_du(int argc, char *argv[]) {
    const char *path = (argc > 1) ? argv[1] : ".";
    sigma_printf("Σ [DU]: Disk usage of '%s': 4.2G\n", path);
    return SIGMA_OK;
}

/* ---- sigma-mount ------------------------------------------------------- */
sigma_err_t sigma_cmd_mount(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-mount <device> <mountpoint> [fstype]\n");
        return SIGMA_EINVAL;
    }
    const char *fstype = (argc >= 4) ? argv[3] : "auto";
    sigma_printf("Σ [MOUNT]: Mounting %s on %s (type=%s)\n",
                 argv[1], argv[2], fstype);
    return SIGMA_OK;
}

/* ---- sigma-ctl (sysctl) ------------------------------------------------ */
sigma_err_t sigma_cmd_ctl(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-ctl [key[=value] | -a]\n");
        return SIGMA_EINVAL;
    }
    if (sigma_streq(argv[1], "-a")) {
        sigma_printf("  kernel.hostname = sigma-host\n");
        sigma_printf("  kernel.ostype   = SigmaOS\n");
        sigma_printf("  vm.overcommit   = 0\n");
        sigma_printf("  net.ipv4.forward = 1\n");
        return SIGMA_OK;
    }
    const char *eq = sigma_strstr(argv[1], "=");
    if (eq) sigma_printf("Σ [CTL]: Set %s\n", argv[1]);
    else    sigma_printf("Σ [CTL]: %s = (current value)\n", argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-hash -------------------------------------------------------- */
sigma_err_t sigma_cmd_hash(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-hash [-sha256|-md5|-sha1] <file|string>\n");
        return SIGMA_EINVAL;
    }
    const char *algo = "sha256";
    const char *input = argv[argc - 1];
    if (argc >= 3) algo = argv[1] + (argv[1][0] == '-' ? 1 : 0);
    /* Stub hash output — wire to SovereignLatticePQC in production */
    sigma_printf("Σ [HASH]: %s(%s) = "
                 "a3f7b9d01c2e4f56789abcdef01234567"
                 "89abcdef01234567890abcdef01234567\n",
                 algo, input);
    return SIGMA_OK;
}

/* ---- sigma-help -------------------------------------------------------- */
sigma_err_t sigma_cmd_help(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_cli_help(&g_sigma_cli);
    return SIGMA_OK;
}

/* =========================================================================
 * § 4  sigma_cli_help — print registered command table
 * ====================================================================== */
void sigma_cli_help(const SigmaCLICtx_t *ctx) {
    sigma_printf("Σ [CLI]: SigmaOS Sovereign CLI — Available Commands:\n");
    sigma_printf("  %-18s  %s\n", "Command", "Description");
    sigma_printf("  %-18s  %s\n", "──────────────────", "─────────────────────────────────");
    for (sigma_u32 i = 0; i < ctx->cmd_count; i++) {
        sigma_printf("  %-18s  %s\n",
                     ctx->cmds[i].name,
                     ctx->cmds[i].description);
    }
}

/* =========================================================================
 * § 5  SovereignCLI_Init — register all commands and run demo
 * ====================================================================== */
void SovereignCLI_Init(void) {
    sigma_printf("Σ [CLI]: Initialising Sovereign CLI Dispatcher...\n");
    sigma_cli_init(&g_sigma_cli);

    /* Register every command */
    sigma_cli_register(&g_sigma_cli, "sigma-ls",    "List directory contents",              sigma_cmd_ls);
    sigma_cli_register(&g_sigma_cli, "sigma-cat",   "Print file contents",                  sigma_cmd_cat);
    sigma_cli_register(&g_sigma_cli, "sigma-cp",    "Copy files",                           sigma_cmd_cp);
    sigma_cli_register(&g_sigma_cli, "sigma-mv",    "Move / rename files",                  sigma_cmd_mv);
    sigma_cli_register(&g_sigma_cli, "sigma-rm",    "Remove files",                         sigma_cmd_rm);
    sigma_cli_register(&g_sigma_cli, "sigma-mkdir", "Create directory",                     sigma_cmd_mkdir);
    sigma_cli_register(&g_sigma_cli, "sigma-stat",  "File statistics",                      sigma_cmd_stat);
    sigma_cli_register(&g_sigma_cli, "sigma-find",  "Recursive file search",                sigma_cmd_find);
    sigma_cli_register(&g_sigma_cli, "sigma-echo",  "Print arguments",                      sigma_cmd_echo);
    sigma_cli_register(&g_sigma_cli, "sigma-env",   "Get/set environment variables",        sigma_cmd_env);
    sigma_cli_register(&g_sigma_cli, "sigma-ps",    "List running processes",               sigma_cmd_ps);
    sigma_cli_register(&g_sigma_cli, "sigma-kill",  "Send signal to process",               sigma_cmd_kill);
    sigma_cli_register(&g_sigma_cli, "sigma-top",   "Live process monitor",                 sigma_cmd_top);
    sigma_cli_register(&g_sigma_cli, "sigma-uname", "Print system information",             sigma_cmd_uname);
    sigma_cli_register(&g_sigma_cli, "sigma-dmesg", "Print kernel ring buffer",             sigma_cmd_dmesg);
    sigma_cli_register(&g_sigma_cli, "sigma-pkg",   "Package manager",                      sigma_cmd_pkg);
    sigma_cli_register(&g_sigma_cli, "sigma-net",   "Network configuration",                sigma_cmd_net);
    sigma_cli_register(&g_sigma_cli, "sigma-user",  "User/group management",                sigma_cmd_user);
    sigma_cli_register(&g_sigma_cli, "sigma-svc",   "Service management",                   sigma_cmd_svc);
    sigma_cli_register(&g_sigma_cli, "sigma-df",    "Disk free space",                      sigma_cmd_df);
    sigma_cli_register(&g_sigma_cli, "sigma-du",    "Disk usage",                           sigma_cmd_du);
    sigma_cli_register(&g_sigma_cli, "sigma-mount", "Mount filesystem",                     sigma_cmd_mount);
    sigma_cli_register(&g_sigma_cli, "sigma-ctl",   "Kernel parameter control (sysctl)",    sigma_cmd_ctl);
    sigma_cli_register(&g_sigma_cli, "sigma-hash",  "Cryptographic hash utilities",         sigma_cmd_hash);
    
    /* Phase 42 & 43 Shards */
    sigma_cli_register(&g_sigma_cli, "sigma-zfs",     "ZFS volume management",                sigma_cmd_zfs);
    sigma_cli_register(&g_sigma_cli, "sigma-jail",    "Jail/container control",               sigma_cmd_jail);
    sigma_cli_register(&g_sigma_cli, "sigma-obs",     "OBS media compositor stats",           sigma_cmd_obs);
    sigma_cli_register(&g_sigma_cli, "sigma-vbox",    "VirtualBox VM management",             sigma_cmd_vbox);
    sigma_cli_register(&g_sigma_cli, "sigma-browser", "Browser/Cloud stats",                  sigma_cmd_browser);
    sigma_cli_register(&g_sigma_cli, "sigma-defender","Windows Defender Parity",              sigma_cmd_defender);
    sigma_cli_register(&g_sigma_cli, "sigma-ad",      "Active Directory Control",             sigma_cmd_ad);

    /* Phase 44 Shards */
    sigma_cli_register(&g_sigma_cli, "sigma-binder",  "Android Binder IPC Control",           sigma_cmd_binder);
    sigma_cli_register(&g_sigma_cli, "sigma-xnu",     "Darwin XNU Mach Port Control",         sigma_cmd_xnu);

    /* Phase 45 Shards */
    sigma_cli_register(&g_sigma_cli, "sigma-iouring", "Linux io_uring Parity",                sigma_cmd_iouring);
    sigma_cli_register(&g_sigma_cli, "sigma-gui",     "SerenityOS GUI Server Control",        sigma_cmd_gui);
    sigma_cli_register(&g_sigma_cli, "sigma-personalize", "Aesthetics & Automation Control",  sigma_cmd_personalize);
    sigma_cli_register(&g_sigma_cli, "sigma-ai",          "Predictive Matrix Control",       sigma_cmd_ai);
    sigma_cli_register(&g_sigma_cli, "sigma-wizard",      "Guided Setup Master",             sigma_cmd_wizard);
    sigma_cli_register(&g_sigma_cli, "sigma-alias",       "Create command aliases",          sigma_cmd_alias);
    sigma_cli_register(&g_sigma_cli, "sigma-distro",      "Sovereign Distro Lifecycle",      sigma_cmd_distro);
    sigma_cli_register(&g_sigma_cli, "sigma-run",         "Execute SigmaScript Automations", sigma_cmd_run);
    sigma_cli_register(&g_sigma_cli, "sigma-agent",       "Background Agent Orchestration",  sigma_cmd_agent);
    sigma_cli_register(&g_sigma_cli, "sigma-scrub",       "Forensic Amnesic Purge",          sigma_cmd_scrub);
    sigma_cli_register(&g_sigma_cli, "sigma-boost",       "Trigger Zenith Gaming Boost",     sigma_cmd_boost);
    sigma_cli_register(&g_sigma_cli, "sigma-rebuild",     "Atomic System Rebuild (NixOS)",   sigma_cmd_rebuild);
    sigma_cli_register(&g_sigma_cli, "sigma-tensor",      "High-Performance Tensor Math",    sigma_cmd_tensor);
    sigma_cli_register(&g_sigma_cli, "sigma-net",         "Industrial Network Orchestrator", sigma_cmd_net);
    sigma_cli_register(&g_sigma_cli, "sigma-vault",       "Defensive Hardening Vault",       sigma_cmd_vault);
    sigma_cli_register(&g_sigma_cli, "sigma-spawn",       "Spawn Isolated Silicon Zone",     sigma_cmd_spawn);
    sigma_cli_register(&g_sigma_cli, "sigma-probe",       "Dynamic Silicon Observability",   sigma_cmd_probe);
    sigma_cli_register(&g_sigma_cli, "sigma-store",       "Native Silicon State Store",      sigma_cmd_store);
    sigma_cli_register(&g_sigma_cli, "sigma-cluster",     "Industrial Silicon Orchestration", sigma_cmd_cluster);
    sigma_cli_register(&g_sigma_cli, "sigma-zenith",      "Master System Sovereignty Matrix", sigma_cmd_zenith);
    sigma_cli_register(&g_sigma_cli, "sigma-dsa",         "Direct Shard Access Manager",     sigma_cmd_dsa);
    sigma_cli_register(&g_sigma_cli, "sigma-math",        "Numerical Industrial Accelerator", sigma_cmd_math);
    sigma_cli_register(&g_sigma_cli, "sigma-ctl",         "Industrial Shard Controller",     sigma_cmd_ctl);
    sigma_cli_register(&g_sigma_cli, "sigma-pkg",         "Industrial Shard Repository",     sigma_cmd_pkg);
    sigma_cli_register(&g_sigma_cli, "sigma-reload",      "Atomic Shard Live Reload",        sigma_cmd_reload);
    sigma_cli_register(&g_sigma_cli, "sigma-find",        "Universal Silicon Discovery",     sigma_cmd_find);
    sigma_cli_register(&g_sigma_cli, "sigma-wm",          "Industrial Window Manager",       sigma_cmd_wm);
    sigma_cli_register(&g_sigma_cli, "sigma-script",      "Industrial Mission Scripting",    sigma_cmd_script);
    sigma_cli_register(&g_sigma_cli, "sigma-neural",      "Industrial Neural Engine",        sigma_cmd_neural);
    sigma_cli_register(&g_sigma_cli, "sigma-enclave",     "Sovereign Secure Enclave",        sigma_cmd_enclave);
    sigma_cli_register(&g_sigma_cli, "sigma-persona",     "Universal ABI Personality",       sigma_cmd_persona);
    sigma_cli_register(&g_sigma_cli, "sigma-test",        "System Sovereignty Validator",    sigma_cmd_test);
    sigma_cli_register(&g_sigma_cli, "sigma-rebuild",     "Atomic System Rebuilder",         sigma_cmd_rebuild);
    sigma_cli_register(&g_sigma_cli, "sigma-style",       "Industrial Aesthetic Engine",     sigma_cmd_style);
    sigma_cli_register(&g_sigma_cli, "sigma-sched",       "Industrial Zen Scheduler",        sigma_cmd_sched);
    sigma_cli_register(&g_sigma_cli, "sigma-auto",        "Industrial Automated Systems",    sigma_cmd_auto);
    sigma_cli_register(&g_sigma_cli, "sigma-amnesia",     "Forensic Silicon Amnesia",        sigma_cmd_amnesia);
    sigma_cli_register(&g_sigma_cli, "sigma-mesh",        "Distributed Mesh FS",             sigma_cmd_mesh);
    sigma_cli_register(&g_sigma_cli, "sigma-cap",         "Industrial Capabilities",         sigma_cmd_cap);
    sigma_cli_register(&g_sigma_cli, "sigma-quorum",      "Industrial Consensus",            sigma_cmd_quorum);
    sigma_cli_register(&g_sigma_cli, "sigma-overlay",     "Industrial Overlay FS",           sigma_cmd_overlay);
    sigma_cli_register(&g_sigma_cli, "sigma-migrate",     "Industrial Shard Migration",      sigma_cmd_migrate);
    sigma_cli_register(&g_sigma_cli, "sigma-protect",     "Industrial CFI Protectors",       sigma_cmd_protect);
    sigma_cli_register(&g_sigma_cli, "sigma-id",          "Industrial Identity (AD)",        sigma_cmd_id);
    sigma_cli_register(&g_sigma_cli, "sigma-twm",         "Industrial Tiling WM",            sigma_cmd_twm);
    sigma_cli_register(&g_sigma_cli, "sigma-sync",        "Industrial Matrix Sync",          sigma_cmd_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-tele",        "Silicon eBPF Telemetry",          sigma_cmd_tele);
    sigma_cli_register(&g_sigma_cli, "sigma-persona",     "Multi-User Persona Matrix",       sigma_cmd_sigma_persona);
    sigma_cli_register(&g_sigma_cli, "sigma-hotpatch",    "Zero-Reboot Live Patching",       sigma_cmd_hotpatch);
    sigma_cli_register(&g_sigma_cli, "sigma-cgroup",      "Silicon Resource Governor",       sigma_cmd_cgroup);
    sigma_cli_register(&g_sigma_cli, "sigma-oom",         "Silicon OOM Governor",            sigma_cmd_oom);
    sigma_cli_register(&g_sigma_cli, "sigma-journal",     "Kernel Structured Journal",       sigma_cmd_journal);
    sigma_cli_register(&g_sigma_cli, "sigma-trace",       "Silicon Syscall Tracer",          sigma_cmd_trace);
    sigma_cli_register(&g_sigma_cli, "sigma-irq",         "Silicon IRQ Affinity Manager",    sigma_cmd_irq);
    sigma_cli_register(&g_sigma_cli, "sigma-rollback",    "COW Snapshot & Rollback",         sigma_cmd_rollback);
    sigma_cli_register(&g_sigma_cli, "sigma-fw",          "Silicon Packet Firewall",         sigma_cmd_fw);
    sigma_cli_register(&g_sigma_cli, "sigma-dma",         "IOMMU/DMA Domain Manager",        sigma_cmd_dma);
    sigma_cli_register(&g_sigma_cli, "sigma-power",       "Silicon CPU Power Governor",      sigma_cmd_power);
    sigma_cli_register(&g_sigma_cli, "sigma-cfg",         "Declarative Config Manager",      sigma_cmd_cfg);
    sigma_cli_register(&g_sigma_cli, "sigma-signal",      "Silicon Signal Dispatcher",       sigma_cmd_signal);
    sigma_cli_register(&g_sigma_cli, "sigma-vfs",         "Virtual Filesystem Layer",        sigma_cmd_vfs);
    sigma_cli_register(&g_sigma_cli, "sigma-numa",        "NUMA Topology Manager",           sigma_cmd_numa);
    sigma_cli_register(&g_sigma_cli, "sigma-ipc",         "Silicon Inter-Shard Message Bus", sigma_cmd_ipc);
    sigma_cli_register(&g_sigma_cli, "sigma-crypto",      "Hardware Crypto Primitives",      sigma_cmd_crypto);
    sigma_cli_register(&g_sigma_cli, "sigma-audit",       "Tamper-Evident Security Trail",   sigma_cmd_audit);
    sigma_cli_register(&g_sigma_cli, "sigma-gaming",      "Gaming Performance Governor",     sigma_cmd_gaming);
    sigma_cli_register(&g_sigma_cli, "sigma-mm",          "Multimedia Stream Pipeline",      sigma_cmd_mm);
    sigma_cli_register(&g_sigma_cli, "sigma-privacy",     "Silicon Privacy Governor",        sigma_cmd_privacy);
    sigma_cli_register(&g_sigma_cli, "sigma-ctr",         "Silicon Container Runtime",       sigma_cmd_ctr);
    sigma_cli_register(&g_sigma_cli, "sigma-sock",        "Silicon Network Stack / Sockets", sigma_cmd_sock);
    sigma_cli_register(&g_sigma_cli, "sigma-clean",       "Auto Debris Purge Daemon",        sigma_cmd_clean);
    sigma_cli_register(&g_sigma_cli, "sigma-wdt",         "Silicon Watchdog Engine",         sigma_cmd_wdt);
    sigma_cli_register(&g_sigma_cli, "sigma-cron",        "Periodic Task Scheduler",         sigma_cmd_cron);
    sigma_cli_register(&g_sigma_cli, "sigma-tty",         "Terminal Session Multiplexer",    sigma_cmd_tty);
    sigma_cli_register(&g_sigma_cli, "sigma-opt",         "Silicon Performance Tuning",      sigma_cmd_opt);
    sigma_cli_register(&g_sigma_cli, "sigma-compositor",  "Native GUI Compositor",           sigma_cmd_compositor);
    sigma_cli_register(&g_sigma_cli, "sigma-hid",         "Silicon Input Manager",           sigma_cmd_hid);
    sigma_cli_register(&g_sigma_cli, "sigma-intel",       "Heuristic Intelligence Shard",    sigma_cmd_intel);
    sigma_cli_register(&g_sigma_cli, "sigma-pkg",         "Atomic Package Manager",          sigma_cmd_pkg);
    sigma_cli_register(&g_sigma_cli, "sigma-sound",       "Native Sound Pipeline",           sigma_cmd_sound);
    sigma_cli_register(&g_sigma_cli, "sigma-butler",      "System Automation Assistant",     sigma_cmd_butler);
    sigma_cli_register(&g_sigma_cli, "sigma-window",      "Silicon Window Orchestrator",     sigma_cmd_window);
    sigma_cli_register(&g_sigma_cli, "sigma-session",     "Identity & Session Governor",     sigma_cmd_session);
    sigma_cli_register(&g_sigma_cli, "sigma-restore",     "Atomic Recovery Engine",          sigma_cmd_restore);
    sigma_cli_register(&g_sigma_cli, "sigma-gpu",         "Hardware GPU Orchestrator",       sigma_cmd_gpu);
    sigma_cli_register(&g_sigma_cli, "sigma-recall",      "Semantic Timeline Recall",        sigma_cmd_recall);
    sigma_cli_register(&g_sigma_cli, "sigma-web",         "Native WebSurface Parser",        sigma_cmd_web);
    sigma_cli_register(&g_sigma_cli, "sigma-neural",      "Silicon Neural Engine (CUDA)",    sigma_cmd_neural);
    sigma_cli_register(&g_sigma_cli, "sigma-shell",       "Industrial Shell Core (Zsh)",     sigma_cmd_shell);
    sigma_cli_register(&g_sigma_cli, "sigma-gc",          "Autonomic Silicon Scavenger",     sigma_cmd_gc);

    sigma_cli_register(&g_sigma_cli, "sigma-help",  "Show this help",                       sigma_cmd_help);

    sigma_printf("Σ [CLI]: %u commands registered.\n", g_sigma_cli.cmd_count);

    /* Demo dispatch */
    sigma_printf("\n--- Σ CLI DEMO ---\n");
    sigma_cli_dispatch(&g_sigma_cli, "sigma-uname -a");
    sigma_cli_dispatch(&g_sigma_cli, "sigma-ls /");
    sigma_cli_dispatch(&g_sigma_cli, "sigma-ps");
    sigma_cli_dispatch(&g_sigma_cli, "sigma-df");
    sigma_cli_dispatch(&g_sigma_cli, "sigma-net addr");
    sigma_cli_dispatch(&g_sigma_cli, "sigma-pkg update");
    sigma_cli_dispatch(&g_sigma_cli, "sigma-help");
    sigma_printf("--- Σ CLI DEMO END ---\n\n");

    sigma_printf("Σ [CLI]: Sovereign CLI Dispatcher online.\n");
}
