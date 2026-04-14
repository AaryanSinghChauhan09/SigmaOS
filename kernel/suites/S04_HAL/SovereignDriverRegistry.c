#include "../../include/sigma_base.h"

#include "../include/SovereignDriver.h"
#include "../include/sigma_libc.h"
#include "../include/sigma_string.h"

static SigmaDevice_t s_devices[MAX_DEVICES];
static sigma_u32     s_dev_count = 0;
static SigmaDriver_t s_drivers[MAX_DRIVERS];
static sigma_u32     s_drv_count = 0;

void SovereignDriver_InitRegistry(void) {
    sigma_memset(s_devices, 0, sizeof(s_devices));
    sigma_memset(s_drivers, 0, sizeof(s_drivers));
    s_dev_count = 0;
    s_drv_count = 0;
    sigma_printf("Σ [DRV]: Sovereign Driver Registry Initialised.\n");
}

sigma_err_t sigma_driver_register(const char* name, SigmaBusType_t bus, sigma_u32 vendor, sigma_u32 device, SigmaDriverProbe_t probe, SigmaDriverRemove_t remove) {
    if (s_drv_count >= MAX_DRIVERS) return SIGMA_ENOSPC;

    SigmaDriver_t* d = &s_drivers[s_drv_count++];
    sigma_strncpy(d->name, name, DEVICE_NAME_LEN);
    d->bus       = bus;
    d->vendor_id = vendor;
    d->device_id = device;
    d->probe     = probe;
    d->remove    = remove;
    d->in_use    = SIGMA_TRUE;
    
    sigma_printf("Σ [DRV]: Registered driver '%s'\n", name);
    return SIGMA_OK;
}

sigma_err_t sigma_device_register(const char* name, SigmaBusType_t bus, SigmaDevType_t type, sigma_u32 vendor, sigma_u32 device, sigma_u32 irq, sigma_u64 mmio_base, sigma_u64 mmio_size) {
    if (s_dev_count >= MAX_DEVICES) return SIGMA_ENOSPC;

    SigmaDevice_t* dev = &s_devices[s_dev_count++];
    sigma_strncpy(dev->name, name, DEVICE_NAME_LEN);
    dev->bus       = bus;
    dev->type      = type;
    dev->vendor_id = vendor;
    dev->device_id = device;
    dev->irq       = irq;
    dev->mmio_base = mmio_base;
    dev->mmio_size = mmio_size;
    dev->in_use    = SIGMA_TRUE;
    dev->powered   = SIGMA_TRUE;

    sigma_printf("Σ [BUS]: Device '%s' registered [VID=%x DID=%x MMIO=%p IRQ=%u]\n", name, vendor, device, (void*)mmio_base, irq);

    /* Auto-probe matching driver */
    for (sigma_u32 i = 0; i < s_drv_count; i++) {
        SigmaDriver_t* drv = &s_drivers[i];
        if (drv->bus == bus && (drv->vendor_id == 0 || drv->vendor_id == vendor) && (drv->device_id == 0 || drv->device_id == device)) {
            if (drv->probe && sigma_ok(drv->probe(dev))) {
                dev->driver = drv;
                sigma_printf("Σ [BUS]: Bound '%s' -> driver '%s'\n", name, drv->name);
                break;
            }
        }
    }
    return SIGMA_OK;
}

