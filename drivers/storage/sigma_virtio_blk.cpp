/*
 * Σ SigmaOS — sigma_virtio_blk: Sovereign VirtIO Block Device Driver
 * Absorbs: Linux drivers/block/virtio_blk.c, QEMU virtio-blk-pci spec
 * Features: VirtIO legacy (v0.9.5) block I/O — read and write via virtqueues
 * Distros: Fedora (KVM), Ubuntu (cloud-img), Arch (virt-install), Debian cloud
 * Zero-Dependency: No libc. Raw PCI config + MMIO virtqueue I/O.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

/* ─────────────── I/O Port Helpers ─────────────── */
static inline u8  vio_inb(u16 port) {
    u8 v; __asm__ volatile("inb %1,%0":"=a"(v):"dN"(port)); return v;
}
static inline u16 vio_inw(u16 port) {
    u16 v; __asm__ volatile("inw %1,%0":"=a"(v):"dN"(port)); return v;
}
static inline u32 vio_inl(u16 port) {
    u32 v; __asm__ volatile("inl %1,%0":"=a"(v):"dN"(port)); return v;
}
static inline void vio_outb(u16 port, u8  v) { __asm__ volatile("outb %0,%1"::"a"(v),"dN"(port)); }
static inline void vio_outw(u16 port, u16 v) { __asm__ volatile("outw %0,%1"::"a"(v),"dN"(port)); }
static inline void vio_outl(u16 port, u32 v) { __asm__ volatile("outl %0,%1"::"a"(v),"dN"(port)); }

/* ─────────────── VirtIO Register Map (Legacy PIO, BAR0) ─────────────── */
/* From VirtIO 0.9.5 Spec §2.1 */
#define VIRTIO_PCI_HOST_FEATURES    0x00  /* (R)  32-bit: Feature bits from device */
#define VIRTIO_PCI_GUEST_FEATURES   0x04  /* (W)  32-bit: Features acknowledged */
#define VIRTIO_PCI_QUEUE_PFN        0x08  /* (RW) 32-bit: Queue descriptor table page */
#define VIRTIO_PCI_QUEUE_SIZE       0x0C  /* (R)  16-bit: Queue size */
#define VIRTIO_PCI_QUEUE_SEL        0x0E  /* (W)  16-bit: Queue select index */
#define VIRTIO_PCI_QUEUE_NOTIFY     0x10  /* (W)  16-bit: Queue notify (kick) */
#define VIRTIO_PCI_STATUS           0x12  /* (RW)  8-bit: Device status */
#define VIRTIO_PCI_ISR              0x13  /* (R)   8-bit: ISR status */
#define VIRTIO_PCI_CONFIG           0x14  /* (R)  Device-specific config space */

/* Device status bits */
#define VIRTIO_STATUS_ACKNOWLEDGE   0x01
#define VIRTIO_STATUS_DRIVER        0x02
#define VIRTIO_STATUS_DRIVER_OK     0x04
#define VIRTIO_STATUS_FEATURES_OK   0x08
#define VIRTIO_STATUS_FAILED        0x80

/* VirtIO Block feature bits */
#define VIRTIO_BLK_F_SIZE_MAX   (1 << 1)
#define VIRTIO_BLK_F_SEG_MAX    (1 << 2)
#define VIRTIO_BLK_F_GEOMETRY   (1 << 4)
#define VIRTIO_BLK_F_RO         (1 << 5)
#define VIRTIO_BLK_F_BLK_SIZE   (1 << 6)
#define VIRTIO_BLK_F_FLUSH      (1 << 9)
#define VIRTIO_BLK_F_TOPOLOGY   (1 << 10)

/* ─────────────── VirtQueue Descriptor (16 bytes) ─────────────── */
struct __attribute__((packed)) VirtqDesc {
    u64 addr;   /* Physical address of buffer */
    u32 len;    /* Length in bytes */
    u16 flags;  /* VIRTQ_DESC_F_* */
    u16 next;   /* Next descriptor chain index */
};

#define VIRTQ_DESC_F_NEXT     0x0001  /* Chained descriptor */
#define VIRTQ_DESC_F_WRITE    0x0002  /* Buffer written by device */
#define VIRTQ_DESC_F_INDIRECT 0x0004  /* Buffer contains indirect descriptor table */

/* ─────────────── VirtQueue Available Ring ─────────────── */
struct __attribute__((packed)) VirtqAvail {
    u16 flags;            /* VIRTQ_AVAIL_F_NO_INTERRUPT */
    u16 idx;              /* Next index to be written */
    u16 ring[16];         /* Queue indices (max 16 for our use) */
    u16 used_event;       /* Unused in legacy */
};

/* ─────────────── VirtQueue Used Ring ─────────────── */
struct __attribute__((packed)) VirtqUsedElem {
    u32 id;    /* Descriptor chain head index */
    u32 len;   /* Written length */
};
struct __attribute__((packed)) VirtqUsed {
    u16 flags;
    u16 idx;
    VirtqUsedElem ring[16];
    u16 avail_event;
};

