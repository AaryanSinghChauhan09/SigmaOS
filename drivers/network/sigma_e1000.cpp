/*
 * Σ SigmaOS Zenith — Intel e1000 NIC Driver
 * Absorbs: Linux kernel drivers/net/ethernet/intel/e1000/e1000_main.c
 * Zero-Dependency: No libc, no stdlib, no predefined headers or functions.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef u64 size_t;

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

/* ─────────────── e1000 Register Offsets (MMIO) ─────────────── */
/* From Intel 8254x Software Developer's Manual */
#define E1000_CTRL      0x0000  /* Device Control */
#define E1000_STATUS    0x0008  /* Device Status */
#define E1000_EECD      0x0010  /* EEPROM/Flash Control */
#define E1000_EERD      0x0014  /* EEPROM Read */
#define E1000_ICR       0x00C0  /* Interrupt Cause Read */
#define E1000_ITR       0x00C4  /* Interrupt Throttling Rate */
#define E1000_ICS       0x00C8  /* Interrupt Cause Set */
#define E1000_IMS       0x00D0  /* Interrupt Mask Set/Read */
#define E1000_IMC       0x00D8  /* Interrupt Mask Clear */
#define E1000_RCTL      0x0100  /* Receive Control */
#define E1000_TCTL      0x0400  /* Transmit Control */
#define E1000_RDBAL     0x2800  /* RX Descriptor Base Low */
#define E1000_RDBAH     0x2804  /* RX Descriptor Base High */
#define E1000_RDLEN     0x2808  /* RX Descriptor Length */
#define E1000_RDH       0x2810  /* RX Descriptor Head */
#define E1000_RDT       0x2818  /* RX Descriptor Tail */
#define E1000_TDBAL     0x3800  /* TX Descriptor Base Low */
#define E1000_TDBAH     0x3804  /* TX Descriptor Base High */
#define E1000_TDLEN     0x3808  /* TX Descriptor Length */
#define E1000_TDH       0x3810  /* TX Descriptor Head */
#define E1000_TDT       0x3818  /* TX Descriptor Tail */
#define E1000_RAL       0x5400  /* Receive Address Low */
#define E1000_RAH       0x5404  /* Receive Address High */

/* Control register bits */
#define E1000_CTRL_RST  (1 << 26)  /* Device Reset */
#define E1000_CTRL_SLU  (1 << 6)   /* Set Link Up */
#define E1000_CTRL_ASDE (1 << 5)   /* Auto-Speed Detection Enable */

/* RX Control bits */
#define E1000_RCTL_EN        (1 << 1)  /* Receiver Enable */
#define E1000_RCTL_SBP       (1 << 2)  /* Store Bad Packets */
#define E1000_RCTL_UPE       (1 << 3)  /* Unicast Promiscuous Enable */
#define E1000_RCTL_MPE       (1 << 4)  /* Multicast Promiscuous Enable */
#define E1000_RCTL_BAM       (1 << 15) /* Broadcast Accept Mode */
#define E1000_RCTL_BSIZE_2048 0        /* Buffer size 2048 bytes */
#define E1000_RCTL_SECRC     (1 << 26) /* Strip CRC */

/* TX Control bits */
#define E1000_TCTL_EN        (1 << 1)  /* Transmit Enable */
#define E1000_TCTL_PSP       (1 << 3)  /* Pad Short Packets */

/* ─────────────── Descriptor Structures ─────────────── */
struct __attribute__((packed)) E1000_RxDesc {
    u64 buffer_addr;    /* Physical address of receive buffer */
    u16 length;
    u16 checksum;
    u8  status;
    u8  errors;
    u16 special;
};

struct __attribute__((packed)) E1000_TxDesc {
    u64 buffer_addr;    /* Physical address of transmit buffer */
    u16 length;
    u8  cso;            /* Checksum Offset */
    u8  cmd;            /* Command field */
    u8  status;
    u8  css;            /* Checksum Start Field */
    u16 special;
};

#define RX_DESC_COUNT  32
#define TX_DESC_COUNT  8
#define PACKET_BUF_SZ  2048

#define TX_CMD_EOP   (1 << 0)  /* End of Packet */
#define TX_CMD_IFCS  (1 << 1)  /* Insert FCS/CRC */
#define TX_CMD_RS    (1 << 3)  /* Report Status */
#define TX_STATUS_DD (1 << 0)  /* Descriptor Done */

/* ─────────────── Driver State ─────────────── */
struct SigmaE1000 {
    u64 mmio_base;           /* Base address for Memory-Mapped I/O */
    u8  mac_addr[6];
    struct E1000_RxDesc rx_descs[RX_DESC_COUNT];
    struct E1000_TxDesc tx_descs[TX_DESC_COUNT];
    u8 rx_buffers[RX_DESC_COUNT][PACKET_BUF_SZ];
    u8 tx_buffers[TX_DESC_COUNT][PACKET_BUF_SZ];
    u32 rx_tail;
    u32 tx_tail;
    bool link_up;
};

static struct SigmaE1000 e1000;

/* ─────────────── MMIO Read/Write ─────────────── */
static inline u32 e1000_read(u32 offset) {
    return *((volatile u32*)(e1000.mmio_base + offset));
}

static inline void e1000_write(u32 offset, u32 val) {
    *((volatile u32*)(e1000.mmio_base + offset)) = val;
}

