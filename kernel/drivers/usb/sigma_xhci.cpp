/*
 * Σ SigmaOS Zenith — USB XHCI Host Controller Driver (Foundation)
 * Absorbs: Linux drivers/usb/host/xhci.c
 * Zero-Dependency: No libc.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* ─────────── xHCI Capability Registers ─────────── */

struct __attribute__((packed)) xhci_cap_regs {
    u8  caplength;
    u8  rsvd;
    u16 hciversion;
    u32 hcsparams1;   // Structural Parameters 1
    u32 hcsparams2;   // Structural Parameters 2
    u32 hcsparams3;   // Structural Parameters 3
    u32 hccparams1;   // Capability Parameters 1
    u32 dboff;        // Doorbell Offset
    u32 rtsoff;       // Runtime Register Space Offset
    u32 hccparams2;   // Capability Parameters 2
};

/* ─────────── xHCI Operational Registers ─────────── */

struct __attribute__((packed)) xhci_op_regs {
    u32 usbcmd;      // USB Command
    u32 usbsts;      // USB Status
    u32 pagesize;
    u32 rsvd1[2];
    u32 dnctrl;      // Device Notification Control
    u64 crcr;        // Command Ring Control
    u32 rsvd2[4];
    u64 dcbaap;      // Device Context Base Address Array Pointer
    u32 config;      // Configure
};

/* ─────────── USB Device Descriptor ─────────── */

struct __attribute__((packed)) usb_device_descriptor {
    u8  bLength;
    u8  bDescriptorType;
    u16 bcdUSB;
    u8  bDeviceClass;
    u8  bDeviceSubClass;
    u8  bDeviceProtocol;
    u8  bMaxPacketSize0;
    u16 idVendor;
    u16 idProduct;
    u16 bcdDevice;
    u8  iManufacturer;
    u8  iProduct;
    u8  iSerialNumber;
    u8  bNumConfigurations;
};

/* USBCMD bits */
#define XHCI_CMD_RUN    (1 << 0)
#define XHCI_CMD_HCRST  (1 << 1)

/* USBSTS bits */
#define XHCI_STS_HCH   (1 << 0)  // HC Halted
#define XHCI_STS_CNR   (1 << 11) // Controller Not Ready

static volatile struct xhci_cap_regs*  xhci_cap  = 0;
static volatile struct xhci_op_regs*   xhci_op   = 0;

extern "C" bool sigma_xhci_init(u64 mmio_base) {
    xhci_cap = (volatile struct xhci_cap_regs*)mmio_base;

    sigma_vga_printf("xHCI: Version %x, CapLength %u\n",
        xhci_cap->hciversion, xhci_cap->caplength);

    u32 max_slots  = (xhci_cap->hcsparams1) & 0xFF;
    u32 max_intrs  = (xhci_cap->hcsparams1 >> 8) & 0x7FF;
    u32 max_ports  = (xhci_cap->hcsparams1 >> 24) & 0xFF;
    sigma_vga_printf("xHCI: MaxSlots=%u, MaxIntrs=%u, MaxPorts=%u\n",
        max_slots, max_intrs, max_ports);

    // Locate operational registers
    xhci_op = (volatile struct xhci_op_regs*)(mmio_base + xhci_cap->caplength);

    // 1. Stop the controller
    xhci_op->usbcmd &= ~XHCI_CMD_RUN;
    u32 spin = 0;
    while (!(xhci_op->usbsts & XHCI_STS_HCH) && spin < 100000) spin++;
    if (spin >= 100000) {
        sigma_vga_printf("xHCI: Failed to halt controller\n");
        return false;
    }

    // 2. Reset the controller
    xhci_op->usbcmd |= XHCI_CMD_HCRST;
    spin = 0;
    while ((xhci_op->usbcmd & XHCI_CMD_HCRST) && spin < 100000) spin++;
    while ((xhci_op->usbsts & XHCI_STS_CNR)   && spin < 200000) spin++;

    sigma_vga_printf("xHCI: Controller reset complete\n");

    // 3. Set Max Device Slots
    xhci_op->config = max_slots;

    sigma_vga_printf("xHCI: Initialized with %u device slots\n", max_slots);
    return true;
}

extern "C" void sigma_xhci_port_status(u32 port) {
    if (!xhci_op) return;
    // Port status/control registers start at operational base + 0x400
    volatile u32* portsc = (volatile u32*)((u64)xhci_op + 0x400 + (port * 0x10));
    u32 val = *portsc;

    bool connected = (val & 1) != 0;
    bool enabled   = (val & (1 << 1)) != 0;
    u32  speed     = (val >> 10) & 0xF;

    const char* speed_str = "Unknown";
    if (speed == 1) speed_str = "Full (12 Mbps)";
    if (speed == 2) speed_str = "Low (1.5 Mbps)";
    if (speed == 3) speed_str = "High (480 Mbps)";
    if (speed == 4) speed_str = "Super (5 Gbps)";

    sigma_vga_printf("xHCI Port %u: %s, %s, Speed=%s\n",
        port,
        connected ? "Connected" : "Disconnected",
        enabled ? "Enabled" : "Disabled",
        speed_str);
}
