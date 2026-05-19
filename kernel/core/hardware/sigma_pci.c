/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: PCI/PCIe BUS ENUMERATION
 * =============================================================================
 * Inspired by: Linux kernel drivers/pci/probe.c
 *              FreeBSD sys/dev/pci/pci.c
 * =============================================================================
 * Scans the PCI configuration space to discover hardware devices.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define PCI_MAX_DEVICES 64
#define PCI_VENDOR_ID   0x00
#define PCI_CLASS_REV   0x08
#define PCI_HEADER_TYPE 0x0E

typedef struct {
    sigma_u16 bus;
    sigma_u16 slot;
    sigma_u16 func;
    sigma_u16 vendor_id;
    sigma_u16 device_id;
    sigma_u8  class_code;
    sigma_u8  subclass;
    sigma_bool active;
} sigma_pci_device_t;

static sigma_pci_device_t pci_devices[PCI_MAX_DEVICES];
static sigma_u32 pci_dev_count = 0;

/* Simulated PCI Config Read (real kernel uses inb/outb on ports 0xCF8/0xCFC) */
static sigma_u32 pci_config_read(sigma_u16 bus, sigma_u16 slot, sigma_u16 func, sigma_u8 offset) {
    if (bus == 0 && slot == 0 && func == 0 && offset == PCI_VENDOR_ID) {
        return 0x12348086; /* Intel dummy host bridge */
    }
    if (bus == 0 && slot == 1 && func == 0 && offset == PCI_VENDOR_ID) {
        return 0x100E8086; /* Intel e1000 NIC */
    }
    if (bus == 0 && slot == 2 && func == 0 && offset == PCI_VENDOR_ID) {
        return 0x10501AF4; /* VirtIO GPU */
    }
    return 0xFFFFFFFF; /* Unimplemented */
}

void pci_subsystem_init(void) {
    sigma_memset(pci_devices, 0, sizeof(pci_devices));
    pci_dev_count = 0;
    sigma_printf("[pci] PCI Configuration Space Enumerator initialized\n");
}

void pci_probe_bus(void) {
    sigma_printf("[pci] Scanning PCI Bus 0...\n");
    
    for (sigma_u16 bus = 0; bus < 1; bus++) {
        for (sigma_u16 slot = 0; slot < 32; slot++) {
            for (sigma_u16 func = 0; func < 8; func++) {
                sigma_u32 vendor_device = pci_config_read(bus, slot, func, PCI_VENDOR_ID);
                
                if (vendor_device != 0xFFFFFFFF) {
                    if (pci_dev_count < PCI_MAX_DEVICES) {
                        sigma_pci_device_t* dev = &pci_devices[pci_dev_count++];
                        dev->bus       = bus;
                        dev->slot      = slot;
                        dev->func      = func;
                        dev->vendor_id = vendor_device & 0xFFFF;
                        dev->device_id = (vendor_device >> 16) & 0xFFFF;
                        dev->active    = SIGMA_TRUE;
                        
                        /* In a real kernel, we read Class/Subclass from offset 0x08 */
                        dev->class_code = 0x02; /* Network controller dummy */
                        
                        sigma_printf("[pci] Found Device: %02x:%02x.%d Vendor: 0x%04X Device: 0x%04X\n",
                                     bus, slot, func, dev->vendor_id, dev->device_id);
                    }
                }
            }
        }
    }
    sigma_printf("[pci] Enumeration complete. Found %u devices.\n", pci_dev_count);
}

sigma_pci_device_t* pci_find_device(sigma_u16 vendor, sigma_u16 device) {
    for (sigma_u32 i = 0; i < pci_dev_count; i++) {
        if (pci_devices[i].active && pci_devices[i].vendor_id == vendor && pci_devices[i].device_id == device) {
            return &pci_devices[i];
        }
    }
    return SIGMA_NULL;
}
