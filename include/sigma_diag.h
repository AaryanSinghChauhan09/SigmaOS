/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN SYSTEM DIAGNOSTICS (S-DIAG)
 * =========================================================================
 * Mission: Silicon-direct fault localization and machine-state debugging.
 * =========================================================================
 */

#ifndef SIGMA_DIAG_H
#define SIGMA_DIAG_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t component_id;
    uint32_t error_vector;
    uint32_t silicon_tick;
    bool is_critical;
} sigma_diag_event_t;

/* --- Diagnostic Primitives --- */
void diag_init(void);
void diag_report_fault(uint32_t component_id, uint32_t error_code);
void diag_localize_fault(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DIAG_H */
