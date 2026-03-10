/*
 * Cosmos AI-OS: Low-Level PCI Scanner (C Layer)
 * ===============================================
 * Mission: Direct hardware interaction (Ring-0) for PCI enumeration.
 */

#include <stddef.h>
#include <stdint.h>


// Direct I/O port mapping for x86_64
static inline void outl(uint16_t port, uint32_t val) {
  __asm__ volatile("outl %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint32_t inl(uint16_t port) {
  uint32_t ret;
  __asm__ volatile("inl %1, %0" : "=a"(ret) : "Nd"(port));
  return ret;
}

#define PCI_CONFIG_ADDRESS 0xCF8
#define PCI_CONFIG_DATA 0xCFC

// Structure exported to Python's ctypes
typedef struct {
  uint16_t vendor_id;
  uint16_t device_id;
  uint8_t bus;
  uint8_t slot;
  uint8_t func;
} cosmos_pci_device_t;

cosmos_pci_device_t found_devices[256];
int device_count = 0;

uint32_t pci_config_read(uint8_t bus, uint8_t device, uint8_t func,
                         uint8_t offset) {
  uint32_t address;
  uint32_t lbus = (uint32_t)bus;
  uint32_t ldevice = (uint32_t)device;
  uint32_t lfunc = (uint32_t)func;

  address = (uint32_t)((lbus << 16) | (ldevice << 11) | (lfunc << 8) |
                       (offset & 0xfc) | ((uint32_t)0x80000000));

  outl(PCI_CONFIG_ADDRESS, address);
  return inl(PCI_CONFIG_DATA);
}

// Exposed to Python ctypes
int cosmos_scan_pci_bus() {
  device_count = 0;
  for (uint16_t bus = 0; bus < 256; bus++) {
    for (uint8_t slot = 0; slot < 32; slot++) {
      uint32_t val = pci_config_read(bus, slot, 0, 0);
      if ((val & 0xFFFF) != 0xFFFF) { // Valid device exists
        cosmos_pci_device_t *dev = &found_devices[device_count++];
        dev->vendor_id = val & 0xFFFF;
        dev->device_id = (val >> 16) & 0xFFFF;
        dev->bus = bus;
        dev->slot = slot;
        dev->func = 0; // Simplified for func 0

        // Stop if buffer is full
        if (device_count >= 256)
          return device_count;
      }
    }
  }
  return device_count;
}

cosmos_pci_device_t *cosmos_get_pci_devices() { return found_devices; }
