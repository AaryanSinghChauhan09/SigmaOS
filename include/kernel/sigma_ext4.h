/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: EXT4 FILESYSTEM TYPES
 * =============================================================================
 * Definitions for Ext4 on-disk structures.
 * =============================================================================
 */

#ifndef SIGMA_EXT4_H
#define SIGMA_EXT4_H

#include "../sigma_kernel_types.h"

#define EXT4_SUPER_MAGIC 0xEF53
#define EXT4_NDIR_BLOCKS 12
#define EXT4_IND_BLOCK   EXT4_NDIR_BLOCKS
#define EXT4_DIND_BLOCK  (EXT4_IND_BLOCK + 1)
#define EXT4_TIND_BLOCK  (EXT4_DIND_BLOCK + 1)
#define EXT4_N_BLOCKS    (EXT4_TIND_BLOCK + 1)

/* Ext4 Superblock Structure */
typedef struct __attribute__((packed)) {
    sigma_u32 s_inodes_count;       /* Inodes count */
    sigma_u32 s_blocks_count_lo;    /* Blocks count */
    sigma_u32 s_r_blocks_count_lo;  /* Reserved blocks count */
    sigma_u32 s_free_blocks_count_lo;/* Free blocks count */
    sigma_u32 s_free_inodes_count;  /* Free inodes count */
    sigma_u32 s_first_data_block;   /* First Data Block */
    sigma_u32 s_log_block_size;     /* Block size */
    sigma_u32 s_log_cluster_size;   /* Allocation cluster size */
    sigma_u32 s_blocks_per_group;   /* # Blocks per group */
    sigma_u32 s_clusters_per_group; /* # Clusters per group */
    sigma_u32 s_inodes_per_group;   /* # Inodes per group */
    sigma_u32 s_mtime;              /* Mount time */
    sigma_u32 s_wtime;              /* Write time */
    sigma_u16 s_mnt_count;          /* Mount count */
    sigma_u16 s_max_mnt_count;      /* Maximal mount count */
    sigma_u16 s_magic;              /* Magic signature */
    sigma_u16 s_state;              /* File system state */
    sigma_u16 s_errors;             /* Behaviour when detecting errors */
    sigma_u16 s_minor_rev_level;    /* minor revision level */
    sigma_u32 s_lastcheck;          /* time of last check */
    sigma_u32 s_checkinterval;      /* max. time between checks */
    sigma_u32 s_creator_os;         /* OS */
    sigma_u32 s_rev_level;          /* Revision level */
    sigma_u16 s_def_resuid;         /* Default uid for reserved blocks */
    sigma_u16 s_def_resgid;         /* Default gid for reserved blocks */
    
    /* Dynamic revision fields */
    sigma_u32 s_first_ino;          /* First non-reserved inode */
    sigma_u16 s_inode_size;         /* size of inode structure */
    sigma_u16 s_block_group_nr;     /* block group # of this superblock */
    sigma_u32 s_feature_compat;     /* compatible feature set */
    sigma_u32 s_feature_incompat;   /* incompatible feature set */
    sigma_u32 s_feature_ro_compat;  /* readonly-compatible feature set */
    sigma_u8  s_uuid[16];           /* 128-bit uuid for volume */
    char      s_volume_name[16];    /* volume name */
    char      s_last_mounted[64];   /* directory where last mounted */
    sigma_u32 s_algorithm_usage_bitmap; /* For compression */
    /* Many more fields exist, truncated for simplicity */
    sigma_u8  padding[1024 - 208]; 
} ext4_superblock_t;

/* Ext4 Block Group Descriptor */
typedef struct __attribute__((packed)) {
    sigma_u32 bg_block_bitmap_lo;       /* Blocks bitmap block */
    sigma_u32 bg_inode_bitmap_lo;       /* Inodes bitmap block */
    sigma_u32 bg_inode_table_lo;        /* Inodes table block */
    sigma_u16 bg_free_blocks_count_lo;  /* Free blocks count */
    sigma_u16 bg_free_inodes_count_lo;  /* Free inodes count */
    sigma_u16 bg_used_dirs_count_lo;    /* Directories count */
    sigma_u16 bg_flags;                 /* EXT4_BG_flags (INODE_UNINIT, etc) */
    sigma_u32 bg_exclude_bitmap_lo;     /* Exclude bitmap for snapshots */
    sigma_u16 bg_block_bitmap_csum_lo;  /* crc32c(s_uuid+grp_num+bmap) LE */
    sigma_u16 bg_inode_bitmap_csum_lo;  /* crc32c(s_uuid+grp_num+ibmap) LE */
    sigma_u16 bg_itable_unused_lo;      /* Unused inodes count */
    sigma_u16 bg_checksum;              /* crc16(sb_uuid+group+desc) */
} ext4_group_desc_t;

