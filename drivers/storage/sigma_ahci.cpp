/*
 * Σ SigmaOS — sigma_ahci: Sovereign SATA AHCI Storage Driver
 * Zero-Dependency: No libc. 
 * Absorbs: Linux libahci architecture.
 */

typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define AHCI_PORT_CMD_ST    (1 << 0)
#define AHCI_PORT_CMD_FRE   (1 << 4)
#define AHCI_PORT_CMD_FR    (1 << 14)
#define AHCI_PORT_CMD_CR    (1 << 15)

struct ahci_port_regs {
    u32 clb;
    u32 clbu;
    u32 fb;
    u32 fbu;
    u32 is;
    u32 ie;
    u32 cmd;
    u32 rsv0;
    u32 tfd;
    u32 sig;
    u32 ssts;
    u32 sctl;
    u32 serr;
    u32 sact;
    u32 ci;
    u32 sntf;
    u32 fbs;
    u32 rsv1[11];
    u32 vendor[4];
};

struct ahci_mem_regs {
    u32 cap;
    u32 ghc;
    u32 is;
    u32 pi;
    u32 vs;
    u32 ccc_ctl;
    u32 ccc_pts;
    u32 em_loc;
    u32 em_ctl;
    u32 cap2;
    u32 bohc;
    u8  rsv[0xA0 - 0x2C];
    u8  vendor[0x100 - 0xA0];
    ahci_port_regs ports[32];
};

static ahci_mem_regs* abar;

extern "C" int sigma_ahci_init(u64 mmio_address) {
    sigma_vga_printf("[AHCI] Initializing SATA AHCI Controller at 0x%X...\n", (u32)mmio_address);
    abar = (ahci_mem_regs*)(unsigned long)mmio_address;

    // Get implemented ports
    u32 pi = abar->pi;
    sigma_vga_printf("[AHCI] Ports implemented mask: 0x%X\n", pi);

    for (int i = 0; i < 32; i++) {
        if (pi & (1 << i)) {
            u32 sig = abar->ports[i].sig;
            u32 ssts = abar->ports[i].ssts;
            
            u8 det = ssts & 0x0F;
            u8 ipm = (ssts >> 8) & 0x0F;

            if (det == 3 && ipm == 1) {
                if (sig == 0x00000101) {
                    sigma_vga_printf("[AHCI] Port %d: SATA Drive Detected.\n", i);
                } else if (sig == 0xEB140101) {
                    sigma_vga_printf("[AHCI] Port %d: SATAPI Drive Detected.\n", i);
                }
            }
        }
    }

    sigma_vga_printf("[AHCI] Initialization complete.\n");
    return 0;
}
