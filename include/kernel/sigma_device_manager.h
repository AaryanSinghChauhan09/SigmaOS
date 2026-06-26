/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN DEVICE MANAGER (v1.0)
 * =============================================================================
 * Mission: Hardware enumeration, hotplug detection, driver binding, and
 *          device tree management with parent-child relationships.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_DEVICE_MANAGER_H
#define SIGMA_DEVICE_MANAGER_H

#include "../sigma_kernel_types.h"

#define DEVMGR_MAX_DEVICES  128
#define DEVMGR_NAME_LEN      48
#define DEVMGR_DRIVER_LEN    32
#define DEVMGR_MAX_HOTPLUG   16

typedef enum {
    DEV_TYPE_BLOCK   = 0,   /* Storage: NVMe, SATA, USB mass storage */
    DEV_TYPE_CHAR    = 1,   /* Character: serial, terminal */
    DEV_TYPE_NET     = 2,   /* Network: Ethernet, WiFi */
    DEV_TYPE_GPU     = 3,   /* Graphics: Vulkan-capable GPUs */
    DEV_TYPE_INPUT   = 4,   /* Input: keyboard, mouse, touchpad */
    DEV_TYPE_USB     = 5,   /* USB hub / controller */
    DEV_TYPE_AUDIO   = 6,   /* Audio: sound cards */
    DEV_TYPE_SENSOR  = 7,   /* Sensor: IMU, temperature, light */
    DEV_TYPE_PLATFORM = 8   /* Platform: timers, interrupt controllers */
} sigma_dev_type_t;

typedef enum {
    DEV_STATUS_DETECTED     = 0,
    DEV_STATUS_DRIVER_BOUND = 1,
    DEV_STATUS_ACTIVE       = 2,
    DEV_STATUS_SUSPENDED    = 3,
    DEV_STATUS_FAILED       = 4,
    DEV_STATUS_REMOVED      = 5
} sigma_dev_status_t;

typedef enum {
    HOTPLUG_ARRIVAL = 0,
    HOTPLUG_REMOVAL = 1
} sigma_hotplug_event_type_t;

typedef struct {
    sigma_u32           id;
    char                name[DEVMGR_NAME_LEN];
    sigma_dev_type_t    type;
    sigma_dev_status_t  status;
    char                driver[DEVMGR_DRIVER_LEN];
    sigma_u32           parent_id;     /* 0 = root device */
    sigma_u16           vendor_id;
    sigma_u16           device_id;
    sigma_paddr_t       mmio_base;     /* memory-mapped I/O base */
    sigma_usize         mmio_size;
    sigma_u8            irq;           /* interrupt line */
    sigma_bool          hotpluggable;
} sigma_device_t;

typedef struct {
    sigma_hotplug_event_type_t event;
    sigma_u32                  device_id;
    sigma_u64                  timestamp;
} sigma_hotplug_event_t;

#ifdef __cplusplus
extern "C" {
#endif

void              devmgr_init(void);
void              devmgr_scan(void);
sigma_u32         devmgr_register_device(const char* name, sigma_dev_type_t type,
                                         sigma_u32 parent_id,
                                         sigma_u16 vendor_id, sigma_u16 device_id);
int               devmgr_bind_driver(sigma_u32 dev_id, const char* driver_name);
int               devmgr_unbind_driver(sigma_u32 dev_id);
int               devmgr_set_status(sigma_u32 dev_id, sigma_dev_status_t status);
const sigma_device_t* devmgr_get_device(sigma_u32 dev_id);
void              devmgr_print_tree(void);
sigma_u32         devmgr_get_device_count(void);
int               devmgr_hotplug_push(sigma_hotplug_event_type_t event,
                                       sigma_u32 dev_id);
int               devmgr_hotplug_pop(sigma_hotplug_event_t* out);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DEVICE_MANAGER_H */
