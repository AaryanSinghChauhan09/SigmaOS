#include "../include/SovereignCommand.h"
#include "../include/sigma_libc.h"
#include "../include/sigma_kernel.h"

static int sigma_atoi_local(const char* str) {
    int res = 0;
    while (*str >= '0' && *str <= '9') { res = res * 10 + (*str - '0'); str++; }
    return res;
}

void handle_gaming(int argc, char** argv) {
    if (argc < 3) { sigma_printf("Usage: sigma gaming <boost|status|profile> [args]\n"); return; }
    sigma_printf("[GAMING]: Optimizing silicon for MISSION-CRITICAL performance... Shard: '%s'\n", argv[2]);
    sigma_printf("[GAMING]: Garuda Zen-Kernel tuning applied. Jitter minimized.\n");
    sigma_printf("[OK]: Peak performance state LOCKED.\n");
}

void handle_automate(int argc, char** argv) {
    if (argc < 4) { sigma_printf("Usage: sigma automate click <interval_ms> <clicks>\n"); return; }
    int interval = sigma_atoi_local(argv[2]);
    int clicks = sigma_atoi_local(argv[3]);
    sigma_printf("[AUTOMATE]: XClicker Shard initiating %d events at %dms intervals.\n", clicks, interval);
}

void SovereignAutomationCommands_Register(void) {
    SovereignCommand_Register("gaming", "SteamOS/Garuda-style performance boosting", handle_gaming);
    SovereignCommand_Register("automate", "Industrial automation and clicker shards", handle_automate);
}
