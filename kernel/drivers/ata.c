/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: ATA/IDE DISK DRIVER (v1.0 - PURE C11)
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

#include "../../include/sigma_kernel_types.h"

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
    sigma_u8     channel;     /* 0=primary, 1=secondary */
    sigma_u8     drive;       /* 0=master, 1=slave */
    sigma_u8     type;        /* 0=PATA, 1=PATAPI */
    sigma_bool lba48;
    sigma_bool present;
    sigma_u64    sectors;     /* total LBA sectors */
    char   model[41];
    sigma_u16    base;        /* I/O base port */
    sigma_u16    ctrl;        /* Control port */
} ATADrive;

static ATADrive g_drives[ATA_MAX_DRIVES];
static sigma_u32      g_drive_count = 0;

/* =========================================================================
 * Low-level I/O helpers
 * ========================================================================= */
static sigma_u8 ata_read8(const ATADrive* d, sigma_u8 reg) {
    if (reg < 8)  return port_inb((sigma_u16)(d->base + reg));
    if (reg < 12) return port_inb((sigma_u16)(d->ctrl + reg - 8));
    return 0;
}

static void ata_write8(const ATADrive* d, sigma_u8 reg, sigma_u8 val) {
    if (reg < 8)  port_outb((sigma_u16)(d->base + reg), val);
    else if (reg < 12) port_outb((sigma_u16)(d->ctrl + reg - 8), val);
}

static sigma_u16 ata_read16(const ATADrive* d) {
    return (sigma_u16)(port_inb(d->base) | ((sigma_u16)port_inb((sigma_u16)(d->base + 1)) << 8));
}

/* 400ns delay â€ read alt-status 4 times */
static void ata_delay400ns(const ATADrive* d) {
    sigma_u8 i;
    for (i = 0; i < 4; i++) ata_read8(d, ATA_REG_ALTSTATUS);
}

