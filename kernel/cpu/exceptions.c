/* 
 Σ SIGMAOS ZENITH: SOVEREIGN EXCEPTION HANDLER (v2300.0)
 Mission: Graceful Hardware-Direct Error Mitigation.
*/

#include <stdint.h>
#include "SigmaSovereignInternal.h"

// Σ EXCEPTION FRAME
typedef struct {
    uint64_t ip;
    uint64_t cs;
    uint64_t flags;
    uint64_t sp;
    uint64_t ss;
} sigma_exception_frame;

// Σ ZENITH PANIC: HARDWARE-DIRECT ERROR REPORTER
void sigma_zenith_panic(const char* mission_error, sigma_exception_frame* frame) {
    sigma_print("\n====================================================\n");
    sigma_print("Σ SIGMAOS ZENITH PANIC: MISSION CRITICALLY BREACHED\n");
    sigma_print("====================================================\n");
    sigma_print("ERROR: "); sigma_print(mission_error); sigma_print("\n");
    sigma_print("IP:    0x"); // Print IP hex (placeholder)
    sigma_print("\nSYSTEM HALTED. SOVEREIGNTY MAINTAINED.\n");
    while(1) { __asm__("hlt"); }
}

// Σ PAGE FAULT HANDLER (#PF)
void sigma_exc_page_fault(sigma_exception_frame* frame, uint64_t error_code) {
    sigma_zenith_panic("Page Fault Violation at 0x00... [CRITICAL]", frame);
}

// Σ GENERAL PROTECTION FAULT (#GP)
void sigma_exc_gp_fault(sigma_exception_frame* frame, uint64_t error_code) {
    sigma_zenith_panic("General Protection Fault: Silicon Access Denied.", frame);
}
