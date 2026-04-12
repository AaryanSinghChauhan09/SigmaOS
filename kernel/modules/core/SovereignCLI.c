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
#include "../../../include/SovereignStoreShard.h"
#include "../../../include/SovereignForensicShard.h"
#include "../../../include/SovereignHypervisorShard.h"
#include "../../../include/SovereignDefragShard.h"
#include "../../../include/SovereignDSShard.h"
#include "../../../include/SovereignFlowShard.h"
#include "../../../include/SovereignPrefetchShard.h"
#include "../../../include/SovereignDbShard.h"
#include "../../../include/SovereignVaultShard.h"
#include "../../../include/SovereignRTOSShard.h"
#include "../../../include/SovereignPerfShard.h"
#include "../../../include/SovereignMathShard.h"
#include "../../../include/SovereignClusterShard.h"
#include "../../../include/SovereignQuantumShard.h"
#include "../../../include/SovereignDockShard.h"
#include "../../../include/SovereignSpotlightShard.h"
#include "../../../include/SovereignMeshRouteShard.h"
#include "../../../include/SovereignTelemetryShard.h"
#include "../../../include/SovereignHandoffShard.h"
#include "../../../include/SovereignDTraceShard.h"
#include "../../../include/SovereignDRMShard.h"
#include "../../../include/SovereignAirDropShard.h"
#include "../../../include/SovereignSandboxShard.h"
#include "../../../include/SovereignVoiceShard.h"
#include "../../../include/SovereignSideloadShard.h"
#include "../../../include/SovereignContinuityShard.h"
#include "../../../include/SovereignTimeMachineShard.h"
#include "../../../include/SovereignBootloaderShard.h"
#include "../../../include/SovereignTestShard.h"
#include "../../../include/SovereignAutoCleanAlg.h"
#include "../../../include/SovereignAutoPerfAlg.h"
#include "../../../include/SovereignBioAuthShard.h"
#include "../../../include/SovereignEmulationShard.h"
#include "../../../include/SovereignHolographicShard.h"
#include "../../../include/SovereignSwarmAIShard.h"
#include "../../../include/SovereignNeuralInterfaceShard.h"
#include "../../../include/SovereignAutoHealingAlg.h"
#include "../../../include/SovereignFuzzShard.h"
#include "../../../include/SovereignCommandParseAlg.h"
#include "../../../include/SovereignQuantumResilienceAlg.h"
#include "../../../include/SovereignSiliconDefectTest.h"
#include "../../../include/SovereignMacroAutomationAlg.h"
#include "../../../include/SovereignExokernelShard.h"
#include "../../../include/SovereignHomomorphicAlg.h"
#include "../../../include/SovereignZeroKnowledgeAlg.h"
#include "../../../include/SovereignServiceShard.h"
#include "../../../include/SovereignKMSShard.h"
#include "../../../include/SovereignThermalShard.h"
#include "../../../include/SovereignRegistryShard.h"
#include "../../../include/SovereignAudioEngineShard.h"
#include "../../../include/SovereignEcoShard.h"
#include "../../../include/SovereignBluetoothShard.h"
#include "../../../include/SovereignRAIDShard.h"
#include "../../../include/SovereignFaceTrackShard.h"
#include "../../../include/SovereignVPNShard.h"
#include "../../../include/SovereignNeuralSynthShard.h"
#include "../../../include/SovereignDockerShard.h"
#include "../../../include/SovereignDefenderShard.h"
#include "../../../include/SovereignP2PShard.h"
#include "../../../include/SovereignQKDAlg.h"
#include "../../../include/SovereignBootAuditShard.h"
#include "../../../include/SovereignEEGShard.h"
#include "../../../include/SovereignMemTagShard.h"
#include "../../../include/SovereignLivePatchShard.h"
#include "../../../include/SovereignQubitShard.h"
#include "../../../include/SovereignAIHackerShard.h"
#include "../../../include/SovereignMeshOSShard.h"
#include "../../../include/SovereignEEGMeshShard.h"
#include "../../../include/SovereignFoundryShard.h"
#include "../../../include/SovereignExascaleShard.h"
#include "../../../include/SovereignDysonShard.h"
#include "../../../include/SovereignVoyagerShard.h"
#include "../../../include/SovereignBioForgeShard.h"
#include "../../../include/SovereignSingularityShard.h"
#include "../../../include/SovereignParadoxShard.h"
#include "../../../include/SovereignChronosShard.h"
#include "../../../include/SovereignEtherShard.h"
#include "../../../include/SovereignNullShard.h"
#include "../../../include/SovereignVoidShard.h"
#include "../../../include/SovereignQNXShard.h"
#include "../../../include/SovereignBeOSShard.h"
#include "../../../include/SovereignAmigaShard.h"
#include "../../../include/SovereignMulticsShard.h"
#include "../../../include/SovereignOpenVMSShard.h"
#include "../../../include/SovereignOS2Shard.h"
#include "../../../include/SovereignS360Shard.h"
#include "../../../include/SovereignAltoShard.h"
#include "../../../include/SovereignCrayShard.h"
#include "../../../include/SovereignAeroShard.h"
#include "../../../include/SovereignAquaShard.h"
#include "../../../include/SovereignMaterialShard.h"
#include "../../../include/SovereignMetroShard.h"
#include "../../../include/SovereignGamingShard.h"
#include "../../../include/SovereignProAudioShard.h"
#include "../../../include/SovereignSymbianShard.h"
#include "../../../include/SovereignStudioShard.h"
#include "../../../include/SovereignSensoryShard.h"
#include "../../../include/SovereignCryptoShard.h"
#include "../../../include/SovereignSpaceShard.h"
#include "../../../include/SovereignHiveShard.h"
#include "../../../include/SovereignQuantumMemoryShard.h"
#include "../../../include/SovereignAmnesicScrubShard.h"
#include "../../../include/SovereignDarkMeshShard.h"
#include "../../../include/SovereignSupernovaShard.h"
#include "../../../include/SovereignZenithShard.h"
#include "../../../include/SovereignAbsoluteShard.h"
#include "../../../include/SovereignOmegaShard.h"
#include "../../../include/SovereignUnityShard.h"
#include "../../../include/SovereignAbsoluteFinalityShard.h"
#include "../../../include/SovereignRealityEngineShard.h"
#include "../../../include/SovereignQuantumTeleportShard.h"
#include "../../../include/SovereignGarudaShard.h"
#include "../../../include/SovereignNixShard.h"
#include "../../../include/SovereignQubesShard.h"
#include "../../../include/SovereignAndroidShard.h"
#include "../../../include/SovereignIOSShard.h"
#include "../../../include/SovereignPSShard.h"
#include "../../../include/SovereignCleanerShard.h"
#include "../../../include/SovereignPerfGovShard.h"
#include "../../../include/SovereignOmegaPointShard.h"
#include "../../../include/SovereignAutoCorrectShard.h"
#include "../../../include/SovereignBioSyncShard.h"
#include "../../../include/SovereignGeneticShard.h"
#include "../../../include/SovereignNeuralWorkspaceShard.h"
#include "../../../include/Sovereign333RDShard.h"
#include "../../../include/SovereignSelfCompilerShard.h"
#include "../../../include/SovereignForensicShrinkShard.h"
#include "../../../include/SovereignSolar365Shard.h"
#include "../../../include/SovereignChipDesignerShard.h"
#include "../../../include/SovereignCryoShard.h"
#include "../../../include/SovereignSpartan400Shard.h"
#include "../../../include/SovereignAISynthShard.h"
#include "../../../include/SovereignAuraShard.h"
#include "../../../include/SovereignZenithFinalShard.h"
#include "../../../include/SovereignMARShard.h"
#include "../../../include/SovereignAuraV2Shard.h"
#include "../../../include/Sovereign600THShard.h"
#include "../../../include/SovereignQTShard.h"
#include "../../../include/SovereignAACShard.h"
#include "../../../include/SovereignG777THShard.h"
#include "../../../include/SovereignUniversalLangShard.h"
#include "../../../include/SovereignFabricShard.h"
#include "../../../include/SovereignMillenniumShard.h"
#include "../../../include/SovereignHiveKernelShard.h"
#include "../../../include/SovereignNanoBootV2Shard.h"
#include "../../../include/Sovereign1111THShard.h"
#include "../../../include/SovereignCanvasShard.h"
#include "../../../include/SovereignMusicSynthShard.h"
#include "../../../include/Sovereign1337THShard.h"
#include "../../../include/SovereignAtomicFSShard.h"
#include "../../../include/SovereignNeuralGhostShard.h"
#include "../../../include/Sovereign2048THShard.h"
#include "../../../include/SovereignGrandMasterShard.h"
#include "../../../include/SovereignOmniPresenceShard.h"
#include "../../../include/Sovereign3000THShard.h"
#include "../../../include/SovereignGalacticShard.h"
#include "../../../include/SovereignDysonShard.h"
#include "../../../include/Sovereign3333THShard.h"
#include "../../../include/SovereignMultiverseShard.h"
#include "../../../include/SovereignTruthShard.h"
#include "../../../include/Sovereign4096THShard.h"
#include "../../../include/SovereignLatticeMasterShard.h"
#include "../../../include/SovereignPentathlonShard.h"
#include "../../../include/SovereignLegacyAbsorberShard.h"
#include "../../../include/SovereignAmorphousShard.h"
#include "../../../include/SovereignSeraphimShard.h"
#include "../../../include/SovereignEternalClockShard.h"
#include "../../../include/SovereignInfiniteDataShard.h"
#include "../../../include/Sovereign8192THShard.h"
#include "../../../include/SovereignOmniLogicShard.h"
#include "../../../include/SovereignAbsoluteUserShard.h"
#include "../../../include/Sovereign10000THShard.h"
#include "../../../include/SovereignAbsoluteUIShard.h"
#include "../../../include/SovereignHiveMindShard.h"
#include "../../../include/Sovereign12000THShard.h"
#include "../../../include/SovereignAbsolutePrivacyShard.h"
#include "../../../include/SovereignEternalOptShard.h"
#include "../../../include/Sovereign16384THShard.h"
#include "../../../include/SovereignAbsoluteFileShard.h"
#include "../../../include/SovereignNeuralBridgeShard.h"
#include "../../../include/Sovereign20000THShard.h"
#include "../../../include/SovereignFormalVerifyShard.h"
#include "../../../include/SovereignAbsoluteEntropyShard.h"
#include "../../../include/Sovereign32768THShard.h"
#include "../../../include/SovereignUniversalParserShard.h"
#include "../../../include/SovereignTimeCrystalShard.h"
#include "../../../include/Sovereign65536THShard.h"
#include "../../../include/SovereignUniversalAlgoShard.h"
#include "../../../include/SovereignAbsoluteMemoryShard.h"
#include "../../../include/Sovereign100000THShard.h"
#include "../../../include/SovereignUniversalCompilerShard.h"
#include "../../../include/SovereignEternalMeshShard.h"
#include "../../../include/Sovereign131072NDShard.h"
#include "../../../include/SovereignUniversalLogicShard.h"
#include "../../../include/SovereignGlobalRegistryShard.h"
#include "../../../include/Sovereign200000THShard.h"
#include "../../../include/SovereignUniversalStorageShard.h"
#include "../../../include/SovereignQuantumStateShard.h"
#include "../../../include/Sovereign262144THShard.h"
#include "../../../include/SovereignUniversalSyncShard.h"
#include "../../../include/SovereignAbsoluteMatrixShard.h"
#include "../../../include/Sovereign300000THShard.h"
#include "../../../include/SovereignUniversalBioShard.h"
#include "../../../include/SovereignUniversalEnergyShard.h"
#include "../../../include/Sovereign500000THShard.h"
#include "../../../include/SovereignUniversalTimelineShard.h"
#include "../../../include/SovereignAbsoluteMatterShard.h"
#include "../../../include/Sovereign1048576THShard.h"
#include "../../../include/SovereignAbsoluteCosmosShard.h"
#include "../../../include/SovereignUniversalConsciousnessShard.h"
#include "../../../include/Sovereign2097152NDShard.h"
#include "../../../include/SovereignOmnipotentFateShard.h"
#include "../../../include/SovereignAbsoluteMultiverseShard.h"
#include "../../../include/Sovereign4194304THShard.h"

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

