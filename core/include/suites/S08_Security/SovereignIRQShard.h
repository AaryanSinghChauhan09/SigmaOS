/* =========================================================================
 * S SIGMAOS: SOVEREIGN IRQ SHARD HEADER
 * ========================================================================= */
#ifndef SOVEREIGN_IRQ_SHARD_H
#define SOVEREIGN_IRQ_SHARD_H
#include "sigma_types.h"
typedef enum { IRQ_EDGE, IRQ_LEVEL, IRQ_MSI, IRQ_MSI_X } SigmaIRQType_t;
sigma_err_t sigma_irq_register     (sigma_u32 irq, const char* dev, SigmaIRQType_t type, sigma_u32 cpu);
void        sigma_irq_balance       (void);
sigma_err_t sigma_irq_set_affinity  (sigma_u32 irq, sigma_u32 cpu);
void        SovereignIRQShard_Init  (void);
void        SovereignIRQ_Audit      (void);
#endif
