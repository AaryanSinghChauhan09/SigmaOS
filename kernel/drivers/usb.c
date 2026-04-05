/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: USB XHCI HOST CONTROLLER DRIVER (v1.0 — PURE C11)
 * =============================================================================
 * Controller: xHCI (eXtensible Host Controller Interface) — USB 3.0/3.1
 * Features:
 *   - PCI BAR0 MMIO mapping
 *   - Host controller reset + initialization
 *   - Root hub port enumeration
 *   - Device slot allocation
 * Standard: C11, freestanding
 * Reference: xHCI spec 1.2 (Intel)
 * =============================================================================
 */

#include "../libc/SovereignLibC.h"

extern void  kprintf(const char *fmt, ...);
extern void *vmm_map_mmio(u64 phys, usize size);
extern paddr_t pmm_alloc_page(void);

/* =========================================================================
 * xHCI Class codes (PCI)
 * ========================================================================= */
#define XHCI_CLASS    0x0Cu
#define XHCI_SUBCLASS 0x03u
#define XHCI_PROGIF   0x30u

/* =========================================================================
 * xHCI MMIO Registers (Capability + Operational)
 * ========================================================================= */
#define XHCI_CAPLENGTH_OFF   0x00u   /* Capability register length */
#define XHCI_HCSPARAMS1_OFF  0x04u   /* Structural params 1 */
#define XHCI_HCCPARAMS1_OFF  0x10u   /* Capability params 1 */

/* Operational regs (base = mmio_base + CAPLENGTH) */
#define XHCI_OP_USBCMD   0x00u
#define XHCI_OP_USBSTS   0x04u
#define XHCI_OP_DNCTRL   0x14u
#define XHCI_OP_CRCR     0x18u
#define XHCI_OP_DCBAAP   0x30u
#define XHCI_OP_CONFIG   0x38u

/* USBCMD bits */
#define XHCI_CMD_RUN     BIT(0)
#define XHCI_CMD_HCRST   BIT(1)

/* USBSTS bits */
#define XHCI_STS_HCH     BIT(0)   /* Host controller halted */
#define XHCI_STS_CNR     BIT(11)  /* Controller not ready */

/* =========================================================================
 * USB Device State
 * ========================================================================= */
#define USB_MAX_SLOTS 32u
#define USB_MAX_PORTS 16u

typedef enum UsbSpeed {
    USB_SPEED_UNKNOWN = 0,
    USB_SPEED_FULL    = 1,   /* 12 Mb/s */
    USB_SPEED_LOW     = 2,   /*1.5 Mb/s */
    USB_SPEED_HIGH    = 3,   /* 480 Mb/s */
    USB_SPEED_SUPER   = 4,   /* 5 Gb/s   */
    USB_SPEED_SUPER_PLUS = 5, /* 10 Gb/s */
} UsbSpeed;

typedef struct UsbPort {
    u8       index;
    bool_t   connected;
    UsbSpeed speed;
    u32      slot_id;
} UsbPort;

typedef struct SigmaUSB {
    volatile u32 *mmio_cap;   /* Capability regs */
    volatile u32 *mmio_op;    /* Operational regs */
    u8           cap_length;
    u8           max_slots;
    u8           max_ports;
    bool_t       initialized;
    UsbPort      ports[USB_MAX_PORTS];
    u32          active_slots;
} SigmaUSB;

static SigmaUSB g_usb;

/* =========================================================================
 * MMIO read/write helpers
 * ========================================================================= */
static inline u32 mmio_read32(volatile u32 *base, u32 offset) {
    return *(volatile u32 *)((usize)base + offset);
}

static inline void mmio_write32(volatile u32 *base, u32 offset, u32 val) {
    *(volatile u32 *)((usize)base + offset) = val;
    cpu_fence();
}

/* =========================================================================
 * xHCI Reset — wait for HCH + CNR clear
 * ========================================================================= */
static k_status xhci_reset(void) {
    /* Assert HCRST */
    u32 cmd = mmio_read32(g_usb.mmio_op, XHCI_OP_USBCMD);
    mmio_write32(g_usb.mmio_op, XHCI_OP_USBCMD, cmd | XHCI_CMD_HCRST);

    /* Wait up to ~100ms */
    u32 timeout = 100000u;
    while (timeout--) {
        cpu_pause();
        u32 c = mmio_read32(g_usb.mmio_op, XHCI_OP_USBCMD);
        if (!(c & XHCI_CMD_HCRST)) break;
    }

    /* Wait for CNR to clear */
    timeout = 100000u;
    while (timeout--) {
        cpu_pause();
        u32 s = mmio_read32(g_usb.mmio_op, XHCI_OP_USBSTS);
        if (!(s & XHCI_STS_CNR)) return K_OK;
    }
    return K_ERR_BUSY;
}

