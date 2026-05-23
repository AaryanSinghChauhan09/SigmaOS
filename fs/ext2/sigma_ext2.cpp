/*
 * Σ SigmaOS Zenith — ext2 Filesystem Shard
 * Absorbs: Linux fs/ext2/ super.c/inode.c
 * Zero-Dependency: No libc, no stdlib.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

#define EXT2_SUPER_MAGIC 0xEF53

struct ext2_super_block {
    u32 s_inodes_count;
    u32 s_blocks_count;
    u32 s_r_blocks_count;
    u32 s_free_blocks_count;
    u32 s_free_inodes_count;
    u32 s_first_data_block;
    u32 s_log_block_size;
    u32 s_log_frag_size;
    u32 s_blocks_per_group;
    u32 s_frags_per_group;
    u32 s_inodes_per_group;
    u32 s_mtime;
    u32 s_wtime;
    u16 s_mnt_count;
    u16 s_max_mnt_count;
    u16 s_magic;
    u16 s_state;
    u16 s_errors;
    u16 s_minor_rev_level;
    u32 s_lastcheck;
    u32 s_checkinterval;
    u32 s_creator_os;
    u32 s_rev_level;
    u16 s_def_resuid;
    u16 s_def_resgid;
};

extern "C" bool sigma_ata_read_sector(u32 lba, u8* buffer);
extern "C" void sigma_vga_printf(const char* fmt, ...);

static struct ext2_super_block sb;

extern "C" bool sigma_ext2_mount(u32 partition_lba) {
    u8 sector[512];
    /* ext2 superblock starts at byte 1024 (LBA 2 if partition starts at 0, 
       but relative to partition it's at offset 1024, so LBA 2 of partition) */
    if (!sigma_ata_read_sector(partition_lba + 2, sector)) {
        return false;
    }
    
    // Copy superblock
    u8* p = (u8*)&sb;
    for(int i=0; i<sizeof(sb); i++) {
        p[i] = sector[i];
    }
    
    if (sb.s_magic != EXT2_SUPER_MAGIC) {
        sigma_vga_printf("ext2 mount failed: bad magic 0x%X\n", sb.s_magic);
        return false;
    }
    
    sigma_vga_printf("ext2 mounted! Inodes: %u, Blocks: %u\n", sb.s_inodes_count, sb.s_blocks_count);
    return true;
}
