/*
 * Σ SigmaOS — sigma_driver_fw: Universal Sovereign Driver API
 * Zero-Dependency: No libc.
 * Replaces hardcoded driver structures with a plug-and-play metadata-driven framework.
 */

typedef unsigned int   u32;
typedef unsigned short u16;
typedef unsigned char  u8;

/* Device Classes */
#define SIGMA_DEV_CLASS_STORAGE 0x01
#define SIGMA_DEV_CLASS_NETWORK 0x02
#define SIGMA_DEV_CLASS_DISPLAY 0x03
#define SIGMA_DEV_CLASS_INPUT   0x04

/* Sovereign Driver Metadata (Self-Describing) */
struct SigmaDriverMetadata {
    const char* name;
    u32         device_class;
    u16         supported_vendor_id;
    u16         supported_device_id;
    
    /* Standardized Lifecycle API */
    int (*init)(void* hardware_context);
    int (*shutdown)();
    int (*reset)();
};

#define MAX_DRIVERS 64
static SigmaDriverMetadata* driver_registry[MAX_DRIVERS];
static u32 driver_count = 0;

/*
 * Register a driver at boot or runtime
 */
extern "C" int sigma_register_driver(SigmaDriverMetadata* meta) {
    if (driver_count >= MAX_DRIVERS) return -1;
    driver_registry[driver_count++] = meta;
    return 0;
}

/*
 * Match a hardware ID (e.g., from PCI scan) to an installed driver
 */
extern "C" SigmaDriverMetadata* sigma_find_driver(u16 vendor_id, u16 device_id) {
    for (u32 i = 0; i < driver_count; i++) {
        if (driver_registry[i]->supported_vendor_id == vendor_id &&
            driver_registry[i]->supported_device_id == device_id) {
            return driver_registry[i];
        }
    }
    return 0; /* No driver found */
}