/* ─────────────── EEPROM Read for MAC Address ─────────────── */
static u16 e1000_eeprom_read(u8 addr) {
    e1000_write(E1000_EERD, (1) | ((u32)addr << 8));
    u32 val;
    do { val = e1000_read(E1000_EERD); } while (!(val & (1 << 4)));
    return (u16)((val >> 16) & 0xFFFF);
}

/* ─────────────── API: Initialize e1000 ─────────────── */
extern "C" bool sigma_e1000_init(u64 mmio_base) {
    e1000.mmio_base = mmio_base;
    e1000.rx_tail   = 0;
    e1000.tx_tail   = 0;

    /* 1. Reset device */
    e1000_write(E1000_CTRL, e1000_read(E1000_CTRL) | E1000_CTRL_RST);
    /* Delay for reset to complete */
    for (volatile u32 i = 0; i < 100000; i++);

    /* 2. Read MAC from EEPROM */
    u16 mac01 = e1000_eeprom_read(0);
    u16 mac23 = e1000_eeprom_read(1);
    u16 mac45 = e1000_eeprom_read(2);
    e1000.mac_addr[0] = (u8)(mac01 & 0xFF);
    e1000.mac_addr[1] = (u8)(mac01 >> 8);
    e1000.mac_addr[2] = (u8)(mac23 & 0xFF);
    e1000.mac_addr[3] = (u8)(mac23 >> 8);
    e1000.mac_addr[4] = (u8)(mac45 & 0xFF);
    e1000.mac_addr[5] = (u8)(mac45 >> 8);

    /* 3. Setup RX Descriptors */
    sovereign_memset(e1000.rx_descs, 0, sizeof(e1000.rx_descs));
    for (u32 i = 0; i < RX_DESC_COUNT; i++) {
        e1000.rx_descs[i].buffer_addr = (u64)e1000.rx_buffers[i];
    }

    e1000_write(E1000_RDBAL, (u32)(u64)e1000.rx_descs);
    e1000_write(E1000_RDBAH, (u32)((u64)e1000.rx_descs >> 32));
    e1000_write(E1000_RDLEN, RX_DESC_COUNT * sizeof(struct E1000_RxDesc));
    e1000_write(E1000_RDH, 0);
    e1000_write(E1000_RDT, RX_DESC_COUNT - 1);

    /* 4. Configure RX Control */
    e1000_write(E1000_RCTL, E1000_RCTL_EN | E1000_RCTL_BAM |
                            E1000_RCTL_UPE | E1000_RCTL_MPE | E1000_RCTL_SECRC);

    /* 5. Setup TX Descriptors */
    sovereign_memset(e1000.tx_descs, 0, sizeof(e1000.tx_descs));
    for (u32 i = 0; i < TX_DESC_COUNT; i++) {
        e1000.tx_descs[i].buffer_addr = (u64)e1000.tx_buffers[i];
        e1000.tx_descs[i].status = TX_STATUS_DD; /* Mark as done so we can send */
    }

    e1000_write(E1000_TDBAL, (u32)(u64)e1000.tx_descs);
    e1000_write(E1000_TDBAH, (u32)((u64)e1000.tx_descs >> 32));
    e1000_write(E1000_TDLEN, TX_DESC_COUNT * sizeof(struct E1000_TxDesc));
    e1000_write(E1000_TDH, 0);
    e1000_write(E1000_TDT, 0);

    /* 6. Configure TX Control */
    e1000_write(E1000_TCTL, E1000_TCTL_EN | E1000_TCTL_PSP);

    /* 7. Bring up link */
    e1000_write(E1000_CTRL, e1000_read(E1000_CTRL) | E1000_CTRL_SLU | E1000_CTRL_ASDE);

    e1000.link_up = !!(e1000_read(E1000_STATUS) & 0x2);
    return e1000.link_up;
}

/* ─────────────── API: Transmit Packet ─────────────── */
extern "C" bool sigma_e1000_send(const u8* data, u16 length) {
    if (length > PACKET_BUF_SZ) return false;

    u32 idx = e1000.tx_tail % TX_DESC_COUNT;

    /* Wait until descriptor is available */
    while (!(e1000.tx_descs[idx].status & TX_STATUS_DD));

    sovereign_memcpy(e1000.tx_buffers[idx], data, length);
    e1000.tx_descs[idx].length = length;
    e1000.tx_descs[idx].cmd    = TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS;
    e1000.tx_descs[idx].status = 0;

    e1000.tx_tail++;
    e1000_write(E1000_TDT, e1000.tx_tail % TX_DESC_COUNT);
    return true;
}

/* ─────────────── API: Receive Packet ─────────────── */
extern "C" u16 sigma_e1000_recv(u8* out_buf) {
    u32 idx = e1000.rx_tail;
    struct E1000_RxDesc* desc = &e1000.rx_descs[idx];

    if (!(desc->status & 0x1)) return 0; /* Not done */

    u16 len = desc->length;
    sovereign_memcpy(out_buf, e1000.rx_buffers[idx], len);

    /* Reset descriptor */
    desc->status = 0;
    e1000.rx_tail = (e1000.rx_tail + 1) % RX_DESC_COUNT;
    e1000_write(E1000_RDT, idx);

    return len;
}
