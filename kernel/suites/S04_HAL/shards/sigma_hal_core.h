/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S04_HAL/shards/sigma_hal_core.h
 * =========================================================================
 * Sovereign Hardware Abstraction Layer (HAL)
 * Unifies GPU, Wi-Fi, and generic peripheral access under the SOOF.
 * =========================================================================
 */

#ifndef SIGMA_HAL_CORE_H
#define SIGMA_HAL_CORE_H

#include "../../S01_Genesis/shards/SovereignCommon.h"

typedef enum {
    HAL_DEVICE_GPU   = 1,
    HAL_DEVICE_WIFI  = 2,
    HAL_DEVICE_AUDIO = 3,
    HAL_DEVICE_PERIPHERAL = 4
} sigma_hal_device_type_t;

typedef struct {
    sigma_obj_t base; // SOOF Inheritance
    sigma_hal_device_type_t type;
    sigma_u32 pci_vendor_id;
    sigma_u32 pci_device_id;
    sigma_u64 mmio_base;
    sigma_err_t (*init)(void* self);
    sigma_err_t (*reset)(void* self);
    sigma_err_t (*power_state)(void* self, sigma_u8 state);
} sigma_hal_device_t;

/* GPU specific extension */
typedef struct {
    sigma_hal_device_t dev;
    sigma_u32 vram_size_mb;
    sigma_u32 max_compute_units;
    sigma_err_t (*submit_command_buffer)(void* self, sigma_u64 ptr_addr, sigma_u32 size);
} sigma_hal_gpu_t;

/* Wi-Fi specific extension */
typedef struct {
    sigma_hal_device_t dev;
    sigma_u32 max_bandwidth_mbps;
    sigma_u8 mac_addr[6];
    sigma_err_t (*scan_networks)(void* self);
    sigma_err_t (*connect_ap)(void* self, const char* ssid, const char* psk);
} sigma_hal_wifi_t;

/* Public API */
void sigma_hal_init(void);
sigma_err_t sigma_hal_register_device(sigma_hal_device_t* device);
sigma_hal_device_t* sigma_hal_get_device(sigma_u32 id);

#endif /* SIGMA_HAL_CORE_H */