/* ─────────────── VirtIO Block Request Header ─────────────── */
#define VIRTIO_BLK_T_IN    0  /* Read */
#define VIRTIO_BLK_T_OUT   1  /* Write */
#define VIRTIO_BLK_T_FLUSH 4  /* Flush */

struct __attribute__((packed)) VirtioBlkReqHdr {
    u32 type;      /* VIRTIO_BLK_T_* */
    u32 reserved;
    u64 sector;    /* LBA sector number */
};

/* Status byte returned by device */
#define VIRTIO_BLK_S_OK     0
#define VIRTIO_BLK_S_IOERR  1
#define VIRTIO_BLK_S_UNSUPP 2

/* ─────────────── Virtqueue Memory (4KB page-aligned) ─────────────── */
#define QUEUE_SIZE  16  /* Max 16 outstanding descriptors */
#define PAGE_SIZE   4096

/* Aligned static buffers — we use a flat page for the entire vring */
static u8 __attribute__((aligned(4096))) vring_mem[4 * PAGE_SIZE];

/* Pointers into vring_mem */
static VirtqDesc*  vring_desc;   /* Descriptor table */
static VirtqAvail* vring_avail;  /* Available ring */
static VirtqUsed*  vring_used;   /* Used ring (device-written) */

/* Request structures (one per outstanding I/O, we only do synchronous) */
static VirtioBlkReqHdr io_req_hdr;
static u8              io_status;

/* ─────────────── Driver State ─────────────── */
static u16   vio_iobase = 0;       /* PCI BAR0 I/O port base */
static bool  vio_initialized = false;
static u16   last_used_idx   = 0;

/* ─────────────── Vring Layout Calculation ─────────────── */
/* VirtIO spec vring_size(): descriptors + avail + used, with 4096-byte alignment */
static void vring_setup() {
    /* Descriptor table at page 0 */
    vring_desc  = (VirtqDesc*)vring_mem;
    /* Available ring immediately after descriptors */
    u32 avail_off = QUEUE_SIZE * sizeof(VirtqDesc);
    vring_avail = (VirtqAvail*)(vring_mem + avail_off);
    /* Used ring on next 4096-byte boundary */
    u32 used_off = (avail_off + sizeof(VirtqAvail) + PAGE_SIZE - 1) & ~(PAGE_SIZE - 1);
    vring_used  = (VirtqUsed*)(vring_mem + used_off);

    /* Zero entire vring memory */
    for (u32 i = 0; i < sizeof(vring_mem); i++) vring_mem[i] = 0;

    /* Mark all descriptors as free */
    for (u32 i = 0; i < QUEUE_SIZE; i++) {
        vring_desc[i].flags = 0;
        vring_desc[i].next  = (u16)(i + 1);
    }
}

/* ─────────────── PCI Helper: find virtio-blk device on bus ─────────────── */
/* Read 16-bit PCI config (CF8/CFC mechanism) */
static u16 pci_read16(u8 bus, u8 dev, u8 func, u8 off) {
    u32 addr = 0x80000000u | ((u32)bus<<16) | ((u32)dev<<11) |
               ((u32)func<<8) | (off & 0xFC);
    vio_outl(0xCF8, addr);
    return (vio_inl(0xCFC) >> ((off & 2) * 8)) & 0xFFFF;
}
static u32 pci_read32(u8 bus, u8 dev, u8 func, u8 off) {
    u32 addr = 0x80000000u | ((u32)bus<<16) | ((u32)dev<<11) |
               ((u32)func<<8) | (off & 0xFC);
    vio_outl(0xCF8, addr);
    return vio_inl(0xCFC);
}

/* ─────────────── Synchronous Virtqueue Submit & Poll ─────────────── */
static int vio_submit_poll(u32 desc0) {
    /* Post to available ring */
    u16 avail_idx = vring_avail->idx;
    vring_avail->ring[avail_idx % QUEUE_SIZE] = (u16)desc0;
    /* Memory barrier */
    __asm__ volatile("" ::: "memory");
    vring_avail->idx = avail_idx + 1;
    __asm__ volatile("" ::: "memory");

    /* Kick the device: write queue index 0 to QUEUE_NOTIFY */
    vio_outw((u16)(vio_iobase + VIRTIO_PCI_QUEUE_NOTIFY), 0);

    /* Poll used ring until device posts completion */
    u32 timeout = 10000000;
    while (vring_used->idx == last_used_idx && --timeout)
        __asm__ volatile("pause");
    if (!timeout) return -1; /* Timeout */

    last_used_idx = vring_used->idx;
    return 0;
}

