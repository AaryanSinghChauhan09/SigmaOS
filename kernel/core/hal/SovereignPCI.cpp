#include "sigma_types.h"
#include "../../../../include/sigma_log.h"
#include "SovereignLibC.h"

// PCI Configuration Space Addresses
#define PCI_CONFIG_ADDRESS 0xCF8
#define PCI_CONFIG_DATA    0xCFC

// Simulated outportl / inportl
extern "C" void outportl(sigma_u16 port, sigma_u32 data);
extern "C" sigma_u32 inportl(sigma_u16 port);

class SovereignPCIDriver {
public:
    static SovereignPCIDriver& getInstance() {
        static SovereignPCIDriver instance;
        return instance;
    }

    void init() {
        sigma_log_info("[PCI] Initializing Sovereign PCI Bus Enumerator...\n");
        enumerateBuses();
    }

    sigma_u32 readConfig(sigma_u8 bus, sigma_u8 device, sigma_u8 func, sigma_u8 offset) {
        sigma_u32 address = (sigma_u32)((bus << 16) | (device << 11) | (func << 8) | (offset & 0xFC) | ((sigma_u32)0x80000000));
        // Simulated I/O
        // outportl(PCI_CONFIG_ADDRESS, address);
        // return inportl(PCI_CONFIG_DATA);
        
        // Mock returning empty for most devices
        if (bus == 0 && device == 0 && func == 0) return 0x80861234; // Host bridge
        if (bus == 0 && device == 2 && func == 0) return 0x10DE1C03; // GPU mock
        if (bus == 0 && device == 29 && func == 0) return 0x80861C2D; // USB mock
        return 0xFFFFFFFF;
    }

    void enumerateBuses() {
        for (sigma_u16 bus = 0; bus < 256; bus++) {
            for (sigma_u8 device = 0; device < 32; device++) {
                sigma_u32 vendor_device = readConfig(bus, device, 0, 0);
                if ((vendor_device & 0xFFFF) != 0xFFFF) {
                    sigma_u16 vendor = vendor_device & 0xFFFF;
                    sigma_u16 dev_id = vendor_device >> 16;
                    sigma_log_info("[PCI] Discovered Device: Bus %u, Dev %u -> Vendor: 0x%04X, Device: 0x%04X\n", bus, device, vendor, dev_id);
                    
                    if (vendor == 0x10DE) { // Mock NVIDIA
                        sigma_log_info("[PCI] -> Registered NVIDIA GPU Stub\n");
                    }
                    if (vendor == 0x8086 && dev_id == 0x1C2D) {
                        sigma_log_info("[PCI] -> Registered Intel USB Stub\n");
                    }
                }
            }
        }
        sigma_log_info("[PCI] Bus Enumeration Complete.\n");
    }
};

extern "C" void pci_init() {
    SovereignPCIDriver::getInstance().init();
}
