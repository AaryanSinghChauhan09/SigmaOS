/**
 * @file simulate_sovereign.c
 * @brief Zero-Dependency Simulator Entry.
 * 
 * Objective: Elimination of host-side 'libc' leakage.
 */

#include "sigma_libc.h"
#include "sigma_string.h"

/* No <stdio.h>, No <stdlib.h> */

extern void sigma_kmain(void);

int main() {
    // sigma_print instead of printf
    sigma_print("[SIMULATOR]: Starting SigmaOS Sovereign Simulation (Total Parity mode)...\n");
    
    // Call kernel entry directly
    sigma_kmain();
    
    return 0;
}
