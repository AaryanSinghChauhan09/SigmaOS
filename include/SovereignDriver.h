/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DRIVER INTERFACE (v1.0)
 * =========================================================================
 * Mission: Unified device model and driver registry for hardware autonomy.
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_DRIVER_H
#define SOVEREIGN_DRIVER_H

#include "sigma_types.h"

#define MAX_DEVICES       256
#define MAX_DRIVERS        64
#define DEVICE_NAME_LEN    64

typedef enum {
    BUS_PLATFORM = 0,
    BUS_PCI,
    BUS_USB,
    BUS_I2C,
    BUS_SPI,
    BUS_VIRTIO
} SigmaBusType_t;

typedef enum {
    DEV_TYPE_BLOCK = 0,
    DEV_TYPE_CHAR,
    DEV_TYPE_NET,
    DEV_TYPE_USB,
    DEV_TYPE_GRAPHICS
} SigmaDevType_t;

typedef struct SigmaDevice SigmaDevice_t;
typedef struct SigmaDriver SigmaDriver_t;

typedef sigma_err_t (*SigmaDriverProbe_t)(SigmaDevice_t* dev);
typedef void        (*SigmaDriverRemove_t)(SigmaDevice_t* dev);

struct SigmaDriver {
    char                name[DEVICE_NAME_LEN];
    SigmaBusType_t      bus;
    sigma_u32           vendor_id;
    sigma_u32           device_id;
    SigmaDriverProbe_t  probe;
    SigmaDriverRemove_t remove;
    sigma_bool          in_use;
};

struct SigmaDevice {
    char             name[DEVICE_NAME_LEN];
    SigmaBusType_t   bus;
    SigmaDevType_t   type;
    sigma_u32        vendor_id;
    sigma_u32        device_id;
    sigma_u32        irq;
    sigma_u64        mmio_base;
    sigma_u64        mmio_size;
    SigmaDriver_t*   driver;
    void*            driver_data;
    sigma_bool       in_use;
    sigma_bool       powered;
};

/* Registry API */
void SovereignDriver_InitRegistry(void);
sigma_err_t sigma_driver_register(const char* name, SigmaBusType_t bus, sigma_u32 vendor, sigma_u32 device, SigmaDriverProbe_t probe, SigmaDriverRemove_t remove);
sigma_err_t sigma_device_register(const char* name, SigmaBusType_t bus, SigmaDevType_t type, sigma_u32 vendor, sigma_u32 device, sigma_u32 irq, sigma_u64 mmio_base, sigma_u64 mmio_size);

/* Subsystem Discovery */
void sigma_pcie_scan(void);

#endif /* SOVEREIGN_DRIVER_H */
