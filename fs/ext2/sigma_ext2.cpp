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

struct ext2_block_group_desc {
    u32 bg_block_bitmap;
    u32 bg_inode_bitmap;
    u32 bg_inode_table;
    u16 bg_free_blocks_count;
    u16 bg_free_inodes_count;
    u16 bg_used_dirs_count;
    u16 bg_pad;
    u32 bg_reserved[3];
};

struct ext2_inode {
    u16 i_mode;
    u16 i_uid;
    u32 i_size;
    u32 i_atime;
    u32 i_ctime;
    u32 i_mtime;
    u32 i_dtime;
    u16 i_gid;
    u16 i_links_count;
    u32 i_blocks;
    u32 i_flags;
    u32 i_osd1;
    u32 i_block[15];
    u32 i_generation;
    u32 i_file_acl;
    u32 i_dir_acl;
    u32 i_faddr;
    u32 i_osd2[3];
};

struct ext2_dir_entry {
    u32 inode;
    u16 rec_len;
    u8  name_len;
    u8  file_type;
    char name[255];
};

extern "C" bool sigma_ata_read_sector(u32 lba, u8* buffer);
extern "C" void sigma_vga_printf(const char* fmt, ...);

static struct ext2_super_block sb;
static u32 mounted_partition_lba = 0;

static inline u32 sigma_ext2_block_size() {
    return 1024 << sb.s_log_block_size;
}

extern "C" bool sigma_ext2_read_block(u32 block, u8* buf) {
    u32 bsize = sigma_ext2_block_size();
    u32 sectors_per_block = bsize / 512;
    u32 lba = mounted_partition_lba + (block * sectors_per_block);
    for (u32 i = 0; i < sectors_per_block; i++) {
        if (!sigma_ata_read_sector(lba + i, buf + (i * 512))) {
            return false;
        }
    }
    return true;
}

extern "C" bool sigma_ext2_mount(u32 partition_lba) {
    u8 sector[512];
    mounted_partition_lba = partition_lba;
    
    // ext2 superblock is at byte 1024, i.e., partition LBA + 2 (since 1 LBA = 512 bytes)
    if (!sigma_ata_read_sector(partition_lba + 2, sector)) {
        return false;
    }
    
    // Copy superblock from sector
    u8* p = (u8*)&sb;
    for(u32 i = 0; i < sizeof(sb); i++) {
        p[i] = sector[i];
    }
    
    if (sb.s_magic != EXT2_SUPER_MAGIC) {
        sigma_vga_printf("ext2 mount failed: bad magic 0x%X\n", sb.s_magic);
        return false;
    }
    
    sigma_vga_printf("ext2 mounted! Inodes: %u, Blocks: %u, BlockSize: %u\n", 
        sb.s_inodes_count, sb.s_blocks_count, sigma_ext2_block_size());
    return true;
}

extern "C" bool sigma_ext2_read_inode(u32 inode_num, struct ext2_inode* out_inode) {
    if (inode_num == 0) return false;
    
    // 1. Determine Block Group
    u32 inodes_per_group = sb.s_inodes_per_group;
    u32 bg = (inode_num - 1) / inodes_per_group;
    u32 index = (inode_num - 1) % inodes_per_group;
    
    // 2. Read Block Group Descriptor Table to find the inode table location
    u32 bsize = sigma_ext2_block_size();
    u32 bgdt_block = sb.s_first_data_block + 1;
    
    // Allocate space for one block
    u8 block_buf[4096];
    if (bsize > 4096) return false; // Fail safe
    
    if (!sigma_ext2_read_block(bgdt_block, block_buf)) {
        return false;
    }
    
    struct ext2_block_group_desc* bgd = &((struct ext2_block_group_desc*)block_buf)[bg];
    u32 inode_table_block = bgd->bg_inode_table;
    
    // 3. Locate the block and offset containing the requested inode
    u32 inode_size = (sb.s_rev_level >= 1) ? 128 : 128; // Standard ext2 inode is 128 bytes
    u32 inode_offset_bytes = index * inode_size;
    u32 inode_block = inode_table_block + (inode_offset_bytes / bsize);
    u32 offset_within_block = inode_offset_bytes % bsize;
    
    if (!sigma_ext2_read_block(inode_block, block_buf)) {
        return false;
    }
    
    u8* src = block_buf + offset_within_block;
    u8* dest = (u8*)out_inode;
    for (u32 i = 0; i < sizeof(struct ext2_inode); i++) {
        dest[i] = src[i];
    }
    
    return true;
}

extern "C" int sigma_ext2_read_file(struct ext2_inode* inode, u8* out_buf, u32 max_len) {
    u32 bsize = sigma_ext2_block_size();
    u32 total_bytes = inode->i_size;
    if (total_bytes > max_len) total_bytes = max_len;
    
    u32 blocks_to_read = (total_bytes + bsize - 1) / bsize;
    u32 bytes_read = 0;
    
    u8 block_buf[4096];
    if (bsize > 4096) return -1;
    
    for (u32 i = 0; i < blocks_to_read; i++) {
        if (i < 12) {
            // Direct blocks
            u32 block_num = inode->i_block[i];
            if (block_num == 0) {
                // Sparse block, pad with zeroes
                for (u32 j = 0; j < bsize && bytes_read < total_bytes; j++) {
                    out_buf[bytes_read++] = 0;
                }
            } else {
                if (!sigma_ext2_read_block(block_num, block_buf)) {
                    return -1;
                }
                u32 to_copy = total_bytes - bytes_read;
                if (to_copy > bsize) to_copy = bsize;
                for (u32 j = 0; j < to_copy; j++) {
                    out_buf[bytes_read++] = block_buf[j];
                }
            }
        } else {
            // Indirect block logic can be extended here
            break; 
        }
    }
    return (int)bytes_read;
}

static bool sigma_str_match(const char* a, const char* b, u32 len) {
    for (u32 i = 0; i < len; i++) {
        if (a[i] != b[i]) return false;
    }
    return b[len] == '\0';
}

extern "C" u32 sigma_ext2_lookup(const char* path) {
    struct ext2_inode current_inode;
    // Read root inode (number 2)
    if (!sigma_ext2_read_inode(2, &current_inode)) {
        return 0;
    }
    
    u32 bsize = sigma_ext2_block_size();
    u8 block_buf[4096];
    if (bsize > 4096) return 0;
    
    // Simplified single directory lookup for paths like "/filename"
    if (path[0] == '/') path++;
    if (path[0] == '\0') return 2; // return root inode number
    
    // Read root directory entries
    if ((current_inode.i_mode & 0x4000) == 0) {
        // Not a directory
        return 0;
    }
    
    u32 block_num = current_inode.i_block[0]; // first block
    if (!sigma_ext2_read_block(block_num, block_buf)) {
        return 0;
    }
    
    u32 offset = 0;
    while (offset < current_inode.i_size && offset < bsize) {
        struct ext2_dir_entry* entry = (struct ext2_dir_entry*)(block_buf + offset);
        if (entry->rec_len == 0) break;
        
        if (sigma_str_match(entry->name, path, entry->name_len)) {
            return entry->inode;
        }
        offset += entry->rec_len;
    }
    return 0;
}
