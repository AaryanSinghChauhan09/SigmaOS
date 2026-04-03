/* 
 Σ SIGMAOS ZENITH: SOVEREIGN SIGNAL DISPATCHER (v2400.0)
 Mission: Catch-Identify-Redirect Hardware Violations.
*/

#include <stdint.h>
#include <stdbool.h>
#include "SigmaSovereignInternal.h"

// Σ SIGNAL TYPES
#define SIG_SEGV 11 // Segmentation Fault
#define SIG_ILL  4  // Illegal Instruction
#define SIG_FPE  8  // Floating Point Exception

// Σ SIGNAL HANDLER REGISTRY
typedef void (*sigma_sig_handler)(int);
static sigma_sig_handler g_SignalTable[32];

// Σ CORE DISPATCHER
void sigma_signal_dispatch(int signum) {
    if (g_SignalTable[signum]) {
        g_SignalTable[signum](signum);
    } else {
        sigma_print("\nΣ [SIGNAL]: Unhandled Silicon Violation: ");
        // sigma_print_int(signum);
        sigma_print("\n");
        while(1);
    }
}

// Σ SIGNAL REGISTRATION
void sigma_signal_register(int signum, sigma_sig_handler handler) {
    if (signum < 32) g_SignalTable[signum] = handler;
}
