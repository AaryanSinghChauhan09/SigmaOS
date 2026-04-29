/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PREDICTIVE UX (S-PREDICTUX)
 * =========================================================================
 * Mission: Pre-loads UI assets and shards into memory based on predictive
 * user behavior models, achieving negative-latency user experiences.
 * =========================================================================
 */

#ifndef SIGMA_PREDICTUX_H
#define SIGMA_PREDICTUX_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- Predictive UX Primitives --- */
void predictux_init(void);
void predictux_record_interaction(uint32_t widget_id);
void predictux_preload_predicted_assets(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PREDICTUX_H */
