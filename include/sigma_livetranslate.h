/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LIVE TRANSLATE (S-LIVETRANSLATE)
 * =========================================================================
 * Mission: Real-time, fully offline translation of text, UI labels, and
 * voice input across 50+ languages using on-device neural inference.
 * =========================================================================
 */

#ifndef SIGMA_LIVETRANSLATE_H
#define SIGMA_LIVETRANSLATE_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Live Translate Primitives --- */
void livetranslate_init(void);
const char* livetranslate_text(const char* input, const char* from_lang, const char* to_lang);
void livetranslate_overlay_ui(const char* target_lang);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LIVETRANSLATE_H */
