#ifndef SIGMA_GPU_STACK_H
#define SIGMA_GPU_STACK_H

#include <stdint.h>

// SigmaOS Universal GPU Driver Stack
// Native hardware manipulation for modern graphics

// Auto-detect and initialize primary display adapter
void hal_gpu_init(void);

// Allocate VRAM for the hardware compositor
void* hal_gpu_alloc_vram(uint64_t size_bytes);

// Perform a hardware-accelerated bit-block transfer
void hal_gpu_bitblt(void* dest, void* src, uint32_t width, uint32_t height);

// Set power state for ACPI/battery monitor integrations
void hal_gpu_set_power_state(uint8_t power_level);

#endif // SIGMA_GPU_STACK_H
