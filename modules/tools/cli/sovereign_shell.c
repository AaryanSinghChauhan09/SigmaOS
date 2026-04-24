#include <stdint.h>
#include "sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Sovereign Shell (s-cli)
// Direct kernel interaction tool for debugging and system management
// ---------------------------------------------------------

extern void serial_write(const char* str); // Mock IO

// Mocks for external kernel state
extern uint32_t contract_count;
extern uint32_t proc_count;
extern uint32_t active_drivers;

static void print_banner(void) {
    serial_write("\n");
    serial_write("   _____ _                       ____  _____\n");
    serial_write("  / ____(_)                     / __ \\/ ____|\n");
    serial_write(" | (___  _  __ _ _ __ ___   __ | |  | | (___  \n");
    serial_write("  \\___ \\| |/ _` | '_ ` _ \\ / _` | |  | |\\___ \\ \n");
    serial_write("  ____) | | (_| | | | | | | (_| | |__| |____) |\n");
    serial_write(" |_____/|_|\\__, |_| |_| |_|\\__,_|\\____/|_____/ \n");
    serial_write("            __/ |                              \n");
    serial_write("           |___/     Sovereign Lattice Interface\n\n");
}

static void cmd_help(void) {
    serial_write("Available Commands:\n");
    serial_write("  help       - Show this menu\n");
    serial_write("  status     - Show global lattice health and metrics\n");
    serial_write("  caps       - Dump capability registry\n");
    serial_write("  contracts  - View active memory leases\n");
    serial_write("  tokens     - View sovereign resource economy ledger\n");
    serial_write("  ai_accel   - Check ML hardware accelerator state\n");
}

static void cmd_status(void) {
    serial_write("[*] System Status: ONLINE (TPM Verified)\n");
    serial_write("[*] Active Processes: 1 (s-cli)\n");
    serial_write("[*] Loaded Capsules: 14\n");
    serial_write("[*] Active Policies: strict_security, round_robin_scheduler\n");
}

static void cmd_contracts(void) {
    serial_write("[*] Memory Contracts Ledger:\n");
    serial_write("  ID | Lessee PID | Pages | Expiry | Status\n");
    serial_write("  -----------------------------------------\n");
    serial_write("  0  | 0 (kernel) | 1024  | NEVER  | ACTIVE\n");
    serial_write("  1  | 1 (s-cli)  | 16    | 99999  | ACTIVE\n");
}

static void cmd_tokens(void) {
    serial_write("[*] Sovereign Tokens Economy:\n");
    serial_write("  PID | Type  | Balance | Status\n");
    serial_write("  ------------------------------\n");
    serial_write("  1   | CPU   | 50000ns | ACTIVE\n");
    serial_write("  1   | MEM   | 16 pgs  | ACTIVE\n");
}

static void cmd_ai_accel(void) {
    serial_write("[*] AI/ML Accelerator Status:\n");
    serial_write("  -> NPU 0: Online (Zero-Copy Buffer Ready)\n");
    serial_write("  -> Active Tensors: 0\n");
}

// Very basic command loop mock
void shell_main(void) {
    print_banner();
    cmd_help();
    
    // In a real environment, this would read from keyboard input
    // For simulation, we run a set of commands automatically
    
    serial_write("\ns-cli> status\n");
    cmd_status();
    
    serial_write("\ns-cli> contracts\n");
    cmd_contracts();

    serial_write("\ns-cli> tokens\n");
    cmd_tokens();

    serial_write("\ns-cli> ai_accel\n");
    cmd_ai_accel();
    
    serial_write("\ns-cli> _\n");
}
