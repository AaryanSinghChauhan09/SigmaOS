/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SMART CLIPBOARD (S-CLIPBOARD)
 * =========================================================================
 * Mission: A context-aware, globally synchronized clipboard providing
 * seamless data transfer across the entire Sovereign Lattice.
 * =========================================================================
 */

#ifndef SIGMA_CLIPBOARD_H
#define SIGMA_CLIPBOARD_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    CLIP_TYPE_TEXT,
    CLIP_TYPE_IMAGE,
    CLIP_TYPE_FILE_REF
} sigma_clip_type_t;

/* --- Clipboard Primitives --- */
void clipboard_init(void);
void clipboard_copy(sigma_clip_type_t type, const void* data, uint32_t size);
void* clipboard_paste(sigma_clip_type_t* out_type, uint32_t* out_size);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CLIPBOARD_H */
