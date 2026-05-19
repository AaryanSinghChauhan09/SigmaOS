/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: EXT4 FILESYSTEM SUPERBLOCK PARSER
 * =============================================================================
 * Inspired by: Linux kernel fs/ext4/super.c
 *              FreeBSD sys/fs/ext2fs/ext2_vfsops.c
 * =============================================================================
 * Safely parses the Ext4/Ext3/Ext2 superblock for volume mounting.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define EXT4_SUPER_MAGIC 0xEF53
#define EXT4_MIN_BLOCK_SIZE 1024
#define EXT4_MAX_BLOCK_SIZE 65536

typedef struct {
    sigma_u32 s_inodes_count;
    sigma_u32 s_blocks_count_lo;
    sigma_u32 s_r_blocks_count_lo;
    sigma_u32 s_free_blocks_count_lo;
    sigma_u32 s_free_inodes_count;
    sigma_u32 s_first_data_block;
    sigma_u32 s_log_block_size;
    sigma_u32 s_log_cluster_size;
    sigma_u32 s_blocks_per_group;
    sigma_u32 s_clusters_per_group;
    sigma_u32 s_inodes_per_group;
    sigma_u32 s_mtime;
    sigma_u32 s_wtime;
    sigma_u16 s_mnt_count;
    sigma_u16 s_max_mnt_count;
    sigma_u16 s_magic;
    sigma_u16 s_state;
    sigma_u16 s_errors;
    sigma_u16 s_minor_rev_level;
    sigma_u32 s_lastcheck;
    sigma_u32 s_checkinterval;
    sigma_u32 s_creator_os;
    sigma_u32 s_rev_level;
    sigma_u16 s_def_resuid;
    sigma_u16 s_def_resgid;
    
    /* EXT4 Dynamic Rev fields follow (simplified) */
    sigma_u32 s_first_ino;
    sigma_u16 s_inode_size;
    sigma_u16 s_block_group_nr;
    sigma_u32 s_feature_compat;
    sigma_u32 s_feature_incompat;
    sigma_u32 s_feature_ro_compat;
    sigma_u8  s_uuid[16];
    char      s_volume_name[16];
} __attribute__((packed)) ext4_super_block_t;

int ext4_parse_superblock(const void* block_data, sigma_u32 len) {
    if (len < sizeof(ext4_super_block_t)) {
        sigma_printf("[ext4] ERR: Insufficient data for superblock\n");
        return -1;
    }

    const ext4_super_block_t* sb = (const ext4_super_block_t*)block_data;

    if (sb->s_magic != EXT4_SUPER_MAGIC) {
        sigma_printf("[ext4] ERR: Invalid magic signature (found 0x%X, expected 0x%X)\n", 
                     sb->s_magic, EXT4_SUPER_MAGIC);
        return -1;
    }

    sigma_u32 block_size = 1024 << sb->s_log_block_size;
    
    if (block_size < EXT4_MIN_BLOCK_SIZE || block_size > EXT4_MAX_BLOCK_SIZE) {
        sigma_printf("[ext4] ERR: Invalid block size (%u bytes)\n", block_size);
        return -1;
    }

    char vol_name[17];
    sigma_u32 i = 0;
    while (i < 16 && sb->s_volume_name[i]) {
        vol_name[i] = sb->s_volume_name[i];
        i++;
    }
    vol_name[i] = '\0';

    sigma_printf("\n--- Σ EXT4 VOLUME MOUNTED ---\n");
    sigma_printf("| Volume Name : %s\n", vol_name[0] ? vol_name : "<unnamed>");
    sigma_printf("| Block Size  : %u bytes\n", block_size);
    sigma_printf("| Inode Size  : %u bytes\n", sb->s_rev_level > 0 ? sb->s_inode_size : 128);
    sigma_printf("| Total Blocks: %u\n", sb->s_blocks_count_lo);
    sigma_printf("| Total Inodes: %u\n", sb->s_inodes_count);
    sigma_printf("| Free Blocks : %u\n", sb->s_free_blocks_count_lo);
    sigma_printf("| Free Inodes : %u\n", sb->s_free_inodes_count);
    sigma_printf("-----------------------------\n");

    return 0;
}
