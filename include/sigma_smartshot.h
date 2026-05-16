/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN SMART SCREENSHOT (S-SMARTSHOT)
 * =========================================================================
 * Mission: An intelligent screenshot tool that auto-recognizes content,
 * performs instant OCR, and offers contextual actions (translate, share,
 * save to Memory Palace) at capture time.
 * =========================================================================
 */

#ifndef SIGMA_SMARTSHOT_H
#define SIGMA_SMARTSHOT_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Smart Screenshot Primitives --- */
void smartshot_init(void);
void smartshot_capture_region(uint32_t x, uint32_t y, uint32_t w, uint32_t h);
void smartshot_capture_fullscreen(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SMARTSHOT_H */
