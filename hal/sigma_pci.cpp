/**
 * =========================================================================
 * Σ SIGMAOS: PCI EXPRESS HAL (Phase E)
 * =========================================================================
 * Hardware Abstraction Layer for PCI/PCIe device discovery, BAR mapping,
 * and MSI/MSI-X interrupt vector routing.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_error_codes.h"
#include <sigma_libc.h>

namespace SigmaOS {
namespace HAL {

#define PCI_CONFIG_ADDRESS 0xCF8
#define PCI_CONFIG_DATA    0xCFC

#define PCI_CAPABILITY_LIST 0x34
#define PCI_CAP_ID_MSI      0x05
#define PCI_CAP_ID_MSIX     0x11

// Hardware stub for port I/O
static inline void outl(sigma_u16 port, sigma_u32 val) {
#if defined(__x86_64__)
    __asm__ volatile ( "outl %0, %1" : : "a"(val), "Nd"(port) );
#else
    (void)port; (void)val;
#endif
}

static inline sigma_u32 inl(sigma_u16 port) {
    sigma_u32 ret = 0;
#if defined(__x86_64__)
    __asm__ volatile ( "inl %1, %0" : "=a"(ret) : "Nd"(port) );
#else
    (void)port;
#endif
    return ret;
}

static sigma_u32 pci_config_read(sigma_u8 bus, sigma_u8 slot, sigma_u8 func, sigma_u8 offset) {
    sigma_u32 address = (sigma_u32)((bus << 16) | (slot << 11) |
              (func << 8) | (offset & 0xFC) | ((sigma_u32)0x80000000));
    outl(PCI_CONFIG_ADDRESS, address);
    return inl(PCI_CONFIG_DATA);
}

static void pci_config_write(sigma_u8 bus, sigma_u8 slot, sigma_u8 func, sigma_u8 offset, sigma_u32 value) {
    sigma_u32 address = (sigma_u32)((bus << 16) | (slot << 11) |
              (func << 8) | (offset & 0xFC) | ((sigma_u32)0x80000000));
    outl(PCI_CONFIG_ADDRESS, address);
    outl(PCI_CONFIG_DATA, value);
}

// -------------------------------------------------------------------------
// MSI-X Configuration
// -------------------------------------------------------------------------
sigma_status sigma_pci_enable_msix(sigma_u8 bus, sigma_u8 slot, sigma_u8 func) {
    sys_print("[PCIe] Searching for MSI-X capabilities on %02x:%02x.%x...\n", bus, slot, func);
    
    // Read Status register to check if Capabilities List is supported
    sigma_u32 status_cmd = pci_config_read(bus, slot, func, 0x04);
    if ((status_cmd & (1 << 20)) == 0) { // Status bit 4 (bit 20 of dword)
        sys_print("[PCIe] Capabilities List not supported.\n");
        return K_ERR_INVAL;
    }
    
    sigma_u32 cap_ptr = pci_config_read(bus, slot, func, PCI_CAPABILITY_LIST) & 0xFF;
    
    while (cap_ptr != 0) {
        sigma_u32 cap_header = pci_config_read(bus, slot, func, cap_ptr);
        sigma_u8 cap_id = cap_header & 0xFF;
        
        if (cap_id == PCI_CAP_ID_MSIX) {
            sys_print("[PCIe] Found MSI-X capability at offset 0x%02X.\n", cap_ptr);
            
            // Enable MSI-X
            // The Message Control register is 16 bits at offset + 2
            sigma_u32 msg_ctrl = pci_config_read(bus, slot, func, cap_ptr);
            msg_ctrl |= (1 << 31); // Set MSI-X Enable bit (bit 15 of word at +2)
            pci_config_write(bus, slot, func, cap_ptr, msg_ctrl);
            
            sys_print("[PCIe] MSI-X enabled successfully.\n");
            return SIGMA_SUCCESS;
        }
        
        // Next capability pointer is the second byte
        cap_ptr = (cap_header >> 8) & 0xFF;
    }
    
    sys_print("[PCIe] MSI-X capability not found.\n");
    return K_ERR_INVAL;
}

} // namespace HAL
} // namespace SigmaOS
