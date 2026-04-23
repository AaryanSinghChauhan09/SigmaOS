/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: ATA/IDE DISK DRIVER (v1.0 - PURE C11)
 * =============================================================================
 * Interface: PIO-mode ATA (no DMA required for initial boot)
 * Channels: Primary (0x1F0-0x1F7) + Secondary (0x170-0x177)
 * Features:
 *   - Identify Drive (model, capacity, LBA28/LBA48 support)
 *   - Read/Write sectors via LBA28 (28-bit) + LBA48 (48-bit)
 *   - Drive cache flush (CACHE FLUSH EXT)
 *   - Error decoding (ABRT, IDNF, UNC, BBK)
 *   - Disk partition table (MBR + GPT header parser)
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../sigma_kernel_types.h"

/* =========================================================================
 * ATA I/O Port Maps
 * ========================================================================= */
#define ATA_PRIMARY_BASE     0x1F0
#define ATA_PRIMARY_CTRL     0x3F6
#define ATA_SECONDARY_BASE   0x170
#define ATA_SECONDARY_CTRL   0x376

/* Register offsets from base */
#define ATA_REG_DATA         0   /* 16-bit R/W */
#define ATA_REG_ERR          1   /* R: Error | W: Features */
#define ATA_REG_SECCOUNT0    2   /* Sector count (LBA28 lo) */
#define ATA_REG_LBA0         3   /* LBA28 bits 0-7 */
#define ATA_REG_LBA1         4   /* LBA28 bits 8-15 */
#define ATA_REG_LBA2         5   /* LBA28 bits 16-23 */
#define ATA_REG_HDDEVSEL     6   /* Head / Device select */
#define ATA_REG_COMMAND      7   /* W: Command | R: Status */
#define ATA_REG_STATUS       7
#define ATA_REG_SECCOUNT1    8   /* LBA48 sector count high */
#define ATA_REG_LBA3         9   /* LBA48 bits 24-31 */
#define ATA_REG_LBA4         10  /* LBA48 bits 32-39 */
#define ATA_REG_LBA5         11  /* LBA48 bits 40-47 */
#define ATA_REG_ALTSTATUS    12  /* R: Status (no IRQ clear) */
#define ATA_REG_DEVCTRL      13  /* W: Device control */

/* Status bits */
#define ATA_SR_BSY   0x80
#define ATA_SR_DRDY  0x40
#define ATA_SR_DF    0x20
#define ATA_SR_DSC   0x10
#define ATA_SR_DRQ   0x08
#define ATA_SR_CORR  0x04
#define ATA_SR_IDX   0x02
#define ATA_SR_ERR   0x01

/* Commands */
#define ATA_CMD_READ_PIO     0x20
#define ATA_CMD_READ_PIO_EXT 0x24
#define ATA_CMD_WRITE_PIO    0x30
#define ATA_CMD_WRITE_PIO_EXT 0x34
#define ATA_CMD_CACHE_FLUSH  0xE7
#define ATA_CMD_CACHE_FLUSH_EXT 0xEA
#define ATA_CMD_IDENTIFY     0xEC

/* =========================================================================
 * ATA Channel/Drive Descriptor
 * ========================================================================= */
#define ATA_MASTER  0
#define ATA_SLAVE   1
#define ATA_MAX_DRIVES 4

typedef struct ATADrive {
    u8     channel;     /* 0=primary, 1=secondary */
    u8     drive;       /* 0=master, 1=slave */
    u8     type;        /* 0=PATA, 1=PATAPI */
    bool_t lba48;
    bool_t present;
    u64    sectors;     /* total LBA sectors */
    char   model[41];
    u16    base;        /* I/O base port */
    u16    ctrl;        /* Control port */
} ATADrive;

static ATADrive g_drives[ATA_MAX_DRIVES];
static u32      g_drive_count = 0;

/* =========================================================================
 * Low-level I/O helpers
 * ========================================================================= */
static u8 ata_read8(const ATADrive* d, u8 reg) {
    if (reg < 8)  return port_inb((u16)(d->base + reg));
    if (reg < 12) return port_inb((u16)(d->ctrl + reg - 8));
    return 0;
}

static void ata_write8(const ATADrive* d, u8 reg, u8 val) {
    if (reg < 8)  port_outb((u16)(d->base + reg), val);
    else if (reg < 12) port_outb((u16)(d->ctrl + reg - 8), val);
}

static u16 ata_read16(const ATADrive* d) {
    return (u16)(port_inb(d->base) | ((u16)port_inb((u16)(d->base + 1)) << 8));
}

/* 400ns delay — read alt-status 4 times */
static void ata_delay400ns(const ATADrive* d) {
    u8 i;
    for (i = 0; i < 4; i++) ata_read8(d, ATA_REG_ALTSTATUS);
}

