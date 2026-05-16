/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: INTEL E1000 DRIVER (v1.0)
 * =============================================================================
 * Principles: Zero-Copy Networking, Silicon-Direct Communication.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

#define E1000_REG_TDT   0x3818 /* Transmit Descriptor Tail */
#define E1000_REG_STATUS 0x0008

typedef struct {
    sigma_u64 addr;
    sigma_u16 length;
    sigma_u8  cso;
    sigma_u8  cmd;
    sigma_u8  status;
    sigma_u8  css;
    sigma_u16 special;
} __attribute__((packed)) tx_desc_t;

static tx_desc_t* tx_ring;
static sigma_u32 tx_tail = 0;

extern void mmio_write32(sigma_u64 addr, sigma_u32 val);
extern sigma_u32 mmio_read32(sigma_u64 addr);

void e1000_send_packet(void* data, sigma_u16 len) {
    tx_desc_t* desc = &tx_ring[tx_tail];
    desc->addr = (sigma_u64)(sigma_usize)data;
    desc->length = len;
    desc->cmd = (1 << 0) | (1 << 3); /* EOP | RS */
    desc->status = 0;

    tx_tail = (tx_tail + 1) % 256;
    /* mmio_write32(E1000_REG_TDT, tx_tail); */
}
