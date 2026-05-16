/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN EYE TRACKING (S-EYETRACK)
 * =========================================================================
 * Mission: Utmost accessibility and hands-free control via highly 
 * accurate, silicon-accelerated pupillary tracking.
 * =========================================================================
 */

#ifndef SIGMA_EYETRACK_H
#define SIGMA_EYETRACK_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Eye Tracking Primitives --- */
void eyetrack_init(void);
void eyetrack_process_frame(const void* frame_data);
void eyetrack_calibrate(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_EYETRACK_H */
