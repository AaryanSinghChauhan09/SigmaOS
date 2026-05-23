/*
 * Σ SigmaOS Zenith — ATA / SATA Hard Disk Driver Shard
 * Zero-Dependency Implementation. No predefined libraries.
 */

/* Internal freestanding types */
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;
typedef uint64_t size_t;

/* Sovereign memory utility (No libc) */
static void sovereign_memset(void* ptr, uint8_t value, size_t num) {
    uint8_t* p = (uint8_t*)ptr;
    while (num--) {
        *p++ = value;
    }
}

/* x86 port I/O routines (No external headers) */
static inline void sigma_outb(uint16_t port, uint8_t val) {
    __asm__ volatile ( "outb %0, %1" : : "a"(val), "Nd"(port) );
}

static inline uint8_t sigma_inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ( "inb %1, %0" : "=a"(ret) : "Nd"(port) );
    return ret;
}

static inline void sigma_inw_stream(uint16_t port, uint16_t* target, size_t count) {
    __asm__ volatile ( "rep insw" : "+D"(target), "+c"(count) : "d"(port) : "memory" );
}

/* ATA Ports */
#define ATA_PRIMARY_DATA         0x1F0
#define ATA_PRIMARY_ERR          0x1F1
#define ATA_PRIMARY_SECCOUNT     0x1F2
#define ATA_PRIMARY_LBA_LO       0x1F3
#define ATA_PRIMARY_LBA_MID      0x1F4
#define ATA_PRIMARY_LBA_HI       0x1F5
#define ATA_PRIMARY_DRV_HEAD     0x1F6
#define ATA_PRIMARY_COMM_STAT    0x1F7
#define ATA_PRIMARY_ALT_STAT     0x3F6

/* Status Bits */
#define ATA_SR_BSY     0x80
#define ATA_SR_DRQ     0x08
#define ATA_SR_ERR     0x01

/* Sovereign Shard Registration */
struct SigmaShard {
    uint32_t shard_id;
    const char* name;
    bool is_active;
};

static struct SigmaShard ata_shard = {
    0x0201, "ATA_SATA_DRIVER", false
};

/* ATA Wait routine */
static void ata_wait_bsy() {
    while (sigma_inb(ATA_PRIMARY_COMM_STAT) & ATA_SR_BSY);
}

static void ata_wait_drq() {
    while (!(sigma_inb(ATA_PRIMARY_COMM_STAT) & ATA_SR_DRQ));
}

/* API: Initialize ATA driver */
extern "C" void sigma_ata_init() {
    ata_shard.is_active = true;
    /* Soft reset */
    sigma_outb(ATA_PRIMARY_ALT_STAT, 0x04);
    for (int i = 0; i < 1000; i++) { __asm__ volatile("pause"); }
    sigma_outb(ATA_PRIMARY_ALT_STAT, 0x00);
}

/* API: Read 512-byte sector */
extern "C" bool sigma_ata_read_sector(uint32_t lba, uint8_t* buffer) {
    if (!ata_shard.is_active) return false;

    ata_wait_bsy();
    
    /* Select drive and LBA bits 24-27 */
    sigma_outb(ATA_PRIMARY_DRV_HEAD, 0xE0 | ((lba >> 24) & 0x0F));
    
    /* Sector count (1) */
    sigma_outb(ATA_PRIMARY_SECCOUNT, 1);
    
    /* LBA 0-23 */
    sigma_outb(ATA_PRIMARY_LBA_LO, (uint8_t) lba);
    sigma_outb(ATA_PRIMARY_LBA_MID, (uint8_t)(lba >> 8));
    sigma_outb(ATA_PRIMARY_LBA_HI, (uint8_t)(lba >> 16));
    
    /* Send Read command (0x20) */
    sigma_outb(ATA_PRIMARY_COMM_STAT, 0x20);
    
    ata_wait_bsy();
    
    if (sigma_inb(ATA_PRIMARY_COMM_STAT) & ATA_SR_ERR) {
        return false;
    }
    
    ata_wait_drq();
    
    /* Read 256 words (512 bytes) */
    sigma_inw_stream(ATA_PRIMARY_DATA, (uint16_t*)buffer, 256);
    
    return true;
}
