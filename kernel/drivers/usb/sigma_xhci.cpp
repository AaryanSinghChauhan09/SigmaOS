/**
 * =========================================================================
 * Σ SIGMAOS: xHCI USB 3.0 CONTROLLER DRIVER
 * =========================================================================
 * Sovereign, zero-dependency implementation of the eXtensible Host 
 * Controller Interface (xHCI).
 * =========================================================================
 */

#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace USB {

/* --- xHCI Data Structures --- */

/* Transfer Request Block (TRB) - 16 bytes */
struct XhciTRB {
    sigma_u32 param_lo;
    sigma_u32 param_hi;
    sigma_u32 status;
    sigma_u32 control;
} __attribute__((packed));

/* Event Ring Segment Table (ERST) Entry - 16 bytes */
struct XhciERSTEntry {
    sigma_u64 ring_segment_base;
    sigma_u16 ring_segment_size;
    sigma_u16 reserved1;
    sigma_u32 reserved2;
} __attribute__((packed));

/* Device Context Base Address Array (DCBAA) */
struct XhciDCBAA {
    sigma_u64 pointers[256]; /* Slot 0 is scratchpad array, 1-255 are device contexts */
} __attribute__((packed));

/* --- xHCI Registers --- */

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

/* Port Register Set (array starting at op_regs + 0x400) */
struct XhciPortRegs {
    sigma_u32 portsc;
    sigma_u32 portpmsc;
    sigma_u32 portli;
    sigma_u32 porthlpmc;
} __attribute__((packed));

struct XhciInterrupterRegs {
    sigma_u32 iman;
    sigma_u32 imod;
    sigma_u32 erstsz;
    sigma_u32 reserved;
    sigma_u64 erstba;
    sigma_u64 erdp;
} __attribute__((packed));

/* --- Controller State --- */

class XhciController {
public:
    static XhciController& getInstance() {
        static XhciController instance;
        return instance;
    }

    /* PCI Enumeration Hook */
    sigma_status probePCI(sigma_u16 vendor_id, sigma_u16 device_id, sigma_u8 class_code, sigma_u8 subclass) {
        if (class_code == 0x0C && subclass == 0x03) {
            sigma_log_info("[xHCI] Found USB 3.0 Controller (Ven: 0x%04X, Dev: 0x%04X)", vendor_id, device_id);
            /* In reality, we'd read BAR0 here to get the MMIO base */
            return init(0xFA000000); /* Simulated MMIO base */
        }
        return K_ERR_NOTFOUND;
    }

    sigma_status init(sigma_u64 mmio_base) {
        m_base_address = mmio_base;
        m_cap_regs = (XhciCapRegs*)mmio_base;
        m_op_regs  = (XhciOpRegs*)(mmio_base + m_cap_regs->caplength);
        
        sigma_u32 hcsp1 = m_cap_regs->hcsparams1;
        m_num_slots = hcsp1 & 0xFF;
        m_num_ports = (hcsp1 >> 24) & 0xFF;

        sigma_log_info("[xHCI] Base: 0x%llX | Slots: %u | Ports: %u", 
                       (unsigned long long)mmio_base, m_num_slots, m_num_ports);

        /* 1. Halt Controller */
        m_op_regs->usbcmd &= ~1; /* Clear Run/Stop */
        while ((m_op_regs->usbsts & 1) == 0); /* Wait for halt */

        /* 2. Reset Controller */
        m_op_regs->usbcmd |= 2; /* Set HCRST */
        while ((m_op_regs->usbcmd & 2) != 0); /* Wait for reset completion */
        while ((m_op_regs->usbsts & (1 << 29)) != 0); /* Wait for Controller Not Ready to clear */

        /* 3. Program Max Device Slots Enabled */
        m_op_regs->config = m_num_slots;

        /* 4. Setup DCBAA (Device Context Base Address Array) */
        /* Mock physical allocation for now */
        m_op_regs->dcbaap = 0x2000000; 

        /* 5. Setup Command Ring */
        /* Link TRB to itself to form a ring */
        m_cmd_ring[255].param_lo = 0x3000000; /* Physical address of cmd ring */
        m_cmd_ring[255].control = (6 << 10) | 2; /* TRB Type 6 (Link), Toggle Cycle */
        m_op_regs->crcr = 0x3000000 | 1; /* Set physical address + Ring Cycle State */

        /* 6. Setup Interrupter (Event Ring) */
        /* Runtime registers are located at mmio_base + rtsoff */
        sigma_u64 rt_base = mmio_base + m_cap_regs->rtsoff;
        XhciInterrupterRegs* ir0 = (XhciInterrupterRegs*)(rt_base + 0x20);

        m_erst[0].ring_segment_base = 0x4000000; /* Mock event ring buffer physical address */
        m_erst[0].ring_segment_size = 256; /* TRBs */
        
        ir0->erstsz = 1;
        ir0->erstba = 0x5000000; /* Mock ERST physical address */
        ir0->erdp   = 0x4000000; /* Dequeue pointer matches ring start */
        ir0->iman  |= 3;         /* Enable interrupts */

        /* 7. Start Controller */
        m_op_regs->usbcmd |= 1; /* Set Run/Stop */
        while ((m_op_regs->usbsts & 1) != 0); /* Wait for running state */

        m_initialized = SIGMA_TRUE;
        sigma_log("[xHCI] Initialization complete. Polling ports...");
        pollPorts();

        return K_OK;
    }

    void pollPorts() {
        if (!m_initialized) return;

        /* Port registers start at op_regs + 0x400 */
        sigma_u64 port_base = (sigma_u64)m_op_regs + 0x400;

        for (sigma_u32 i = 0; i < m_num_ports; i++) {
            XhciPortRegs* port = (XhciPortRegs*)(port_base + (i * 0x10));
            sigma_u32 portsc = port->portsc;

            /* Check Current Connect Status (Bit 0) */
            if (portsc & 1) {
                sigma_u32 speed = (portsc >> 10) & 0xF;
                sigma_log_info("[xHCI] Device detected on Port %u (Speed ID: %u)", i + 1, speed);
                
                /* Reset port if not enabled */
                if ((portsc & 2) == 0) {
                    sigma_log_info("[xHCI] Resetting Port %u...", i + 1);
                    port->portsc = portsc | (1 << 4); /* Set Port Reset */
                }
            }
        }
    }

private:
    XhciController() : m_initialized(SIGMA_FALSE) {}

    sigma_u64             m_base_address;
    volatile XhciCapRegs* m_cap_regs;
    volatile XhciOpRegs*  m_op_regs;
    sigma_u32             m_num_ports;
    sigma_u32             m_num_slots;
    sigma_bool            m_initialized;
    
    /* Mock memory for structures until proper DMA allocator is added */
    XhciDCBAA             m_dcbaa;
    XhciTRB               m_cmd_ring[256];
    XhciERSTEntry         m_erst[1];
    XhciTRB               m_evt_ring[256];
};

} // namespace USB
} // namespace SigmaOS

/* --- C API Wrappers --- */
extern "C" {
    sigma_status xhci_probe_pci(sigma_u16 vid, sigma_u16 did, sigma_u8 class_code, sigma_u8 subclass) {
        return SigmaOS::USB::XhciController::getInstance().probePCI(vid, did, class_code, subclass);
    }
}