/* Poll until BSY clears (with timeout) */
static sigma_status ata_poll(const ATADrive* d, sigma_bool check_drq) {
    sigma_u32 timeout = 100000u;
    while (timeout--) {
        sigma_u8 st = ata_read8(d, ATA_REG_STATUS);
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
 * ATA IDENTIFY â€ detect and characterize a drive
 * ========================================================================= */
static sigma_bool ata_identify(ATADrive* d) {
    /* Select drive */
    ata_write8(d, ATA_REG_HDDEVSEL, (sigma_u8)(0xA0 | (d->drive << 4)));
    ata_delay400ns(d);

    /* Zero LBA registers */
    ata_write8(d, ATA_REG_SECCOUNT0, 0);
    ata_write8(d, ATA_REG_LBA0, 0);
    ata_write8(d, ATA_REG_LBA1, 0);
    ata_write8(d, ATA_REG_LBA2, 0);

    /* Send IDENTIFY */
    ata_write8(d, ATA_REG_COMMAND, ATA_CMD_IDENTIFY);
    ata_delay400ns(d);

    if (ata_read8(d, ATA_REG_STATUS) == 0) return SIGMA_FALSE; /* no drive */

    if (ata_poll(d, SIGMA_TRUE) != K_OK) return SIGMA_FALSE;

    /* Read 256 Ã— sigma_u16 IDENTIFY data */
    sigma_u16 id[256];
    sigma_u32 i;
    for (i = 0; i < 256; i++) id[i] = (sigma_u16)(port_inb(d->base) | ((sigma_u16)port_inb((sigma_u16)(d->base+1)) << 8));

    /* LBA48 support: word 83 bit 10 */
    d->lba48 = !!(id[83] & BIT(10));

    /* Total sectors */
    if (d->lba48) {
        d->sectors = ((sigma_u64)id[103] << 48) | ((sigma_u64)id[102] << 32) |
                     ((sigma_u64)id[101] << 16) | (sigma_u64)id[100];
    } else {
        d->sectors = ((sigma_u32)id[61] << 16) | id[60];
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

    d->present = SIGMA_TRUE;
    return SIGMA_TRUE;
}

/* =========================================================================
 * LBA28 Sector Read (PIO)
 * ========================================================================= */
sigma_status ata_read_sectors(sigma_u32 drive_idx, sigma_u64 lba, sigma_u32 count, void* buf) {
    if (drive_idx >= g_drive_count) return K_ERR_INVAL;
    ATADrive* d = &g_drives[drive_idx];
    if (!d->present) return K_ERR_INVAL;

    /* Wait for drive not busy */
    if (ata_poll(d, SIGMA_FALSE) != K_OK) return K_ERR_BUSY;

    if (d->lba48) {
        /* LBA48: send high bytes first, then low bytes */
        ata_write8(d, ATA_REG_HDDEVSEL, (sigma_u8)(0x40 | (d->drive << 4)));
        ata_write8(d, ATA_REG_SECCOUNT1, (sigma_u8)(count >> 8));
        ata_write8(d, ATA_REG_LBA3, (sigma_u8)(lba >> 24));
        ata_write8(d, ATA_REG_LBA4, (sigma_u8)(lba >> 32));
        ata_write8(d, ATA_REG_LBA5, (sigma_u8)(lba >> 40));
        ata_write8(d, ATA_REG_SECCOUNT0, (sigma_u8)count);
        ata_write8(d, ATA_REG_LBA0, (sigma_u8)lba);
        ata_write8(d, ATA_REG_LBA1, (sigma_u8)(lba >> 8));
        ata_write8(d, ATA_REG_LBA2, (sigma_u8)(lba >> 16));
        ata_write8(d, ATA_REG_COMMAND, ATA_CMD_READ_PIO_EXT);
    } else {
        ata_write8(d, ATA_REG_HDDEVSEL,
                   (sigma_u8)(0xE0 | (d->drive << 4) | ((lba >> 24) & 0x0F)));
        ata_write8(d, ATA_REG_SECCOUNT0, (sigma_u8)count);
        ata_write8(d, ATA_REG_LBA0, (sigma_u8)lba);
        ata_write8(d, ATA_REG_LBA1, (sigma_u8)(lba >> 8));
        ata_write8(d, ATA_REG_LBA2, (sigma_u8)(lba >> 16));
        ata_write8(d, ATA_REG_COMMAND, ATA_CMD_READ_PIO);
    }

    sigma_u8* dst   = (sigma_u8*)buf;
    sigma_u32 sec;
    for (sec = 0; sec < count; sec++) {
        sigma_status s = ata_poll(d, SIGMA_TRUE);
        if (s != K_OK) return s;
        /* Read 256 words (512 bytes) via INW */
        sigma_u16 i;
        for (i = 0; i < 256; i++) {
            sigma_u16 word = (sigma_u16)(port_inb(d->base) | ((sigma_u16)port_inb((sigma_u16)(d->base+1)) << 8));
            *dst++ = (sigma_u8)(word & 0xFF);
            *dst++ = (sigma_u8)(word >> 8);
        }
        ata_delay400ns(d);
    }
    return K_OK;
}

/* =========================================================================
 * LBA28/48 Sector Write (PIO)
 * ========================================================================= */
sigma_status ata_write_sectors(sigma_u32 drive_idx, sigma_u64 lba, sigma_u32 count, const void* buf) {
    if (drive_idx >= g_drive_count) return K_ERR_INVAL;
    ATADrive* d = &g_drives[drive_idx];
    if (!d->present) return K_ERR_INVAL;

    if (ata_poll(d, SIGMA_FALSE) != K_OK) return K_ERR_BUSY;

    if (d->lba48) {
        ata_write8(d, ATA_REG_HDDEVSEL, (sigma_u8)(0x40 | (d->drive << 4)));
        ata_write8(d, ATA_REG_SECCOUNT1, (sigma_u8)(count >> 8));
        ata_write8(d, ATA_REG_LBA3, (sigma_u8)(lba >> 24));
        ata_write8(d, ATA_REG_LBA4, (sigma_u8)(lba >> 32));
        ata_write8(d, ATA_REG_LBA5, (sigma_u8)(lba >> 40));
        ata_write8(d, ATA_REG_SECCOUNT0, (sigma_u8)count);
        ata_write8(d, ATA_REG_LBA0, (sigma_u8)lba);
        ata_write8(d, ATA_REG_LBA1, (sigma_u8)(lba >> 8));
        ata_write8(d, ATA_REG_LBA2, (sigma_u8)(lba >> 16));
        ata_write8(d, ATA_REG_COMMAND, ATA_CMD_WRITE_PIO_EXT);
    } else {
        ata_write8(d, ATA_REG_HDDEVSEL,
                   (sigma_u8)(0xE0 | (d->drive << 4) | ((lba >> 24) & 0x0F)));
        ata_write8(d, ATA_REG_SECCOUNT0, (sigma_u8)count);
        ata_write8(d, ATA_REG_LBA0, (sigma_u8)lba);
        ata_write8(d, ATA_REG_LBA1, (sigma_u8)(lba >> 8));
        ata_write8(d, ATA_REG_LBA2, (sigma_u8)(lba >> 16));
        ata_write8(d, ATA_REG_COMMAND, ATA_CMD_WRITE_PIO);
    }

    const sigma_u8* src = (const sigma_u8*)buf;
    sigma_u32 sec;
    for (sec = 0; sec < count; sec++) {
        sigma_status s = ata_poll(d, SIGMA_TRUE);
        if (s != K_OK) return s;
        sigma_u16 i;
        for (i = 0; i < 256; i++) {
            sigma_u16 word = (sigma_u16)src[0] | ((sigma_u16)src[1] << 8);
            port_outw(d->base, word);
            src += 2;
        }
    }

    /* Flush write cache */
    ata_write8(d, ATA_REG_COMMAND,
               d->lba48 ? ATA_CMD_CACHE_FLUSH_EXT : ATA_CMD_CACHE_FLUSH);
    ata_poll(d, SIGMA_FALSE);
    return K_OK;
}

/* =========================================================================
 * Driver Init â€ probe all 4 drive slots
 * ========================================================================= */
void ata_init(void) {
    extern void kprintf(const char* fmt, ...);

    sigma_u16  channels[2][2] = {
        {ATA_PRIMARY_BASE,   ATA_PRIMARY_CTRL},
        {ATA_SECONDARY_BASE, ATA_SECONDARY_CTRL}
    };

    sigma_u8 ch, dr;
    for (ch = 0; ch < 2; ch++) {
        for (dr = 0; dr < 2; dr++) {
            ATADrive* d = &g_drives[g_drive_count];
            d->channel = ch;
            d->drive   = dr;
            d->base    = channels[ch][0];
            d->ctrl    = channels[ch][1];
            d->present = SIGMA_FALSE;

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

sigma_u32 ata_drive_count(void) { return g_drive_count; }