/* ─────────────── Public API ─────────────── */
extern "C" int sigma_virtio_blk_init(u16 iobase) {
    vio_iobase = iobase;

    /* 1. Reset device */
    vio_outb((u16)(iobase + VIRTIO_PCI_STATUS), 0x00);

    /* 2. Acknowledge device */
    vio_outb((u16)(iobase + VIRTIO_PCI_STATUS), VIRTIO_STATUS_ACKNOWLEDGE);

    /* 3. Tell device we know how to drive it */
    vio_outb((u16)(iobase + VIRTIO_PCI_STATUS),
             VIRTIO_STATUS_ACKNOWLEDGE | VIRTIO_STATUS_DRIVER);

    /* 4. Negotiate features — accept BLK_SIZE feature only */
    u32 host_features = vio_inl((u16)(iobase + VIRTIO_PCI_HOST_FEATURES));
    u32 guest_features = host_features & VIRTIO_BLK_F_BLK_SIZE;
    vio_outl((u16)(iobase + VIRTIO_PCI_GUEST_FEATURES), guest_features);

    /* 5. Select queue 0 (requestq) */
    vio_outw((u16)(iobase + VIRTIO_PCI_QUEUE_SEL), 0);
    u16 qsize = vio_inw((u16)(iobase + VIRTIO_PCI_QUEUE_SIZE));
    if (qsize == 0) return -1; /* No queue */

    /* 6. Setup vring memory */
    vring_setup();

    /* 7. Tell device about our vring physical address (in 4KB pages) */
    u32 pfn = (u32)(u64)vring_mem / PAGE_SIZE;
    vio_outl((u16)(iobase + VIRTIO_PCI_QUEUE_PFN), pfn);

    /* 8. Driver OK */
    vio_outb((u16)(iobase + VIRTIO_PCI_STATUS),
             VIRTIO_STATUS_ACKNOWLEDGE | VIRTIO_STATUS_DRIVER | VIRTIO_STATUS_DRIVER_OK);

    vio_initialized = true;
    last_used_idx   = 0;
    return 0;
}

/* Read 'count' 512-byte sectors starting at 'lba' into 'buf' */
extern "C" int sigma_virtio_blk_read(u64 lba, u32 count, u8* buf) {
    if (!vio_initialized) return -1;

    io_req_hdr.type     = VIRTIO_BLK_T_IN;
    io_req_hdr.reserved = 0;
    io_req_hdr.sector   = lba;
    io_status = 0xFF; /* Will be overwritten by device */

    /* Build 3-descriptor chain: header | data (write) | status (write) */
    /* Desc 0: request header (device reads) */
    vring_desc[0].addr  = (u64)(u32)&io_req_hdr;
    vring_desc[0].len   = sizeof(VirtioBlkReqHdr);
    vring_desc[0].flags = VIRTQ_DESC_F_NEXT;
    vring_desc[0].next  = 1;

    /* Desc 1: data buffer (device writes sectors into it) */
    vring_desc[1].addr  = (u64)(u32)buf;
    vring_desc[1].len   = count * 512;
    vring_desc[1].flags = VIRTQ_DESC_F_WRITE | VIRTQ_DESC_F_NEXT;
    vring_desc[1].next  = 2;

    /* Desc 2: status byte (device writes VIRTIO_BLK_S_*) */
    vring_desc[2].addr  = (u64)(u32)&io_status;
    vring_desc[2].len   = 1;
    vring_desc[2].flags = VIRTQ_DESC_F_WRITE;
    vring_desc[2].next  = 0;

    if (vio_submit_poll(0) < 0) return -1;
    return (io_status == VIRTIO_BLK_S_OK) ? 0 : -1;
}

/* Write 'count' 512-byte sectors starting at 'lba' from 'buf' */
extern "C" int sigma_virtio_blk_write(u64 lba, u32 count, const u8* buf) {
    if (!vio_initialized) return -1;

    io_req_hdr.type     = VIRTIO_BLK_T_OUT;
    io_req_hdr.reserved = 0;
    io_req_hdr.sector   = lba;
    io_status = 0xFF;

    vring_desc[0].addr  = (u64)(u32)&io_req_hdr;
    vring_desc[0].len   = sizeof(VirtioBlkReqHdr);
    vring_desc[0].flags = VIRTQ_DESC_F_NEXT;
    vring_desc[0].next  = 1;

    vring_desc[1].addr  = (u64)(u32)buf; /* Device reads our data */
    vring_desc[1].len   = count * 512;
    vring_desc[1].flags = VIRTQ_DESC_F_NEXT; /* No WRITE — device reads */
    vring_desc[1].next  = 2;

    vring_desc[2].addr  = (u64)(u32)&io_status;
    vring_desc[2].len   = 1;
    vring_desc[2].flags = VIRTQ_DESC_F_WRITE;
    vring_desc[2].next  = 0;

    if (vio_submit_poll(0) < 0) return -1;
    return (io_status == VIRTIO_BLK_S_OK) ? 0 : -1;
}

/* Get disk capacity in 512-byte sectors (from VirtIO config space offset 0) */
extern "C" u64 sigma_virtio_blk_capacity() {
    if (!vio_initialized) return 0;
    u32 lo = vio_inl((u16)(vio_iobase + VIRTIO_PCI_CONFIG + 0));
    u32 hi = vio_inl((u16)(vio_iobase + VIRTIO_PCI_CONFIG + 4));
    return ((u64)hi << 32) | lo;
}