/* Ext4 Inode Structure */
typedef struct __attribute__((packed)) {
    sigma_u16 i_mode;       /* File mode */
    sigma_u16 i_uid;        /* Low 16 bits of Owner Uid */
    sigma_u32 i_size_lo;    /* Size in bytes */
    sigma_u32 i_atime;      /* Access time */
    sigma_u32 i_ctime;      /* Inode Change time */
    sigma_u32 i_mtime;      /* Modification time */
    sigma_u32 i_dtime;      /* Deletion Time */
    sigma_u16 i_gid;        /* Low 16 bits of Group Id */
    sigma_u16 i_links_count;/* Links count */
    sigma_u32 i_blocks_lo;  /* Blocks count */
    sigma_u32 i_flags;      /* File flags (e.g. EXTENTS) */
    
    union {
        struct {
            sigma_u32 l_i_version;
        } linux1;
        struct {
            sigma_u32 h_i_translator;
        } hurd1;
        struct {
            sigma_u32 m_i_reserved1;
        } masix1;
    } osd1;
    
    sigma_u32 i_block[EXT4_N_BLOCKS]; /* Pointers to blocks or extents */
    
    sigma_u32 i_generation; /* File version (for NFS) */
    sigma_u32 i_file_acl_lo;/* File ACL */
    sigma_u32 i_size_high;
    sigma_u32 i_obso_faddr; /* Obsoleted fragment address */
    
    union {
        struct {
            sigma_u16 l_i_blocks_high;
            sigma_u16 l_i_file_acl_high;
            sigma_u16 l_i_uid_high;
            sigma_u16 l_i_gid_high;
            sigma_u16 l_i_checksum_lo;
            sigma_u16 l_i_reserved;
        } linux2;
        /* hurd2/masix2 omitted */
    } osd2;
    
    sigma_u16 i_extra_isize;
    sigma_u16 i_checksum_hi;
    sigma_u32 i_ctime_extra;
    sigma_u32 i_mtime_extra;
    sigma_u32 i_atime_extra;
    sigma_u32 i_crtime;
    sigma_u32 i_crtime_extra;
    sigma_u32 i_version_hi;
} ext4_inode_t;

/* Ext4 Extent Structures */
#define EXT4_EXT_MAGIC 0xF30A

typedef struct __attribute__((packed)) {
    sigma_u32 ee_block;     /* first logical block extent covers */
    sigma_u16 ee_len;       /* number of blocks covered by extent */
    sigma_u16 ee_start_hi;  /* high 16 bits of physical block */
    sigma_u32 ee_start_lo;  /* low 32 bits of physical block */
} ext4_extent_t;

typedef struct __attribute__((packed)) {
    sigma_u32 ei_block;     /* index covers logical blocks from 'block' */
    sigma_u32 ei_leaf_lo;   /* pointer to the physical block of the next level */
    sigma_u16 ei_leaf_hi;   /* high 16 bits of physical block */
    sigma_u16 ei_unused;
} ext4_extent_idx_t;

typedef struct __attribute__((packed)) {
    sigma_u16 eh_magic;     /* probably will support different formats */
    sigma_u16 eh_entries;   /* number of valid entries */
    sigma_u16 eh_max;       /* capacity of store in entries */
    sigma_u16 eh_depth;     /* has tree real underlying blocks? */
    sigma_u32 eh_generation;/* generation of the tree */
} ext4_extent_header_t;

/* Ext4 Directory Entry */
typedef struct __attribute__((packed)) {
    sigma_u32 inode;        /* Inode number */
    sigma_u16 rec_len;      /* Directory entry length */
    sigma_u8  name_len;     /* Name length */
    sigma_u8  file_type;
    char      name[255];    /* File name */
} ext4_dir_entry_t;

/* Features */
#define EXT4_FEATURE_INCOMPAT_EXTENTS 0x0040

#endif /* SIGMA_EXT4_H */
