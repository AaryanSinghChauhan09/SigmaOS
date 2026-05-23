/*
 * Σ SigmaOS Zenith — Realtek RTL8139 NIC Driver Shard
 * Absorbs: Linux drivers/net/ethernet/realtek/8139too.c
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

/* ─────────────── Port I/O ─────────────── */
static inline u8 rtl_inb(u16 port) {
    u8 ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline u16 rtl_inw(u16 port) {
    u16 ret;
    __asm__ volatile ("inw %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline u32 rtl_inl(u16 port) {
    u32 ret;
    __asm__ volatile ("inl %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void rtl_outb(u16 port, u8 val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline void rtl_outw(u16 port, u16 val) {
    __asm__ volatile ("outw %0, %1" : : "a"(val), "Nd"(port));
}

static inline void rtl_outl(u16 port, u32 val) {
    __asm__ volatile ("outl %0, %1" : : "a"(val), "Nd"(port));
}

/* ─────────────── RTL8139 Register Offsets (I/O Port Mapped) ─────────────── */
#define RTL_IDR0         0x00  /* MAC Address bytes 0-3 */
#define RTL_IDR4         0x04  /* MAC Address bytes 4-5 */
#define RTL_TSD0         0x10  /* Transmit Status Descriptor 0 */
#define RTL_TSAD0        0x20  /* Transmit Start Address Descriptor 0 */
#define RTL_RBSTART      0x30  /* Receive Buffer Start Address */
#define RTL_CMD          0x37  /* Command Register */
#define RTL_CAPR         0x38  /* Current Address of Packet Read */
#define RTL_CBR          0x3A  /* Current Buffer Address */
#define RTL_IMR          0x3C  /* Interrupt Mask Register */
#define RTL_ISR          0x3E  /* Interrupt Status Register */
#define RTL_TCR          0x40  /* Transmit Configuration Register */
#define RTL_RCR          0x44  /* Receive Configuration Register */
#define RTL_CONFIG1      0x52  /* Configuration Register 1 */

/* Command Register Bits */
#define RTL_CMD_RST      0x10  /* Reset */
#define RTL_CMD_RE       0x08  /* Receiver Enable */
#define RTL_CMD_TE       0x04  /* Transmitter Enable */

/* Interrupt bits */
#define RTL_INT_ROK      0x0001  /* Receive OK */
#define RTL_INT_TOK      0x0004  /* Transmit OK */

/* RCR bits */
#define RTL_RCR_AAP      (1 << 0)  /* Accept All Packets */
#define RTL_RCR_APM      (1 << 1)  /* Accept Physical Match */
#define RTL_RCR_AM       (1 << 2)  /* Accept Multicast */
#define RTL_RCR_AB       (1 << 3)  /* Accept Broadcast */
#define RTL_RCR_WRAP     (1 << 7)  /* Wrap around buffer */

/* ─────────────── Driver State ─────────────── */
#define RX_BUF_SIZE    (8192 + 16 + 1500)   /* 8K + header + extra */
#define TX_BUF_SIZE    1536

struct SigmaRTL8139 {
    u16 io_base;
    u8  mac_addr[6];
    u32 tx_cur;            /* Current TX descriptor (0-3) */
    u32 rx_offset;         /* Current RX buffer offset */
    bool link_up;
};

static struct SigmaRTL8139 rtl;
static u8 rx_buffer[RX_BUF_SIZE] __attribute__((aligned(4)));
static u8 tx_buffers[4][TX_BUF_SIZE] __attribute__((aligned(4)));

/* ─────────────── API: Initialize RTL8139 ─────────────── */
extern "C" bool sigma_rtl8139_init(u16 io_base) {
    rtl.io_base   = io_base;
    rtl.tx_cur    = 0;
    rtl.rx_offset = 0;

    /* 1. Power on */
    rtl_outb(io_base + RTL_CONFIG1, 0x00);

    /* 2. Software reset */
    rtl_outb(io_base + RTL_CMD, RTL_CMD_RST);
    u32 timeout = 100000;
    while ((rtl_inb(io_base + RTL_CMD) & RTL_CMD_RST) && --timeout);
    if (!timeout) return false;

    /* 3. Read MAC address from IDR registers */
    for (u32 i = 0; i < 4; i++)
        rtl.mac_addr[i] = rtl_inb(io_base + RTL_IDR0 + i);
    rtl.mac_addr[4] = rtl_inb(io_base + RTL_IDR4);
    rtl.mac_addr[5] = rtl_inb(io_base + RTL_IDR4 + 1);

    /* 4. Setup RX buffer */
    sovereign_memset(rx_buffer, 0, RX_BUF_SIZE);
    rtl_outl(io_base + RTL_RBSTART, (u32)(u64)rx_buffer);

    /* 5. Configure interrupts: ROK and TOK */
    rtl_outw(io_base + RTL_IMR, RTL_INT_ROK | RTL_INT_TOK);

    /* 6. Configure RX: Accept broadcast, multicast, physical match, wrap */
    rtl_outl(io_base + RTL_RCR, RTL_RCR_AB | RTL_RCR_AM | RTL_RCR_APM | RTL_RCR_WRAP);

    /* 7. Configure TX */
    rtl_outl(io_base + RTL_TCR, 0x03000000); /* IFG: normal, no loopback */

    /* 8. Enable RX and TX */
    rtl_outb(io_base + RTL_CMD, RTL_CMD_RE | RTL_CMD_TE);

    rtl.link_up = true;
    return true;
}

/* ─────────────── API: Transmit Packet ─────────────── */
extern "C" bool sigma_rtl8139_send(const u8* data, u16 length) {
    if (length > TX_BUF_SIZE) return false;

    u32 desc = rtl.tx_cur % 4;

    /* Copy data to TX buffer */
    sovereign_memcpy(tx_buffers[desc], data, length);

    /* Set TX Start Address */
    rtl_outl(rtl.io_base + RTL_TSAD0 + desc * 4, (u32)(u64)tx_buffers[desc]);

    /* Set TX Status: length and clear OWN bit (bit 13) to start DMA */
    rtl_outl(rtl.io_base + RTL_TSD0 + desc * 4, (u32)length & 0x1FFF);

    /* Wait for TOK */
    u32 timeout = 100000;
    while (timeout--) {
        u32 status = rtl_inl(rtl.io_base + RTL_TSD0 + desc * 4);
        if (status & (1 << 15)) break; /* TOK bit */
        if (status & (1 << 14)) return false; /* TUN: TX underrun */
    }

    rtl.tx_cur++;
    return true;
}

/* ─────────────── API: Receive Packet ─────────────── */
extern "C" u16 sigma_rtl8139_recv(u8* out_buf) {
    /* Check command register — buffer empty flag */
    if (rtl_inb(rtl.io_base + RTL_CMD) & 0x01) return 0; /* RX buffer empty */

    /* Read packet header at current offset */
    u8* header = rx_buffer + rtl.rx_offset;
    u16 status = *((u16*)(header));
    u16 length = *((u16*)(header + 2));

    if (!(status & 0x01)) return 0; /* ROK not set — bad packet */
    if (length > 1500) return 0;    /* Sanity check */

    /* Copy packet data (skip 4-byte header) */
    sovereign_memcpy(out_buf, header + 4, length - 4);

    /* Advance RX offset (align to 4 bytes) */
    rtl.rx_offset = (rtl.rx_offset + length + 4 + 3) & ~3;
    rtl.rx_offset %= RX_BUF_SIZE;

    /* Update CAPR */
    rtl_outw(rtl.io_base + RTL_CAPR, rtl.rx_offset - 16);

    return length - 4;
}

/* ─────────────── API: IRQ Handler ─────────────── */
extern "C" void sigma_rtl8139_irq_handler() {
    u16 status = rtl_inw(rtl.io_base + RTL_ISR);
    /* Acknowledge all pending interrupts */
    rtl_outw(rtl.io_base + RTL_ISR, status);
}
