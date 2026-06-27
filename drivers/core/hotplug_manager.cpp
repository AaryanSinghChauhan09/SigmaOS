// SPDX-License-Identifier: GPL-2.0-or-later
// hotplug_manager.cpp — Driver hot-plug manager for SigmaOS
//
// Detects hardware insertion/removal (USB, PCIe, SATA) and:
//   1. Matches the device to a driver shard
//   2. Spawns the driver shard if not already running
//   3. Passes MMIO base + IRQ to the driver via IPC
//   4. Safely removes the driver shard on device removal
//
// Monitors:
//   • USB: USB hub interrupt transfers report connect/disconnect
//   • PCIe: PCIe hotplug capability registers (presence detect changed)
//   • SATA: AHCI port interrupt (device presence changed)
//
// Inspired by:
//   • Linux drivers/base/dd.c (device/driver matching)
//   • udev (event-driven device management)
//   • Genode driver manager

#include "../include/drivers/driver_interface.h"
#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <stdbool.h>

// ── Device descriptor ─────────────────────────────────────────────────────

typedef enum device_bus {
    BUS_PCI  = 0,
    BUS_USB  = 1,
    BUS_SATA = 2,
    BUS_I2C  = 3,
    BUS_SPI  = 4,
} device_bus_t;

typedef struct hotplug_device {
    device_bus_t bus;
    uint32_t     vendor_id;
    uint32_t     device_id;
    uint32_t     class_code;
    uintptr_t    mmio_base;
    size_t       mmio_size;
    uint32_t     irq;
    char         path[64];     // e.g. "pci:0000:01:00.0"
    bool         present;
    uint32_t     driver_shard; // 0 = no driver loaded
} hotplug_device_t;

#define MAX_DEVICES 128
static hotplug_device_t g_devices[MAX_DEVICES];
static uint32_t         g_dev_count = 0;

// ── Driver matching table ─────────────────────────────────────────────────

typedef struct driver_match {
    uint32_t    vendor_id;     // 0xFFFF = wildcard
    uint32_t    device_id;     // 0xFFFF = wildcard
    uint32_t    class_code;    // 0xFFFFFF = wildcard (matched if vendor/dev match)
    const char *shard_binary;  // path to driver shard executable
    const char *name;
} driver_match_t;

static const driver_match_t driver_table[] = {
    // NVMe controllers
    { 0xFFFF, 0xFFFF, 0x010802, "/sbin/drivers/nvme_shard",  "nvme"        },
    // AHCI SATA controllers
    { 0xFFFF, 0xFFFF, 0x010601, "/sbin/drivers/ahci_shard",  "ahci"        },
    // USB XHCI (USB 3.x)
    { 0xFFFF, 0xFFFF, 0x0C0330, "/sbin/drivers/xhci_shard",  "xhci"        },
    // USB EHCI (USB 2.0)
    { 0xFFFF, 0xFFFF, 0x0C0320, "/sbin/drivers/ehci_shard",  "ehci"        },
    // Intel e1000 Ethernet
    { 0x8086, 0x100E, 0xFFFFFF, "/sbin/drivers/e1000_shard", "e1000"       },
    // Intel i219 Ethernet
    { 0x8086, 0x15BC, 0xFFFFFF, "/sbin/drivers/e1000e_shard","e1000e"      },
    // Realtek RTL8111 Ethernet
    { 0x10EC, 0x8168, 0xFFFFFF, "/sbin/drivers/r8169_shard", "r8169"       },
    // virtio-net (QEMU/KVM)
    { 0x1AF4, 0x1000, 0xFFFFFF, "/sbin/drivers/virtio_net",  "virtio-net"  },
    // virtio-blk
    { 0x1AF4, 0x1001, 0xFFFFFF, "/sbin/drivers/virtio_blk",  "virtio-blk"  },
    // Intel HD Audio
    { 0xFFFF, 0xFFFF, 0x040300, "/sbin/drivers/hda_shard",   "intel-hda"   },
    // Intel i915 GPU
    { 0x8086, 0xFFFF, 0x030000, "/sbin/drivers/i915_shard",  "i915"        },
    // USB HID (keyboard/mouse)
    { 0xFFFF, 0xFFFF, 0x030000, "/sbin/drivers/usbhid_shard","usb-hid"     },
    { 0, 0, 0, NULL, NULL }
};

// ── Driver matching ───────────────────────────────────────────────────────

