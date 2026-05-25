#ifndef SOVEREIGN_DDK_API_H
#define SOVEREIGN_DDK_API_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DRIVER DEVELOPMENT KIT (DDK)
 * =========================================================================
 * Capability-gated, formally verifiable driver framework.
 * No POSIX dependencies. Direct integration with Sovereign HAL.
 * =========================================================================
 */

// Error codes
#define SIGMA_DDK_OK 0
#define SIGMA_DDK_ERR_INVAL -1
#define SIGMA_DDK_ERR_NO_MEM -2
#define SIGMA_DDK_ERR_CAP_DENIED -3

typedef int sigma_status_t;

// Driver Metadata
typedef struct {
    const char* driver_name;
    uint32_t driver_version;
    uint32_t device_class;
} sigma_driver_info_t;

// Capability Token (Opaque)
typedef struct sigma_cap_token sigma_cap_token_t;

// -------------------------------------------------------------------------
// Core DDK API
// -------------------------------------------------------------------------

/**
 * Register a driver shard with the kernel HAL registry.
 */
sigma_status_t sigma_register_driver(const sigma_driver_info_t* info, sigma_cap_token_t* token);

/**
 * Allocate physically contiguous, cache-coherent memory for DMA.
 * Requires DMA capability token.
 */
void* sigma_alloc_dma_region(size_t size, sigma_cap_token_t* token, uint64_t* out_phys_addr);

/**
 * Bind an interrupt vector to a specific hardware handler.
 * Includes formal priority checking against other drivers.
 */
sigma_status_t sigma_irq_install(uint32_t vector, void (*handler)(void), sigma_cap_token_t* token);

/**
 * Yield the driver shard CPU time back to the Sovereign Scheduler.
 */
void sigma_driver_yield(void);

#ifdef __cplusplus
}
#endif

#endif // SOVEREIGN_DDK_API_H
