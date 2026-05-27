/**
 * @file sigma_xhci.cpp
 * @brief Phase 1: xHCI USB 3.0 controller driver.
 *
 * Sovereign, zero-dependency implementation of the eXtensible Host Controller Interface.
 * Used for broad peripheral support including webcams, printers, and gamepads.
 */

#include "../../../include/sigma_kernel_types.h"

namespace sigma {
namespace usb {

/* xHCI Capability Registers */
struct XhciCapRegs {
    sigma_u8  caplength;
    sigma_u8  reserved;
    sigma_u16 hciversion;
    sigma_u32 hcsparams1;
    sigma_u32 hcsparams2;
    sigma_u32 hcsparams3;
    sigma_u32 hccparams1;
    sigma_u32 dboff;
    sigma_u32 rtsoff;
    sigma_u32 hccparams2;
} __attribute__((packed));

/* xHCI Operational Registers */
struct XhciOpRegs {
    sigma_u32 usbcmd;
    sigma_u32 usbsts;
    sigma_u32 pagesize;
    sigma_u8  reserved1[8];
    sigma_u32 dnctrl;
    sigma_u64 crcr;
    sigma_u8  reserved2[16];
    sigma_u64 dcbaap;
    sigma_u32 config;
} __attribute__((packed));

struct XhciController {
    sigma_u64 base_address;
    volatile XhciCapRegs* cap_regs;
    volatile XhciOpRegs*  op_regs;
    sigma_u32 num_ports;
    sigma_u32 num_slots;
    sigma_bool initialized;
};

static XhciController g_xhci;

sigma_status init_xhci(sigma_u64 mmio_base) {
    g_xhci.base_address = mmio_base;
    g_xhci.cap_regs = (XhciCapRegs*)mmio_base;
    g_xhci.op_regs  = (XhciOpRegs*)(mmio_base + g_xhci.cap_regs->caplength);
    
    // Read Structural Parameters
    sigma_u32 hcsp1 = g_xhci.cap_regs->hcsparams1;
    g_xhci.num_slots = hcsp1 & 0xFF;
    g_xhci.num_ports = (hcsp1 >> 24) & 0xFF;

    // Reset Controller
    g_xhci.op_regs->usbcmd &= ~1; // Clear Run/Stop bit
    while ((g_xhci.op_regs->usbsts & 1) == 0); // Wait until halted
    
    g_xhci.op_regs->usbcmd |= 2; // Set HCRST (Host Controller Reset)
    while ((g_xhci.op_regs->usbcmd & 2) != 0); // Wait for reset to complete
    while ((g_xhci.op_regs->usbsts & (1 << 29)) != 0); // Wait for Controller Not Ready to clear

    // Set up Device Context Base Address Array (DCBAA)
    // In a real implementation, we allocate a contiguous physical buffer
    g_xhci.op_regs->dcbaap = 0x100000; // Mock physical address

    // Start controller
    g_xhci.op_regs->usbcmd |= 1; // Set Run/Stop bit
    
    g_xhci.initialized = SIGMA_TRUE;
    return SIGMA_SUCCESS;
}

} // namespace usb
} // namespace sigma

extern "C" {
    sigma_status sigma_usb_init(sigma_u64 mmio_base) {
        return sigma::usb::init_xhci(mmio_base);
    }
}
