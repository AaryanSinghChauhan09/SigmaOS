/*
 * Σ SigmaOS Zenith — PCI Bus Enumerator
 * Absorbs: Linux drivers/pci/probe.c, pci-driver.c
 * Zero-Dependency: No libc.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* PCI Configuration Space Access (Mechanism 1) */
#define PCI_CONFIG_ADDR 0xCF8
#define PCI_CONFIG_DATA 0xCFC

static inline void sigma_outl(u16 port, u32 val) {
    __asm__ volatile("outl %0, %1" : : "a"(val), "Nd"(port));
}
static inline u32 sigma_inl(u16 port) {
    u32 val;
    __asm__ volatile("inl %1, %0" : "=a"(val) : "Nd"(port));
    return val;
}

static u32 pci_read(u8 bus, u8 slot, u8 func, u8 offset) {
    u32 address = (1 << 31)
        | ((u32)bus  << 16)
        | ((u32)slot << 11)
        | ((u32)func << 8)
        | (offset & 0xFC);
    sigma_outl(PCI_CONFIG_ADDR, address);
    return sigma_inl(PCI_CONFIG_DATA);
}

struct sigma_pci_device {
    u8  bus;
    u8  slot;
    u8  func;
    u16 vendor_id;
    u16 device_id;
    u8  class_code;
    u8  subclass;
    u8  prog_if;
    u32 bar[6];
};

#define MAX_PCI_DEVICES 64
static struct sigma_pci_device pci_devices[MAX_PCI_DEVICES];
static u32 pci_device_count = 0;

static const char* pci_class_name(u8 cls, u8 sub) {
    if (cls == 0x01 && sub == 0x06) return "SATA (AHCI)";
    if (cls == 0x01 && sub == 0x01) return "IDE";
    if (cls == 0x01 && sub == 0x08) return "NVMe";
    if (cls == 0x02 && sub == 0x00) return "Ethernet";
    if (cls == 0x03 && sub == 0x00) return "VGA";
    if (cls == 0x04 && sub == 0x01) return "Audio (Multimedia)";
    if (cls == 0x04 && sub == 0x03) return "Audio (HDA)";
    if (cls == 0x06 && sub == 0x00) return "Host Bridge";
    if (cls == 0x06 && sub == 0x01) return "ISA Bridge";
    if (cls == 0x06 && sub == 0x04) return "PCI-to-PCI Bridge";
    if (cls == 0x0C && sub == 0x03) return "USB";
    if (cls == 0x0C && sub == 0x05) return "SMBus";
    return "Unknown";
}

extern "C" void sigma_pci_enumerate() {
    pci_device_count = 0;

    sigma_vga_printf("PCI: Enumerating devices...\n");

    for (u16 bus = 0; bus < 256; bus++) {
        for (u8 slot = 0; slot < 32; slot++) {
            u32 id = pci_read((u8)bus, slot, 0, 0x00);
            u16 vendor = id & 0xFFFF;
            u16 device = (id >> 16) & 0xFFFF;

            if (vendor == 0xFFFF) continue;

            u32 cls_reg = pci_read((u8)bus, slot, 0, 0x08);
            u8 class_code = (cls_reg >> 24) & 0xFF;
            u8 subclass   = (cls_reg >> 16) & 0xFF;
            u8 prog_if    = (cls_reg >> 8)  & 0xFF;

            if (pci_device_count < MAX_PCI_DEVICES) {
                struct sigma_pci_device* dev = &pci_devices[pci_device_count];
                dev->bus       = (u8)bus;
                dev->slot      = slot;
                dev->func      = 0;
                dev->vendor_id = vendor;
                dev->device_id = device;
                dev->class_code = class_code;
                dev->subclass   = subclass;
                dev->prog_if    = prog_if;

                // Read BARs
                for (int b = 0; b < 6; b++) {
                    dev->bar[b] = pci_read((u8)bus, slot, 0, 0x10 + b * 4);
                }

                pci_device_count++;
            }

            sigma_vga_printf("  [%02x:%02x.0] %04x:%04x — %s\n",
                (u32)bus, (u32)slot, vendor, device,
                pci_class_name(class_code, subclass));
        }
    }

    sigma_vga_printf("PCI: Found %u devices\n", pci_device_count);
}

extern "C" struct sigma_pci_device* sigma_pci_find(u8 class_code, u8 subclass) {
    for (u32 i = 0; i < pci_device_count; i++) {
        if (pci_devices[i].class_code == class_code &&
            pci_devices[i].subclass == subclass) {
            return &pci_devices[i];
        }
    }
    return 0;
}

extern "C" u32 sigma_pci_get_device_count() {
    return pci_device_count;
}
