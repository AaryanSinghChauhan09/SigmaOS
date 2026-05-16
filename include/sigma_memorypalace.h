/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN MEMORY PALACE (S-MEMORYPALACE)
 * =========================================================================
 * Mission: A revolutionary timeline-based file explorer that organizes
 * data by when and where it was used, completely replacing hierarchical folders.
 * =========================================================================
 */

#ifndef SIGMA_MEMORYPALACE_H
#define SIGMA_MEMORYPALACE_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Memory Palace Primitives --- */
void memorypalace_init(void);
void memorypalace_record_file_access(uint32_t file_id, uint64_t timestamp);
void memorypalace_query_timeline(uint64_t start_time, uint64_t end_time);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MEMORYPALACE_H */