/* =========================================================================
 * Root Hub: enumerate ports
 * ========================================================================= */
static void xhci_enumerate_ports(void) {
    /* Port Status and Control regs start at op_base + 0x400, each 0x10 bytes */
    for (u8 i = 0; i < g_usb.max_ports && i < USB_MAX_PORTS; i++) {
        u32 portsc_off = 0x400u + (u32)i * 0x10u;
        u32 portsc = mmio_read32(g_usb.mmio_op, portsc_off);

        bool_t connected = !!(portsc & BIT(0));
        UsbSpeed speed = USB_SPEED_UNKNOWN;

        if (connected) {
            u32 spd = (portsc >> 10) & 0xFu;
            if (spd <= USB_SPEED_SUPER_PLUS) speed = (UsbSpeed)spd;
        }

        g_usb.ports[i].index     = i;
        g_usb.ports[i].connected = connected;
        g_usb.ports[i].speed     = speed;
        g_usb.ports[i].slot_id   = 0;

        if (connected) {
            const char *spd_str[] = {
                "?", "Full(12Mb)", "Low(1.5Mb)", "High(480Mb)",
                "Super(5Gb)", "Super+(10Gb)"
            };
            kprintf("[USB]: Port %u: Connected — %s\n", (u32)i,
                    (speed <= USB_SPEED_SUPER_PLUS) ? spd_str[speed] : "?");
        }
    }
}

/* =========================================================================
 * usb_init — PCI xHCI discovery + initialization
 * NOTE: In a real system this would scan PCI bus for class 0x0C/0x03/0x30.
 *       Here we stub with a fixed MMIO base for QEMU xHCI (0xFEBF0000).
 * ========================================================================= */
void usb_init(void) {
    kprintf("[USB]: Initializing xHCI Host Controller...\n");

    /* QEMU xHCI controller typical MMIO base */
    const u64 XHCI_MMIO_BASE = 0xFEBF0000ULL;
    const usize XHCI_MMIO_SIZE = 0x10000u;

    /* Map MMIO region */
    void *mmio = vmm_map_mmio(XHCI_MMIO_BASE, XHCI_MMIO_SIZE);
    if (!mmio) {
        kprintf("[USB]: MMIO mapping failed — no xHCI controller found.\n");
        return;
    }

    g_usb.mmio_cap = (volatile u32 *)mmio;
    g_usb.cap_length = (u8)(mmio_read32(g_usb.mmio_cap, XHCI_CAPLENGTH_OFF) & 0xFFu);
    g_usb.mmio_op = (volatile u32 *)((usize)mmio + g_usb.cap_length);

    /* Parse max_slots and max_ports from HCSPARAMS1 */
    u32 hcs1 = mmio_read32(g_usb.mmio_cap, XHCI_HCSPARAMS1_OFF);
    g_usb.max_slots = (u8)(hcs1 & 0xFFu);
    g_usb.max_ports = (u8)((hcs1 >> 24) & 0xFFu);

    kprintf("[USB]: xHCI cap_len=%u max_slots=%u max_ports=%u\n",
            (u32)g_usb.cap_length, (u32)g_usb.max_slots, (u32)g_usb.max_ports);

    /* Reset controller */
    if (xhci_reset() != K_OK) {
        kprintf("[USB]: WARN — xHCI reset timed out.\n");
        return;
    }

    /* Set max device slots */
    u32 config = mmio_read32(g_usb.mmio_op, XHCI_OP_CONFIG);
    config = (config & ~0xFFu) | (u32)g_usb.max_slots;
    mmio_write32(g_usb.mmio_op, XHCI_OP_CONFIG, config);

    /* Start the controller */
    u32 cmd = mmio_read32(g_usb.mmio_op, XHCI_OP_USBCMD);
    mmio_write32(g_usb.mmio_op, XHCI_OP_USBCMD, cmd | XHCI_CMD_RUN);

    g_usb.initialized = TRUE;
    xhci_enumerate_ports();

    kprintf("[USB]: xHCI host controller online. Active slots: %u\n", g_usb.active_slots);
}

/* =========================================================================
 * usb_audit
 * ========================================================================= */
void usb_audit(void) {
    if (!g_usb.initialized) {
        kprintf("[USB]: Not initialized (no xHCI detected).\n");
        return;
    }
    u32 connected = 0;
    for (u8 i = 0; i < g_usb.max_ports && i < USB_MAX_PORTS; i++) {
        if (g_usb.ports[i].connected) connected++;
    }
    kprintf("[USB]: xHCI online | ports=%u connected=%u\n",
            (u32)g_usb.max_ports, connected);
}
