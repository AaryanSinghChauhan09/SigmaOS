#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: USB XHCI HOST CONTROLLER DRIVER (v1.0 â€ PURE C11)
 * =============================================================================
 * Controller: xHCI (eXtensible Host Controller Interface) â€ USB 3.0/3.1
 * Features:
 *   - PCI BAR0 MMIO mapping
 *   - Host controller reset + initialization
 *   - Root hub port enumeration
 *   - Device slot allocation
 * Standard: C11, freestanding
 * Reference: xHCI spec 1.2 (Intel)
 * =============================================================================
 */

#include "core/sigma_kernel_types.h"

<<<<<<< HEAD:suites/S04_HAL_Input/shard_usb.c
extern void  ksigma_printf(const char *fmt, ...);
extern void *vmm_map_mmio(u64 phys, usize size);
extern paddr_t pmm_alloc_page(void);
=======
extern void  kprintf(const char *fmt, ...);
extern void *vmm_map_mmio(sigma_u64 phys, sigma_usize size);
extern sigma_paddr_t pmm_alloc_page(void);
>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/drivers/usb.c

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
    sigma_u8       index;
    sigma_bool   connected;
    UsbSpeed speed;
    sigma_u32      slot_id;
} UsbPort;

typedef struct SigmaUSB {
    volatile sigma_u32 *mmio_cap;   /* Capability regs */
    volatile sigma_u32 *mmio_op;    /* Operational regs */
    sigma_u8           cap_length;
    sigma_u8           max_slots;
    sigma_u8           max_ports;
    sigma_bool       initialized;
    UsbPort      ports[USB_MAX_PORTS];
    sigma_u32          active_slots;
} SigmaUSB;

static SigmaUSB g_usb;

/* =========================================================================
 * MMIO read/write helpers
 * ========================================================================= */
static inline sigma_u32 mmio_read32(volatile sigma_u32 *base, sigma_u32 offset) {
    return *(volatile sigma_u32 *)((sigma_usize)base + offset);
}

static inline void mmio_write32(volatile sigma_u32 *base, sigma_u32 offset, sigma_u32 val) {
    *(volatile sigma_u32 *)((sigma_usize)base + offset) = val;
    cpu_fence();
}

/* =========================================================================
 * xHCI Reset â€ wait for HCH + CNR clear
 * ========================================================================= */
static sigma_status xhci_reset(void) {
    /* Assert HCRST */
    sigma_u32 cmd = mmio_read32(g_usb.mmio_op, XHCI_OP_USBCMD);
    mmio_write32(g_usb.mmio_op, XHCI_OP_USBCMD, cmd | XHCI_CMD_HCRST);

    /* Wait up to ~100ms */
    sigma_u32 timeout = 100000u;
    while (timeout--) {
        cpu_pause();
        sigma_u32 c = mmio_read32(g_usb.mmio_op, XHCI_OP_USBCMD);
        if (!(c & XHCI_CMD_HCRST)) break;
    }

    /* Wait for CNR to clear */
    timeout = 100000u;
    while (timeout--) {
        cpu_pause();
        sigma_u32 s = mmio_read32(g_usb.mmio_op, XHCI_OP_USBSTS);
        if (!(s & XHCI_STS_CNR)) return K_OK;
    }
    return K_ERR_BUSY;
}

/* =========================================================================
 * Root Hub: enumerate ports
 * ========================================================================= */
