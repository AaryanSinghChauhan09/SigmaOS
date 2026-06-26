/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DEVICE MANAGER (v1.0)
 * =========================================================================
 * Hardware enumeration, hotplug event queuing, driver binding, and
 * parent-child device tree management.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_device_manager.h"

namespace SigmaOS {
namespace Kernel {

class SovereignDeviceManager {
public:
    static SovereignDeviceManager& getInstance() {
        static SovereignDeviceManager instance;
        return instance;
    }

    void init() {
        m_device_count = 0;
        m_hotplug_head = 0;
        m_hotplug_tail = 0;
        m_hotplug_count = 0;

        for (sigma_u32 i = 0; i < DEVMGR_MAX_DEVICES; i++) {
            m_devices[i].id = 0;
        }

        sigma_log("[DEVMGR] Sovereign Device Manager initialized.");

        /* Create root platform device */
        sigma_u32 root_id = registerDevice("sigma-platform", DEV_TYPE_PLATFORM, 0, 0x0000, 0x0000);
        
        /* Simulated initial scan */
        sigma_u32 pci_id = registerDevice("pci-bus", DEV_TYPE_PLATFORM, root_id, 0x8086, 0x0000);
        registerDevice("nvme-0", DEV_TYPE_BLOCK, pci_id, 0x144D, 0xA80A); /* Samsung NVMe */
        registerDevice("eth-0",  DEV_TYPE_NET,   pci_id, 0x8086, 0x15F2); /* Intel I225-V */
        
        sigma_u32 usb_id = registerDevice("usb-xhci", DEV_TYPE_USB, pci_id, 0x8086, 0x9D2F);
        registerDevice("usb-kbd", DEV_TYPE_INPUT, usb_id, 0x046D, 0xC31C); /* Logitech Keyboard */

        sigma_log_info("[DEVMGR] Initial scan complete. %u devices found.\n", m_device_count);
    }

    void scan() {
        sigma_log("[DEVMGR] Initiating full hardware scan (PCI/USB/ACPI)...");
        /* In a real kernel, this would traverse ACPI tables and PCI config space */
        sigma_log("[DEVMGR] Scan complete.");
    }

    sigma_u32 registerDevice(const char* name, sigma_dev_type_t type,
                             sigma_u32 parent_id,
                             sigma_u16 vendor_id, sigma_u16 device_id) {
        if (m_device_count >= DEVMGR_MAX_DEVICES) return 0;

        sigma_u32 id = m_device_count + 1;
        sigma_device_t& dev = m_devices[m_device_count];
        dev.id = id;
        sigma_strncpy(dev.name, name, DEVMGR_NAME_LEN);
        dev.type = type;
        dev.status = DEV_STATUS_DETECTED;
        dev.driver[0] = '\0';
        dev.parent_id = parent_id;
        dev.vendor_id = vendor_id;
        dev.device_id = device_id;
        dev.mmio_base = 0;
        dev.mmio_size = 0;
        dev.irq = 0;
        dev.hotpluggable = (type == DEV_TYPE_USB || type == DEV_TYPE_BLOCK);
        
        m_device_count++;
        
        /* Push arrival event */
        hotplugPush(HOTPLUG_ARRIVAL, id);
        
        return id;
    }

    int bindDriver(sigma_u32 dev_id, const char* driver_name) {
        sigma_device_t* dev = findDevice(dev_id);
        if (!dev) return K_ERR_NOTFOUND;

        sigma_strncpy(dev->driver, driver_name, DEVMGR_DRIVER_LEN);
        dev->status = DEV_STATUS_DRIVER_BOUND;
        sigma_log_info("[DEVMGR] Bound driver '%s' to device '%s'\n", driver_name, dev->name);
        
        /* Auto-activate */
        dev->status = DEV_STATUS_ACTIVE;
        return K_OK;
    }

    int unbindDriver(sigma_u32 dev_id) {
        sigma_device_t* dev = findDevice(dev_id);
        if (!dev) return K_ERR_NOTFOUND;

        sigma_log_info("[DEVMGR] Unbound driver '%s' from device '%s'\n", dev->driver, dev->name);
        dev->driver[0] = '\0';
        dev->status = DEV_STATUS_DETECTED;
        return K_OK;
    }

    int setStatus(sigma_u32 dev_id, sigma_dev_status_t status) {
        sigma_device_t* dev = findDevice(dev_id);
        if (!dev) return K_ERR_NOTFOUND;
        dev->status = status;
        return K_OK;
    }

    const sigma_device_t* getDevice(sigma_u32 dev_id) {
        return findDevice(dev_id);
    }

    void printTree() {
        sigma_log("\n╔═══════════════════════════════════════════════════════════════╗");
        sigma_log("║              SOVEREIGN DEVICE TREE                          ║");
        sigma_log("╠══════╦═════════════════╦════════╦═════════════════╦═══════════╣");
        sigma_log("║  ID  ║ Name            ║ VID:PID║ Driver          ║ Status    ║");
        sigma_log("╠══════╬═════════════════╬════════╬═════════════════╬═══════════╣");

        /* Print root devices (parent_id == 0) and recurse */
        for (sigma_u32 i = 0; i < m_device_count; i++) {
            if (m_devices[i].parent_id == 0) {
                printNode(m_devices[i].id, 0);
            }
        }
        sigma_log("╚══════╩═════════════════╩════════╩═════════════════╩═══════════╝");
    }

    sigma_u32 getDeviceCount() const { return m_device_count; }

