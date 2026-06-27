/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * driver_interface.h — SigmaOS driver shard registration protocol
 *
 * Every hardware driver runs as a user-space shard.
 * It registers with the kernel driver bus via IPC.
 * The kernel routes IRQs and MMIO mappings to the correct shard.
 *
 * Inspired by: Fuchsia DDK, Genode device drivers, L4Re DDE
 */

#pragma once
#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
namespace sigmaos {
namespace drivers {
#endif

/* ── Capability bits a driver advertises ──────────────────────────────────── */

typedef uint32_t DriverCapability;
#define DRIVER_CAP_BLOCK_IO    (1u << 0)
#define DRIVER_CAP_NET_IO      (1u << 1)
#define DRIVER_CAP_GRAPHICS    (1u << 2)
#define DRIVER_CAP_AUDIO       (1u << 3)
#define DRIVER_CAP_HID_INPUT   (1u << 4)
#define DRIVER_CAP_POWER_MGMT  (1u << 5)
#define DRIVER_CAP_USB_HOST    (1u << 6)
#define DRIVER_CAP_PCIE        (1u << 7)

/* ── Driver registration (sent to kernel bus shard on startup) ────────────── */

typedef struct sigma_driver_reg {
    uint32_t        shard_id;
    char            name[64];
    DriverCapability caps;
    uint32_t        vendor_id;
    uint32_t        device_id;
    uint32_t        class_code;   /* PCI class (24-bit) or USB class */
    uintptr_t       mmio_base;    /* physical MMIO base (0 if not mapped yet) */
    size_t          mmio_size;
    uint32_t        irq_line;     /* IRQ number (0 = MSI/MSI-X, query bus) */
    uint32_t        flags;
#define DRIVER_FLAG_DMA_CAPABLE  (1u << 0)
#define DRIVER_FLAG_MSI_CAPABLE  (1u << 1)
#define DRIVER_FLAG_HOT_PLUG     (1u << 2)
} sigma_driver_reg_t;

/* ── IPC opcodes for driver ↔ bus communication ───────────────────────────── */

typedef enum sigma_driver_opcode {
    SIGMA_DRV_OP_REGISTER    = 0x1000,
    SIGMA_DRV_OP_UNREGISTER  = 0x1001,
    SIGMA_DRV_OP_IRQ_NOTIFY  = 0x1002,   /* kernel → driver: IRQ fired */
    SIGMA_DRV_OP_MMIO_MAP    = 0x1003,   /* driver requests MMIO mapping */
    SIGMA_DRV_OP_DMA_ALLOC   = 0x1004,   /* allocate DMA-coherent memory */
    SIGMA_DRV_OP_DMA_FREE    = 0x1005,
    SIGMA_DRV_OP_PROBE       = 0x1006,   /* kernel asks: can you handle device X? */
    SIGMA_DRV_OP_PROBE_ACK   = 0x1007,
    SIGMA_DRV_OP_SUSPEND     = 0x1008,   /* power management: suspend */
    SIGMA_DRV_OP_RESUME      = 0x1009,
    SIGMA_DRV_OP_BLOCK_READ  = 0x2000,
    SIGMA_DRV_OP_BLOCK_WRITE = 0x2001,
    SIGMA_DRV_OP_BLOCK_FLUSH = 0x2002,
    SIGMA_DRV_OP_BLOCK_TRIM  = 0x2003,
} sigma_driver_opcode_t;

/* ── IPC message (fixed-size, cache-line aligned) ─────────────────────────── */

typedef struct __attribute__((aligned(64))) sigma_ipc_msg {
    sigma_driver_opcode_t opcode;
    uint32_t  src_shard;
    uint32_t  dst_shard;
    uint64_t  request_id;
    uint64_t  arg1;
    uint64_t  arg2;
    uint64_t  arg3;
    uint8_t   payload[24];
} sigma_ipc_msg_t;

/* ── Block request (used for BLOCK_READ/WRITE) ────────────────────────────── */

typedef struct sigma_block_req {
    uint64_t    lba;
    uint32_t    block_count;
    uint32_t    block_size;
    uint64_t    buffer_pa;   /* DMA physical address */
    uint32_t    caller_shard;
    uint64_t    request_id;
} sigma_block_req_t;

/* ── C driver bus API ─────────────────────────────────────────────────────── */

int  sigma_driver_register(const sigma_driver_reg_t *reg);
int  sigma_driver_unregister(uint32_t shard_id);
int  sigma_driver_find(DriverCapability cap, uint32_t class_code, uint32_t *out_shard);
int  sigma_driver_send(uint32_t dst_shard, const sigma_ipc_msg_t *msg);
int  sigma_driver_recv(sigma_ipc_msg_t *msg, int timeout_ms);
void sigma_driver_bus_enumerate_pci(void);
void sigma_driver_bus_enumerate_usb(void);

#ifdef __cplusplus
} /* namespace drivers */
} /* namespace sigmaos */
#endif
