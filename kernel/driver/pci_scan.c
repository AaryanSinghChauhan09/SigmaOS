// SPDX-License-Identifier: MIT
// SigmaOS PCI Bus Scanning Helper
// Kernel-level support for PCI device enumeration and BAR probing

#include <stddef.h>
#include <stdint.h>
#include <string.h>

// ============================================================================
// PCI Configuration Space I/O (I/O Port based)
// ============================================================================

#define PCI_CONFIG_ADDRESS_PORT 0x0CF8
#define PCI_CONFIG_DATA_PORT 0x0CFC

static inline uint32_t pci_calc_address(uint8_t bus, uint8_t device, uint8_t function, uint8_t offset) {
    return 0x80000000 | (bus << 16) | (device << 11) | (function << 8) | (offset & 0xfc);
}

static inline void outl(uint16_t port, uint32_t value) {
    __asm__ __volatile__("outl %0, %w1" : : "a"(value), "d"(port));
}

static inline uint32_t inl(uint16_t port) {
    uint32_t value;
    __asm__ __volatile__("inl %w1, %0" : "=a"(value) : "d"(port));
    return value;
}

static inline void outb(uint16_t port, uint8_t value) {
    __asm__ __volatile__("outb %0, %w1" : : "a"(value), "d"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t value;
    __asm__ __volatile__("inb %w1, %0" : "=a"(value) : "d"(port));
    return value;
}

static inline void outw(uint16_t port, uint16_t value) {
    __asm__ __volatile__("outw %0, %w1" : : "a"(value), "d"(port));
}

static inline uint16_t inw(uint16_t port) {
    uint16_t value;
    __asm__ __volatile__("inw %w1, %0" : "=a"(value) : "d"(port));
    return value;
}

// ============================================================================
// PCI Device Discovery
// ============================================================================

typedef struct {
    uint8_t bus;
    uint8_t device;
    uint8_t function;
    uint16_t vendor_id;
    uint16_t device_id;
    uint8_t class_code;
    uint8_t subclass;
    uint8_t prog_if;
} pci_device_t;

#define MAX_PCI_DEVICES 256
static pci_device_t discovered_devices[MAX_PCI_DEVICES];
static uint32_t device_count = 0;

uint32_t pci_read_u32(uint8_t bus, uint8_t device, uint8_t function, uint8_t offset) {
    uint32_t addr = pci_calc_address(bus, device, function, offset);
    outl(PCI_CONFIG_ADDRESS_PORT, addr);
    return inl(PCI_CONFIG_DATA_PORT);
}

uint16_t pci_read_u16(uint8_t bus, uint8_t device, uint8_t function, uint8_t offset) {
    uint32_t addr = pci_calc_address(bus, device, function, offset & ~3);
    outl(PCI_CONFIG_ADDRESS_PORT, addr);
    uint32_t data = inl(PCI_CONFIG_DATA_PORT);
    return (data >> ((offset & 3) * 8)) & 0xffff;
}

uint8_t pci_read_u8(uint8_t bus, uint8_t device, uint8_t function, uint8_t offset) {
    uint32_t addr = pci_calc_address(bus, device, function, offset & ~3);
    outl(PCI_CONFIG_ADDRESS_PORT, addr);
    uint32_t data = inl(PCI_CONFIG_DATA_PORT);
    return (data >> ((offset & 3) * 8)) & 0xff;
}

void pci_write_u32(uint8_t bus, uint8_t device, uint8_t function, uint8_t offset, uint32_t value) {
    uint32_t addr = pci_calc_address(bus, device, function, offset);
    outl(PCI_CONFIG_ADDRESS_PORT, addr);
    outl(PCI_CONFIG_DATA_PORT, value);
}

void pci_write_u16(uint8_t bus, uint8_t device, uint8_t function, uint8_t offset, uint16_t value) {
    uint32_t addr = pci_calc_address(bus, device, function, offset & ~3);
    outl(PCI_CONFIG_ADDRESS_PORT, addr);
    uint32_t data = inl(PCI_CONFIG_DATA_PORT);
    uint8_t shift = (offset & 3) * 8;
    data = (data & ~(0xffff << shift)) | ((uint32_t)value << shift);
    outl(PCI_CONFIG_DATA_PORT, data);
}

void pci_write_u8(uint8_t bus, uint8_t device, uint8_t function, uint8_t offset, uint8_t value) {
    uint32_t addr = pci_calc_address(bus, device, function, offset & ~3);
    outl(PCI_CONFIG_ADDRESS_PORT, addr);
    uint32_t data = inl(PCI_CONFIG_DATA_PORT);
    uint8_t shift = (offset & 3) * 8;
    data = (data & ~(0xff << shift)) | ((uint32_t)value << shift);
    outl(PCI_CONFIG_DATA_PORT, data);
}

// ============================================================================
// BAR (Base Address Register) Probing
// ============================================================================

typedef struct {
    uint8_t index;
    uint8_t type;  // 0 = I/O, 1 = Memory 32-bit, 2 = Memory 64-bit
    uint64_t address;
    uint64_t size;
    uint8_t prefetchable;
} pci_bar_t;

void probe_bar(uint8_t bus, uint8_t device, uint8_t function, uint8_t bar_index, pci_bar_t *bar) {
    uint8_t offset = 0x10 + (bar_index * 4);
    uint32_t bar_val = pci_read_u32(bus, device, function, offset);

    bar->index = bar_index;

    if (bar_val == 0) {
        bar->type = 0xFF; // Invalid
        return;
    }

    if (bar_val & 0x01) {
        // I/O Space
        bar->type = 0;
        bar->address = bar_val & 0xfffffffc;

        // Probe size
        pci_write_u32(bus, device, function, offset, 0xffffffff);
        uint32_t size_mask = pci_read_u32(bus, device, function, offset);
        pci_write_u32(bus, device, function, offset, bar_val);
        bar->size = (uint64_t)(((~size_mask) & 0xfffc) + 1);
    } else {
        // Memory Space
        uint8_t mem_type = (bar_val >> 1) & 0x3;
        bar->prefetchable = (bar_val >> 3) & 0x1;

        if (mem_type == 0) {
            // 32-bit memory
            bar->type = 1;
            bar->address = bar_val & 0xfffffff0;

            pci_write_u32(bus, device, function, offset, 0xffffffff);
            uint32_t size_mask = pci_read_u32(bus, device, function, offset);
            pci_write_u32(bus, device, function, offset, bar_val);
            bar->size = (uint64_t)(((~size_mask) & 0xfffffff0) + 1);
        } else if (mem_type == 2) {
            // 64-bit memory
            bar->type = 2;
            uint32_t bar_high = pci_read_u32(bus, device, function, offset + 4);
            bar->address = ((uint64_t)bar_high << 32) | (bar_val & 0xfffffff0);

            pci_write_u32(bus, device, function, offset, 0xffffffff);
            uint32_t size_mask_low = pci_read_u32(bus, device, function, offset);
            pci_write_u32(bus, device, function, offset, bar_val);

            pci_write_u32(bus, device, function, offset + 4, 0xffffffff);
            uint32_t size_mask_high = pci_read_u32(bus, device, function, offset + 4);
            pci_write_u32(bus, device, function, offset + 4, bar_high);

            bar->size = ((uint64_t)size_mask_high << 32) | size_mask_low;
            bar->size = ((~bar->size) & 0xfffffffffffffff0) + 1;
        }
    }
}

// ============================================================================
// Full PCI Enumeration
// ============================================================================

uint32_t sigma_pci_scan() {
    device_count = 0;

    for (uint8_t bus = 0; bus < 256; bus++) {
        for (uint8_t device = 0; device < 32; device++) {
            uint16_t vendor_id = pci_read_u16(bus, device, 0, 0x00);

            if (vendor_id == 0xffff || vendor_id == 0x0000) {
                continue;
            }

            // Device found at function 0
            uint8_t header_type = pci_read_u8(bus, device, 0, 0x0e);
            uint8_t multi_function = (header_type & 0x80) != 0;

            uint8_t max_functions = multi_function ? 8 : 1;

            for (uint8_t function = 0; function < max_functions; function++) {
                vendor_id = pci_read_u16(bus, device, function, 0x00);

                if (vendor_id == 0xffff || vendor_id == 0x0000) {
                    continue;
                }

                if (device_count < MAX_PCI_DEVICES) {
                    pci_device_t *dev = &discovered_devices[device_count++];
                    dev->bus = bus;
                    dev->device = device;
                    dev->function = function;
                    dev->vendor_id = vendor_id;
                    dev->device_id = pci_read_u16(bus, device, function, 0x02);
                    dev->class_code = pci_read_u8(bus, device, function, 0x09);
                    dev->subclass = pci_read_u8(bus, device, function, 0x0a);
                    dev->prog_if = pci_read_u8(bus, device, function, 0x0b);
                }
            }
        }
    }

    return device_count;
}

// ============================================================================
// Device Enable (I/O & Memory Access)
// ============================================================================

void pci_enable_device(uint8_t bus, uint8_t device, uint8_t function) {
    uint16_t cmd = pci_read_u16(bus, device, function, 0x04);
    // Enable I/O space, memory space, and bus master
    cmd |= 0x0007; // Bits 0, 1, 2
    pci_write_u16(bus, device, function, 0x04, cmd);
}

// ============================================================================
// Public API for Rust layer
// ============================================================================

typedef void (*pci_device_callback_t)(uint8_t bus, uint8_t device, uint8_t function, 
                                       uint16_t vendor_id, uint16_t device_id);

void sigma_pci_enumerate(pci_device_callback_t callback) {
    uint32_t count = sigma_pci_scan();
    
    for (uint32_t i = 0; i < count; i++) {
        pci_device_t *dev = &discovered_devices[i];
        if (callback) {
            callback(dev->bus, dev->device, dev->function, dev->vendor_id, dev->device_id);
        }
        pci_enable_device(dev->bus, dev->device, dev->function);
    }
}

uint32_t sigma_pci_count() {
    return device_count;
}

void sigma_pci_get_device(uint32_t index, uint8_t *bus, uint8_t *device, uint8_t *function,
                          uint16_t *vendor_id, uint16_t *device_id) {
    if (index < device_count) {
        pci_device_t *dev = &discovered_devices[index];
        *bus = dev->bus;
        *device = dev->device;
        *function = dev->function;
        *vendor_id = dev->vendor_id;
        *device_id = dev->device_id;
    }
}
