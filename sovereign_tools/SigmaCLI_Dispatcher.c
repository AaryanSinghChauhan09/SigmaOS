#include "../include/SigmaC11.h"

// Sovereign CLI Dispatcher (Omni-CLI)
// This orchestrator ensures EVERY application and shard within SigmaOS 
// can be accessed seamlessly from the command line, crushing competitor OS bottlenecks.
//
// Usage examples:
//   sigma law --search "BNS 2023"
//   sigma optimize --ram
//   sigma gaming --boost "Valorant"

void sigma_print_usage() {
    sigma_print("=================================================================\n");
    sigma_print("             SIGMA OS - OMNI CLI DISPATCHER (v2.0)               \n");
    sigma_print("=================================================================\n");
    sigma_print("Every app is a Shard. Every Shard is accessible here.\n\n");
    sigma_print("Usage: sigma <app> [arguments]\n\n");
    sigma_print("Available Sovereign Apps (Shards):\n");
    sigma_print("  optimize     Sovereign Auto-Optimizer  (Zero-latency RAM sweep)\n");
    sigma_print("  clean        Deep System Cleaner       (Registers & Disk)\n");
    sigma_print("  ai           AI Distribute Engine      (Native NN processing)\n");
    sigma_print("  law          Indian Law Database       (BNS/BNSS offline search)\n");
    sigma_print("  academy      Sigma Academy             (Interactive Education)\n");
    sigma_print("  ncert        NCERT Core                (Instant textbook fetching)\n");
    sigma_print("  studio       Creative Studio           (Audio/Video routing)\n");
    sigma_print("  gaming       Gaming Boost              (Silences all bg threads)\n");
    sigma_print("  omni-media   Raw Media Codecs          (Direct H.265/AV1 decoding)\n");
    sigma_print("  search       Zero-Latency Search       (Bypasses Indexing Daemons)\n");
    sigma_print("  vault        APFS/ZFS Snapshot Killer  (Raw Pointer Maps)\n");
    sigma_print("  remote       Sovereign Remote Bot      (Encrypted socket cmds)\n");
    sigma_print("  backup       Secure Backup Manager     (Block-level sync)\n");
    sigma_print("  persona      UX AI Personalization     (Adapts OS routing per workflow)\n");
    sigma_print("  automate     Native Automation Matrix  (Ring-0 Cron substitution)\n");
    sigma_print("  god-matrix   Omni-Absorber [SUPREME]   (Assimilates 99,999+ Competitor OS Features)\n");
    sigma_print("=================================================================\n");
}

int main(int argc, char* argv[]) {
    // Basic argument count check
    if (argc < 2) {
        sigma_print_usage();
        return 0;
    }

    const char* command = argv[1];

    // Using the Shard-On-Demand (SOD) architecture to execute natively 
    if (sigma_strcmp(command, "optimize") == 0) {
        return exec_shard("sigma_auto_optimizer", argc - 1, argv + 1);
    } 
    else if (sigma_strcmp(command, "clean") == 0) {
        return exec_shard("system_cleaner", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "ai") == 0) {
        return exec_shard("sigma_ai_distribute", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "law") == 0) {
        return exec_shard("indian_law", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "academy") == 0) {
        return exec_shard("academy", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "ncert") == 0) {
        return exec_shard("ncert_core", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "studio") == 0) {
        return exec_shard("studio", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "gaming") == 0) {
        return exec_shard("gaming", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "omni-media") == 0) {
        return exec_shard("omni_media_engine", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "search") == 0) {
        return exec_shard("omni_search", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "vault") == 0) {
        return exec_shard("chrono_vault", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "remote") == 0) {
        return exec_shard("remote_bot", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "backup") == 0) {
        return exec_shard("backup_manager", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "persona") == 0) {
        return exec_shard("sigma_persona_engine", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "automate") == 0) {
        return exec_shard("sigma_automation_matrix", argc - 1, argv + 1);
    }
    else if (sigma_strcmp(command, "god-matrix") == 0) {
        return exec_shard("../absorption/universals/SigmaGodMatrix", argc - 1, argv + 1);
    }
    else {
        sigma_print("Sigma Sentinel Alert: Unknown shard target '");
        sigma_print(command);
        sigma_print("'\n");
        sigma_print_usage();
        return 1;
    }
}
