/*
 * Σ SigmaOS Zenith — SigmaFAT32 Filesystem (FAT32 Clone)
 * Absorbs: Linux VFS design, FAT32 specification, ext2 simplicity
 * Zero-Dependency: No libc, no stdlib, no predefined headers or functions.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

/* ─────────────── Sovereign Utilities ─────────────── */
static u32 sovereign_strlen(const char* s) {
    u32 n = 0;
    while (s[n]) n++;
    return n;
}

static bool sovereign_strncmp(const char* a, const char* b, u32 n) {
    for (u32 i = 0; i < n; i++) {
        if (a[i] != b[i]) return false;
        if (!a[i]) break;
    }
    return true;
}

static void sovereign_strncpy(char* dst, const char* src, u32 n) {
    u32 i;
    for (i = 0; i < n && src[i]; i++) dst[i] = src[i];
    for (; i < n; i++) dst[i] = '\0';
}

static u32 sovereign_toupper(u8 c) {
    return (c >= 'a' && c <= 'z') ? c - 32 : c;
}

/* ─────────────── FAT32 On-Disk Structures ─────────────── */
/* Inspired by the FAT32 specification (Microsoft) */

struct __attribute__((packed)) SigmaFAT32_BPB {
    u8  jump_boot[3];        /* EB 58 90 */
    u8  oem_name[8];         /* "SigmaOS " */
    u16 bytes_per_sector;    /* Must be 512 */
    u8  sectors_per_cluster;
    u16 reserved_sector_count;
    u8  num_fats;
    u16 root_entry_count;    /* 0 for FAT32 */
    u16 total_sectors_16;    /* 0 if >65535 */
    u8  media;
    u16 fat_size_16;         /* 0 for FAT32 */
    u16 sectors_per_track;
    u16 num_heads;
    u32 hidden_sectors;
    u32 total_sectors_32;
    /* FAT32 Extended BPB */
    u32 fat_size_32;
    u16 ext_flags;
    u16 fs_version;
    u32 root_cluster;        /* First cluster of root dir, typically 2 */
    u16 fs_info;
    u16 backup_boot_sector;
    u8  reserved[12];
    u8  drive_num;
    u8  reserved1;
    u8  boot_signature;      /* 0x29 */
    u32 volume_id;
    u8  volume_label[11];    /* "SIGMAOS    " */
    u8  fs_type[8];          /* "FAT32   " */
};

struct __attribute__((packed)) SigmaFAT32_DirEntry {
    u8  name[8];             /* Short 8.3 name */
    u8  ext[3];
    u8  attr;
    u8  reserved;
    u8  creation_time_tenth;
    u16 creation_time;
    u16 creation_date;
    u16 last_access_date;
    u16 cluster_high;        /* High 16 bits of first cluster */
    u16 write_time;
    u16 write_date;
    u16 cluster_low;         /* Low 16 bits of first cluster */
    u32 file_size;
};

/* Directory attributes */
#define ATTR_READ_ONLY  0x01
#define ATTR_HIDDEN     0x02
#define ATTR_SYSTEM     0x04
#define ATTR_VOLUME_ID  0x08
#define ATTR_DIRECTORY  0x10
#define ATTR_ARCHIVE    0x20
#define ATTR_LONG_NAME  0x0F

/* FAT32 special cluster values */
#define FAT32_EOC       0x0FFFFFFF   /* End of chain */
#define FAT32_FREE      0x00000000

/* ─────────────── Filesystem State ─────────────── */
struct SigmaFAT32_FS {
    struct SigmaFAT32_BPB bpb;
    u32 fat_start_lba;
    u32 data_start_lba;
    u32 root_cluster;
    u32 bytes_per_cluster;
    bool mounted;
};

static struct SigmaFAT32_FS sigma_fs;

/* ─────────────── Disk I/O Bridge ─────────────── */
/* Calls the ATA driver from sigma_ata_driver.cpp */
extern "C" bool sigma_ata_read_sector(u32 lba, u8* buffer);

static bool read_sectors(u32 lba, u32 count, u8* buffer) {
    for (u32 i = 0; i < count; i++) {
        if (!sigma_ata_read_sector(lba + i, buffer + (i * 512)))
            return false;
    }
    return true;
}

