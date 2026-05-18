#include "libc/sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS FAT32 Filesystem Parser Prototype
// ---------------------------------------------------------

#pragma pack(push, 1)
typedef struct {
    uint8_t  jump_boot[3];
    char     oem_name[8];
    uint16_t bytes_per_sector;
    uint8_t  sectors_per_cluster;
    uint16_t reserved_sector_count;
    uint8_t  num_fats;
    uint16_t root_entry_count;
    uint16_t total_sectors_16;
    uint8_t  media_type;
    uint16_t fat_size_16;
    uint16_t sectors_per_track;
    uint16_t num_heads;
    uint32_t hidden_sectors;
    uint32_t total_sectors_32;
    // FAT32 Extended fields
    uint32_t fat_size_32;
    uint16_t ext_flags;
    uint16_t fs_version;
    uint32_t root_cluster;
} fat32_bpb_t;

typedef struct {
    char     name[11];
    uint8_t  attr;
    uint8_t  nt_reserved;
    uint8_t  creation_time_tenth;
    uint16_t creation_time;
    uint16_t creation_date;
    uint16_t last_access_date;
    uint16_t first_cluster_hi;
    uint16_t write_time;
    uint16_t write_date;
    uint16_t first_cluster_lo;
    uint32_t file_size;
} fat32_dir_entry_t;
#pragma pack(pop)

// Simulating disk read
extern void disk_read_sectors(uint32_t lba, uint32_t count, void* buffer);

static fat32_bpb_t boot_sector;

// Mount FAT32 Partition
int fat32_init() {
    // Read the boot sector (LBA 0)
    // disk_read_sectors(0, 1, &boot_sector);
    
    // In prototype, just mock it
    boot_sector.bytes_per_sector = 512;
    boot_sector.sectors_per_cluster = 8;
    boot_sector.root_cluster = 2;
    
    return 0; // Successfully parsed BPB
}

// Convert a cluster number to Logical Block Address (LBA)
uint32_t fat32_cluster_to_lba(uint32_t cluster) {
    uint32_t first_data_sector = boot_sector.reserved_sector_count + (boot_sector.num_fats * boot_sector.fat_size_32);
    return first_data_sector + ((cluster - 2) * boot_sector.sectors_per_cluster);
}

// Minimal implementation to find a file in a directory cluster
int fat32_find_file(uint32_t dir_cluster, const char* filename, fat32_dir_entry_t* out_entry) {
    uint8_t buffer[4096]; // Assume 4K cluster size
    uint32_t lba = fat32_cluster_to_lba(dir_cluster);
    
    // disk_read_sectors(lba, boot_sector.sectors_per_cluster, buffer);
    
    fat32_dir_entry_t* entries = (fat32_dir_entry_t*)buffer;
    for (int i = 0; i < (4096 / sizeof(fat32_dir_entry_t)); i++) {
        if (entries[i].name[0] == 0x00) break; // End of directory
        if (entries[i].name[0] == 0xE5) continue; // Deleted file
        
        // Match logic would go here (accounting for 8.3 format)
        // if (match_83_filename(entries[i].name, filename)) {
        //     *out_entry = entries[i];
        //     return 1;
        // }
    }
    return 0; // Not found
}
