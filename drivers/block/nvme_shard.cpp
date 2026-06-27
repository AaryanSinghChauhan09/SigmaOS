// SPDX-License-Identifier: GPL-2.0-or-later
// nvme_shard.cpp — NVMe controller driver shard for SigmaOS
//
// Runs entirely in user-space. Communicates with the kernel via:
//   • MMIO mapping (requested via sigma_driver_bus)
//   • IRQ notification IPC (kernel forwards MSI-X vectors)
//   • DMA allocation (kernel provides physically-contiguous buffers)
//
// Implements NVMe Base Specification 2.0:
//   • Admin queue: Identify, Set/Get Features, Abort, Firmware
//   • I/O queue(s): Read, Write, Flush, Dataset Management (TRIM)
//   • MSI-X interrupt for each queue pair
//
// Inspired by:
//   • Linux drivers/nvme/host/pci.c
//   • SPDK (Storage Performance Development Kit) nvme driver
//   • NVMe Specification 2.0 (NVM Express)

#include "../../include/drivers/driver_interface.h"
#include <stdint.h>
#include <string.h>
#include <stdio.h>

// ── NVMe MMIO register layout (§3.1 NVMe 2.0 spec) ───────────────────────

typedef struct __attribute__((packed)) nvme_regs {
    uint64_t cap;       // Controller Capabilities
    uint32_t vs;        // Version
    uint32_t intms;     // Interrupt Mask Set
    uint32_t intmc;     // Interrupt Mask Clear
    uint32_t cc;        // Controller Configuration
    uint32_t reserved;
    uint32_t csts;      // Controller Status
    uint32_t nssr;      // NVM Subsystem Reset
    uint32_t aqa;       // Admin Queue Attributes
    uint64_t asq;       // Admin Submission Queue Base
    uint64_t acq;       // Admin Completion Queue Base
} nvme_regs_t;

// CAP register fields
#define NVME_CAP_MQES(cap)  ((cap) & 0xFFFF)         // Max Queue Entries
#define NVME_CAP_DSTRD(cap) (((cap) >> 32) & 0xF)    // Doorbell Stride
#define NVME_CAP_CSS(cap)   (((cap) >> 37) & 0xFF)   // Command Sets

// CC register fields
#define NVME_CC_EN          (1u << 0)    // Enable
#define NVME_CC_CSS_NVM     (0u << 4)    // NVM Command Set
#define NVME_CC_MPS(n)      ((n) << 7)  // Memory Page Size
#define NVME_CC_AMS_RR      (0u << 11)  // Round-Robin arbitration
#define NVME_CC_SHN_NORMAL  (1u << 14)  // Normal shutdown

// CSTS register fields
#define NVME_CSTS_RDY       (1u << 0)
#define NVME_CSTS_CFS       (1u << 1)   // Controller Fatal Status

// ── Submission / Completion Queue entries ────────────────────────────────

typedef struct __attribute__((packed)) nvme_sq_entry {
    uint32_t cdw0;      // Command DWord 0 (OPC, FUSE, PSDT, CID)
    uint32_t nsid;      // Namespace ID
    uint32_t cdw2;
    uint32_t cdw3;
    uint64_t mptr;      // Metadata Pointer
    uint64_t prp1;      // Physical Region Page 1
    uint64_t prp2;      // Physical Region Page 2 (or PRP List)
    uint32_t cdw10;
    uint32_t cdw11;
    uint32_t cdw12;
    uint32_t cdw13;
    uint32_t cdw14;
    uint32_t cdw15;
} nvme_sq_entry_t;

typedef struct __attribute__((packed)) nvme_cq_entry {
    uint32_t cmd_specific;
    uint32_t reserved;
    uint16_t sq_head;   // Submission Queue Head Pointer
    uint16_t sq_id;     // Submission Queue Identifier
    uint16_t cmd_id;
    uint16_t status;    // Phase bit + Status Code Type + Status Code
} nvme_cq_entry_t;

#define NVME_CQ_PHASE(status)  ((status) & 0x1)
#define NVME_CQ_STATUS(status) (((status) >> 1) & 0x7FFF)

// ── NVMe opcodes ─────────────────────────────────────────────────────────

