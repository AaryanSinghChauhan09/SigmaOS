#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS PCI Bus Enumeration Prototype
// ---------------------------------------------------------

#define PCI_CONFIG_ADDRESS 0xCF8
#define PCI_CONFIG_DATA    0xCFC

// Forward declarations for I/O ports (would be in HAL)
extern void outl(uint16_t port, uint32_t val);
extern uint32_t inl(uint16_t port);

uint32_t pci_config_read_dword(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t address;
    uint32_t lbus  = (uint32_t)bus;
    uint32_t lslot = (uint32_t)slot;
    uint32_t lfunc = (uint32_t)func;
    
    // Create configuration address
    address = (uint32_t)((lbus << 16) | (lslot << 11) | (lfunc << 8) | (offset & 0xFC) | ((uint32_t)0x80000000));
    
    // Write out the address
    outl(PCI_CONFIG_ADDRESS, address);
    // Read in the data
    return inl(PCI_CONFIG_DATA);
}

void pci_enumerate() {
    for(uint16_t bus = 0; bus < 256; bus++) {
        for(uint8_t slot = 0; slot < 32; slot++) {
            for(uint8_t func = 0; func < 8; func++) {
                uint32_t vendor_device = pci_config_read_dword(bus, slot, func, 0);
                uint16_t vendor = (uint16_t)(vendor_device & 0xFFFF);
                
                if(vendor == 0xFFFF) continue; // Device doesn't exist
                
                // Register device with system...
                // ...
            }
        }
    }
}
