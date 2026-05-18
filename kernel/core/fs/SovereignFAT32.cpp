#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "sigma_hal.h"
#include "libc/SovereignLibC.h"

// FAT32 Boot Sector
#pragma pack(push, 1)
struct FAT32_BootSector {
    sigma_u8  jmp[3];
    char      oem[8];
    sigma_u16 bytes_per_sector;
    sigma_u8  sectors_per_cluster;
    sigma_u16 reserved_sectors;
    sigma_u8  fat_count;
    sigma_u16 root_entries;
    sigma_u16 total_sectors_16;
    sigma_u8  media_descriptor;
    sigma_u16 sectors_per_fat_16;
    sigma_u16 sectors_per_track;
    sigma_u16 heads;
    sigma_u32 hidden_sectors;
    sigma_u32 total_sectors_32;
    sigma_u32 sectors_per_fat_32;
    sigma_u16 ext_flags;
    sigma_u16 fs_version;
    sigma_u32 root_cluster;
    sigma_u16 fs_info_sector;
    sigma_u16 backup_boot_sector;
    sigma_u8  reserved[12];
    sigma_u8  drive_number;
    sigma_u8  reserved1;
    sigma_u8  boot_signature;
    sigma_u32 volume_id;
    char      volume_label[11];
    char      fs_type[8];
};
#pragma pack(pop)

class SovereignFAT32Driver {
public:
    static SovereignFAT32Driver& getInstance() {
        static SovereignFAT32Driver instance;
        return instance;
    }

    void init() {
        sigma_log_info("[FS/FAT32] Initializing Sovereign FAT32 Driver...\n");
        is_mounted = SIGMA_FALSE;
    }

    sigma_bool mount(void* partition_base) {
        sigma_log_info("[FS/FAT32] Attempting to mount FAT32 partition at %p\n", partition_base);
        FAT32_BootSector* boot_sector = (FAT32_BootSector*)partition_base;
        
        // Basic validation
        if (boot_sector->bytes_per_sector != 512 && boot_sector->bytes_per_sector != 4096) {
            sigma_log_info("[FS/FAT32] Invalid bytes per sector: %u\n", boot_sector->bytes_per_sector);
            return SIGMA_FALSE;
        }
        
        root_cluster = boot_sector->root_cluster;
        cluster_size = boot_sector->sectors_per_cluster * boot_sector->bytes_per_sector;
        is_mounted = SIGMA_TRUE;
        
        sigma_log_info("[FS/FAT32] Successfully mounted FAT32 volume '%11.11s'. Cluster size: %u bytes.\n", boot_sector->volume_label, cluster_size);
        return SIGMA_TRUE;
    }

    sigma_u32 readFile(const char* path, void* buffer, sigma_u32 max_size) {
        if (!is_mounted) return 0;
        sigma_log_info("[FS/FAT32] Reading file %s from FAT32 partition...\n", path);
        // Simulated read
        sigma_memset(buffer, 0, max_size);
        return max_size > 512 ? 512 : max_size; // Simulated return size
    }

private:
    SovereignFAT32Driver() : is_mounted(SIGMA_FALSE), root_cluster(0), cluster_size(0) {}
    sigma_bool is_mounted;
    sigma_u32 root_cluster;
    sigma_u32 cluster_size;
};

extern "C" void fat32_init() {
    SovereignFAT32Driver::getInstance().init();
}

extern "C" sigma_bool fat32_mount(void* base) {
    return SovereignFAT32Driver::getInstance().mount(base);
}

extern "C" sigma_u32 fat32_read(const char* path, void* buf, sigma_u32 max_sz) {
    return SovereignFAT32Driver::getInstance().readFile(path, buf, max_sz);
}
 