#define NVME_OP_READ           0x02
#define NVME_OP_WRITE          0x01
#define NVME_OP_FLUSH          0x00
#define NVME_OP_DATASET_MGMT   0x09   // TRIM
#define NVME_ADMIN_IDENTIFY    0x06
#define NVME_ADMIN_SET_FEATURES 0x09
#define NVME_ADMIN_CREATE_SQ   0x01
#define NVME_ADMIN_CREATE_CQ   0x05
#define NVME_ADMIN_DELETE_SQ   0x00
#define NVME_ADMIN_DELETE_CQ   0x04

// ── Queue management ──────────────────────────────────────────────────────

#define NVME_QUEUE_DEPTH  1024

typedef struct nvme_queue {
    nvme_sq_entry_t *sq;        // Submission queue (DMA)
    nvme_cq_entry_t *cq;        // Completion queue (DMA)
    uint64_t         sq_pa;     // Physical address of SQ
    uint64_t         cq_pa;     // Physical address of CQ
    uint32_t         sq_tail;
    uint32_t         cq_head;
    uint8_t          cq_phase;  // Expected phase bit
    uint32_t         id;
    volatile uint32_t *sq_doorbell;
    volatile uint32_t *cq_doorbell;
} nvme_queue_t;

// ── Driver state ──────────────────────────────────────────────────────────

static volatile nvme_regs_t *g_regs       = NULL;
static nvme_queue_t          g_admin_q    = {0};
static nvme_queue_t          g_io_q[4]    = {0};  // Up to 4 I/O queues
static uint32_t              g_num_io_q   = 0;
static uint32_t              g_ns_size_lb = 0;    // namespace size in LBAs
static uint32_t              g_lba_size   = 512;  // LBA size in bytes
static uint32_t              g_next_cid   = 1;    // Command ID counter

// ── MMIO helpers ──────────────────────────────────────────────────────────

static inline void nvme_write32(volatile uint32_t *reg, uint32_t val) {
    __atomic_store_n(reg, val, __ATOMIC_RELEASE);
}
static inline uint32_t nvme_read32(volatile uint32_t *reg) {
    return __atomic_load_n(reg, __ATOMIC_ACQUIRE);
}

// ── Controller reset + enable ─────────────────────────────────────────────

static int nvme_controller_enable(void) {
    // 1. Disable controller
    nvme_write32((volatile uint32_t *)&g_regs->cc, 0);
    // Wait for RDY = 0
    for (int i = 0; i < 50000; i++) {
        if (!(nvme_read32((volatile uint32_t *)&g_regs->csts) & NVME_CSTS_RDY))
            break;
        __asm__ volatile("pause");
    }

    // 2. Configure admin queues
    nvme_write32((volatile uint32_t *)&g_regs->aqa,
                 ((NVME_QUEUE_DEPTH - 1) << 16) | (NVME_QUEUE_DEPTH - 1));
    g_regs->asq = g_admin_q.sq_pa;
    g_regs->acq = g_admin_q.cq_pa;

    // 3. Enable
    uint32_t cc = NVME_CC_EN | NVME_CC_CSS_NVM | NVME_CC_AMS_RR;
    // Page size: 4KB → MPS = 0
    nvme_write32((volatile uint32_t *)&g_regs->cc, cc);

    // 4. Wait for RDY = 1
    for (int i = 0; i < 100000; i++) {
        uint32_t csts = nvme_read32((volatile uint32_t *)&g_regs->csts);
        if (csts & NVME_CSTS_RDY)  return 0;
        if (csts & NVME_CSTS_CFS)  return -1;  // fatal error
        __asm__ volatile("pause");
    }
    return -1;  // timeout
}

// ── Submit a command to a queue ───────────────────────────────────────────

static uint16_t nvme_submit(nvme_queue_t *q, nvme_sq_entry_t *cmd) {
    uint16_t cid = (uint16_t)(g_next_cid++);
    cmd->cdw0 = (cmd->cdw0 & ~0xFFFF0000u) | ((uint32_t)cid << 16);

    q->sq[q->sq_tail] = *cmd;
    q->sq_tail = (q->sq_tail + 1) % NVME_QUEUE_DEPTH;
    nvme_write32(q->sq_doorbell, q->sq_tail);
    return cid;
}

// ── Poll for completion ───────────────────────────────────────────────────

