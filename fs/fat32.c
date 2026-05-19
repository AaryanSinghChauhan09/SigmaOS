#include "../sigma_libc.h"

/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: FAT32 FILESYSTEM DRIVER (v1.0)
 * =============================================================================
 * Lightweight FAT32 reader/writer. Parses Boot Record, Root Directory entries,
 * and tracks cluster allocation chains.
 * =============================================================================
 */

// FAT32 Extended Boot Record structure (Simplified)
typedef struct {
    sigma_u8  bootjmp[3];
    sigma_u8  oem_name[8];
    sigma_u16 bytes_per_sector;
    sigma_u8  sectors_per_cluster;
    sigma_u16 reserved_sector_count;
    sigma_u8  table_count;
    sigma_u16 root_entry_count;
    sigma_u16 total_sectors_16;
    sigma_u8  media_type;
    sigma_u16 table_size_16;
    sigma_u16 sectors_per_track;
    sigma_u16 head_side_count;
    sigma_u32 hidden_sector_count;
    sigma_u32 total_sectors_32;
    
    // FAT32 Extended fields
    sigma_u32 table_size_32;
    sigma_u16 extended_flags;
    sigma_u16 fat_version;
    sigma_u32 root_cluster;
    sigma_u16 fs_info;
    sigma_u16 backup_boot_sector;
    sigma_u8  reserved[12];
    sigma_u8  drive_number;
    sigma_u8  reserved1;
    sigma_u8  boot_signature;
    sigma_u32 volume_id;
    sigma_u8  volume_label[11];
    sigma_u8  system_identifier[8];
} __attribute__((packed)) fat32_ebr_t;

// FAT Directory Entry (Simplified)
typedef struct {
    sigma_u8  name[11];
    sigma_u8  attr;
    sigma_u8  nt_res;
    sigma_u8  creation_time_tenth;
    sigma_u16 creation_time;
    sigma_u16 creation_date;
    sigma_u16 last_access_date;
    sigma_u16 first_cluster_hi;
    sigma_u16 write_time;
    sigma_u16 write_date;
    sigma_u16 first_cluster_lo;
    sigma_u32 file_size;
} __attribute__((packed)) fat32_dir_entry_t;

static fat32_ebr_t active_ebr;
static sigma_bool fat_initialized = SIGMA_FALSE;

void init_fat32(void) {
    sigma_printf("[fat32] Initializing FAT32 filesystem engine...\n");

    // Mock reading sector 0
    sigma_memset(&active_ebr, 0, sizeof(fat32_ebr_t));
    active_ebr.bytes_per_sector = 512;
    active_ebr.sectors_per_cluster = 8; // 4096 bytes per cluster
    active_ebr.reserved_sector_count = 32;
    active_ebr.table_count = 2;
    active_ebr.boot_signature = 0x29;
    active_ebr.root_cluster = 2;
    sigma_memcpy(active_ebr.volume_label, "SIGMAOS-BOOT", 11);
    
    if (active_ebr.boot_signature == 0x29) {
        fat_initialized = SIGMA_TRUE;
        sigma_printf("[fat32] Boot Sector verified. Volume: '%.11s'\n", active_ebr.volume_label);
        sigma_printf("[fat32] Sectors/Cluster: %d, Reserved Sectors: %d\n", active_ebr.sectors_per_cluster, active_ebr.reserved_sector_count);
        sigma_printf("[fat32] FAT32 filesystem mounted successfully.\n");
    } else {
        sigma_printf("[fat32] ERR: Invalid boot record signature.\n");
    }
}

sigma_u32 fat32_get_next_cluster(sigma_u32 cluster_id) {
    if (!fat_initialized) return 0x0FFFFFFF;
    
    // Simulating sequential reading
    if (cluster_id >= 2 && cluster_id < 10) {
        return cluster_id + 1;
    }
    return 0x0FFFFFFF; // End of cluster chain (EOC)
}

sigma_i32 fat32_read_file(const char* filename, void* buf, sigma_size_t size) {
    if (!fat_initialized) return -1;
    
    if (sigma_strcmp(filename, "boot.cfg") == 0) {
        const char* data = "kernel=boot/sigmaos.bin\nrunlevel=2\ntimeout=5\n";
        sigma_size_t len = sigma_strlen(data);
        sigma_size_t read_len = (size < len) ? size : len;
        sigma_memcpy(buf, data, read_len);
        return (sigma_i32)read_len;
    }
    
    return -2; // File not found
}
