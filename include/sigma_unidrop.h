/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN UNIVERSAL DRAG & DROP (S-UNIDROP)
 * =========================================================================
 * Mission: A seamless drag-and-drop layer that allows moving data across
 * process boundaries, microVMs, and even remote paired devices instantly.
 * =========================================================================
 */

#ifndef SIGMA_UNIDROP_H
#define SIGMA_UNIDROP_H

#include "sigma_types.h"
#include "sigma_clipboard.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- UniDrop Primitives --- */
void unidrop_init(void);
void unidrop_begin_drag(uint32_t source_app_id, sigma_clip_type_t data_type, const void* data_ptr, uint32_t size);
void unidrop_update_cursor(uint32_t x, uint32_t y);
bool unidrop_commit_drop(uint32_t target_app_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_UNIDROP_H */