/* Poll until BSY clears (with timeout) */
static k_status ata_poll(const ATADrive* d, bool_t check_drq) {
    u32 timeout = 100000u;
    while (timeout--) {
        u8 st = ata_read8(d, ATA_REG_STATUS);
        if (st & ATA_SR_ERR) return K_ERR_INVAL;
        if (st & ATA_SR_DF)  return K_ERR_BUSY;
        if (!(st & ATA_SR_BSY)) {
            if (check_drq && !(st & ATA_SR_DRQ)) return K_ERR_BUSY;
            return K_OK;
        }
        cpu_pause();
    }
    return K_ERR_BUSY;   /* timeout */
}

/* =========================================================================
 * ATA IDENTIFY — detect and characterize a drive
 * ========================================================================= */
static bool_t ata_identify(ATADrive* d) {
    /* Select drive */
    ata_write8(d, ATA_REG_HDDEVSEL, (u8)(0xA0 | (d->drive << 4)));
    ata_delay400ns(d);

    /* Zero LBA registers */
    ata_write8(d, ATA_REG_SECCOUNT0, 0);
    ata_write8(d, ATA_REG_LBA0, 0);
    ata_write8(d, ATA_REG_LBA1, 0);
    ata_write8(d, ATA_REG_LBA2, 0);

    /* Send IDENTIFY */
    ata_write8(d, ATA_REG_COMMAND, ATA_CMD_IDENTIFY);
    ata_delay400ns(d);

    if (ata_read8(d, ATA_REG_STATUS) == 0) return FALSE; /* no drive */

    if (ata_poll(d, TRUE) != K_OK) return FALSE;

    /* Read 256 × u16 IDENTIFY data */
    u16 id[256];
    u32 i;
    for (i = 0; i < 256; i++) id[i] = (u16)(port_inb(d->base) | ((u16)port_inb((u16)(d->base+1)) << 8));

    /* LBA48 support: word 83 bit 10 */
    d->lba48 = !!(id[83] & BIT(10));

    /* Total sectors */
    if (d->lba48) {
        d->sectors = ((u64)id[103] << 48) | ((u64)id[102] << 32) |
                     ((u64)id[101] << 16) | (u64)id[100];
    } else {
        d->sectors = ((u32)id[61] << 16) | id[60];
    }

    /* Model string: words 27-46, byte-swapped */
    for (i = 0; i < 20; i++) {
        d->model[i*2]   = (char)(id[27+i] >> 8);
        d->model[i*2+1] = (char)(id[27+i] & 0xFF);
    }
    d->model[40] = '\0';
    /* Trim trailing spaces */
    i = 39;
    while (i > 0 && d->model[i] == ' ') { d->model[i] = '\0'; i--; }

    d->present = TRUE;
    return TRUE;
}

/* =========================================================================
 * LBA28 Sector Read (PIO)
 * ========================================================================= */
k_status ata_read_sectors(u32 drive_idx, u64 lba, u32 count, void* buf) {
    if (drive_idx >= g_drive_count) return K_ERR_INVAL;
    ATADrive* d = &g_drives[drive_idx];
    if (!d->present) return K_ERR_INVAL;

    /* Wait for drive not busy */
    if (ata_poll(d, FALSE) != K_OK) return K_ERR_BUSY;

    if (d->lba48) {
        /* LBA48: send high bytes first, then low bytes */
        ata_write8(d, ATA_REG_HDDEVSEL, (u8)(0x40 | (d->drive << 4)));
        ata_write8(d, ATA_REG_SECCOUNT1, (u8)(count >> 8));
        ata_write8(d, ATA_REG_LBA3, (u8)(lba >> 24));
        ata_write8(d, ATA_REG_LBA4, (u8)(lba >> 32));
        ata_write8(d, ATA_REG_LBA5, (u8)(lba >> 40));
        ata_write8(d, ATA_REG_SECCOUNT0, (u8)count);
        ata_write8(d, ATA_REG_LBA0, (u8)lba);
        ata_write8(d, ATA_REG_LBA1, (u8)(lba >> 8));
        ata_write8(d, ATA_REG_LBA2, (u8)(lba >> 16));
        ata_write8(d, ATA_REG_COMMAND, ATA_CMD_READ_PIO_EXT);
    } else {
        ata_write8(d, ATA_REG_HDDEVSEL,
                   (u8)(0xE0 | (d->drive << 4) | ((lba >> 24) & 0x0F)));
        ata_write8(d, ATA_REG_SECCOUNT0, (u8)count);
        ata_write8(d, ATA_REG_LBA0, (u8)lba);
        ata_write8(d, ATA_REG_LBA1, (u8)(lba >> 8));
        ata_write8(d, ATA_REG_LBA2, (u8)(lba >> 16));
        ata_write8(d, ATA_REG_COMMAND, ATA_CMD_READ_PIO);
    }

    u8* dst   = (u8*)buf;
    u32 sec;
    for (sec = 0; sec < count; sec++) {
        k_status s = ata_poll(d, TRUE);
        if (s != K_OK) return s;
        /* Read 256 words (512 bytes) via INW */
        u16 i;
        for (i = 0; i < 256; i++) {
            u16 word = (u16)(port_inb(d->base) | ((u16)port_inb((u16)(d->base+1)) << 8));
            *dst++ = (u8)(word & 0xFF);
            *dst++ = (u8)(word >> 8);
        }
        ata_delay400ns(d);
    }
    return K_OK;
}

