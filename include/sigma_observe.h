/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OBSERVABILITY MATRIX (S-OBSERVE)
 * =========================================================================
 * Mission: Safe, in-kernel programmable observability and tracing, 
 * matching the power of eBPF and DTrace without external dependencies.
 * =========================================================================
 */

#ifndef SIGMA_OBSERVE_H
#define SIGMA_OBSERVE_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t probe_id;
    char target_symbol[64];
    bool is_active;
} sigma_observe_probe_t;

/* --- Observability Primitives --- */
void observe_init(void);
bool observe_attach_probe(const char* symbol, void (*callback)(void));
void observe_trigger_probe(uint32_t probe_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_OBSERVE_H */
