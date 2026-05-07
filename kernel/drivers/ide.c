#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: IDE-PIO DRIVER (v1.0)
 * =============================================================================
 * Principles: Zero-Abstract Hardware Interaction.
 * =============================================================================
 */
#include "core/sigma_kernel_types.h"

#define IDE_DATA        0x1F0
#define IDE_ERROR       0x1F1
#define IDE_SEC_COUNT   0x1F2
#define IDE_LBA_LOW     0x1F3
#define IDE_LBA_MID     0x1F4
#define IDE_LBA_HIGH    0x1F5
#define IDE_DRIVE_SEL   0x1F6
#define IDE_COMMAND     0x1F7
#define IDE_STATUS      0x1F7

extern void port_outb(sigma_u16 port, sigma_u8 val);
extern sigma_u8 port_inb(sigma_u16 port);
extern void port_outw(sigma_u16 port, sigma_u16 val);

void ide_write_sector(sigma_u32 lba, sigma_u8* buffer) {
    port_outb(IDE_DRIVE_SEL, (lba >> 24) | 0xE0);
    port_outb(IDE_SEC_COUNT, 1);
    port_outb(IDE_LBA_LOW, (sigma_u8)lba);
    port_outb(IDE_LBA_MID, (sigma_u8)(lba >> 8));
    port_outb(IDE_LBA_HIGH, (sigma_u8)(lba >> 16));
    port_outb(IDE_COMMAND, 0x30); /* Write Sectors */

    while (!(port_inb(IDE_STATUS) & 0x08)); /* Wait for DRQ */

    for (int i = 0; i < 256; i++) {
        sigma_u16 data = buffer[i*2] | (buffer[i*2+1] << 8);
        port_outw(IDE_DATA, data);
    }
}

void ide_read_sector(sigma_u32 lba, sigma_u8* buffer) {
    port_outb(IDE_DRIVE_SEL, (lba >> 24) | 0xE0);
    port_outb(IDE_SEC_COUNT, 1);
    port_outb(IDE_LBA_LOW, (sigma_u8)lba);
    port_outb(IDE_LBA_MID, (sigma_u8)(lba >> 8));
    port_outb(IDE_LBA_HIGH, (sigma_u8)(lba >> 16));
    port_outb(IDE_COMMAND, 0x20); /* Read Sectors */

    while (!(port_inb(IDE_STATUS) & 0x08));

    for (int i = 0; i < 256; i++) {
        /* Read logic here... */
    }
}
