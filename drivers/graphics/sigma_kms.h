/**
 * =========================================================================
 * Σ SIGMAOS: KMS PUBLIC HEADER
 * =========================================================================
 * Consumed by: Zenith compositor, power manager (sigma_power_manager.cpp)
 * =========================================================================
 */
#pragma once

#include "../../include/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/** Opaque framebuffer descriptor returned to Zenith. */
typedef struct {
    sigma_u64 phys_addr;    /**< Physical base (GART-mapped VRAM aperture) */
    sigma_u64 size_bytes;
    sigma_u32 pitch;        /**< Bytes per scanline (cache-aligned) */
    sigma_u32 width;
    sigma_u32 height;
    sigma_u8  bpp;          /**< 24 or 32 */
    sigma_u8  _pad[7];
} sigma_kms_fb_t;

/** Initialise KMS subsystem — call after sigma_pci_scan_bus(). */
sigma_status sigma_kms_init(void);

/** Return primary framebuffer (adapter 0). NULL if KMS not ready. */
const sigma_kms_fb_t* sigma_kms_get_primary_fb(void);

/** Switch adapter to a different mode. */
sigma_status sigma_kms_set_mode(sigma_u32 adapter_idx, sigma_u32 mode_idx);

/** DPMS power state: 0=ON 1=STANDBY 2=SUSPEND 3=OFF */
sigma_status sigma_kms_dpms(sigma_u32 adapter_idx, sigma_u8 state);

#ifdef __cplusplus
}
#endif
