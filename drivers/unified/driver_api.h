/*
 * =========================================================================
 * SigmaOS: Unified Driver API (driver_api.h)
 * =========================================================================
 * Abstract driver interface. Each driver registers at boot via
 * driver_register(); the kernel locates drivers via driver_get().
 * =========================================================================
 */
#ifndef SIGMA_DRIVER_API_H
#define SIGMA_DRIVER_API_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ── Device class identifiers ───────────────────────────────────────── */
typedef enum {
    DEV_WIFI     = 0,
    DEV_PRINTER  = 1,
    DEV_USB      = 2,
    DEV_IOT      = 3,
    DEV_MAX      = 4
} device_type_t;

/* ── Driver operation table ─────────────────────────────────────────── */
typedef struct driver_ops {
    const char* name;
    sigma_u32 (*init)    (void);
    sigma_u32 (*open)    (sigma_u32 flags);
    sigma_u32 (*close)   (void);
    sigma_u32 (*read)    (void* buf, sigma_u32 len);
    sigma_u32 (*write)   (const void* buf, sigma_u32 len);
    sigma_u32 (*ioctl)   (sigma_u32 cmd, sigma_u64 arg);
    sigma_u32 (*shutdown)(void);
} driver_ops_t;

/* ── Registry API ───────────────────────────────────────────────────── */
/*
 * driver_register — called by each driver at boot (or via constructor).
 * @type : device class
 * @ops  : pointer to a statically allocated driver_ops_t
 * Returns K_OK on success, K_ERR_INVAL if type >= DEV_MAX.
 */
sigma_i32 driver_register(device_type_t type, const driver_ops_t* ops);

/*
 * driver_get — retrieve a registered driver by device class.
 * Returns NULL if no driver is registered for the type.
 */
const driver_ops_t* driver_get(device_type_t type);

/*
 * driver_init_all — call init() on every registered driver.
 * Invoked once from kernel boot sequence.
 */
void driver_init_all(void);

/* ── Convenience macro to self-register on startup ──────────────────── */
#define REGISTER_DRIVER(type, ops_ptr) \
    static void __attribute__((constructor)) _drv_reg_##type(void) { \
        driver_register((type), (ops_ptr)); \
    }

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DRIVER_API_H */
