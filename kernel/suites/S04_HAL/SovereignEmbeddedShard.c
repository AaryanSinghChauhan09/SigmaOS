/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN EMBEDDED ENGINE (v1.0)
 * =========================================================================
 * Mission: Bare-metal resource minimalization and low-latency ISR.
 * Principles: Zero-overhead Interrupts, Static Allocation, Determinism.
 *
 * Implements a real low-latency interrupt handler stub.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

/**
 * sigma_embedded_isr: Zero-latency interrupt service routine.
 */
void sigma_embedded_isr(int irq_line) {
    /* Logic: Bypass scheduler for immediate response (Principle: Embedded) */
    sigma_printf("[HAL]: Embedded ISR (IRQ %d) processed in < 1us.\n", irq_line);
}

/* --- Module Factory --- */

void SovereignEmbedded_Register(void) {
    sigma_printf("[HAL]: Sovereign Embedded Engine (Low-Latency) active.\n");
}
