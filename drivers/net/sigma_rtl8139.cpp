/*
 * Σ SigmaOS — sigma_rtl8139: Sovereign Realtek 8139 Fast Ethernet Driver
 * Zero-Dependency: No libc. Raw PCI PIO/MMIO.
 * Absorbs: Linux 8139too architecture.
 */

typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define RTL8139_MAC0       0x00
#define RTL8139_CR         0x37
#define RTL8139_CAPR       0x38
#define RTL8139_IMR        0x3C
#define RTL8139_ISR        0x3E
#define RTL8139_TCR        0x40
#define RTL8139_RCR        0x44

static u16 rtl_iobase = 0;
static u8  rtl_mac[6];

static inline void outb(u16 port, u8 v)  { __asm__ volatile("outb %0,%1"::"a"(v),"dN"(port)); }
static inline u8  inb(u16 port)          { u8 v; __asm__ volatile("inb %1,%0":"=a"(v):"dN"(port)); return v; }
static inline void outl(u16 port, u32 v) { __asm__ volatile("outl %0,%1"::"a"(v),"dN"(port)); }

extern "C" int sigma_rtl8139_init(u16 iobase) {
    sigma_vga_printf("[RTL8139] Initializing NIC at PIO 0x%X...\n", iobase);
    rtl_iobase = iobase;

    // Power on
    outb(rtl_iobase + 0x52, 0x00);

    // Software reset
    outb(rtl_iobase + RTL8139_CR, 0x10);
    while ((inb(rtl_iobase + RTL8139_CR) & 0x10) != 0) {}

    // Read MAC
    for(int i = 0; i < 6; i++) {
        rtl_mac[i] = inb(rtl_iobase + RTL8139_MAC0 + i);
    }
    sigma_vga_printf("[RTL8139] MAC Address: %02x:%02x:%02x:%02x:%02x:%02x\n",
                     rtl_mac[0], rtl_mac[1], rtl_mac[2],
                     rtl_mac[3], rtl_mac[4], rtl_mac[5]);

    // Enable TX/RX
    outb(rtl_iobase + RTL8139_CR, 0x0C);
    
    // Accept Broadcast/Match MAC/Multicast
    outl(rtl_iobase + RTL8139_RCR, 0x8F);

    sigma_vga_printf("[RTL8139] Initialization complete.\n");
    return 0;
}
