#include "libc/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S04_HAL/shards/sigma_hal_core.c
 * =========================================================================
 * Sovereign Hardware Abstraction Layer (HAL) Core implementation.
 * =========================================================================
 */

#include "sigma_hal_core.h"
#include "S01_Genesis/shards/SovereignCommon.h"
#include "libc/sigma_libc.h"

#define MAX_HAL_DEVICES 256
static sigma_hal_device_t* s_hal_registry[MAX_HAL_DEVICES];
static sigma_u32 s_hal_device_count = 0;

void sigma_hal_init(void) {
    sigma_sigma_memset(s_hal_registry, 0, sizeof(s_hal_registry));
    s_hal_device_count = 0;
    sigma_sigma_printf("S [HAL] Sovereign Hardware Abstraction Layer Initialized.\n");
}

sigma_err_t sigma_hal_register_device(sigma_hal_device_t* device) {
    if (!device) return SIGMA_ERR_NULL;
    if (s_hal_device_count >= MAX_HAL_DEVICES) return SIGMA_ERR_AGAIN;

    device->base.id = ++s_hal_device_count;
    s_hal_registry[s_hal_device_count - 1] = device;

    const char* type_str = "UNKNOWN";
    switch(device->type) {
        case HAL_DEVICE_GPU: type_str = "GPU"; break;
        case HAL_DEVICE_WIFI: type_str = "WIFI"; break;
        case HAL_DEVICE_AUDIO: type_str = "AUDIO"; break;
        case HAL_DEVICE_PERIPHERAL: type_str = "PERIPHERAL"; break;
    }

    sigma_sigma_printf("S [HAL] Registered %s interface [%s] (ID: %u)\n", 
                             type_str, device->base.name, device->base.id);

    if (device->init) {
        return device->init(device);
    }
    return SIGMA_OK;
}

sigma_hal_device_t* sigma_hal_get_device(sigma_u32 id) {
    if (id == 0 || id > s_hal_device_count) return SIGMA_NULL;
    return s_hal_registry[id - 1];
}
