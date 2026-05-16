/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN VISUAL SEARCH (S-VISSEARCH)
 * =========================================================================
 * Mission: Native, OS-level visual parsing and indexing of all images,
 * screenshots, and video frames via the Neural Engine for instant OCR.
 * =========================================================================
 */

#ifndef SIGMA_VISSEARCH_H
#define SIGMA_VISSEARCH_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Visual Search Primitives --- */
void vissearch_init(void);
void vissearch_index_image(const void* pixel_data, uint32_t width, uint32_t height);
void vissearch_query_visual_data(const char* search_term);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VISSEARCH_H */