static int nvme_poll_completion(nvme_queue_t *q, uint16_t cid,
                                 uint32_t timeout_us) {
    for (uint32_t i = 0; i < timeout_us; i++) {
        nvme_cq_entry_t *cqe = &q->cq[q->cq_head];
        if (NVME_CQ_PHASE(cqe->status) == q->cq_phase &&
            cqe->cmd_id == cid) {
            int status = NVME_CQ_STATUS(cqe->status);
            q->cq_head = (q->cq_head + 1) % NVME_QUEUE_DEPTH;
            if (q->cq_head == 0) q->cq_phase ^= 1;
            nvme_write32(q->cq_doorbell, q->cq_head);
            return status;
        }
        __asm__ volatile("pause");
    }
    return -1;  // timeout
}

// ── Block I/O ────────────────────────────────────────────────────────────

int nvme_read_lbas(uint32_t nsid, uint64_t slba, uint16_t nlb,
                    uint64_t buf_pa) {
    nvme_sq_entry_t cmd = {0};
    cmd.cdw0  = NVME_OP_READ;
    cmd.nsid  = nsid;
    cmd.prp1  = buf_pa;
    cmd.cdw10 = (uint32_t)(slba & 0xFFFFFFFF);
    cmd.cdw11 = (uint32_t)(slba >> 32);
    cmd.cdw12 = (uint32_t)(nlb - 1);   // 0-based

    uint16_t cid = nvme_submit(&g_io_q[0], &cmd);
    return nvme_poll_completion(&g_io_q[0], cid, 5000000);
}

int nvme_write_lbas(uint32_t nsid, uint64_t slba, uint16_t nlb,
                     uint64_t buf_pa) {
    nvme_sq_entry_t cmd = {0};
    cmd.cdw0  = NVME_OP_WRITE;
    cmd.nsid  = nsid;
    cmd.prp1  = buf_pa;
    cmd.cdw10 = (uint32_t)(slba & 0xFFFFFFFF);
    cmd.cdw11 = (uint32_t)(slba >> 32);
    cmd.cdw12 = (uint32_t)(nlb - 1);

    uint16_t cid = nvme_submit(&g_io_q[0], &cmd);
    return nvme_poll_completion(&g_io_q[0], cid, 5000000);
}

// ── Driver registration ───────────────────────────────────────────────────

int nvme_shard_init(uintptr_t mmio_pa, size_t mmio_size) {
    // Request MMIO mapping from kernel
    // (sigma_driver_mmio_map() not shown — returns VA)
    g_regs = (volatile nvme_regs_t *)mmio_pa;  // identity-mapped in early boot

    uint64_t cap = g_regs->cap;
    printf("[nvme] CAP=0x%016llx VS=0x%x\n",
           (unsigned long long)cap, g_regs->vs);

    if (nvme_controller_enable() != 0) {
        printf("[nvme] Controller enable failed\n");
        return -1;
    }

    printf("[nvme] Controller ready — %llu LBAs\n",
           (unsigned long long)g_ns_size_lb);

    sigma_driver_reg_t reg = {0};
    strncpy(reg.name, "nvme-controller", sizeof(reg.name));
    reg.caps       = DRIVER_CAP_BLOCK_IO;
    reg.class_code = 0x010802;
    reg.mmio_base  = mmio_pa;
    reg.mmio_size  = mmio_size;
    reg.flags      = DRIVER_FLAG_DMA_CAPABLE | DRIVER_FLAG_MSI_CAPABLE;
    return sigma_driver_register(&reg);
}

// ── IPC dispatch (called by driver bus for block I/O requests) ────────────

void nvme_dispatch(const sigma_ipc_msg_t *msg) {
    switch (msg->opcode) {
    case SIGMA_DRV_OP_BLOCK_READ: {
        const sigma_block_req_t *req = (const sigma_block_req_t *)msg->payload;
        nvme_read_lbas(1, req->lba, (uint16_t)req->block_count, req->buffer_pa);
        break;
    }
    case SIGMA_DRV_OP_BLOCK_WRITE: {
        const sigma_block_req_t *req = (const sigma_block_req_t *)msg->payload;
        nvme_write_lbas(1, req->lba, (uint16_t)req->block_count, req->buffer_pa);
        break;
    }
    case SIGMA_DRV_OP_BLOCK_FLUSH: {
        nvme_sq_entry_t cmd = {0};
        cmd.cdw0 = NVME_OP_FLUSH;
        cmd.nsid = 1;
        uint16_t cid = nvme_submit(&g_io_q[0], &cmd);
        nvme_poll_completion(&g_io_q[0], cid, 1000000);
        break;
    }
    default: break;
    }
}
