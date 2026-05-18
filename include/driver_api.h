#ifndef INCLUDE_DRIVER_API_H
#define INCLUDE_DRIVER_API_H

#include "../sigma_libc.h"

typedef enum {
    DEV_WIFI,
    DEV_PRINTER,
    DEV_USB,
    DEV_IOT
} device_type_t;

typedef struct {
    sigma_u32 (*init)(void);
    sigma_u32 (*read)(void *buf, sigma_u32 len);
    sigma_u32 (*write)(const void *buf, sigma_u32 len);
    sigma_u32 (*shutdown)(void);
} driver_t;

#ifdef __cplusplus
extern "C" {
#endif

void register_driver(device_type_t type, const driver_t *drv);
const driver_t *get_driver(device_type_t type);

#ifdef __cplusplus
}
#endif

#endif // INCLUDE_DRIVER_API_H
