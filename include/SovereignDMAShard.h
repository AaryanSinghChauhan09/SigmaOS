/* =========================================================================
 * Σ SIGMAOS: SOVEREIGN DMA SHARD HEADER
 * ========================================================================= */
#ifndef SOVEREIGN_DMA_SHARD_H
#define SOVEREIGN_DMA_SHARD_H
#include "sigma_types.h"
typedef enum { DMA_PROT_NONE=0, DMA_PROT_READ=1, DMA_PROT_WRITE=2, DMA_PROT_RW=3 } SigmaDMAProt_t;
sigma_err_t sigma_dma_map               (const char* bdf, sigma_u64 iova, sigma_u64 pa,
                                          sigma_u64 size, SigmaDMAProt_t prot);
sigma_err_t sigma_dma_quarantine        (const char* bdf);
void        sigma_dma_integrity_sweep   (void);
void        SovereignDMAShard_Init      (void);
void        SovereignDMA_Audit          (void);
#endif
