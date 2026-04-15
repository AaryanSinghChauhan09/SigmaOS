/*
 * =========================================================================
 * S SIGMAOS: S28_OMNIBUS — SovereignPCIWalk.c
 * =========================================================================
 * Implementation of Idea 196 (Apex Infinity): PCI Configuration Walkthrough.
 * Provides the industrial-grade discovery mechanism for all PCIe devices.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include "sigma_types.h"

/* PCI Access primitives (using HAL S04 logic implicitly) */
extern void outb(uint16_t port, uint8_t val);
extern uint8_t inb(uint16_t port);
extern void hal_wrmsr(uint32_t msr, uint64_t val); // Conceptual

#define PCI_CONFIG_ADDRESS 0xCF8
#define PCI_CONFIG_DATA    0xCFC

uint32_t pci_config_read(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t address;
    uint32_t lbus  = (uint32_t)bus;
    uint32_t lslot = (uint32_t)slot;
    uint32_t lfunc = (uint32_t)func;
    
    /* Create configuration address */
    address = (uint32_t)((lbus << 16) | (lslot << 11) |
              (lfunc << 8) | (offset & 0xfc) | ((uint32_t)0x80000000));
              
    /* Write to address port */
    // Conceptual call to outl (not defined yet, simulating via inline)
    __asm__ volatile ("out %0, %1" : : "a"(address), "d"((uint16_t)0xCF8));
    
    /* Read from data port */
    uint32_t tmp;
    __asm__ volatile ("in %1, %0" : "=a"(tmp) : "d"((uint16_t)0xCFC));
    return tmp;
}

void pci_walk_lattice(void) {
    sigma_printf("S [S28]: Scanning Sovereign PCI Lattice...\n");
    for (uint16_t bus = 0; bus < 256; bus++) {
        for (uint8_t slot = 0; slot < 32; slot++) {
            uint32_t vendor = pci_config_read((uint8_t)bus, slot, 0, 0);
            if ((vendor & 0xFFFF) != 0xFFFF) {
                sigma_printf("S [PCI]: %02X:%02X.0 -> [VENDOR: %04X] [DEVICE: %04X]\n", 
                             bus, slot, vendor & 0xFFFF, vendor >> 16);
            }
        }
    }
}
