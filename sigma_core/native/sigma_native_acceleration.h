/**
 * @file sigma_native_acceleration.h
 * @brief High-performance C headers for SigmaOS native cores.
 * 
 * Part of the initiative to move performance-critical shards to low-level languages.
 */

#ifndef SIGMA_NATIVE_H
#define SIGMA_NATIVE_H

#include <stdint.h>

/**
 * Executes a lattice-based cryptographic transform using SIMD instructions.
 */
int32_t sigma_lattice_transform(const uint8_t* input, uint8_t* output, uint32_t length);

/**
 * Direct hardware-level process scheduler.
 */
void sigma_hw_schedule(uint64_t task_id, uint8_t priority);

#endif // SIGMA_NATIVE_H
