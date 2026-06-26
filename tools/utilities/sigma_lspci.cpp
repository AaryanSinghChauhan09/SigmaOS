/*
 * Σ SigmaOS Zenith — lspci Utility
 * Absorbs: pciutils lspci
 * Zero-Dependency: No libc.
 */

typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_pci_enumerate();
extern "C" u32  sigma_pci_get_device_count();

extern "C" int sigma_lspci_main(int argc, char** argv) {
    sigma_pci_enumerate();
    sigma_vga_printf("\nTotal: %u PCI devices\n", sigma_pci_get_device_count());
    return 0;
}
