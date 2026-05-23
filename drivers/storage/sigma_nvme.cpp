/*
 * Σ SigmaOS Zenith — NVMe Block Device Driver Shard
 * Absorbs: Linux drivers/nvme/host/core.c, NVMe 1.4 specification
 * Zero-Dependency: No libc, no stdlib, no predefined headers or functions.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef u64 size_t;
typedef u64 phys_addr_t;

/* ─────────────── Sovereign Utilities ─────────────── */
static void sovereign_memset(void* ptr, u8 val, size_t n) {
    u8* p = (u8*)ptr;
    while (n--) *p++ = val;
}

static void sovereign_memcpy(void* dst, const void* src, size_t n) {
    u8* d = (u8*)dst;
    const u8* s = (const u8*)src;
    while (n--) *d++ = *s++;
}

/* ─────────────── NVMe Register Map (BAR0 MMIO) ─────────────── */
/* From NVMe Base Specification Revision 1.4 */
#define NVME_REG_CAP      0x0000  /* Controller Capabilities (64-bit) */
#define NVME_REG_VS       0x0008  /* Version */
#define NVME_REG_INTMS    0x000C  /* Interrupt Mask Set */
#define NVME_REG_INTMC    0x0010  /* Interrupt Mask Clear */
#define NVME_REG_CC       0x0014  /* Controller Configuration */
#define NVME_REG_CSTS     0x001C  /* Controller Status */
#define NVME_REG_AQA      0x0024  /* Admin Queue Attributes */
#define NVME_REG_ASQ      0x0028  /* Admin Submission Queue Base (64-bit) */
#define NVME_REG_ACQ      0x0030  /* Admin Completion Queue Base (64-bit) */

/* CC Register fields */
#define NVME_CC_EN        (1 << 0)   /* Controller Enable */
#define NVME_CC_CSS_NVM   (0 << 4)   /* NVM Command Set */
#define NVME_CC_MPS_4K    (0 << 7)   /* Memory Page Size 4KB */
#define NVME_CC_IOSQES    (6 << 16)  /* I/O Submission Queue Entry Size: 64B */
#define NVME_CC_IOCQES    (4 << 20)  /* I/O Completion Queue Entry Size: 16B */

/* CSTS Register fields */
#define NVME_CSTS_RDY     (1 << 0)   /* Controller Ready */
#define NVME_CSTS_CFS     (1 << 1)   /* Controller Fatal Status */

/* ─────────────── NVMe Command Structures ─────────────── */
struct __attribute__((packed)) NVMeSubmissionEntry {
    u32 opcode_fuse_psdt_cid;  /* CDW0: Opcode[7:0], Fuse[9:8], PSDT[15:14], CID[31:16] */
    u32 nsid;                   /* CDW1: Namespace ID */
    u64 reserved;
    u64 mptr;                   /* Metadata Pointer */
    u64 prp1;                   /* PRP Entry 1 */
    u64 prp2;                   /* PRP Entry 2 */
    u32 cdw10;
    u32 cdw11;
    u32 cdw12;
    u32 cdw13;
    u32 cdw14;
    u32 cdw15;
};

struct __attribute__((packed)) NVMeCompletionEntry {
    u32 result;
    u32 reserved;
    u16 sq_head;
    u16 sq_id;
    u16 command_id;
    u16 status;    /* Phase bit in bit 0, Status in bits 15:1 */
};

/* NVMe Admin opcodes */
#define NVME_ADMIN_IDENTIFY     0x06
#define NVME_ADMIN_CREATE_IOSQ  0x01
#define NVME_ADMIN_CREATE_IOCQ  0x05

/* NVMe NVM opcodes */
#define NVME_NVM_READ           0x02
#define NVME_NVM_WRITE          0x01

/* ─────────────── Queue Configuration ─────────────── */
#define ADMIN_QUEUE_SIZE  16
#define IO_QUEUE_SIZE     64