    int hotplugPush(sigma_hotplug_event_type_t event, sigma_u32 dev_id) {
        if (m_hotplug_count >= DEVMGR_MAX_HOTPLUG) return K_ERR_BUSY;

        sigma_hotplug_event_t& ev = m_hotplug_queue[m_hotplug_tail];
        ev.event = event;
        ev.device_id = dev_id;
        ev.timestamp = cpu_rdtsc();

        m_hotplug_tail = (m_hotplug_tail + 1) % DEVMGR_MAX_HOTPLUG;
        m_hotplug_count++;
        return K_OK;
    }

    int hotplugPop(sigma_hotplug_event_t* out) {
        if (m_hotplug_count == 0) return K_ERR_NOTFOUND;

        *out = m_hotplug_queue[m_hotplug_head];
        m_hotplug_head = (m_hotplug_head + 1) % DEVMGR_MAX_HOTPLUG;
        m_hotplug_count--;
        return K_OK;
    }

private:
    SovereignDeviceManager() : m_device_count(0), m_hotplug_head(0),
                               m_hotplug_tail(0), m_hotplug_count(0) {}

    sigma_device_t* findDevice(sigma_u32 id) {
        if (id == 0 || id > m_device_count) return SIGMA_NULL;
        return &m_devices[id - 1];
    }

    void printNode(sigma_u32 dev_id, int depth) {
        sigma_device_t* dev = findDevice(dev_id);
        if (!dev) return;

        char indent[16] = {0};
        for (int i = 0; i < depth && i < 15; i++) indent[i] = ' ';
        indent[depth] = '\0';

        char name_buf[32];
        /* Very basic snprintf equivalent for log alignment */
        int len = 0;
        for (int i=0; indent[i]; i++) name_buf[len++] = indent[i];
        if (depth > 0) { name_buf[len++] = 'L'; name_buf[len++] = '-'; }
        for (int i=0; dev->name[i] && len < 31; i++) name_buf[len++] = dev->name[i];
        name_buf[len] = '\0';

        const char* status_str = "UNKNOWN";
        switch (dev->status) {
            case DEV_STATUS_DETECTED:     status_str = "DETECTED"; break;
            case DEV_STATUS_DRIVER_BOUND: status_str = "BOUND"; break;
            case DEV_STATUS_ACTIVE:       status_str = "ACTIVE"; break;
            case DEV_STATUS_SUSPENDED:    status_str = "SUSPENDED"; break;
            case DEV_STATUS_FAILED:       status_str = "FAILED"; break;
            case DEV_STATUS_REMOVED:      status_str = "REMOVED"; break;
        }

        sigma_log_info("║ %4u ║ %-15s ║ %04x:%04x║ %-15s ║ %-9s ║\n",
                       dev->id, name_buf, dev->vendor_id, dev->device_id,
                       dev->driver[0] ? dev->driver : "none", status_str);

        /* Find and print children */
        for (sigma_u32 i = 0; i < m_device_count; i++) {
            if (m_devices[i].parent_id == dev_id) {
                printNode(m_devices[i].id, depth + 1);
            }
        }
    }

    sigma_device_t        m_devices[DEVMGR_MAX_DEVICES];
    sigma_u32             m_device_count;
    sigma_hotplug_event_t m_hotplug_queue[DEVMGR_MAX_HOTPLUG];
    sigma_u32             m_hotplug_head;
    sigma_u32             m_hotplug_tail;
    sigma_u32             m_hotplug_count;
};

} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void devmgr_init(void) { SigmaOS::Kernel::SovereignDeviceManager::getInstance().init(); }
void devmgr_scan(void) { SigmaOS::Kernel::SovereignDeviceManager::getInstance().scan(); }

sigma_u32 devmgr_register_device(const char* name, sigma_dev_type_t type,
                                 sigma_u32 parent_id, sigma_u16 vendor_id, sigma_u16 device_id) {
    return SigmaOS::Kernel::SovereignDeviceManager::getInstance()
               .registerDevice(name, type, parent_id, vendor_id, device_id);
}
int devmgr_bind_driver(sigma_u32 dev_id, const char* driver_name) {
    return SigmaOS::Kernel::SovereignDeviceManager::getInstance().bindDriver(dev_id, driver_name);
}
int devmgr_unbind_driver(sigma_u32 dev_id) {
    return SigmaOS::Kernel::SovereignDeviceManager::getInstance().unbindDriver(dev_id);
}
int devmgr_set_status(sigma_u32 dev_id, sigma_dev_status_t status) {
    return SigmaOS::Kernel::SovereignDeviceManager::getInstance().setStatus(dev_id, status);
}
const sigma_device_t* devmgr_get_device(sigma_u32 dev_id) {
    return SigmaOS::Kernel::SovereignDeviceManager::getInstance().getDevice(dev_id);
}
void devmgr_print_tree(void) {
    SigmaOS::Kernel::SovereignDeviceManager::getInstance().printTree();
}
sigma_u32 devmgr_get_device_count(void) {
    return SigmaOS::Kernel::SovereignDeviceManager::getInstance().getDeviceCount();
}
int devmgr_hotplug_push(sigma_hotplug_event_type_t event, sigma_u32 dev_id) {
    return SigmaOS::Kernel::SovereignDeviceManager::getInstance().hotplugPush(event, dev_id);
}
int devmgr_hotplug_pop(sigma_hotplug_event_t* out) {
    return SigmaOS::Kernel::SovereignDeviceManager::getInstance().hotplugPop(out);
}

} // extern "C"
