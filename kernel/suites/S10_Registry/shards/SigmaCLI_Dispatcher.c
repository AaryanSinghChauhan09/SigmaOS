/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CLI DISPATCHER (v50.2-OMEGA — TRANS-OS SUPREMACY)
 * =========================================================================
 * The universal user-facing entry point for the Omni-CLI. 
 * Orchestrates thousands of shards via the Sentient Command Matrix.
 * =========================================================================
 */

#include "sigma_kernel.h"

/* Local utility: replaces strcmp for zero-dependency architecture */
static int sigma_strcmp(const char* a, const char* b) {
    while (*a && (*a == *b)) { a++; b++; }
    return *(const unsigned char*)a - *(const unsigned char*)b;
}

/* exec_shard: delegates to the Shard-On-Demand (SOD) loader */
extern int exec_shard(const char* name, int argc, char** argv);

void sigma_print_usage(void) {
    sigma_print("=================================================================\n");
    sigma_print("        SIGMA OS - OMNI CLI DISPATCHER v50.2-OMEGA             \n");
    sigma_print("=================================================================\n");
    sigma_print("Universal ABI compatibility. Zero-Dependency Mastery. Sentient OS.\n\n");
    sigma_print("Usage: sigma <module> [subcommand] [args...]\n\n");

    sigma_print("--- OMEGA SUITES ---\n");
    sigma_print("  batch        Batch job submission & multiprogramming\n");
    sigma_print("  distribute   MapReduce & Task offloading (P2P Cluster)\n");
    sigma_print("  sentience    AI-driven predictive kernel shell\n");
    sigma_print("  oop          Object registry & dynamic class dispatch\n");
    sigma_print("  persona      Personalized adaptive kernel profiles\n");
    sigma_print("  udf          Execute sandboxed User-Defined Functions\n\n");

    sigma_print("--- CORE SYSTEM ---\n");
    sigma_print("  sys          Kernel tuning, IPC, IRQ, sysctl\n");
    sigma_print("  shard        Hot-deploy & scale sovereign shards\n");
    sigma_print("  optimize     Predictive zero-latency RAM optimization\n");
    sigma_print("  clean        Amnesic silicon wipe (Singularity level)\n\n");

    sigma_print("--- DATABASE & DATA SCIENCE ---\n");
    sigma_print("  df           High-speed dataframe & tensor persistence\n");
    sigma_print("  db           SQL/NoSQL hybrid sovereign engine\n");
    sigma_print("  ai           Neural inference & anomaly detection\n\n");

    sigma_print("--- DISTRO & TOOL ABSORPTION ---\n");
    sigma_print("  god-matrix   ∞ ABSOLUTE ABSORPTION: Neutralize ALL competitors ∞\n");
    sigma_print("=================================================================\n");
}

int SigmaCLI_Dispatcher_ToolMain(int argc, char* argv[]) {
    if (argc < 2) {
        sigma_print_usage();
        return 0;
    }

    const char* command = argv[1];

    /* ---- Omega Suite Dispatch ---- */
    if (sigma_strcmp(command, "batch") == 0)
        return exec_shard("SovereignMultiProcessShard", argc - 1, argv + 1);
    if (sigma_strcmp(command, "distribute") == 0)
        return exec_shard("SovereignNetworkShard", argc - 1, argv + 1);
    if (sigma_strcmp(command, "sentience") == 0)
        return exec_shard("SovereignIntelligenceShard", argc - 1, argv + 1);
    if (sigma_strcmp(command, "oop") == 0)
        return exec_shard("SovereignObjectShard", argc - 1, argv + 1);
    if (sigma_strcmp(command, "persona") == 0)
        return exec_shard("SovereignPersonaShard", argc - 1, argv + 1);
    if (sigma_strcmp(command, "udf") == 0)
        return exec_shard("SovereignUDFEngine", argc - 1, argv + 1);
    if (sigma_strcmp(command, "df") == 0)
        return exec_shard("SovereignDataframeShard", argc - 1, argv + 1);

    /* ---- Traditional God-Matrix ---- */
    if (sigma_strcmp(command, "god-matrix") == 0) {
        sigma_print("[OMEGA-INIT]: Initiating Global Mesh Absorption...\n");
        return exec_shard("SigmaGodMatrix", argc - 1, argv + 1);
    }

    /* ---- Help ---- */
    if (sigma_strcmp(command, "help") == 0 || sigma_strcmp(command, "--help") == 0) {
        sigma_print_usage();
        return 0;
    }

    sigma_print("Sigma Sentinel Alert: Forwarding unknown target to Sentience Engine...\n");
    return exec_shard("SovereignIntelligenceShard", argc - 1, argv + 1);
}