struct NVMeQueue {
    struct NVMeSubmissionEntry* sq;  /* Submission Queue */
    struct NVMeCompletionEntry* cq;  /* Completion Queue */
    volatile u32* sq_doorbell;
    volatile u32* cq_doorbell;
    u32 sq_tail;
    u32 cq_head;
    u32 size;
    u8  cq_phase;    /* Expected phase bit */
    u16 next_cid;    /* Next command ID */
};

/* ─────────────── Driver State ─────────────── */
struct SigmaNVMe {
    u64 bar0;         /* MMIO base address (from PCI BAR0) */
    u32 doorbell_stride;
    struct NVMeQueue admin_queue;
    struct NVMeQueue io_queue;
    bool initialized;
};

static struct SigmaNVMe nvme;

/* Pre-allocated queue memory (placed in known physical region) */
static struct NVMeSubmissionEntry admin_sq[ADMIN_QUEUE_SIZE]
    __attribute__((aligned(4096)));
static struct NVMeCompletionEntry admin_cq[ADMIN_QUEUE_SIZE]
    __attribute__((aligned(4096)));
static struct NVMeSubmissionEntry io_sq[IO_QUEUE_SIZE]
    __attribute__((aligned(4096)));
static struct NVMeCompletionEntry io_cq[IO_QUEUE_SIZE]
    __attribute__((aligned(4096)));

/* ─────────────── MMIO Accessors ─────────────── */
static inline u32 nvme_read32(u32 offset) {
    return *((volatile u32*)(nvme.bar0 + offset));
}

static inline void nvme_write32(u32 offset, u32 val) {
    *((volatile u32*)(nvme.bar0 + offset)) = val;
}

static inline u64 nvme_read64(u32 offset) {
    u32 lo = nvme_read32(offset);
    u32 hi = nvme_read32(offset + 4);
    return ((u64)hi << 32) | lo;
}

static inline void nvme_write64(u32 offset, u64 val) {
    nvme_write32(offset, (u32)(val & 0xFFFFFFFF));
    nvme_write32(offset + 4, (u32)(val >> 32));
}

/* ─────────────── Queue Doorbell Calculation ─────────────── */
static volatile u32* nvme_sq_doorbell(u32 qid) {
    return (volatile u32*)(nvme.bar0 + 0x1000 + (2 * qid) * nvme.doorbell_stride);
}

static volatile u32* nvme_cq_doorbell(u32 qid) {
    return (volatile u32*)(nvme.bar0 + 0x1000 + (2 * qid + 1) * nvme.doorbell_stride);
}

/* ─────────────── Submit Command & Poll Completion ─────────────── */
static void nvme_submit(struct NVMeQueue* q, struct NVMeSubmissionEntry* cmd) {
    sovereign_memcpy(&q->sq[q->sq_tail], cmd, sizeof(*cmd));
    q->sq_tail = (q->sq_tail + 1) % q->size;
    *q->sq_doorbell = q->sq_tail;
}

static bool nvme_poll_completion(struct NVMeQueue* q, u32* result) {
    u32 timeout = 1000000;
    while (timeout--) {
        struct NVMeCompletionEntry* cqe = &q->cq[q->cq_head];
        if ((cqe->status & 0x01) == q->cq_phase) {
            /* Completion arrived */
            if (result) *result = cqe->result;
            q->cq_head = (q->cq_head + 1) % q->size;
            if (q->cq_head == 0) q->cq_phase ^= 1; /* Flip phase */
            *q->cq_doorbell = q->cq_head;
            return true;
        }
    }
    return false; /* Timeout */
}

