#include "../../../include/libc/sigma_libc.h"
#include "../../../include/libc/sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Hardware Auto-Detection & Module Loader
// Self-configuring kernel logic (Sovereign udev/PCI/USB)
// ---------------------------------------------------------

#define MAX_DEVICES 128

typedef enum {
    HW_PCI,
    HW_USB,
    HW_PLATFORM,
    HW_ISA
} hw_bus_t;

typedef struct {
    hw_bus_t bus;
    uint32_t vendor_id;
    uint32_t device_id;
    char     compatible_module[32];
    uint8_t  found;
} hw_device_entry_t;

static hw_device_entry_t device_database[] = {
    {HW_PCI, 0x8086, 0x100E, "e1000_driver", 0}, // Intel PRO/1000
    {HW_PCI, 0x10EC, 0x8139, "rtl8139_driver", 0}, // Realtek 8139
    {HW_USB, 0x046D, 0xC077, "hid_mouse_driver", 0}, // Logitech Mouse
    {HW_PLATFORM, 0x0, 0x0, "virtio_net", 0} // QEMU VirtIO Net
};

#define DB_SIZE (sizeof(device_database) / sizeof(hw_device_entry_t))

extern int capsule_load_by_name(const char* name); // To be implemented/extended from module_loader

// Scans the hardware bus (Mock implementation)
void hw_scan_and_config() {
    // In real implementation, this would iterate over PCI config space
    // and USB descriptors.
    
    for (int i = 0; i < DB_SIZE; i++) {
        // Mock: say we found the first 2 devices
        if (i < 2) {
            device_database[i].found = 1;
            // Trigger capsule load
            // capsule_load_by_name(device_database[i].compatible_module);
        }
    }
}

// Get the name of a driver for a specific device
const char* hw_get_driver(uint32_t vendor, uint32_t device) {
    for (int i = 0; i < DB_SIZE; i++) {
        if (device_database[i].vendor_id == vendor && device_database[i].device_id == device) {
            return device_database[i].compatible_module;
        }
    }
    return SIGMA_NULL;
}