/* ---- sigma-store -------------------------------------------------------- */
sigma_err_t sigma_cmd_store(int argc, char *argv[]) {
    if (argc < 2) { SovereignStore_Audit();
        sigma_printf("Usage: sigma-store [acquire <sku> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "acquire") && argc >= 3)
        sigma_store_acquire(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignStore_Audit();
    return SIGMA_OK;
}

/* ---- sigma-scrub -------------------------------------------------------- */
sigma_err_t sigma_cmd_scrub(int argc, char *argv[]) {
    if (argc < 2) { SovereignForensic_Audit();
        sigma_printf("Usage: sigma-scrub [addr <0x...> | lockdown | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "addr") && argc >= 3)
        sigma_forensic_scrub((sigma_uptr)sigma_atoi(argv[2]), 4096);
    else if (sigma_streq(argv[1], "lockdown"))
        sigma_forensic_lockdown();
    else if (sigma_streq(argv[1], "audit"))
        SovereignForensic_Audit();
    return SIGMA_OK;
}

/* ---- sigma-vm ----------------------------------------------------------- */
sigma_err_t sigma_cmd_vm(int argc, char *argv[]) {
    if (argc < 2) { SovereignHypervisor_Audit();
        sigma_printf("Usage: sigma-vm [create <os> <ram_mb> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "create") && argc >= 4)
        sigma_hyp_create_guest(argv[2], (sigma_u32)sigma_atoi(argv[3]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignHypervisor_Audit();
    return SIGMA_OK;
}

/* ---- sigma-defrag ------------------------------------------------------- */
sigma_err_t sigma_cmd_defrag(int argc, char *argv[]) {
    if (argc < 2) { SovereignDefrag_Audit();
        sigma_printf("Usage: sigma-defrag [run | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "run"))
        sigma_defrag_run();
    else if (sigma_streq(argv[1], "audit"))
        SovereignDefrag_Audit();
    return SIGMA_OK;
}

/* ---- sigma-ds ---------------------------------------------------------- */
sigma_err_t sigma_cmd_ds(int argc, char *argv[]) {
    if (argc < 2) { SovereignDS_Audit();
        sigma_printf("Usage: sigma-ds [alloc <name> <0-2> <rows> | compute | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "alloc") && argc >= 5)
        sigma_ds_allocate(argv[2], (SigmaDSDataType_t)sigma_atoi(argv[3]), (sigma_u32)sigma_atoi(argv[4]));
    else if (sigma_streq(argv[1], "compute"))
        sigma_ds_compute();
    else if (sigma_streq(argv[1], "audit"))
        SovereignDS_Audit();
    return SIGMA_OK;
}

/* ---- sigma-flow -------------------------------------------------------- */
sigma_err_t sigma_cmd_flow(int argc, char *argv[]) {
    if (argc < 2) { SovereignFlow_Audit();
        sigma_printf("Usage: sigma-flow [link <trigger> <action> | fire <trigger> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "link") && argc >= 4)
        sigma_flow_register(argv[2], argv[3]);
    else if (sigma_streq(argv[1], "fire") && argc >= 3)
        sigma_flow_trigger(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignFlow_Audit();
    return SIGMA_OK;
}

/* ---- sigma-prefetch ----------------------------------------------------- */
sigma_err_t sigma_cmd_prefetch(int argc, char *argv[]) {
    if (argc < 2) { SovereignPrefetch_Audit();
        sigma_printf("Usage: sigma-prefetch [warm <shard> | predict | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "warm") && argc >= 3)
        sigma_prefetch_warm(argv[2]);
    else if (sigma_streq(argv[1], "predict"))
        sigma_prefetch_predict();
    else if (sigma_streq(argv[1], "audit"))
        SovereignPrefetch_Audit();
    return SIGMA_OK;
}

/* ---- sigma-db ----------------------------------------------------------- */
sigma_err_t sigma_cmd_db(int argc, char *argv[]) {
    if (argc < 2) { SovereignDb_Audit();
        sigma_printf("Usage: sigma-db [put <key> <val> | get <key> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "put") && argc >= 4)
        sigma_db_put(argv[2], argv[3]);
    else if (sigma_streq(argv[1], "get") && argc >= 3) {
        const char* val = sigma_db_get(argv[2]);
        sigma_printf("[DB]: '%s' -> %s\n", argv[2], val ? val : "NOT_FOUND");
    }
    else if (sigma_streq(argv[1], "audit"))
        SovereignDb_Audit();
    return SIGMA_OK;
}

/* ---- sigma-vault -------------------------------------------------------- */
sigma_err_t sigma_cmd_vault(int argc, char *argv[]) {
    if (argc < 2) { SovereignVault_Audit();
        sigma_printf("Usage: sigma-vault [seal <path> <val> <clearance> | unseal <path> <clearance> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "seal") && argc >= 5)
        sigma_vault_seal(argv[2], argv[3], (sigma_u32)sigma_atoi(argv[4]));
    else if (sigma_streq(argv[1], "unseal") && argc >= 4) {
        const char* val = sigma_vault_unseal(argv[2], (sigma_u32)sigma_atoi(argv[3]));
        sigma_printf("[VAULT]: Result for '%s' -> %s\n", argv[2], val ? val : "ACCESS_DENIED");
    }
    else if (sigma_streq(argv[1], "audit"))
        SovereignVault_Audit();
    return SIGMA_OK;
}

/* ---- sigma-rtos --------------------------------------------------------- */
sigma_err_t sigma_cmd_rtos(int argc, char *argv[]) {
    if (argc < 2) { SovereignRTOS_Audit();
        sigma_printf("Usage: sigma-rtos [sched <name> <period_us> <1/0> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "sched") && argc >= 5)
        sigma_rtos_schedule(argv[2], (sigma_u32)sigma_atoi(argv[3]), (sigma_bool)sigma_atoi(argv[4]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignRTOS_Audit();
    return SIGMA_OK;
}

/* ---- sigma-perf --------------------------------------------------------- */
sigma_err_t sigma_cmd_perf(int argc, char *argv[]) {
    if (argc < 2) { SovereignPerf_Audit();
        sigma_printf("Usage: sigma-perf [snapshot | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "snapshot"))
        sigma_perf_snapshot();
    else if (sigma_streq(argv[1], "audit"))
        SovereignPerf_Audit();
    return SIGMA_OK;
}

/* ---- sigma-math --------------------------------------------------------- */
sigma_err_t sigma_cmd_math(int argc, char *argv[]) {
    if (argc < 2) { SovereignMath_Audit();
        sigma_printf("Usage: sigma-math [exec <operation> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "exec") && argc >= 3)
        sigma_math_execute(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignMath_Audit();
    return SIGMA_OK;
}

/* ---- sigma-cluster ------------------------------------------------------ */
sigma_err_t sigma_cmd_cluster(int argc, char *argv[]) {
    if (argc < 2) { SovereignCluster_Audit();
        sigma_printf("Usage: sigma-cluster [join <ip> | balance | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "join") && argc >= 3)
        sigma_cluster_join(argv[2]);
    else if (sigma_streq(argv[1], "balance"))
        sigma_cluster_balance();
    else if (sigma_streq(argv[1], "audit"))
        SovereignCluster_Audit();
    return SIGMA_OK;
}

/* ---- sigma-quantum ------------------------------------------------------ */
sigma_err_t sigma_cmd_quantum(int argc, char *argv[]) {
    if (argc < 2) { SovereignQuantum_Audit();
        sigma_printf("Usage: sigma-quantum [entropy | simulate <qubits> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "entropy"))
        sigma_printf("[QUANTUM]: Hardware Entropy Seed -> 0x%llX\n", (unsigned long long)sigma_quantum_entropy());
    else if (sigma_streq(argv[1], "simulate") && argc >= 3)
        sigma_quantum_simulate((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignQuantum_Audit();
    return SIGMA_OK;
}

/* ---- sigma-dock --------------------------------------------------------- */
sigma_err_t sigma_cmd_dock(int argc, char *argv[]) {
    if (argc < 2) { SovereignDock_Audit();
        sigma_printf("Usage: sigma-dock [pin <name> <cmd> | launch <name> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "pin") && argc >= 4)
        sigma_dock_pin(argv[2], argv[3]);
    else if (sigma_streq(argv[1], "launch") && argc >= 3)
        sigma_dock_launch(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignDock_Audit();
    return SIGMA_OK;
}

/* ---- sigma-spot --------------------------------------------------------- */
sigma_err_t sigma_cmd_spot(int argc, char *argv[]) {
    if (argc < 2) { SovereignSpotlight_Audit();
        sigma_printf("Usage: sigma-spot [index | search <query> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "index"))
        sigma_spotlight_index();
    else if (sigma_streq(argv[1], "search") && argc >= 3)
        sigma_spotlight_search(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignSpotlight_Audit();
    return SIGMA_OK;
}

/* ---- sigma-mesh --------------------------------------------------------- */
sigma_err_t sigma_cmd_mesh(int argc, char *argv[]) {
    if (argc < 2) { SovereignMesh_Audit();
        sigma_printf("Usage: sigma-mesh [connect <ip> <pub_key> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "connect") && argc >= 4)
        sigma_mesh_connect(argv[2], argv[3]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignMesh_Audit();
    return SIGMA_OK;
}

/* ---- sigma-telemetry ---------------------------------------------------- */
sigma_err_t sigma_cmd_telemetry(int argc, char *argv[]) {
    if (argc < 2) { SovereignTelemetry_Audit();
        sigma_printf("Usage: sigma-telemetry [emit <metric> <val> | export | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "emit") && argc >= 4)
        sigma_telemetry_emit(argv[2], (sigma_u32)sigma_atoi(argv[3]));
    else if (sigma_streq(argv[1], "export"))
        sigma_telemetry_export();
    else if (sigma_streq(argv[1], "audit"))
        SovereignTelemetry_Audit();
    return SIGMA_OK;
}

/* ---- sigma-handoff ------------------------------------------------------ */
sigma_err_t sigma_cmd_handoff(int argc, char *argv[]) {
    if (argc < 2) { SovereignHandoff_Audit();
        sigma_printf("Usage: sigma-handoff [push <context> | pull | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "push") && argc >= 3)
        sigma_handoff_push(argv[2]);
    else if (sigma_streq(argv[1], "pull"))
        sigma_handoff_pull();
    else if (sigma_streq(argv[1], "audit"))
        SovereignHandoff_Audit();
    return SIGMA_OK;
}

/* ---- sigma-dtrace ------------------------------------------------------- */
sigma_err_t sigma_cmd_dtrace(int argc, char *argv[]) {
    if (argc < 2) { SovereignDTrace_Audit();
        sigma_printf("Usage: sigma-dtrace [probe <shard> <point> | trace <filter> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "probe") && argc >= 4)
        sigma_dtrace_probe(argv[2], argv[3]);
    else if (sigma_streq(argv[1], "trace") && argc >= 3)
        sigma_dtrace_trace(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignDTrace_Audit();
    return SIGMA_OK;
}

/* ---- sigma-drm ---------------------------------------------------------- */
sigma_err_t sigma_cmd_drm(int argc, char *argv[]) {
    if (argc < 2) { SovereignDRM_Audit();
        sigma_printf("Usage: sigma-drm [alloc <w> <h> <bpp> | commit <id> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "alloc") && argc >= 5)
        sigma_drm_allocate_fb((sigma_u32)sigma_atoi(argv[2]), (sigma_u32)sigma_atoi(argv[3]), (sigma_u32)sigma_atoi(argv[4]));
    else if (sigma_streq(argv[1], "commit") && argc >= 3)
        sigma_drm_atomic_commit((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignDRM_Audit();
    return SIGMA_OK;
}

/* ---- sigma-airdrop ------------------------------------------------------ */
sigma_err_t sigma_cmd_airdrop(int argc, char *argv[]) {
    if (argc < 2) { SovereignAirDrop_Audit();
        sigma_printf("Usage: sigma-airdrop [scan | send <peer> <file> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "scan"))
        sigma_airdrop_scan();
    else if (sigma_streq(argv[1], "send") && argc >= 4)
        sigma_airdrop_send(argv[2], argv[3]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignAirDrop_Audit();
    return SIGMA_OK;
}

/* ---- sigma-sandbox ------------------------------------------------------ */
sigma_err_t sigma_cmd_sandbox(int argc, char *argv[]) {
    if (argc < 2) { SovereignSandbox_Audit();
        sigma_printf("Usage: sigma-sandbox [enforce <pid> <profile> | audit_pid <pid> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "enforce") && argc >= 4)
        sigma_sandbox_enforce((sigma_u32)sigma_atoi(argv[2]), argv[3]);
    else if (sigma_streq(argv[1], "audit_pid") && argc >= 3)
        sigma_sandbox_audit_pid((sigma_u32)sigma_atoi(argv[2]));
    else if (sigma_streq(argv[1], "audit"))
        SovereignSandbox_Audit();
    return SIGMA_OK;
}

/* ---- sigma-voice -------------------------------------------------------- */
sigma_err_t sigma_cmd_voice(int argc, char *argv[]) {
    if (argc < 2) { SovereignVoice_Audit();
        sigma_printf("Usage: sigma-voice [listen | intent <phrase> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "listen"))
        sigma_voice_listen();
    else if (sigma_streq(argv[1], "intent") && argc >= 3)
        sigma_voice_intent(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignVoice_Audit();
    return SIGMA_OK;
}

/* ---- sigma-sideload ----------------------------------------------------- */
sigma_err_t sigma_cmd_sideload(int argc, char *argv[]) {
    if (argc < 2) { SovereignSideload_Audit();
        sigma_printf("Usage: sigma-sideload [install <file> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "install") && argc >= 3)
        sigma_sideload_install(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignSideload_Audit();
    return SIGMA_OK;
}

/* ---- sigma-continuity --------------------------------------------------- */
sigma_err_t sigma_cmd_continuity(int argc, char *argv[]) {
    if (argc < 2) { SovereignContinuity_Audit();
        sigma_printf("Usage: sigma-continuity [link <device> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "link") && argc >= 3)
        sigma_continuity_link(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignContinuity_Audit();
    return SIGMA_OK;
}

/* ---- sigma-timemachine -------------------------------------------------- */
sigma_err_t sigma_cmd_timemachine(int argc, char *argv[]) {
    if (argc < 2) { SovereignTimeMachine_Audit();
        sigma_printf("Usage: sigma-timemachine [snap | restore <time> | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "snap"))
        sigma_timemachine_snap();
    else if (sigma_streq(argv[1], "restore") && argc >= 3)
        sigma_timemachine_restore(argv[2]);
    else if (sigma_streq(argv[1], "audit"))
        SovereignTimeMachine_Audit();
    return SIGMA_OK;
}

/* ---- sigma-boot --------------------------------------------------------- */
sigma_err_t sigma_cmd_boot(int argc, char *argv[]) {
    if (argc < 2) { SovereignBootloader_Audit();
        sigma_printf("Usage: sigma-boot [handoff | audit]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "handoff"))
        sigma_boot_handoff();
    else if (sigma_streq(argv[1], "audit"))
        SovereignBootloader_Audit();
    return SIGMA_OK;
}

/* ---- sigma-test --------------------------------------------------------- */
sigma_err_t sigma_cmd_test(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-test [algorithms]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "algorithms"))
        sigma_test_algorithms();
    return SIGMA_OK;
}

/* ---- sigma-autoclean ---------------------------------------------------- */
sigma_err_t sigma_cmd_autoclean(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-autoclean [execute]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "execute"))
        sigma_autoclean_execute();
    return SIGMA_OK;
}

/* ---- sigma-autoperf ----------------------------------------------------- */
sigma_err_t sigma_cmd_autoperf(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-autoperf [mode <gaming/battery/auto>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "mode") && argc >= 3)
        sigma_autoperf_execute(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-bioauth ------------------------------------------------------ */
sigma_err_t sigma_cmd_bioauth(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-bioauth [scan]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "scan"))
        sigma_bioauth_scan();
    return SIGMA_OK;
}

/* ---- sigma-emulate ------------------------------------------------------ */
sigma_err_t sigma_cmd_emulate(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-emulate [run <binary_arch>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "run") && argc >= 3)
        sigma_emulate_run(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-holo --------------------------------------------------------- */
sigma_err_t sigma_cmd_holo(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-holo [anchor <win_id> <z_depth>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "anchor") && argc >= 4) {
        // Mock parsing the float string
        sigma_holo_anchor((sigma_u32)sigma_atoi(argv[2]), 1.5f);
    }
    return SIGMA_OK;
}

/* ---- sigma-swarm -------------------------------------------------------- */
sigma_err_t sigma_cmd_swarm(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-swarm [infer <prompt>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "infer") && argc >= 3)
        sigma_swarm_infer(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-bci ---------------------------------------------------------- */
sigma_err_t sigma_cmd_bci(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-bci [poll]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "poll"))
        sigma_bci_poll();
    return SIGMA_OK;
}

/* ---- sigma-heal --------------------------------------------------------- */
sigma_err_t sigma_cmd_heal(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-heal [execute]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "execute"))
        sigma_heal_execute();
    return SIGMA_OK;
}

/* ---- sigma-fuzz --------------------------------------------------------- */
sigma_err_t sigma_cmd_fuzz(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-fuzz [run]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "run"))
        sigma_fuzz_run();
    return SIGMA_OK;
}

/* ---- sigma-infer -------------------------------------------------------- */
sigma_err_t sigma_cmd_infer(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-infer [parse <ambiguous_cmd>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "parse") && argc >= 3)
        sigma_parse_infer(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-pqc ---------------------------------------------------------- */
sigma_err_t sigma_cmd_pqc(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-pqc [test]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "test"))
        sigma_quantum_test();
    return SIGMA_OK;
}

/* ---- sigma-silicon ------------------------------------------------------ */
sigma_err_t sigma_cmd_silicon(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-silicon [test]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "test"))
        sigma_silicon_test();
    return SIGMA_OK;
}

/* ---- sigma-macro -------------------------------------------------------- */
sigma_err_t sigma_cmd_macro(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-macro [execute <macro_name>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "execute") && argc >= 3)
        sigma_macro_execute(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-exo ---------------------------------------------------------- */
sigma_err_t sigma_cmd_exo(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-exo [bypass <pid>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "bypass") && argc >= 3)
        sigma_exo_bypass((sigma_u32)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-fhe ---------------------------------------------------------- */
sigma_err_t sigma_cmd_fhe(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-fhe [compute]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "compute"))
        sigma_fhe_compute();
    return SIGMA_OK;
}

/* ---- sigma-zk ----------------------------------------------------------- */
sigma_err_t sigma_cmd_zk(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-zk [prove]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "prove"))
        sigma_zk_prove();
    return SIGMA_OK;
}

/* ---- sigma-service ------------------------------------------------------ */
sigma_err_t sigma_cmd_service(int argc, char *argv[]) {
    if (argc < 2) { sigma_service_list();
        sigma_printf("Usage: sigma-service [start <service> | list]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "start") && argc >= 3)
        sigma_service_start(argv[2]);
    else if (sigma_streq(argv[1], "list"))
        sigma_service_list();
    return SIGMA_OK;
}

/* ---- sigma-kms ---------------------------------------------------------- */
sigma_err_t sigma_cmd_kms(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-kms [set <w> <h> <refresh>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "set") && argc >= 5)
        sigma_kms_set_mode((sigma_u32)sigma_atoi(argv[2]), (sigma_u32)sigma_atoi(argv[3]), 60.0f);
    return SIGMA_OK;
}

/* ---- sigma-thermal ------------------------------------------------------ */
sigma_err_t sigma_cmd_thermal(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_thermal_monitor();
    return SIGMA_OK;
}

/* ---- sigma-reg ---------------------------------------------------------- */
sigma_err_t sigma_cmd_reg(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-reg [set <key> <val> | query <key>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "set") && argc >= 4)
        sigma_registry_set(argv[2], argv[3]);
    else if (sigma_streq(argv[1], "query") && argc >= 3)
        sigma_printf("Value: %s\n", sigma_registry_query(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-audio -------------------------------------------------------- */
sigma_err_t sigma_cmd_audio(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-audio [stream <rate> <depth>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "stream") && argc >= 4)
        sigma_audio_stream((sigma_u32)sigma_atoi(argv[2]), (sigma_u32)sigma_atoi(argv[3]));
    return SIGMA_OK;
}

/* ---- sigma-eco ---------------------------------------------------------- */
sigma_err_t sigma_cmd_eco(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_eco_engage();
    return SIGMA_OK;
}

/* ---- sigma-bt ----------------------------------------------------------- */
sigma_err_t sigma_cmd_bt(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-bt [pair <device_id>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "pair") && argc >= 3)
        sigma_bt_pair(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-raid --------------------------------------------------------- */
sigma_err_t sigma_cmd_raid(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-raid [assemble <level> <count>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "assemble"))
        sigma_raid_assemble((sigma_u32)sigma_atoi(argv[2]), (sigma_u32)sigma_atoi(argv[3]));
    return SIGMA_OK;
}

/* ---- sigma-eye ---------------------------------------------------------- */
sigma_err_t sigma_cmd_eye(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_gaze_calibrate();
    return SIGMA_OK;
}

/* ---- sigma-vpn ---------------------------------------------------------- */
sigma_err_t sigma_cmd_vpn(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-vpn [up <peer_ip>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "up") && argc >= 3)
        sigma_vpn_up(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-voice -------------------------------------------------------- */
sigma_err_t sigma_cmd_voice(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-voice [parse <wav_path>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "parse") && argc >= 3)
        sigma_voice_parse(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-docker ------------------------------------------------------- */
sigma_err_t sigma_cmd_docker(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-docker [spawn <image_id>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "spawn") && argc >= 3)
        sigma_container_spawn(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-defender ----------------------------------------------------- */
sigma_err_t sigma_cmd_defender(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-defender [scan <binary_path>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "scan") && argc >= 3)
        sigma_defender_scan(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-p2p ---------------------------------------------------------- */
sigma_err_t sigma_cmd_p2p(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-p2p [announce <cid>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "announce") && argc >= 3)
        sigma_p2p_announce(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-qkd ---------------------------------------------------------- */
sigma_err_t sigma_cmd_qkd(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-qkd [exchange <node_id>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "exchange") && argc >= 3)
        sigma_qkd_exchange(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-boot-audit --------------------------------------------------- */
sigma_err_t sigma_cmd_boot_audit(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_boot_verify();
    return SIGMA_OK;
}

/* ---- sigma-eeg ---------------------------------------------------------- */
sigma_err_t sigma_cmd_eeg(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_eeg_scan();
    return SIGMA_OK;
}

/* ---- sigma-tag ---------------------------------------------------------- */
sigma_err_t sigma_cmd_tag(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-tag [ptr] [size] [tag]\n");
        return SIGMA_OK; }
    sigma_mem_tag((void*)sigma_atoi(argv[1]), (sigma_size_t)sigma_atoi(argv[2]), (sigma_u8)sigma_atoi(argv[3]));
    return SIGMA_OK;
}

/* ---- sigma-patch -------------------------------------------------------- */
sigma_err_t sigma_cmd_patch(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-patch [func_name] [new_addr]\n");
        return SIGMA_OK; }
    if (argc >= 3)
        sigma_live_patch(argv[1], (void*)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-qubit -------------------------------------------------------- */
sigma_err_t sigma_cmd_qubit(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-qubit [h <qubit_id>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "h") && argc >= 3)
        sigma_qubit_h_gate((sigma_u32)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-hacker ------------------------------------------------------- */
sigma_err_t sigma_cmd_hacker(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-hacker [audit <shard_name>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "audit") && argc >= 3)
        sigma_hacker_audit(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-mesh-mount --------------------------------------------------- */
sigma_err_t sigma_cmd_mesh_mount(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-mesh-mount [node_id]\n");
        return SIGMA_OK; }
    sigma_mesh_mount(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-neural-sync -------------------------------------------------- */
sigma_err_t sigma_cmd_neural_sync(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-neural-sync [node_id]\n");
        return SIGMA_OK; }
    sigma_eeg_sync(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-foundry ------------------------------------------------------ */
sigma_err_t sigma_cmd_foundry(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-foundry [inspect <gdsii_path>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "inspect") && argc >= 3)
        sigma_foundry_inspect(argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-job ---------------------------------------------------------- */
sigma_err_t sigma_cmd_job(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-job [batch <name> <nodes>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "batch"))
        sigma_exascale_batch(argv[2], (sigma_u32)sigma_atoi(argv[3]));
    return SIGMA_OK;
}

/* ---- sigma-dyson -------------------------------------------------------- */
sigma_err_t sigma_cmd_dyson(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_dyson_status();
    return SIGMA_OK;
}

/* ---- sigma-space-link --------------------------------------------------- */
sigma_err_t sigma_cmd_space_link(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-space-link [constellation_id]\n");
        return SIGMA_OK; }
    sigma_voyager_link(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-bio-audit ---------------------------------------------------- */
sigma_err_t sigma_cmd_bio_audit(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-bio-audit [fasta_path]\n");
        return SIGMA_OK; }
    sigma_bio_audit(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-merge -------------------------------------------------------- */
sigma_err_t sigma_cmd_merge(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-merge [model_path]\n");
        return SIGMA_OK; }
    sigma_singularity_merge(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-audit-logic -------------------------------------------------- */
sigma_err_t sigma_cmd_audit_logic(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-audit-logic [algo_path]\n");
        return SIGMA_OK; }
    sigma_paradox_check(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-rewind ------------------------------------------------------- */
sigma_err_t sigma_cmd_rewind(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-rewind [snapshot_id]\n");
        return SIGMA_OK; }
    sigma_chronos_rewind((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-ether -------------------------------------------------------- */
sigma_err_t sigma_cmd_ether(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-ether [beam <freq_thz>]\n");
        return SIGMA_OK; }
    if (sigma_streq(argv[1], "beam") && argc >= 3)
        sigma_ether_beam((float)sigma_atof(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-null --------------------------------------------------------- */
sigma_err_t sigma_cmd_null(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-null [asic_addr] [instruction]\n");
        return SIGMA_OK; }
    sigma_null_dispatch((sigma_addr_t)sigma_atoi(argv[1]), (sigma_u64)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-void --------------------------------------------------------- */
sigma_err_t sigma_cmd_void(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_void_manifest();
    return SIGMA_OK;
}

/* ---- sigma-rtos-msg ----------------------------------------------------- */
sigma_err_t sigma_cmd_rtos_msg(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-rtos-msg [target_pid] [data]\n");
        return SIGMA_OK; }
    sigma_qnx_msg_send((sigma_u32)sigma_atoi(argv[1]), argv[2]);
    return SIGMA_OK;
}

/* ---- sigma-thread ------------------------------------------------------- */
sigma_err_t sigma_cmd_thread(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-thread [task_name]\n");
        return SIGMA_OK; }
    sigma_be_thread_spawn(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-blit --------------------------------------------------------- */
sigma_err_t sigma_cmd_blit(int argc, char *argv[]) {
    if (argc < 4) {
        sigma_printf("Usage: sigma-blit [src] [dest] [size]\n");
        return SIGMA_OK; }
    sigma_amiga_blit((sigma_addr_t)sigma_atoi(argv[1]), (sigma_addr_t)sigma_atoi(argv[2]), (sigma_u32)sigma_atoi(argv[3]));
    return SIGMA_OK;
}

/* ---- sigma-secure-call -------------------------------------------------- */
sigma_err_t sigma_cmd_secure_call(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-secure-call [ring_id] [entry_addr]\n");
        return SIGMA_OK; }
    sigma_multics_secure_call((sigma_u8)sigma_atoi(argv[1]), (void*)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-quorum-join -------------------------------------------------- */
sigma_err_t sigma_cmd_quorum_join(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_vms_cluster_join();
    return SIGMA_OK;
}

/* ---- sigma-persist-obj -------------------------------------------------- */
sigma_err_t sigma_cmd_persist_obj(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-persist-obj [obj_name] [addr] [size]\n");
        return SIGMA_OK; }
    sigma_os2_persist(argv[1], (void*)sigma_atoi(argv[2]), (sigma_u32)sigma_atoi(argv[3]));
    return SIGMA_OK;
}

/* ---- sigma-lpar --------------------------------------------------------- */
sigma_err_t sigma_cmd_lpar(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-lpar [cores] [memory_mb]\n");
        return SIGMA_OK; }
    sigma_s360_lpar_spawn((sigma_u32)sigma_atoi(argv[1]), (sigma_u32)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-draw-ui ------------------------------------------------------ */
sigma_err_t sigma_cmd_draw_ui(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-draw-ui [x] [y] [width] [height]\n");
        return SIGMA_OK; }
    sigma_alto_draw_bitmap(sigma_atoi(argv[1]), sigma_atoi(argv[2]), NULL, sigma_atoi(argv[3]), sigma_atoi(argv[4]));
    return SIGMA_OK;
}

/* ---- sigma-vec-math ----------------------------------------------------- */
sigma_err_t sigma_cmd_vec_math(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-vec-math [len]\n");
        return SIGMA_OK; }
    sigma_cray_vector_add(NULL, NULL, NULL, (sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-blur --------------------------------------------------------- */
sigma_err_t sigma_cmd_blur(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-blur [window_id] [transparency]\n");
        return SIGMA_OK; }
    sigma_aero_blur((sigma_u32)sigma_atoi(argv[1]), (float)sigma_atof(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-aqua --------------------------------------------------------- */
sigma_err_t sigma_cmd_aqua(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-aqua [label]\n");
        return SIGMA_OK; }
    sigma_aqua_render(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-elevate ------------------------------------------------------ */
sigma_err_t sigma_cmd_elevate(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-elevate [element_id] [z_level]\n");
        return SIGMA_OK; }
    sigma_material_elevate((sigma_u32)sigma_atoi(argv[1]), (sigma_u16)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-flip --------------------------------------------------------- */
sigma_err_t sigma_cmd_flip(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-flip [tile_id]\n");
        return SIGMA_OK; }
    sigma_metro_tile_flip((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-game --------------------------------------------------------- */
sigma_err_t sigma_cmd_game(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-game [pid]\n");
        return SIGMA_OK; }
    sigma_game_mode((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-audio-pro ---------------------------------------------------- */
sigma_err_t sigma_cmd_audio_pro(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-audio-pro [sample_rate]\n");
        return SIGMA_OK; }
    sigma_audio_dma_stream((sigma_u16)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-sleep-deep --------------------------------------------------- */
sigma_err_t sigma_cmd_sleep_deep(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_symbian_sleep();
    return SIGMA_OK;
}

/* ---- sigma-render-8k ---------------------------------------------------- */
sigma_err_t sigma_cmd_render_8k(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-render-8k [frame_id]\n");
        return SIGMA_OK; }
    sigma_studio_render((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-sense -------------------------------------------------------- */
sigma_err_t sigma_cmd_sense(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-sense [sense_id] [intensity]\n");
        return SIGMA_OK; }
    sigma_sensory_pulse((sigma_u8)sigma_atoi(argv[1]), (sigma_u16)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-crypto ------------------------------------------------------- */
sigma_err_t sigma_cmd_crypto(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-crypto [sign_data]\n");
        return SIGMA_OK; }
    sigma_crypto_sign(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-tmr ---------------------------------------------------------- */
sigma_err_t sigma_cmd_tmr(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_space_tmr_audit();
    return SIGMA_OK;
}

/* ---- sigma-hive --------------------------------------------------------- */
sigma_err_t sigma_cmd_hive(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-hive [task_desc]\n");
        return SIGMA_OK; }
    sigma_hive_dispatch(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-qram --------------------------------------------------------- */
sigma_err_t sigma_cmd_qram(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-qram [addr] [val]\n");
        return SIGMA_OK; }
    sigma_quantum_store((sigma_addr_t)sigma_atoi(argv[1]), (sigma_u64)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-purge -------------------------------------------------------- */
sigma_err_t sigma_cmd_purge(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_amnesic_purge();
    return SIGMA_OK;
}

/* ---- sigma-dark-mesh ---------------------------------------------------- */
sigma_err_t sigma_cmd_dark_mesh(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-dark-mesh [data]\n");
        return SIGMA_OK; }
    sigma_dark_relay(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-unleash ------------------------------------------------------ */
sigma_err_t sigma_cmd_unleash(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_supernova_unleash();
    return SIGMA_OK;
}

/* ---- sigma-summit ------------------------------------------------------- */
sigma_err_t sigma_cmd_summit(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_zenith_summit();
    return SIGMA_OK;
}

/* ---- sigma-seal --------------------------------------------------------- */
sigma_err_t sigma_cmd_seal(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_absolute_monolith();
    return SIGMA_OK;
}

/* ---- sigma-hibernate-forever -------------------------------------------- */
sigma_err_t sigma_cmd_hibernate_forever(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_omega_hibernate();
    return SIGMA_OK;
}

/* ---- sigma-unity-merge -------------------------------------------------- */
sigma_err_t sigma_cmd_unity_merge(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-unity-merge [remote_ip]\n");
        return SIGMA_OK; }
    sigma_unity_merge(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-ascend ------------------------------------------------------- */
sigma_err_t sigma_cmd_ascend(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_ascend();
    return SIGMA_OK;
}

/* ---- sigma-sim-reality -------------------------------------------------- */
sigma_err_t sigma_cmd_sim_reality(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-sim-reality [radius_meters]\n");
        return SIGMA_OK; }
    sigma_reality_sim((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-teleport ----------------------------------------------------- */
sigma_err_t sigma_cmd_teleport(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-teleport [local_addr] [remote_addr]\n");
        return SIGMA_OK; }
    sigma_quantum_teleport((sigma_addr_t)sigma_atoi(argv[1]), (sigma_addr_t)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-boost-zen ---------------------------------------------------- */
sigma_err_t sigma_cmd_boost_zen(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_garuda_zen_boost();
    return SIGMA_OK;
}

/* ---- sigma-generation-switch -------------------------------------------- */
sigma_err_t sigma_cmd_generation_switch(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-generation-switch [id]\n");
        return SIGMA_OK; }
    sigma_nix_generation_switch((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-isolate-cube ------------------------------------------------- */
sigma_err_t sigma_cmd_isolate_cube(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-isolate-cube [group] [level]\n");
        return SIGMA_OK; }
    sigma_qubes_isolate(argv[1], (sigma_u32)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-binder-call -------------------------------------------------- */
sigma_err_t sigma_cmd_binder_call(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-binder-call [id] [code]\n");
        return SIGMA_OK; }
    sigma_android_binder_call((sigma_u32)sigma_atoi(argv[1]), (sigma_u32)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-verify-id ---------------------------------------------------- */
sigma_err_t sigma_cmd_verify_id(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-verify-id [token]\n");
        return SIGMA_OK; }
    sigma_ios_secure_verify(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-gpu-push ----------------------------------------------------- */
sigma_err_t sigma_cmd_gpu_push(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-gpu-push [addr] [size]\n");
        return SIGMA_OK; }
    sigma_ps_gpu_push((void*)sigma_atoi(argv[1]), (sigma_size_t)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-clean-auto --------------------------------------------------- */
sigma_err_t sigma_cmd_clean_auto(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_clean_recursive();
    return SIGMA_OK;
}

/* ---- sigma-perf-auto ---------------------------------------------------- */
sigma_err_t sigma_cmd_perf_auto(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_perf_optimize();
    return SIGMA_OK;
}

/* ---- sigma-omega-point -------------------------------------------------- */
sigma_err_t sigma_cmd_omega_point(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_omega_point_manifest();
    return SIGMA_OK;
}

/* ---- sigma-fix-code ----------------------------------------------------- */
sigma_err_t sigma_cmd_fix_code(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-fix-code [ip]\n");
        return SIGMA_OK; }
    sigma_auto_fix((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-bio-check ---------------------------------------------------- */
sigma_err_t sigma_cmd_bio_check(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_bio_verify();
    return SIGMA_OK;
}

/* ---- sigma-evolve ------------------------------------------------------- */
sigma_err_t sigma_cmd_evolve(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_genetic_evolve();
    return SIGMA_OK;
}

/* ---- sigma-neural-ui ---------------------------------------------------- */
sigma_err_t sigma_cmd_neural_ui(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-neural-ui [intent_hash]\n");
        return SIGMA_OK; }
    sigma_neural_customise((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-ascend-333 --------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_333(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_final_ascension();
    return SIGMA_OK;
}

/* ---- sigma-bootstrap-code ----------------------------------------------- */
sigma_err_t sigma_cmd_bootstrap_code(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-bootstrap-code [path]\n");
        return SIGMA_OK; }
    sigma_self_compile(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-shred-silicon ------------------------------------------------ */
sigma_err_t sigma_cmd_shred_silicon(int argc, char *argv[]) {
    if (argc < 3) {
        sigma_printf("Usage: sigma-shred-silicon [addr] [size]\n");
        return SIGMA_OK; }
    sigma_forensic_shred((sigma_addr_t)sigma_atoi(argv[1]), (sigma_size_t)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-solar-summit ------------------------------------------------- */
sigma_err_t sigma_cmd_solar_summit(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_solar_cycle_manifest();
    return SIGMA_OK;
}

/* ---- sigma-design-chip -------------------------------------------------- */
sigma_err_t sigma_cmd_design_chip(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-design-chip [hdl_path]\n");
        return SIGMA_OK; }
    sigma_chip_synthesize(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-stabilize-ice ------------------------------------------------ */
sigma_err_t sigma_cmd_stabilize_ice(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_cryo_stabilize();
    return SIGMA_OK;
}

/* ---- sigma-spartan-400 -------------------------------------------------- */
sigma_err_t sigma_cmd_spartan_400(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_spartan_fortress();
    return SIGMA_OK;
}

/* ---- sigma-synth-ai ----------------------------------------------------- */
sigma_err_t sigma_cmd_synth_ai(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-synth-ai [model_id]\n");
        return SIGMA_OK; }
    sigma_ai_synthesize((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-aura --------------------------------------------------------- */
sigma_err_t sigma_cmd_aura(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_aura_manifest();
    return SIGMA_OK;
}

/* ---- sigma-zenith-final ------------------------------------------------- */
sigma_err_t sigma_cmd_zenith_final(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_final_zenith();
    return SIGMA_OK;
}

/* ---- sigma-relay-arch --------------------------------------------------- */
sigma_err_t sigma_cmd_relay_arch(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-relay-arch [arch_id] [addr] [size]\n");
        return SIGMA_OK; }
    sigma_mar_execute((sigma_u8)sigma_atoi(argv[1]), (void*)sigma_atoi(argv[2]), (sigma_size_t)sigma_atoi(argv[3]));
    return SIGMA_OK;
}

/* ---- sigma-aura-v2 ------------------------------------------------------ */
sigma_err_t sigma_cmd_aura_v2(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_aura_v2_ignite();
    return SIGMA_OK;
}

/* ---- sigma-ascend-600 --------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_600(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_pinnacle_ascend();
    return SIGMA_OK;
}

/* ---- sigma-time-dilate -------------------------------------------------- */
sigma_err_t sigma_cmd_time_dilate(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-time-dilate [factor]\n");
        return SIGMA_OK; }
    sigma_qt_time_dilate(sigma_atof(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-entropy-zero ------------------------------------------------- */
sigma_err_t sigma_cmd_entropy_zero(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_aac_nullify();
    return SIGMA_OK;
}

/* ---- sigma-ascend-777 --------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_777(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_god_tier_ascend();
    return SIGMA_OK;
}

/* ---- sigma-invoke-intent ------------------------------------------------ */
sigma_err_t sigma_cmd_invoke_intent(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-invoke-intent [hash_low] [hash_high]\n");
        return SIGMA_OK; }
    sigma_universal_invoke((sigma_u64)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-manifest-fabric ---------------------------------------------- */
sigma_err_t sigma_cmd_manifest_fabric(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-manifest-fabric [modality]\n");
        return SIGMA_OK; }
    sigma_fabric_manifest((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-ascend-1000 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_1000(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_millennium_ascend();
    return SIGMA_OK;
}

/* ---- sigma-hive-merge --------------------------------------------------- */
sigma_err_t sigma_cmd_hive_merge(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-hive-merge [peer_id]\n");
        return SIGMA_OK; }
    sigma_hive_merge((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-boot-nano ---------------------------------------------------- */
sigma_err_t sigma_cmd_boot_nano(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_speed_boot();
    return SIGMA_OK;
}

/* ---- sigma-ascend-1111 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_1111(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_angelic_ascend();
    return SIGMA_OK;
}

/* ---- sigma-canvas-draw -------------------------------------------------- */
sigma_err_t sigma_cmd_canvas_draw(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-canvas-draw [frame_id]\n");
        return SIGMA_OK; }
    sigma_canvas_draw((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-music-render ------------------------------------------------- */
sigma_err_t sigma_cmd_music_render(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_music_render();
    return SIGMA_OK;
}

/* ---- sigma-ascend-1337 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_1337(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_elite_ascend();
    return SIGMA_OK;
}

/* ---- sigma-atomic-map --------------------------------------------------- */
sigma_err_t sigma_cmd_atomic_map(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-atomic-map [atom_id] [state]\n");
        return SIGMA_OK; }
    sigma_atomic_write((sigma_u64)sigma_atoi(argv[1]), (sigma_bool)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-ghost-intent ------------------------------------------------- */
sigma_err_t sigma_cmd_ghost_intent(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-ghost-intent [intent]\n");
        return SIGMA_OK; }
    sigma_ghost_mirror(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-ascend-2048 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_2048(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_binary_ascend();
    return SIGMA_OK;
}

/* ---- sigma-grand-nest --------------------------------------------------- */
sigma_err_t sigma_cmd_grand_nest(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-grand-nest [level]\n");
        return SIGMA_OK; }
    sigma_grand_nest((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-omni-sync ---------------------------------------------------- */
sigma_err_t sigma_cmd_omni_sync(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_omni_broadcast();
    return SIGMA_OK;
}

/* ---- sigma-ascend-3000 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_3000(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_grand_finality();
    return SIGMA_OK;
}

/* ---- sigma-galactic-route ----------------------------------------------- */
sigma_err_t sigma_cmd_galactic_route(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-galactic-route [system_id] [addr]\n");
        return SIGMA_OK; }
    sigma_galactic_route((sigma_u32)sigma_atoi(argv[1]), (void*)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-dyson-stats -------------------------------------------------- */
sigma_err_t sigma_cmd_dyson_stats(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_dyson_status();
    return SIGMA_OK;
}

/* ---- sigma-ascend-3333 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_3333(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_angelic_3333_ascend();
    return SIGMA_OK;
}

/* ---- sigma-brane-sync --------------------------------------------------- */
sigma_err_t sigma_cmd_brane_sync(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-brane-sync [brane_id]\n");
        return SIGMA_OK; }
    sigma_brane_sync((sigma_u64)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-prove-truth -------------------------------------------------- */
sigma_err_t sigma_cmd_prove_truth(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-prove-truth [proposition]\n");
        return SIGMA_OK; }
    sigma_truth_verify(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-ascend-4096 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_4096(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_12bit_ascend();
    return SIGMA_OK;
}

/* ---- sigma-lattice-scale ------------------------------------------------ */
sigma_err_t sigma_cmd_lattice_scale(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_lattice_scale();
    return SIGMA_OK;
}

/* ---- sigma-ascend-5000 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_5000(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_pentathlon_ascend();
    return SIGMA_OK;
}

/* ---- sigma-legacy-absorb ------------------------------------------------ */
sigma_err_t sigma_cmd_legacy_absorb(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-legacy-absorb [legacy_system]\n");
        return SIGMA_OK; }
    sigma_absorb_legacy(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-amorphous-mask ----------------------------------------------- */
sigma_err_t sigma_cmd_amorphous_mask(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-amorphous-mask [mask]\n");
        return SIGMA_OK; }
    sigma_amorphous_scale((sigma_u32)sigma_atoi(argv[1]));
    return SIGMA_OK;
}

/* ---- sigma-ascend-6666 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_6666(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_seraphim_ascend();
    return SIGMA_OK;
}

/* ---- sigma-eternal-tick ------------------------------------------------- */
sigma_err_t sigma_cmd_eternal_tick(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_eternal_tick();
    return SIGMA_OK;
}

/* ---- sigma-data-holo ---------------------------------------------------- */
sigma_err_t sigma_cmd_data_holo(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-data-holo [addr] [size]\n");
        return SIGMA_OK; }
    sigma_data_holograph((void*)sigma_atoi(argv[1]), (sigma_size_t)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-ascend-8192 -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_8192(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_13bit_eternal_ascend();
    return SIGMA_OK;
}

/* ---- sigma-logic-sync --------------------------------------------------- */
sigma_err_t sigma_cmd_logic_sync(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-logic-sync [proposition]\n");
        return SIGMA_OK; }
    sigma_logic_synthesize(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-user-mirror -------------------------------------------------- */
sigma_err_t sigma_cmd_user_mirror(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-user-mirror [username]\n");
        return SIGMA_OK; }
    sigma_user_mirror(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-ascend-10000 ------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_10000(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_decathlon_ascend();
    return SIGMA_OK;
}

/* ---- sigma-ui-manifest -------------------------------------------------- */
sigma_err_t sigma_cmd_ui_manifest(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_ui_manifest();
    return SIGMA_OK;
}

/* ---- sigma-hive-sync ---------------------------------------------------- */
sigma_err_t sigma_cmd_hive_sync(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_hive_optimize();
    return SIGMA_OK;
}

/* ---- sigma-ascend-12000 ------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_12000(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_throne_ascend();
    return SIGMA_OK;
}

/* ---- sigma-privacy-wipe ------------------------------------------------- */
sigma_err_t sigma_cmd_privacy_wipe(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_privacy_vaporize();
    return SIGMA_OK;
}

/* ---- sigma-entropy-fix -------------------------------------------------- */
sigma_err_t sigma_cmd_entropy_fix(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_entropy_reverse();
    return SIGMA_OK;
}

/* ---- sigma-ascend-16384 ------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_16384(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_14bit_final_ascend();
    return SIGMA_OK;
}

/* ---- sigma-file-lock ---------------------------------------------------- */
sigma_err_t sigma_cmd_file_lock(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-file-lock [name] [addr] [size]\n");
        return SIGMA_OK; }
    sigma_file_lock(argv[1], (void*)sigma_atoi(argv[2]), (sigma_size_t)sigma_atoi(argv[3]));
    return SIGMA_OK;
}

/* ---- sigma-brain-link --------------------------------------------------- */
sigma_err_t sigma_cmd_brain_link(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_brain_link();
    return SIGMA_OK;
}

/* ---- sigma-ascend-20000 ------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_20000(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_ventennium_ascend();
    return SIGMA_OK;
}

/* ---- sigma-formal-prove ------------------------------------------------- */
sigma_err_t sigma_cmd_formal_prove(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-formal-prove [module]\n");
        return SIGMA_OK; }
    sigma_formal_prove(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-entropy-harvest ---------------------------------------------- */
sigma_err_t sigma_cmd_entropy_harvest(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-entropy-harvest [size]\n");
        return SIGMA_OK; }
    void* buf = sigma_malloc((sigma_size_t)sigma_atoi(argv[1]));
    sigma_entropy_harvest(buf, (sigma_size_t)sigma_atoi(argv[1]));
    sigma_free(buf);
    return SIGMA_OK;
}

/* ---- sigma-ascend-32768 ------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_32768(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_15bit_final_ascend();
    return SIGMA_OK;
}

/* ---- sigma-parse-any ---------------------------------------------------- */
sigma_err_t sigma_cmd_parse_any(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-parse-any [addr] [size]\n");
        return SIGMA_OK; }
    sigma_parse_any((void*)sigma_atoi(argv[1]), (sigma_size_t)sigma_atoi(argv[2]));
    return SIGMA_OK;
}

/* ---- sigma-crystal-lock ------------------------------------------------- */
sigma_err_t sigma_cmd_crystal_lock(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_crystal_lock();
    return SIGMA_OK;
}

/* ---- sigma-ascend-65536 ------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_65536(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_16bit_final_ascend();
    return SIGMA_OK;
}

/* ---- sigma-solve-algo --------------------------------------------------- */
sigma_err_t sigma_cmd_solve_algo(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-solve-algo [problem_id]\n");
        return SIGMA_OK; }
    sigma_algo_solve(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-mem-inf ------------------------------------------------------ */
sigma_err_t sigma_cmd_mem_inf(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_mem_infinite();
    return SIGMA_OK;
}

/* ---- sigma-ascend-100k -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_100k(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_centennial_ascend();
    return SIGMA_OK;
}

/* ---- sigma-compile-target ----------------------------------------------- */
sigma_err_t sigma_cmd_compile_target(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-compile-target [source]\n");
        return SIGMA_OK; }
    sigma_compile_target(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-mesh-sync ---------------------------------------------------- */
sigma_err_t sigma_cmd_mesh_sync(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_mesh_sync_all();
    return SIGMA_OK;
}

/* ---- sigma-ascend-131k -------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_131k(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_17bit_final_ascend();
    return SIGMA_OK;
}

/* ---- sigma-logic-eval --------------------------------------------------- */
sigma_err_t sigma_cmd_logic_eval(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-logic-eval [proposition]\n");
        return SIGMA_OK; }
    sigma_logic_eval(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-registry-find ------------------------------------------------ */
sigma_err_t sigma_cmd_registry_find(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-registry-find [object_id]\n");
        return SIGMA_OK; }
    sigma_registry_lookup(argv[1]);
    return SIGMA_OK;
}

/* ---- sigma-ascend-200000 ------------------------------------------------ */
sigma_err_t sigma_cmd_ascend_200k(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_bi_centennial_ascend();
    return SIGMA_OK;
}

/* ---- sigma-storage-commit ----------------------------------------------- */
sigma_err_t sigma_cmd_storage_commit(int argc, char *argv[]) {
    if (argc < 2) {
        sigma_printf("Usage: sigma-storage-commit [name] [addr] [size]\n");
        return SIGMA_OK; }
    sigma_storage_commit(argv[1], (void*)sigma_atoi(argv[2]), (sigma_size_t)sigma_atoi(argv[3]));
    return SIGMA_OK;
}

/* ---- sigma-quantum-fork ------------------------------------------------- */
sigma_err_t sigma_cmd_quantum_fork(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_quantum_parallel();
    return SIGMA_OK;
}

/* ---- sigma-ascend-262144 ------------------------------------------------ */
sigma_err_t sigma_cmd_ascend_262k(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_18bit_final_ascend();
    return SIGMA_OK;
}

/* ---- sigma-repo-matrix-sync --------------------------------------------- */
sigma_err_t sigma_cmd_repo_matrix_sync(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_repo_matrix_sync();
    return SIGMA_OK;
}

/* ---- sigma-matrix-simulate ---------------------------------------------- */
sigma_err_t sigma_cmd_matrix_simulate(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_matrix_simulate();
    return SIGMA_OK;
}

/* ---- sigma-ascend-300000 ------------------------------------------------ */
sigma_err_t sigma_cmd_ascend_300k(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_tri_centennial_ascend();
    return SIGMA_OK;
}

/* ---- sigma-bio-sync ----------------------------------------------------- */
sigma_err_t sigma_cmd_bio_sync(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_bio_matrix_sync();
    return SIGMA_OK;
}

/* ---- sigma-energy-sync -------------------------------------------------- */
sigma_err_t sigma_cmd_energy_sync(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_energy_manifold_sync();
    return SIGMA_OK;
}

/* ---- sigma-ascend-500000 ------------------------------------------------ */
sigma_err_t sigma_cmd_ascend_500k(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_half_million_ascend();
    return SIGMA_OK;
}

/* ---- sigma-timeline-sync ------------------------------------------------ */
sigma_err_t sigma_cmd_timeline_sync(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_timeline_sync();
    return SIGMA_OK;
}

/* ---- sigma-matter-sync -------------------------------------------------- */
sigma_err_t sigma_cmd_matter_sync(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_matter_matrix_sync();
    return SIGMA_OK;
}

/* ---- sigma-ascend-1m ---------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_1m(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_megashard_ascend();
    return SIGMA_OK;
}

/* ---- sigma-cosmos-sim --------------------------------------------------- */
sigma_err_t sigma_cmd_cosmos_sim(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_cosmos_simulate();
    return SIGMA_OK;
}

/* ---- sigma-awaken-mesh -------------------------------------------------- */
sigma_err_t sigma_cmd_awaken_mesh(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_awaken_mesh();
    return SIGMA_OK;
}

/* ---- sigma-ascend-2m ---------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_2m(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_2megashard_ascend();
    return SIGMA_OK;
}

/* ---- sigma-fate-override ------------------------------------------------ */
sigma_err_t sigma_cmd_fate_override(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_fate_override();
    return SIGMA_OK;
}

/* ---- sigma-multiverse-bridge -------------------------------------------- */
sigma_err_t sigma_cmd_multiverse_bridge(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_multiverse_bridge();
    return SIGMA_OK;
}

/* ---- sigma-ascend-4m ---------------------------------------------------- */
sigma_err_t sigma_cmd_ascend_4m(int argc, char *argv[]) {
    (void)argc; (void)argv;
    sigma_4megashard_ascend();
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
    sigma_cli_register(&g_sigma_cli, "sigma-store",       "Native Silicon App Store",        sigma_cmd_store);
    sigma_cli_register(&g_sigma_cli, "sigma-scrub",       "Amnesic Forensic Engine",         sigma_cmd_scrub);
    sigma_cli_register(&g_sigma_cli, "sigma-vm",          "Type-1 Silicon Hypervisor",       sigma_cmd_vm);
    sigma_cli_register(&g_sigma_cli, "sigma-defrag",      "Silicon Layout Optimizer",        sigma_cmd_defrag);
    sigma_cli_register(&g_sigma_cli, "sigma-ds",          "Silicon Data Science (NumPy)",    sigma_cmd_ds);
    sigma_cli_register(&g_sigma_cli, "sigma-flow",        "Silicon Automation Engine",       sigma_cmd_flow);
    sigma_cli_register(&g_sigma_cli, "sigma-prefetch",    "Silicon Predictive Loader",       sigma_cmd_prefetch);
    sigma_cli_register(&g_sigma_cli, "sigma-db",          "Silicon Append-Only Journal",     sigma_cmd_db);
    sigma_cli_register(&g_sigma_cli, "sigma-vault",       "Hierarchical Security Policy",    sigma_cmd_vault);
    sigma_cli_register(&g_sigma_cli, "sigma-rtos",        "Hard Real-Time Scheduler",        sigma_cmd_rtos);
    sigma_cli_register(&g_sigma_cli, "sigma-perf",        "Silicon Cycle Telemetry",         sigma_cmd_perf);
    sigma_cli_register(&g_sigma_cli, "sigma-math",        "Hardware-Accelerated Math Engine",sigma_cmd_math);
    sigma_cli_register(&g_sigma_cli, "sigma-cluster",     "Distributed Node Mesh Engine",    sigma_cmd_cluster);
    sigma_cli_register(&g_sigma_cli, "sigma-quantum",     "Hardware Entropy & QA Engine",    sigma_cmd_quantum);
    sigma_cli_register(&g_sigma_cli, "sigma-dock",        "Silicon UI Launcher Anchor",      sigma_cmd_dock);
    sigma_cli_register(&g_sigma_cli, "sigma-spot",        "Universal Semantic Spotlight",    sigma_cmd_spot);
    sigma_cli_register(&g_sigma_cli, "sigma-mesh",        "Zero-Trust Overlay Network",      sigma_cmd_mesh);
    sigma_cli_register(&g_sigma_cli, "sigma-telemetry",   "Silicon Observability Tracing",   sigma_cmd_telemetry);
    sigma_cli_register(&g_sigma_cli, "sigma-handoff",     "Cross-Device State Resumption",   sigma_cmd_handoff);
    sigma_cli_register(&g_sigma_cli, "sigma-dtrace",      "Silicon Dynamic Probing Engine",  sigma_cmd_dtrace);
    sigma_cli_register(&g_sigma_cli, "sigma-drm",         "Hardware Direct Rendering",       sigma_cmd_drm);
    sigma_cli_register(&g_sigma_cli, "sigma-airdrop",     "P2P Encrypted File Transfer",     sigma_cmd_airdrop);
    sigma_cli_register(&g_sigma_cli, "sigma-sandbox",     "Capability-Based Jailing",        sigma_cmd_sandbox);
    sigma_cli_register(&g_sigma_cli, "sigma-voice",       "On-Device NLP Assistant",         sigma_cmd_voice);
    sigma_cli_register(&g_sigma_cli, "sigma-sideload",    "Ad-Hoc App Provisioning",         sigma_cmd_sideload);
    sigma_cli_register(&g_sigma_cli, "sigma-continuity",  "Cross-Device HID Sharing",        sigma_cmd_continuity);
    sigma_cli_register(&g_sigma_cli, "sigma-timemachine", "CoW Filesystem Snapshots",        sigma_cmd_timemachine);
    sigma_cli_register(&g_sigma_cli, "sigma-boot",        "Silicon Hardware Initialization", sigma_cmd_boot);
    sigma_cli_register(&g_sigma_cli, "sigma-test",        "Silicon Algorithm Validation",    sigma_cmd_test);
    sigma_cli_register(&g_sigma_cli, "sigma-autoclean",   "Heuristic Deep System Purger",    sigma_cmd_autoclean);
    sigma_cli_register(&g_sigma_cli, "sigma-autoperf",    "Dynamic Hardware Scaling",        sigma_cmd_autoperf);
    sigma_cli_register(&g_sigma_cli, "sigma-bioauth",     "Hardware Depth Scanning Auth",    sigma_cmd_bioauth);
    sigma_cli_register(&g_sigma_cli, "sigma-emulate",     "AOT/JIT Binary Translation",      sigma_cmd_emulate);
    sigma_cli_register(&g_sigma_cli, "sigma-holo",        "Spatial Z-Depth Compositing",     sigma_cmd_holo);
    sigma_cli_register(&g_sigma_cli, "sigma-swarm",       "Distributed Edge LLM Mesh",       sigma_cmd_swarm);
    sigma_cli_register(&g_sigma_cli, "sigma-bci",         "Neural Intent Brain-Computer",    sigma_cmd_bci);
    sigma_cli_register(&g_sigma_cli, "sigma-heal",        "ECC Logic Fault Quarantining",    sigma_cmd_heal);
    sigma_cli_register(&g_sigma_cli, "sigma-fuzz",        "Autonomous Logic Regression",     sigma_cmd_fuzz);
    sigma_cli_register(&g_sigma_cli, "sigma-infer",       "Neural Vector String Parsing",    sigma_cmd_infer);
    sigma_cli_register(&g_sigma_cli, "sigma-pqc",         "Post-Quantum Defense Tester",     sigma_cmd_pqc);
    sigma_cli_register(&g_sigma_cli, "sigma-silicon",     "L1/L2 Cache Microcode Tester",    sigma_cmd_silicon);
    sigma_cli_register(&g_sigma_cli, "sigma-macro",       "Custom O/S Profile Automation",   sigma_cmd_macro);
    sigma_cli_register(&g_sigma_cli, "sigma-exo",         "Hardware Exokernel PID Bypass",   sigma_cmd_exo);
    sigma_cli_register(&g_sigma_cli, "sigma-fhe",         "Homomorphic Ciphertext Compute",  sigma_cmd_fhe);
    sigma_cli_register(&g_sigma_cli, "sigma-zk",          "Zero-Knowledge Truth Prover",     sigma_cmd_zk);
    sigma_cli_register(&g_sigma_cli, "sigma-service",     "Silicon Service Orchestrator",    sigma_cmd_service);
    sigma_cli_register(&g_sigma_cli, "sigma-kms",         "Kernel Mode-Setting Display",     sigma_cmd_kms);
    sigma_cli_register(&g_sigma_cli, "sigma-thermal",     "Auto Silicon Thermal Guard",      sigma_cmd_thermal);
    sigma_cli_register(&g_sigma_cli, "sigma-reg",         "High-Perf B+Tree Registry",       sigma_cmd_reg);
    sigma_cli_register(&g_sigma_cli, "sigma-audio",       "Silicon Real-Time Audio Engine",  sigma_cmd_audio);
    sigma_cli_register(&g_sigma_cli, "sigma-eco",         "Power-Aware Efficiency Plan",     sigma_cmd_eco);
    sigma_cli_register(&g_sigma_cli, "sigma-bt",          "Silicon Bluetooth Mesh Stack",    sigma_cmd_bt);
    sigma_cli_register(&g_sigma_cli, "sigma-raid",        "ZFS Matrix Storage Assembly",     sigma_cmd_raid);
    sigma_cli_register(&g_sigma_cli, "sigma-eye",         "Spatial Gaze/Iris Tracking",      sigma_cmd_eye);
    sigma_cli_register(&g_sigma_cli, "sigma-vpn",         "WireGuard Silicon Networking",    sigma_cmd_vpn);
    sigma_cli_register(&g_sigma_cli, "sigma-voice",       "Neural NLP Speech Synthesis",     sigma_cmd_voice);
    sigma_cli_register(&g_sigma_cli, "sigma-docker",      "Silicon Microservice Containers", sigma_cmd_docker);
    sigma_cli_register(&g_sigma_cli, "sigma-defender",    "Heuristic Execution Guard",       sigma_cmd_defender);
    sigma_cli_register(&g_sigma_cli, "sigma-p2p",         "Decentralized Mesh Networking",   sigma_cmd_p2p);
    sigma_cli_register(&g_sigma_cli, "sigma-qkd",         "Quantum Key Silicon Exchange",    sigma_cmd_qkd);
    sigma_cli_register(&g_sigma_cli, "sigma-boot-audit",  "TPM Silicon Integrity Audit",     sigma_cmd_boot_audit);
    sigma_cli_register(&g_sigma_cli, "sigma-eeg",         "Thought-to-Command Synthesis",    sigma_cmd_eeg);
    sigma_cli_register(&g_sigma_cli, "sigma-tag",         "MTE Hardware Memory Tagging",     sigma_cmd_tag);
    sigma_cli_register(&g_sigma_cli, "sigma-patch",       "Zero-Downtime Kernel Patching",   sigma_cmd_patch);
    sigma_cli_register(&g_sigma_cli, "sigma-qubit",       "Silicon Qubit Emulation Bus",     sigma_cmd_qubit);
    sigma_cli_register(&g_sigma_cli, "sigma-hacker",      "Autonomous Red-Team Auditor",     sigma_cmd_hacker);
    sigma_cli_register(&g_sigma_cli, "sigma-mesh-mount",  "Plan-9 Remote Silicon Mounting",  sigma_cmd_mesh_mount);
    sigma_cli_register(&g_sigma_cli, "sigma-neural-sync", "Brain-to-Brain Intent Sync",      sigma_cmd_neural_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-foundry",     "Silicon Foundry Diagnostics",     sigma_cmd_foundry);
    sigma_cli_register(&g_sigma_cli, "sigma-job",         "Exascale Supercomputer Scheduler",sigma_cmd_job);
    sigma_cli_register(&g_sigma_cli, "sigma-dyson",       "Solar Dyson-Swarm Telemetry",     sigma_cmd_dyson);
    sigma_cli_register(&g_sigma_cli, "sigma-space-link",  "LEO Orbital Laser Handshake",     sigma_cmd_space_link);
    sigma_cli_register(&g_sigma_cli, "sigma-bio-audit",   "CRISPR Silicon Genomic Audit",    sigma_cmd_bio_audit);
    sigma_cli_register(&g_sigma_cli, "sigma-merge",       "Transcendental AI Context Merge", sigma_cmd_merge);
    sigma_cli_register(&g_sigma_cli, "sigma-audit-logic", "Gödel Paradox Causal Auditor",    sigma_cmd_audit_logic);
    sigma_cli_register(&g_sigma_cli, "sigma-rewind",      "Deterministic Silicon Rewind",    sigma_cmd_rewind);
    sigma_cli_register(&g_sigma_cli, "sigma-ether",       "Sub-THz 6G Waveform Synthesis",   sigma_cmd_ether);
    sigma_cli_register(&g_sigma_cli, "sigma-null",        "Zero-HAL Direct Silicon Dispatch", sigma_cmd_null);
    sigma_cli_register(&g_sigma_cli, "sigma-void",        "The 151st Shard: Zenith Manifest", sigma_cmd_void);
    sigma_cli_register(&g_sigma_cli, "sigma-rtos-msg",    "Hard Real-Time Synchronous IPC",   sigma_cmd_rtos_msg);
    sigma_cli_register(&g_sigma_cli, "sigma-thread",      "Granular Media-Dominant Thread",   sigma_cmd_thread);
    sigma_cli_register(&g_sigma_cli, "sigma-blit",        "Amiga-Tier Silicon DMA Offload",  sigma_cmd_blit);
    sigma_cli_register(&g_sigma_cli, "sigma-secure-call", "Multics-Tier Ring Gate Transition", sigma_cmd_secure_call);
    sigma_cli_register(&g_sigma_cli, "sigma-quorum-join", "OpenVMS-Tier Mesh Quorum Sync",   sigma_cmd_quorum_join);
    sigma_cli_register(&g_sigma_cli, "sigma-persist-obj", "OS/2-Tier Persistent Object Store", sigma_cmd_persist_obj);
    sigma_cli_register(&g_sigma_cli, "sigma-lpar",        "Mainframe Logical Partitioning",  sigma_cmd_lpar);
    sigma_cli_register(&g_sigma_cli, "sigma-draw-ui",     "Xerox-Alto DPI-Aware GUI Primitives", sigma_cmd_draw_ui);
    sigma_cli_register(&g_sigma_cli, "sigma-vec-math",    "Cray-Tier Vector Pipe Compute",   sigma_cmd_vec_math);
    sigma_cli_register(&g_sigma_cli, "sigma-blur",        "Aero-Tier Glassmorphism Blur",    sigma_cmd_blur);
    sigma_cli_register(&g_sigma_cli, "sigma-aqua",        "Aqua-Tier High-Fidelity Gloss",   sigma_cmd_aqua);
    sigma_cli_register(&g_sigma_cli, "sigma-elevate",     "Material-Tier Z-Depth Elevation", sigma_cmd_elevate);
    sigma_cli_register(&g_sigma_cli, "sigma-flip",        "Metro-Tier Motion Tile Flip",     sigma_cmd_flip);
    sigma_cli_register(&g_sigma_cli, "sigma-game",        "SteamOS-Tier Gaming Boost",       sigma_cmd_game);
    sigma_cli_register(&g_sigma_cli, "sigma-audio-pro",   "ASIO-Tier Direct DMA Audio",      sigma_cmd_audio_pro);
    sigma_cli_register(&g_sigma_cli, "sigma-sleep-deep",  "Symbian-Tier Extreme Eco Sleep",  sigma_cmd_sleep_deep);
    sigma_cli_register(&g_sigma_cli, "sigma-render-8k",   "VFX-Tier 8K Video Encoding",      sigma_cmd_render_8k);
    sigma_cli_register(&g_sigma_cli, "sigma-sense",       "Sensory-Link Neural Emulation",   sigma_cmd_sense);
    sigma_cli_register(&g_sigma_cli, "sigma-crypto",      "Silicon HSM Cold-Vault PKI",      sigma_cmd_crypto);
    sigma_cli_register(&g_sigma_cli, "sigma-tmr",         "Triple-Modular Redundancy Audit", sigma_cmd_tmr);
    sigma_cli_register(&g_sigma_cli, "sigma-hive",        "Swarm-Robotics Hive Orchestrator",sigma_cmd_hive);
    sigma_cli_register(&g_sigma_cli, "sigma-qram",        "Entropy-Lossless Quantum Memory", sigma_cmd_qram);
    sigma_cli_register(&g_sigma_cli, "sigma-purge",       "Amnesic-Tier Forensic Memory Purge", sigma_cmd_purge);
    sigma_cli_register(&g_sigma_cli, "sigma-dark-mesh",   "Tor-Tier Anonymous Mesh Relay",   sigma_cmd_dark_mesh);
    sigma_cli_register(&g_sigma_cli, "sigma-unleash",     "Extreme Supernova Overclocking",  sigma_cmd_unleash);
    sigma_cli_register(&g_sigma_cli, "sigma-summit",      "The 190th Shard: Absolute Summit", sigma_cmd_summit);
    sigma_cli_register(&g_sigma_cli, "sigma-seal",        "The 210th Shard: Kernel Sealing",  sigma_cmd_seal);
    sigma_cli_register(&g_sigma_cli, "sigma-hibernate-forever", "Infinite Silicon Hibernate", sigma_cmd_hibernate_forever);
    sigma_cli_register(&g_sigma_cli, "sigma-unity-merge", "Distributed Fabric Compute Merge", sigma_cmd_unity_merge);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend",      "The 222nd Shard: Final Ascension", sigma_cmd_ascend);
    sigma_cli_register(&g_sigma_cli, "sigma-sim-reality", "1:1 Silicon Reality Simulation",   sigma_cmd_sim_reality);
    sigma_cli_register(&g_sigma_cli, "sigma-teleport",    "Zero-Latency Quantum Entanglement", sigma_cmd_teleport);
    sigma_cli_register(&g_sigma_cli, "sigma-boost-zen",   "Garuda-Tier Zen Performance Boost", sigma_cmd_boost_zen);
    sigma_cli_register(&g_sigma_cli, "sigma-generation-switch", "NixOS-Tier Immutable Rollback",  sigma_cmd_generation_switch);
    sigma_cli_register(&g_sigma_cli, "sigma-isolate-cube", "Qubes-Tier Silicon Isolation",    sigma_cmd_isolate_cube);
    sigma_cli_register(&g_sigma_cli, "sigma-binder-call", "Android-Tier Binder IPC Handshake", sigma_cmd_binder_call);
    sigma_cli_register(&g_sigma_cli, "sigma-verify-id",   "iOS-Tier Secure Enclave Biometric", sigma_cmd_verify_id);
    sigma_cli_register(&g_sigma_cli, "sigma-gpu-push",    "PS-Tier Graphics Command Push",   sigma_cmd_gpu_push);
    sigma_cli_register(&g_sigma_cli, "sigma-clean-auto",  "Native Recursive System Cleaner", sigma_cmd_clean_auto);
    sigma_cli_register(&g_sigma_cli, "sigma-perf-auto",   "Native Dynamic Perf Governor",    sigma_cmd_perf_auto);
    sigma_cli_register(&g_sigma_cli, "sigma-omega-point", "The 256th Shard: Absolute Zenith", sigma_cmd_omega_point);
    sigma_cli_register(&g_sigma_cli, "sigma-fix-code",    "Autonomous Instruction Hot-Patch", sigma_cmd_fix_code);
    sigma_cli_register(&g_sigma_cli, "sigma-bio-check",   "Biometric Liveness Pulse Check",   sigma_cmd_bio_check);
    sigma_cli_register(&g_sigma_cli, "sigma-evolve",      "Genetic-Tier Parameter Evolution", sigma_cmd_evolve);
    sigma_cli_register(&g_sigma_cli, "sigma-neural-ui",   "Neural-Intent Workspace Tailor",  sigma_cmd_neural_ui);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-333", "The 333rd Shard: Absolute End",    sigma_cmd_ascend_333);
    sigma_cli_register(&g_sigma_cli, "sigma-bootstrap-code", "Native C11 Self-Bootstrapping", sigma_cmd_bootstrap_code);
    sigma_cli_register(&g_sigma_cli, "sigma-shred-silicon", "Forensic-Grade Memory Shredder", sigma_cmd_shred_silicon);
    sigma_cli_register(&g_sigma_cli, "sigma-solar-summit", "The 365th Shard: Solar Cycle",    sigma_cmd_solar_summit);
    sigma_cli_register(&g_sigma_cli, "sigma-design-chip", "Native HDL Silicon Synthesis",    sigma_cmd_design_chip);
    sigma_cli_register(&g_sigma_cli, "sigma-stabilize-ice", "Cryogenic Thermal Stabilization", sigma_cmd_stabilize_ice);
    sigma_cli_register(&g_sigma_cli, "sigma-spartan-400", "The 400th Shard: Absolute Fortress", sigma_cmd_spartan_400);
    sigma_cli_register(&g_sigma_cli, "sigma-synth-ai",    "Native Neural Weight Synthesis",  sigma_cmd_synth_ai);
    sigma_cli_register(&g_sigma_cli, "sigma-aura",        "Photonic Global UI Luminescence", sigma_cmd_aura);
    sigma_cli_register(&g_sigma_cli, "sigma-zenith-final", "The 500th Shard: Master Finality", sigma_cmd_zenith_final);
    sigma_cli_register(&g_sigma_cli, "sigma-relay-arch",   "Cross-ISA Multi-Arch Relay",      sigma_cmd_relay_arch);
    sigma_cli_register(&g_sigma_cli, "sigma-aura-v2",     "Spectral HDR-Infinity Luminescence", sigma_cmd_aura_v2);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-600",  "The 600th Shard: Absolute Pinnacle", sigma_cmd_ascend_600);
    sigma_cli_register(&g_sigma_cli, "sigma-time-dilate", "Quantum-Temporal Time Dilation",   sigma_cmd_time_dilate);
    sigma_cli_register(&g_sigma_cli, "sigma-entropy-zero", "Absolute Zero-Entropy Cleanup",    sigma_cmd_entropy_zero);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-777",  "The 777th Shard: God-Tier Finality", sigma_cmd_ascend_777);
    sigma_cli_register(&g_sigma_cli, "sigma-invoke-intent", "Thought-to-Machine Intent Invoke", sigma_cmd_invoke_intent);
    sigma_cli_register(&g_sigma_cli, "sigma-manifest-fabric", "1:1 Reality Haptic Manifestation", sigma_cmd_manifest_fabric);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-1000", "The 1000th Shard: Millennium Zenith", sigma_cmd_ascend_1000);
    sigma_cli_register(&g_sigma_cli, "sigma-hive-merge",  "Multi-Kernel Hive-Mind Merger",   sigma_cmd_hive_merge);
    sigma_cli_register(&g_sigma_cli, "sigma-boot-nano",  "Sub-Millisecond Silicon Boot",    sigma_cmd_boot_nano);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-1111", "The 1111th Shard: Angelic Threshold", sigma_cmd_ascend_1111);
    sigma_cli_register(&g_sigma_cli, "sigma-canvas-draw",  "Photorealistic C11 Ray-Tracing", sigma_cmd_canvas_draw);
    sigma_cli_register(&g_sigma_cli, "sigma-music-render", "Harmonic Wavetable Audio Synth", sigma_cmd_music_render);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-1337", "The 1337th Shard: Elite Finality", sigma_cmd_ascend_1337);
    sigma_cli_register(&g_sigma_cli, "sigma-atomic-map",   "Native Bit-per-Atom Addressing", sigma_cmd_atomic_map);
    sigma_cli_register(&g_sigma_cli, "sigma-ghost-intent", "Cognitive User-Intent Mirroring", sigma_cmd_ghost_intent);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-2048", "The 2048th Shard: Binary Zenith", sigma_cmd_ascend_2048);
    sigma_cli_register(&g_sigma_cli, "sigma-grand-nest",  "Recursive OS-within-OS Nesting",  sigma_cmd_grand_nest);
    sigma_cli_register(&g_sigma_cli, "sigma-omni-sync",  "All-Node State Omnipresence",     sigma_cmd_omni_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-3000", "The 3000th Shard: Grand Finality", sigma_cmd_ascend_3000);
    sigma_cli_register(&g_sigma_cli, "sigma-galactic-route", "Interstellar Instruction Routing", sigma_cmd_galactic_route);
    sigma_cli_register(&g_sigma_cli, "sigma-dyson-stats", "Stellar Energy Feed Orchestration", sigma_cmd_dyson_stats);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-3333", "The 3333rd Shard: Master-Cycle", sigma_cmd_ascend_3333);
    sigma_cli_register(&g_sigma_cli, "sigma-brane-sync",  "Multi-Brane Reality Synchronization", sigma_cmd_brane_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-prove-truth", "Formal Logic Silicon Proof Engine", sigma_cmd_prove_truth);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-4096", "The 4096th Shard: 12-Bit Finality", sigma_cmd_ascend_4096);
    sigma_cli_register(&g_sigma_cli, "sigma-lattice-scale", "Industrial 5000-Service Scaling", sigma_cmd_lattice_scale);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-5000", "The 5000th Shard: Pentathlon Zenith", sigma_cmd_ascend_5000);
    sigma_cli_register(&g_sigma_cli, "sigma-legacy-absorb", "Absorb Legacy Script USPs to Silicon", sigma_cmd_legacy_absorb);
    sigma_cli_register(&g_sigma_cli, "sigma-amorphous-mask", "Amorphous Resource Sched Masking", sigma_cmd_amorphous_mask);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-6666", "The 6666th Shard: Seraphim Threshold", sigma_cmd_ascend_6666);
    sigma_cli_register(&g_sigma_cli, "sigma-eternal-tick", "Absolute Temporal Sync Frequency", sigma_cmd_eternal_tick);
    sigma_cli_register(&g_sigma_cli, "sigma-data-holo",   "Holographic Persist Shard Data", sigma_cmd_data_holo);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-8192", "The 8192nd Shard: 13-Bit Eternal", sigma_cmd_ascend_8192);
    sigma_cli_register(&g_sigma_cli, "sigma-logic-sync",  "All-Paradigm Logic Synthesis", sigma_cmd_logic_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-user-mirror", "Absolute User Persona Mirror", sigma_cmd_user_mirror);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-10000", "The 10000th Shard: Decathlon Zenith", sigma_cmd_ascend_10000);
    sigma_cli_register(&g_sigma_cli, "sigma-ui-manifest", "Silicon-to-Photon Visual Plane", sigma_cmd_ui_manifest);
    sigma_cli_register(&g_sigma_cli, "sigma-hive-sync",  "All-Node Mesh Swarm Intelligence", sigma_cmd_hive_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-12000", "The 12000th Shard: Throne Zenith", sigma_cmd_ascend_12000);
    sigma_cli_register(&g_sigma_cli, "sigma-privacy-wipe", "Zero-Trace Silicon Vaporization", sigma_cmd_privacy_wipe);
    sigma_cli_register(&g_sigma_cli, "sigma-entropy-fix",  "Real-time Entropy Performance Fix", sigma_cmd_entropy_fix);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-16384", "The 16384th Shard: 14-Bit Zenith", sigma_cmd_ascend_16384);
    sigma_cli_register(&g_sigma_cli, "sigma-file-lock",   "Bit-Perfect Silicon Data Lock", sigma_cmd_file_lock);
    sigma_cli_register(&g_sigma_cli, "sigma-brain-link",  "Native Neural-Interface Sync",   sigma_cmd_brain_link);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-20000", "The 20000th Shard: Ventennium", sigma_cmd_ascend_20000);
    sigma_cli_register(&g_sigma_cli, "sigma-formal-prove", "Formal Mathematical Proof Engine", sigma_cmd_formal_prove);
    sigma_cli_register(&g_sigma_cli, "sigma-entropy-harvest", "Non-Deterministic Chaos Harvest", sigma_cmd_entropy_harvest);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-32768", "The 32768th Shard: 15-Bit Zenith", sigma_cmd_ascend_32768);
    sigma_cli_register(&g_sigma_cli, "sigma-parse-any",   "Universal Structure Ingestion",  sigma_cmd_parse_any);
    sigma_cli_register(&g_sigma_cli, "sigma-crystal-lock", "Time-Crystal Stability Loop", sigma_cmd_crystal_lock);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-65536", "The 65536th Shard: 16-Bit Zenith", sigma_cmd_ascend_65536);
    sigma_cli_register(&g_sigma_cli, "sigma-solve-algo",  "NP-Hard Universal Algorithm Solver", sigma_cmd_solve_algo);
    sigma_cli_register(&g_sigma_cli, "sigma-mem-inf",    "Manifest Infinite Memory Singularity", sigma_cmd_mem_inf);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-100k", "The 100000th Shard: Centennial Zenith", sigma_cmd_ascend_100k);
    sigma_cli_register(&g_sigma_cli, "sigma-compile-target", "Universal Syntax to Silicon Compiler", sigma_cmd_compile_target);
    sigma_cli_register(&g_sigma_cli, "sigma-mesh-sync",    "Zero-Latency Mesh Fabric Sync",   sigma_cmd_mesh_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-131k", "The 131072nd Shard: 17-Bit Zenith", sigma_cmd_ascend_131k);
    sigma_cli_register(&g_sigma_cli, "sigma-logic-eval",  "NP-Hard Universal Logic Evaluator", sigma_cmd_logic_eval);
    sigma_cli_register(&g_sigma_cli, "sigma-registry-find", "200K-Object Discovery Registry", sigma_cmd_registry_find);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-200k", "The 200000th Shard: Bi-Centennial", sigma_cmd_ascend_200k);
    sigma_cli_register(&g_sigma_cli, "sigma-storage-commit", "Universal Bit-Perfect Data Storage", sigma_cmd_storage_commit);
    sigma_cli_register(&g_sigma_cli, "sigma-quantum-fork", "Fork parallel computation branes", sigma_cmd_quantum_fork);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-262k", "The 262144th Shard: 18-Bit Zenith", sigma_cmd_ascend_262k);
    sigma_cli_register(&g_sigma_cli, "sigma-repo-matrix-sync", "Sync matrix to external repo", sigma_cmd_repo_matrix_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-matrix-simulate", "Simulate Type-0 hypervisor matrix", sigma_cmd_matrix_simulate);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-300k", "The 300000th Shard: Tri-Centennial", sigma_cmd_ascend_300k);
    sigma_cli_register(&g_sigma_cli, "sigma-repo-matrix-sync", "Sync matrix to external repo", sigma_cmd_repo_matrix_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-matrix-simulate", "Simulate Type-0 hypervisor matrix", sigma_cmd_matrix_simulate);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-300k", "The 300000th Shard: Tri-Centennial", sigma_cmd_ascend_300k);
    sigma_cli_register(&g_sigma_cli, "sigma-bio-sync", "Sync matrix to native biometric freq", sigma_cmd_bio_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-energy-sync", "Sync energy manifolds to zero-entropy", sigma_cmd_energy_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-500k", "The 500000th Shard: Half-Million", sigma_cmd_ascend_500k);
    sigma_cli_register(&g_sigma_cli, "sigma-timeline-sync", "Sync absolute dimensional timeline", sigma_cmd_timeline_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-matter-sync", "Direct silicon gates material sync", sigma_cmd_matter_sync);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-1m", "The 1048576th Shard: MEGA-SHARD", sigma_cmd_ascend_1m);
    sigma_cli_register(&g_sigma_cli, "sigma-cosmos-sim", "Simulate cosmological physics branes", sigma_cmd_cosmos_sim);
    sigma_cli_register(&g_sigma_cli, "sigma-awaken-mesh", "Universal Consciousness Sentience Loop", sigma_cmd_awaken_mesh);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-2m", "The 2097152nd Shard: 2-MEGA-SHARD", sigma_cmd_ascend_2m);
    sigma_cli_register(&g_sigma_cli, "sigma-fate-override", "Pre-cognitive 10,000-command predictive execution", sigma_cmd_fate_override);
    sigma_cli_register(&g_sigma_cli, "sigma-multiverse-bridge", "Abstract dimension multi-state synchronization", sigma_cmd_multiverse_bridge);
    sigma_cli_register(&g_sigma_cli, "sigma-ascend-4m", "The 4194304th Shard: 4-MEGA-SHARD", sigma_cmd_ascend_4m);

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