static const driver_match_t *match_driver(const hotplug_device_t *dev) {
    for (int i = 0; driver_table[i].shard_binary; i++) {
        const driver_match_t *m = &driver_table[i];
        bool vid_match = (m->vendor_id == 0xFFFF || m->vendor_id == dev->vendor_id);
        bool did_match = (m->device_id == 0xFFFF || m->device_id == dev->device_id);
        bool cls_match = (m->class_code == 0xFFFFFF || m->class_code == dev->class_code);
        if (vid_match && did_match && cls_match) return m;
    }
    return NULL;
}

// ── Shard spawning ────────────────────────────────────────────────────────

extern int sigma_process_spawn(const char *binary, const char *const argv[],
                                uint32_t *out_pid);

static int spawn_driver(hotplug_device_t *dev, const driver_match_t *match) {
    // Pass MMIO base and IRQ as environment variables to the driver shard
    char mmio_arg[32], irq_arg[16];
    snprintf(mmio_arg, sizeof(mmio_arg), "0x%lx", (unsigned long)dev->mmio_base);
    snprintf(irq_arg,  sizeof(irq_arg),  "%u",    dev->irq);

    const char *argv[] = {
        match->shard_binary,
        "--mmio", mmio_arg,
        "--irq",  irq_arg,
        "--path", dev->path,
        NULL
    };

    uint32_t pid = 0;
    int rc = sigma_process_spawn(match->shard_binary, argv, &pid);
    if (rc == 0) {
        dev->driver_shard = pid;
        printf("[hotplug] loaded driver '%s' (pid=%u) for %s\n",
               match->name, pid, dev->path);
    } else {
        printf("[hotplug] failed to load driver '%s' for %s\n",
               match->name, dev->path);
    }
    return rc;
}

// ── Device arrival / removal ──────────────────────────────────────────────

static void on_device_arrived(const hotplug_device_t *info) {
    if (g_dev_count >= MAX_DEVICES) return;
    hotplug_device_t *dev = &g_devices[g_dev_count++];
    *dev = *info;
    dev->present = true;

    printf("[hotplug] device arrived: %s vid=0x%04x did=0x%04x class=0x%06x\n",
           dev->path, dev->vendor_id, dev->device_id, dev->class_code);

    const driver_match_t *match = match_driver(dev);
    if (match) {
        spawn_driver(dev, match);
    } else {
        printf("[hotplug] no driver found for %s\n", dev->path);
    }
}

static void on_device_removed(const char *path) {
    for (uint32_t i = 0; i < g_dev_count; i++) {
        if (strcmp(g_devices[i].path, path) == 0 && g_devices[i].present) {
            g_devices[i].present = false;
            printf("[hotplug] device removed: %s\n", path);
            if (g_devices[i].driver_shard) {
                // Send SUSPEND IPC to driver, then kill it
                sigma_ipc_msg_t msg = {0};
                msg.opcode = SIGMA_DRV_OP_SUSPEND;
                sigma_driver_send(g_devices[i].driver_shard, &msg);
                // sigma_process_signal(g_devices[i].driver_shard, SIGTERM);
                g_devices[i].driver_shard = 0;
            }
            break;
        }
    }
}

// ── PCI hotplug polling ───────────────────────────────────────────────────

extern void sigma_pci_scan_hotplug(
    void (*arrived)(const hotplug_device_t *),
    void (*removed)(const char *path));

extern void sigma_usb_scan_hotplug(
    void (*arrived)(const hotplug_device_t *),
    void (*removed)(const char *path));

// ── Main event loop ───────────────────────────────────────────────────────

int main(void) {
    printf("[hotplug] SigmaOS device hot-plug manager starting\n");

    // Register ourselves with the driver bus
    sigma_driver_reg_t reg = {0};
    strncpy(reg.name, "hotplug-manager", sizeof(reg.name));
    reg.flags = 0;
    sigma_driver_register(&reg);

    // Initial PCI/USB enumeration
    sigma_driver_bus_enumerate_pci();
    sigma_driver_bus_enumerate_usb();

    printf("[hotplug] initial enumeration complete (%u devices)\n", g_dev_count);

    // Monitor for hotplug events
    while (1) {
        sigma_pci_scan_hotplug(on_device_arrived, on_device_removed);
        sigma_usb_scan_hotplug(on_device_arrived, on_device_removed);
        // sigma_sleep_ms(500);
    }
}
