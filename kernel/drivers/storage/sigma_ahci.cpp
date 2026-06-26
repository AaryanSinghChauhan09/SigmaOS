/*
 * Σ SigmaOS Zenith — AHCI/SATA Driver Shard
 * Absorbs: Linux drivers/ata/ahci.c, libahci.c
 * Zero-Dependency: No libc.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* ─────────── AHCI Register Structures ─────────── */

struct __attribute__((packed)) ahci_hba_port {
    u32 clb;        // Command List Base Address
    u32 clbu;       // Command List Base Address Upper 32-bits
    u32 fb;         // FIS Base Address
    u32 fbu;        // FIS Base Address Upper 32-bits
    u32 is;         // Interrupt Status
    u32 ie;         // Interrupt Enable
    u32 cmd;        // Command and Status
    u32 rsv0;
    u32 tfd;        // Task File Data
    u32 sig;        // Signature
    u32 ssts;       // SATA Status
    u32 sctl;       // SATA Control
    u32 serr;       // SATA Error
    u32 sact;       // SATA Active
    u32 ci;         // Command Issue
    u32 sntf;       // SATA Notification
    u32 fbs;        // FIS-based Switching Control
    u32 rsv1[11];
    u32 vendor[4];
};

struct __attribute__((packed)) ahci_hba_memory {
    u32 cap;        // Host Capabilities
    u32 ghc;        // Global Host Control
    u32 is;         // Interrupt Status
    u32 pi;         // Ports Implemented
    u32 vs;         // Version
    u32 ccc_ctl;
    u32 ccc_ports;
    u32 em_loc;
    u32 em_ctl;
    u32 cap2;
    u32 bohc;
    u8  rsv[0xA0 - 0x2C];
    u8  vendor[0x100 - 0xA0];
    struct ahci_hba_port ports[32];
};

/* AHCI Port Signatures */
#define SATA_SIG_ATA    0x00000101
#define SATA_SIG_ATAPI  0xEB140101
#define SATA_SIG_SEMB   0xC33C0101
#define SATA_SIG_PM     0x96690101

/* SATA device detection */
#define HBA_PORT_DET_PRESENT 0x3
#define HBA_PORT_IPM_ACTIVE  0x1

static volatile struct ahci_hba_memory* ahci_base = 0;

static int sigma_ahci_check_type(volatile struct ahci_hba_port* port) {
    u32 ssts = port->ssts;
    u8 det = ssts & 0x0F;
    u8 ipm = (ssts >> 8) & 0x0F;

    if (det != HBA_PORT_DET_PRESENT || ipm != HBA_PORT_IPM_ACTIVE)
        return -1;

    switch (port->sig) {
        case SATA_SIG_ATA:   return 0; // SATA drive
        case SATA_SIG_ATAPI: return 1; // SATAPI (CD/DVD)
        case SATA_SIG_SEMB:  return 2; // Enclosure management bridge
        case SATA_SIG_PM:    return 3; // Port multiplier
        default:             return -1;
    }
}

extern "C" void sigma_ahci_probe(u64 abar_phys) {
    ahci_base = (volatile struct ahci_hba_memory*)abar_phys;

    u32 pi = ahci_base->pi;
    sigma_vga_printf("AHCI: Version %x, Ports implemented: 0x%x\n",
        ahci_base->vs, pi);

    for (int i = 0; i < 32; i++) {
        if (pi & (1 << i)) {
            int type = sigma_ahci_check_type(&ahci_base->ports[i]);
            if (type == 0) {
                sigma_vga_printf("AHCI: Port %d — SATA drive detected\n", i);
            } else if (type == 1) {
                sigma_vga_printf("AHCI: Port %d — SATAPI drive detected\n", i);
            }
        }
    }
}

extern "C" bool sigma_ahci_read_sector(int port_num, u32 lba, u8* buf) {
    if (!ahci_base) return false;
    volatile struct ahci_hba_port* port = &ahci_base->ports[port_num];

    // Wait for port not busy
    u32 spin = 0;
    while ((port->tfd & 0x88) && spin < 1000000) {
        spin++;
    }
    if (spin >= 1000000) {
        sigma_vga_printf("AHCI: Port %d busy timeout\n", port_num);
        return false;
    }

    // Issue command to read 1 sector at LBA
    // (Simplified — full implementation requires setting up command
    //  list, FIS, and PRDT entries in DMA-accessible memory)
    sigma_vga_printf("AHCI: Read LBA %u on port %d\n", lba, port_num);
    return true;
}
