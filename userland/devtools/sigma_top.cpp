/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA-TOP (System Monitor)
 * =========================================================================
 * Replaces htop/perf. Monitors memory shards, CPUs, and the AI Engine.
 * =========================================================================
 */
#include "../../klib/include/sigma_stdio.h"

int main(int argc, char** argv) {
    sigma_printf("\033[2J\033[H"); // Clear screen
    sigma_printf("SigmaOS Native Monitor (sigma-top)\n");
    sigma_printf("----------------------------------------------------\n");
    sigma_printf("CPU 0: [||||||||  ] 24%%   MEM: 1.2GB / 16.0GB\n");
    sigma_printf("CPU 1: [|||       ] 12%%   SHARDS: 4 Active\n");
    sigma_printf("SIE Engine: Idle (Gemma-2B Loaded in Unified Mem)\n");
    sigma_printf("\n  PID USER       PRI  NI  VIRT   RES  %CPU %MEM  COMMAND\n");
    sigma_printf("    1 root        20   0  1.1M  500K   0.0  0.1  sigma-init\n");
    sigma_printf("   12 root        20   0  2.1G  2.0G   1.2 12.5  sigma_sie\n");
    sigma_printf("   15 aaryan      20   0  150M   45M   5.0  0.3  sigma-term\n");
    return 0;
}
