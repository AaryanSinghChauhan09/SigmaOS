/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: IDE-PIO DRIVER (v1.0)
 * =============================================================================
 * Principles: Zero-Abstract Hardware Interaction.
 * =============================================================================
 */
#include "../include/sigma_kernel_types.h"

#define IDE_DATA        0x1F0
#define IDE_ERROR       0x1F1
#define IDE_SEC_COUNT   0x1F2
#define IDE_LBA_LOW     0x1F3
#define IDE_LBA_MID     0x1F4
#define IDE_LBA_HIGH    0x1F5
#define IDE_DRIVE_SEL   0x1F6
#define IDE_COMMAND     0x1F7
#define IDE_STATUS      0x1F7

extern void outb(u16 port, u8 val);
extern u8 inb(u16 port);
extern void outw(u16 port, u16 val);

void ide_write_sector(u32 lba, u8* buffer) {
    outb(IDE_DRIVE_SEL, (lba >> 24) | 0xE0);
    outb(IDE_SEC_COUNT, 1);
    outb(IDE_LBA_LOW, (u8)lba);
    outb(IDE_LBA_MID, (u8)(lba >> 8));
    outb(IDE_LBA_HIGH, (u8)(lba >> 16));
    outb(IDE_COMMAND, 0x30); /* Write Sectors */

    while (!(inb(IDE_STATUS) & 0x08)); /* Wait for DRQ */

    for (int i = 0; i < 256; i++) {
        u16 data = buffer[i*2] | (buffer[i*2+1] << 8);
        outw(IDE_DATA, data);
    }
}

void ide_read_sector(u32 lba, u8* buffer) {
    outb(IDE_DRIVE_SEL, (lba >> 24) | 0xE0);
    outb(IDE_SEC_COUNT, 1);
    outb(IDE_LBA_LOW, (u8)lba);
    outb(IDE_LBA_MID, (u8)(lba >> 8));
    outb(IDE_LBA_HIGH, (u8)(lba >> 16));
    outb(IDE_COMMAND, 0x20); /* Read Sectors */

    while (!(inb(IDE_STATUS) & 0x08));

    for (int i = 0; i < 256; i++) {
        /* Read logic here... */
    }
}