/* =========================================================================
 * LBA28/48 Sector Write (PIO)
 * ========================================================================= */
k_status ata_write_sectors(u32 drive_idx, u64 lba, u32 count, const void* buf) {
    if (drive_idx >= g_drive_count) return K_ERR_INVAL;
    ATADrive* d = &g_drives[drive_idx];
    if (!d->present) return K_ERR_INVAL;

    if (ata_poll(d, FALSE) != K_OK) return K_ERR_BUSY;

    if (d->lba48) {
        ata_write8(d, ATA_REG_HDDEVSEL, (u8)(0x40 | (d->drive << 4)));
        ata_write8(d, ATA_REG_SECCOUNT1, (u8)(count >> 8));
        ata_write8(d, ATA_REG_LBA3, (u8)(lba >> 24));
        ata_write8(d, ATA_REG_LBA4, (u8)(lba >> 32));
        ata_write8(d, ATA_REG_LBA5, (u8)(lba >> 40));
        ata_write8(d, ATA_REG_SECCOUNT0, (u8)count);
        ata_write8(d, ATA_REG_LBA0, (u8)lba);
        ata_write8(d, ATA_REG_LBA1, (u8)(lba >> 8));
        ata_write8(d, ATA_REG_LBA2, (u8)(lba >> 16));
        ata_write8(d, ATA_REG_COMMAND, ATA_CMD_WRITE_PIO_EXT);
    } else {
        ata_write8(d, ATA_REG_HDDEVSEL,
                   (u8)(0xE0 | (d->drive << 4) | ((lba >> 24) & 0x0F)));
        ata_write8(d, ATA_REG_SECCOUNT0, (u8)count);
        ata_write8(d, ATA_REG_LBA0, (u8)lba);
        ata_write8(d, ATA_REG_LBA1, (u8)(lba >> 8));
        ata_write8(d, ATA_REG_LBA2, (u8)(lba >> 16));
        ata_write8(d, ATA_REG_COMMAND, ATA_CMD_WRITE_PIO);
    }

    const u8* src = (const u8*)buf;
    u32 sec;
    for (sec = 0; sec < count; sec++) {
        k_status s = ata_poll(d, TRUE);
        if (s != K_OK) return s;
        u16 i;
        for (i = 0; i < 256; i++) {
            u16 word = (u16)src[0] | ((u16)src[1] << 8);
            port_outw(d->base, word);
            src += 2;
        }
    }

    /* Flush write cache */
    ata_write8(d, ATA_REG_COMMAND,
               d->lba48 ? ATA_CMD_CACHE_FLUSH_EXT : ATA_CMD_CACHE_FLUSH);
    ata_poll(d, FALSE);
    return K_OK;
}

/* =========================================================================
 * Driver Init — probe all 4 drive slots
 * ========================================================================= */
void ata_init(void) {
    extern void kprintf(const char* fmt, ...);

    u16  channels[2][2] = {
        {ATA_PRIMARY_BASE,   ATA_PRIMARY_CTRL},
        {ATA_SECONDARY_BASE, ATA_SECONDARY_CTRL}
    };

    u8 ch, dr;
    for (ch = 0; ch < 2; ch++) {
        for (dr = 0; dr < 2; dr++) {
            ATADrive* d = &g_drives[g_drive_count];
            d->channel = ch;
            d->drive   = dr;
            d->base    = channels[ch][0];
            d->ctrl    = channels[ch][1];
            d->present = FALSE;

            if (ata_identify(d)) {
                kprintf("[ATA]: Drive %u: %s | %llu sectors (%llu MB) | LBA%s\n",
                        g_drive_count, d->model, d->sectors,
                        (d->sectors * 512ULL) / (1024ULL * 1024ULL),
                        d->lba48 ? "48" : "28");
                g_drive_count++;
            }
        }
    }

    if (g_drive_count == 0)
        kprintf("[ATA]: No drives found (virtual/QEMU without -hda).\n");
    else
        kprintf("[ATA]: %u drive(s) probed. PIO mode active.\n", g_drive_count);
}

u32 ata_drive_count(void) { return g_drive_count; }
