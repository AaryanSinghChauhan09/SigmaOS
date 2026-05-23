/*
 * Σ SigmaOS — sigma_e1000: Sovereign Intel E1000 Gigabit Ethernet Driver
 * Zero-Dependency: No libc. Raw PCI memory-mapped I/O.
 * Absorbs: Linux e1000/e1000e architecture.
 */

typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define E1000_REG_CTRL     0x00000
#define E1000_REG_STATUS   0x00008
#define E1000_REG_EEPROM   0x00014
#define E1000_REG_CTRL_EXT 0x00018
#define E1000_REG_IMC      0x000D8
#define E1000_REG_RCTL     0x00100
#define E1000_REG_TCTL     0x00400
#define E1000_REG_RAL      0x05400
#define E1000_REG_RAH      0x05404

static u8* e1000_mmio_base = nullptr;
static u8  mac_addr[6];

static void e1000_write(u16 offset, u32 val) {
    *((volatile u32*)(e1000_mmio_base + offset)) = val;
}

static u32 e1000_read(u16 offset) {
    return *((volatile u32*)(e1000_mmio_base + offset));
}

static void read_mac_address() {
    u32 ral = e1000_read(E1000_REG_RAL);
    u32 rah = e1000_read(E1000_REG_RAH);
    mac_addr[0] = ral & 0xFF;
    mac_addr[1] = (ral >> 8) & 0xFF;
    mac_addr[2] = (ral >> 16) & 0xFF;
    mac_addr[3] = (ral >> 24) & 0xFF;
    mac_addr[4] = rah & 0xFF;
    mac_addr[5] = (rah >> 8) & 0xFF;
}

extern "C" int sigma_e1000_init(u64 mmio_address) {
    sigma_vga_printf("[E1000] Initializing Intel E1000 NIC at 0x%X...\n", (u32)mmio_address);
    e1000_mmio_base = (u8*)(unsigned long)mmio_address;

    // Read MAC address
    read_mac_address();
    sigma_vga_printf("[E1000] MAC Address: %02x:%02x:%02x:%02x:%02x:%02x\n",
                     mac_addr[0], mac_addr[1], mac_addr[2],
                     mac_addr[3], mac_addr[4], mac_addr[5]);

    // Disable interrupts
    e1000_write(E1000_REG_IMC, 0xFFFFFFFF);

    // TODO: Setup TX/RX rings
    sigma_vga_printf("[E1000] Initialization complete.\n");
    return 0;
}
