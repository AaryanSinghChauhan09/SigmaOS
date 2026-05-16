/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN RUST INTEROP (S-RUSTINTEROP)
 * =========================================================================
 * Mission: Provide zero-overhead memory-safe Rust bindings for drivers,
 * positioning SigmaOS as a memory-safe, high-speed singularity.
 * =========================================================================
 */

#ifndef SIGMA_RUST_H
#define SIGMA_RUST_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Rust Interop Primitives --- */
void rust_interop_init(void);
void* rust_alloc_safe_buffer(uint32_t size);
void rust_free_safe_buffer(void* ptr);
bool rust_execute_safe_driver(uint32_t driver_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_RUST_H */