/* ─────────────── Cluster Arithmetic ─────────────── */
static u32 cluster_to_lba(u32 cluster) {
    return sigma_fs.data_start_lba + (cluster - 2) * sigma_fs.bpb.sectors_per_cluster;
}

/* ─────────────── API: Mount FAT32 Volume ─────────────── */
extern "C" bool sigma_fat32_mount(u32 start_lba) {
    u8 sector_buf[512];
    if (!read_sectors(start_lba, 1, sector_buf)) return false;

    /* Copy BPB */
    for (u32 i = 0; i < sizeof(struct SigmaFAT32_BPB); i++)
        ((u8*)&sigma_fs.bpb)[i] = sector_buf[i];

    if (sigma_fs.bpb.bytes_per_sector != 512) return false;

    sigma_fs.fat_start_lba  = start_lba + sigma_fs.bpb.reserved_sector_count;
    sigma_fs.data_start_lba = sigma_fs.fat_start_lba
                            + (sigma_fs.bpb.num_fats * sigma_fs.bpb.fat_size_32);
    sigma_fs.root_cluster      = sigma_fs.bpb.root_cluster;
    sigma_fs.bytes_per_cluster = sigma_fs.bpb.sectors_per_cluster * 512;
    sigma_fs.mounted = true;
    return true;
}

/* ─────────────── API: Read FAT Entry ─────────────── */
static u32 fat32_get_next_cluster(u32 cluster) {
    u32 fat_offset = cluster * 4;
    u32 fat_sector = sigma_fs.fat_start_lba + (fat_offset / 512);
    u32 offset     = fat_offset % 512;
    u8 buf[512];
    read_sectors(fat_sector, 1, buf);
    u32 val = *((u32*)(buf + offset)) & 0x0FFFFFFF;
    return val;
}

/* ─────────────── API: Find File in Directory ─────────────── */
static bool find_in_dir(u32 cluster, const char* name_8_3, struct SigmaFAT32_DirEntry* out) {
    u8 buf[4096]; /* Max 1 cluster */
    u32 spc = sigma_fs.bpb.sectors_per_cluster;
    read_sectors(cluster_to_lba(cluster), spc, buf);

    u32 entries = (spc * 512) / sizeof(struct SigmaFAT32_DirEntry);
    struct SigmaFAT32_DirEntry* dir = (struct SigmaFAT32_DirEntry*)buf;

    for (u32 i = 0; i < entries; i++) {
        if (dir[i].name[0] == 0x00) break;     /* End of directory */
        if (dir[i].name[0] == 0xE5) continue;  /* Deleted entry */
        if (dir[i].attr == ATTR_LONG_NAME) continue;

        char entry_name[12];
        sovereign_strncpy(entry_name, (char*)dir[i].name, 8);
        /* Normalize the name for comparison */
        if (sovereign_strncmp(entry_name, name_8_3, 8)) {
            for (u32 b = 0; b < sizeof(struct SigmaFAT32_DirEntry); b++)
                ((u8*)out)[b] = ((u8*)&dir[i])[b];
            return true;
        }
    }
    return false;
}

/* ─────────────── API: Read File Contents ─────────────── */
extern "C" u32 sigma_fat32_read(const char* filename_8_3, u8* out_buf, u32 max_len) {
    if (!sigma_fs.mounted) return 0;

    struct SigmaFAT32_DirEntry entry;
    if (!find_in_dir(sigma_fs.root_cluster, filename_8_3, &entry)) return 0;

    u32 cluster = ((u32)entry.cluster_high << 16) | entry.cluster_low;
    u32 file_size = entry.file_size;
    u32 bytes_read = 0;
    u32 spc = sigma_fs.bpb.sectors_per_cluster;

    while (cluster < 0x0FFFFFF8 && bytes_read < file_size && bytes_read < max_len) {
        u32 to_read = sigma_fs.bytes_per_cluster;
        if (to_read > max_len - bytes_read) to_read = max_len - bytes_read;
        if (to_read > file_size - bytes_read) to_read = file_size - bytes_read;

        u8 tmp[4096];
        read_sectors(cluster_to_lba(cluster), spc, tmp);
        for (u32 i = 0; i < to_read; i++)
            out_buf[bytes_read + i] = tmp[i];

        bytes_read += to_read;
        cluster = fat32_get_next_cluster(cluster);
    }

    return bytes_read;
}
