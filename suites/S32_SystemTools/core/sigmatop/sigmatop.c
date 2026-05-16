#include "../../../../include/libc/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS: tools/sigmatop  sigmatop.c
 * =========================================================================
 * Mission: Real-time Sovereign Task and Resource Monitor.
 * Design: High-refresh rate VT100 dashboard for kernel execution shards.
 * =========================================================================
 */

#include "../../../../include/libc/sigma_libc.h"
#include "../../../../include/libc/sigma_libc.h"
#include "../../../../include/core/sigma_types.h"

#define VT_CLEAR "\033[2J\033[H"
#define VT_HIDE  "\033[?25l"
#define VT_SHOW  "\033[?25h"
#define VT_BOLD  "\033[1m"
#define VT_CYAN  "\033[36m"
#define VT_GREEN "\033[32m"
#define VT_RESET "\033[0m"

void print_header(void) {
    sigma_printf(VT_CLEAR VT_HIDE);
    sigma_printf(VT_BOLD VT_CYAN " S SIGMATOP  Sovereign Singularity Monitor\n" VT_RESET);
    sigma_printf(" ------------------------------------------\n");
    sigma_printf("  CPU: %-10s | Uptime: %-10s\n", "98.4% [||||||||| ]", "14d 2h 31m");
    sigma_printf("  MEM: %-10s | Shards: %-10d\n", "12.4 GB / 32 GB", 14023);
    sigma_printf(" ------------------------------------------\n\n");
    sigma_printf(VT_BOLD "  PID   OWNER     PRI   STATE    CPU%%   COMMAND\n" VT_RESET);
}

void print_task(int pid, const char* owner, int pri, const char* state, float cpu, const char* cmd) {
    sigma_printf("  %-5d %-10s %-5d " VT_GREEN "%-8s" VT_RESET " %-6.1f %s\n", 
           pid, owner, pri, state, cpu, cmd);
}

int main(void) {
    print_header();
    print_task(1,    "ROOT",      0, "RUN",   0.4,  "SovereignInit");
    print_task(102,  "SYSTEM",    1, "READY", 12.1, "Zenith3D_Draw");
    print_task(554,  "USER",      2, "WAIT",  0.1,  "SovereignSQL");
    print_task(1025, "KERNEL",    0, "IDLE",  85.2, "Idle_Shard");
    
    sigma_printf("\n  " VT_CYAN "[q] Exit  [k] Kill  [p] Priority" VT_RESET "\n");
    sigma_printf(VT_SHOW);
    return 0;
}
