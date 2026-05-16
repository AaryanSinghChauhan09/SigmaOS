/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN NEURAL SEARCH (S-NEURALSEARCH)
 * =========================================================================
 * Mission: Deeply integrated, instantaneous search of files, logic,
 * and processes using embedded tensor mathematics, eliminating indexing lag.
 * =========================================================================
 */

#ifndef SIGMA_NEURALSEARCH_H
#define SIGMA_NEURALSEARCH_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Neural Search Primitives --- */
void neuralsearch_init(void);
void neuralsearch_query(const char* natural_language_query);
void neuralsearch_index_shard(uint32_t shard_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NEURALSEARCH_H */
