/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN FORENSIC SCRUBBER HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_FORENSIC_SCRUBBER_H
#define SOVEREIGN_FORENSIC_SCRUBBER_H

#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

typedef struct {
    SigmaObject_t core;
    sigma_u32     scrub_cycles;
    sigma_sz_t  total_bytes_sanitized;
} SovereignForensicScrubber_t;

SovereignForensicScrubber_t SovereignForensicScrubber_Create(void);
void sigma_scrub_memory_sector(SovereignForensicScrubber_t* self, void* sector, sigma_sz_t size);
void SovereignForensicScrubber_Audit(SovereignForensicScrubber_t* self);
void SovereignForensicScrubber_Init(void);

#endif /* SOVEREIGN_FORENSIC_SCRUBBER_H */
