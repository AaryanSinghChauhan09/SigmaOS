#ifndef SIGMA_CLAW_H
#define SIGMA_CLAW_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* 
 * Sigma-Claw API
 * Sovereign web-crawling primitives for zero-overhead data ingestion.
 */

typedef struct {
    const char* target_url;
    sigma_u32 timeout_ms;
    sigma_u8  extract_semantics; // If 1, parse text automatically and feed to SemanticFS
} sigma_crawl_task_t;

extern sigma_status sys_queue_crawl(sigma_crawl_task_t* task);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_CLAW_H