/* ─────────────── API: Initialize NVMe Controller ─────────────── */
extern "C" bool sigma_nvme_init(u64 bar0_addr) {
    nvme.bar0 = bar0_addr;
    nvme.initialized = false;

    /* Read CAP register for doorbell stride */
    u64 cap = nvme_read64(NVME_REG_CAP);
    nvme.doorbell_stride = 4 << ((cap >> 32) & 0xF);

    /* 1. Disable controller */
    nvme_write32(NVME_REG_CC, 0);
    u32 timeout = 1000000;
    while ((nvme_read32(NVME_REG_CSTS) & NVME_CSTS_RDY) && --timeout);
    if (!timeout) return false;

    /* 2. Setup Admin Queues */
    sovereign_memset(admin_sq, 0, sizeof(admin_sq));
    sovereign_memset(admin_cq, 0, sizeof(admin_cq));

    nvme_write64(NVME_REG_ASQ, (u64)(phys_addr_t)admin_sq);
    nvme_write64(NVME_REG_ACQ, (u64)(phys_addr_t)admin_cq);
    nvme_write32(NVME_REG_AQA, ((ADMIN_QUEUE_SIZE - 1) << 16) | (ADMIN_QUEUE_SIZE - 1));

    nvme.admin_queue.sq = admin_sq;
    nvme.admin_queue.cq = admin_cq;
    nvme.admin_queue.sq_doorbell = nvme_sq_doorbell(0);
    nvme.admin_queue.cq_doorbell = nvme_cq_doorbell(0);
    nvme.admin_queue.sq_tail  = 0;
    nvme.admin_queue.cq_head  = 0;
    nvme.admin_queue.size     = ADMIN_QUEUE_SIZE;
    nvme.admin_queue.cq_phase = 1;
    nvme.admin_queue.next_cid = 0;

    /* 3. Enable controller */
    u32 cc = NVME_CC_EN | NVME_CC_CSS_NVM | NVME_CC_MPS_4K |
             NVME_CC_IOSQES | NVME_CC_IOCQES;
    nvme_write32(NVME_REG_CC, cc);

    timeout = 1000000;
    while (!(nvme_read32(NVME_REG_CSTS) & NVME_CSTS_RDY) && --timeout);
    if (!timeout) return false;
    if (nvme_read32(NVME_REG_CSTS) & NVME_CSTS_CFS) return false;

    nvme.initialized = true;
    return true;
}

/* ─────────────── API: Read LBA from NVMe ─────────────── */
extern "C" bool sigma_nvme_read(u64 lba, u32 num_blocks, u8* buffer) {
    if (!nvme.initialized) return false;

    struct NVMeSubmissionEntry cmd;
    sovereign_memset(&cmd, 0, sizeof(cmd));
    cmd.opcode_fuse_psdt_cid = NVME_NVM_READ | ((u32)nvme.io_queue.next_cid++ << 16);
    cmd.nsid = 1;
    cmd.prp1 = (u64)(phys_addr_t)buffer;
    cmd.cdw10 = (u32)(lba & 0xFFFFFFFF);
    cmd.cdw11 = (u32)(lba >> 32);
    cmd.cdw12 = num_blocks - 1;   /* 0-based count */

    nvme_submit(&nvme.io_queue, &cmd);
    return nvme_poll_completion(&nvme.io_queue, 0);
}

/* ─────────────── API: Write LBA to NVMe ─────────────── */
extern "C" bool sigma_nvme_write(u64 lba, u32 num_blocks, const u8* buffer) {
    if (!nvme.initialized) return false;

    struct NVMeSubmissionEntry cmd;
    sovereign_memset(&cmd, 0, sizeof(cmd));
    cmd.opcode_fuse_psdt_cid = NVME_NVM_WRITE | ((u32)nvme.io_queue.next_cid++ << 16);
    cmd.nsid = 1;
    cmd.prp1 = (u64)(phys_addr_t)buffer;
    cmd.cdw10 = (u32)(lba & 0xFFFFFFFF);
    cmd.cdw11 = (u32)(lba >> 32);
    cmd.cdw12 = num_blocks - 1;

    nvme_submit(&nvme.io_queue, &cmd);
    return nvme_poll_completion(&nvme.io_queue, 0);
}
