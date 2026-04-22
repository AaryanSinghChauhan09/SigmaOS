#include "sigma_libc.h"
#include "sigma_types.h"

/**
 * SigmaOS Sovereign Unified Driver Interface (SUDI)
 * Subsystem: S04 (HAL)
 * Mission: Zero-frustration hardware compatibility via universal probe and wrap.
 */

typedef enum {
    SUDI_BUS_PCI,
    SUDI_BUS_USB,
    SUDI_BUS_NVME,
    SUDI_BUS_GPIO,
    SUDI_BUS_VIRTIO
} SUDIBusType;

typedef struct {
    char        name[32];
    SUDIBusType bus;
    sigma_u16   vendor_id;
    sigma_u16   device_id;
    void*       priv_data;
    sigma_bool  is_active;
} SUDIDevice;

static SUDIDevice driver_registry[256];
static uint32_t device_count = 0;

void sudi_register_device(const char* name, SUDIBusType bus, uint16_t vid, uint16_t did) {
    if (device_count >= 256) return;
    
    SUDIDevice* dev = &driver_registry[device_count++];
    sigma_strncpy(dev->name, name, 31);
    dev->bus = bus;
    dev->vendor_id = vid;
    dev->device_id = did;
    dev->is_active = SIGMA_TRUE;
    
    sigma_printf("S04 [HAL]: Device registered - %s (VID:0x%X, DID:0x%X) on Bus:%d\n", name, vid, did, bus);
}

void sudi_probe_all(void) {
    sigma_printf("S04 [HAL]: Initiating Sovereign Hardware Probe...\n");
    // Mock probing - Silicon detection
    sudi_register_device("Sovereign_GPU_Alpha", SUDI_BUS_PCI, 0x10DE, 0x1DB6); // NVIDIA dummy
    sudi_register_device("Sovereign_WLAN_Link", SUDI_BUS_PCI, 0x8086, 0x2723); // Intel AX200 dummy
}

void S04_Register_SUDI(void) {
    sigma_printf("S04 [HAL]: Sovereign Unified Driver Interface Initialized.\n");
    sudi_probe_all();
}