static void xhci_enumerate_ports(void) {
    /* Port Status and Control regs start at op_base + 0x400, each 0x10 bytes */
    for (sigma_u8 i = 0; i < g_usb.max_ports && i < USB_MAX_PORTS; i++) {
        sigma_u32 portsc_off = 0x400u + (sigma_u32)i * 0x10u;
        sigma_u32 portsc = mmio_read32(g_usb.mmio_op, portsc_off);

        sigma_bool connected = !!(portsc & BIT(0));
        UsbSpeed speed = USB_SPEED_UNKNOWN;

        if (connected) {
            sigma_u32 spd = (portsc >> 10) & 0xFu;
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
<<<<<<< HEAD:suites/S04_HAL_Input/shard_usb.c
            ksigma_printf("[USB]: Port %u: Connected — %s\n", (u32)i,
=======
            kprintf("[USB]: Port %u: Connected â€ %s\n", (sigma_u32)i,
>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/drivers/usb.c
                    (speed <= USB_SPEED_SUPER_PLUS) ? spd_str[speed] : "?");
        }
    }
}

/* =========================================================================
 * usb_init â€ PCI xHCI discovery + initialization
 * NOTE: In a real system this would scan PCI bus for class 0x0C/0x03/0x30.
 *       Here we stub with a fixed MMIO base for QEMU xHCI (0xFEBF0000).
 * ========================================================================= */
void usb_init(void) {
    ksigma_printf("[USB]: Initializing xHCI Host Controller...\n");

    /* QEMU xHCI controller typical MMIO base */
    const sigma_u64 XHCI_MMIO_BASE = 0xFEBF0000ULL;
    const sigma_usize XHCI_MMIO_SIZE = 0x10000u;

    /* Map MMIO region */
    void *mmio = vmm_map_mmio(XHCI_MMIO_BASE, XHCI_MMIO_SIZE);
    if (!mmio) {
<<<<<<< HEAD:suites/S04_HAL_Input/shard_usb.c
        ksigma_printf("[USB]: MMIO mapping failed — no xHCI controller found.\n");
=======
        kprintf("[USB]: MMIO mapping failed â€ no xHCI controller found.\n");
>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/drivers/usb.c
        return;
    }

    g_usb.mmio_cap = (volatile sigma_u32 *)mmio;
    g_usb.cap_length = (sigma_u8)(mmio_read32(g_usb.mmio_cap, XHCI_CAPLENGTH_OFF) & 0xFFu);
    g_usb.mmio_op = (volatile sigma_u32 *)((sigma_usize)mmio + g_usb.cap_length);

    /* Parse max_slots and max_ports from HCSPARAMS1 */
    sigma_u32 hcs1 = mmio_read32(g_usb.mmio_cap, XHCI_HCSPARAMS1_OFF);
    g_usb.max_slots = (sigma_u8)(hcs1 & 0xFFu);
    g_usb.max_ports = (sigma_u8)((hcs1 >> 24) & 0xFFu);

<<<<<<< HEAD:suites/S04_HAL_Input/shard_usb.c
    ksigma_printf("[USB]: xHCI cap_len=%u max_slots=%u max_ports=%u\n",
            (u32)g_usb.cap_length, (u32)g_usb.max_slots, (u32)g_usb.max_ports);

    /* Reset controller */
    if (xhci_reset() != K_OK) {
        ksigma_printf("[USB]: WARN — xHCI reset timed out.\n");
=======
    kprintf("[USB]: xHCI cap_len=%u max_slots=%u max_ports=%u\n",
            (sigma_u32)g_usb.cap_length, (sigma_u32)g_usb.max_slots, (sigma_u32)g_usb.max_ports);

    /* Reset controller */
    if (xhci_reset() != K_OK) {
        kprintf("[USB]: WARN â€ xHCI reset timed out.\n");
>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/drivers/usb.c
        return;
    }

    /* Set max device slots */
    sigma_u32 config = mmio_read32(g_usb.mmio_op, XHCI_OP_CONFIG);
    config = (config & ~0xFFu) | (sigma_u32)g_usb.max_slots;
    mmio_write32(g_usb.mmio_op, XHCI_OP_CONFIG, config);

    /* Start the controller */
    sigma_u32 cmd = mmio_read32(g_usb.mmio_op, XHCI_OP_USBCMD);
    mmio_write32(g_usb.mmio_op, XHCI_OP_USBCMD, cmd | XHCI_CMD_RUN);

    g_usb.initialized = SIGMA_TRUE;
    xhci_enumerate_ports();

    ksigma_printf("[USB]: xHCI host controller online. Active slots: %u\n", g_usb.active_slots);
}

/* =========================================================================
 * usb_audit
 * ========================================================================= */
void usb_audit(void) {
    if (!g_usb.initialized) {
        ksigma_printf("[USB]: Not initialized (no xHCI detected).\n");
        return;
    }
    sigma_u32 connected = 0;
    for (sigma_u8 i = 0; i < g_usb.max_ports && i < USB_MAX_PORTS; i++) {
        if (g_usb.ports[i].connected) connected++;
    }
<<<<<<< HEAD:suites/S04_HAL_Input/shard_usb.c
    ksigma_printf("[USB]: xHCI online | ports=%u connected=%u\n",
            (u32)g_usb.max_ports, connected);
=======
    kprintf("[USB]: xHCI online | ports=%u connected=%u\n",
            (sigma_u32)g_usb.max_ports, connected);
>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/drivers/usb.c
}
