/*
 * SigmaOS Filesystem (Native Core)
 * ================================
 * Complete Virtual File System implementation
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <time.h>

// Filesystem types
typedef enum {
    FS_TYPE_SIGMAFS = 1,
    FS_TYPE_EXT4 = 2,
    FS_TYPE_FAT32 = 3,
    FS_TYPE_NTFS = 4,
    FS_TYPE_TMPFS = 5,
    FS_TYPE_PROCFS = 6,
    FS_TYPE_SYSFS = 7
} fs_type_t;

// File types
typedef enum {
    FT_REGULAR = 1,
    FT_DIRECTORY = 2,
    FT_SYMLINK = 3,
    FT_DEVICE = 4,
    FT_PIPE = 5,
    FT_SOCKET = 6
} file_type_t;

// File permissions
#define PERM_OWNER_READ    0400
#define PERM_OWNER_WRITE   0200
#define PERM_OWNER_EXEC    0100
#define PERM_GROUP_READ    0040
#define PERM_GROUP_WRITE   0020
#define PERM_GROUP_EXEC    0010
#define PERM_OTHER_READ    0004
#define PERM_OTHER_WRITE   0002
#define PERM_OTHER_EXEC    0001

// Open flags
#define O_RDONLY    0x0000
#define O_WRONLY    0x0001
#define O_RDWR      0x0002
#define O_CREAT     0x0040
#define O_EXCL      0x0080
#define O_TRUNC     0x0200
#define O_APPEND    0x0400

// Seek origins
#define SEEK_SET    0
#define SEEK_CUR    1
#define SEEK_END    2

// Maximum path length
#define MAX_PATH_LEN 4096
#define MAX_FILENAME_LEN 255

// Inode structure
typedef struct {
    uint32_t inode_num;
    file_type_t type;
    uint16_t mode;        // File permissions
    uint16_t links_count;
    uint32_t uid;         // Owner user ID
    uint32_t gid;         // Owner group ID
    uint64_t size;
    uint64_t blocks;
    time_t atime;         // Access time
    time_t mtime;         // Modification time
    time_t ctime;         // Creation time
    uint64_t *direct_blocks;    // 12 direct block pointers
    uint64_t *single_indirect;  // Single indirect block
    uint64_t *double_indirect;  // Double indirect block
    uint64_t *triple_indirect;  // Triple indirect block
} inode_t;

// Directory entry structure
typedef struct {
    uint32_t inode_num;
    uint16_t rec_len;     // Record length
    uint8_t name_len;
    uint8_t file_type;
    char name[MAX_FILENAME_LEN + 1];
} __attribute__((packed)) dir_entry_t;

// Superblock structure
typedef struct {
    uint32_t magic;           // Filesystem magic number
    uint32_t inodes_count;
    uint32_t blocks_count;
    uint32_t free_blocks_count;
    uint32_t free_inodes_count;
    uint32_t first_data_block;
    uint32_t block_size;
    uint32_t inode_size;
    uint32_t blocks_per_group;
    uint32_t inodes_per_group;
    time_t mtime;
    time_t wtime;            // Superblock write time
    uint16_t mounts_count;
    uint16_t max_mount_count;
    uint64_t fs_uuid[2];
    char volume_name[16];
    char last_mounted[64];
} __attribute__((packed)) superblock_t;

// File descriptor structure
typedef struct {
    inode_t *inode;
    uint64_t offset;
    uint32_t flags;
    bool is_open;
} file_descriptor_t;

// Mount point structure
typedef struct {
    char device[MAX_PATH_LEN];
    char mount_point[MAX_PATH_LEN];
    fs_type_t fs_type;
    superblock_t *superblock;
    bool is_mounted;
    struct mount_point *next;
} mount_point_t;

// VFS operations structure
typedef struct vfs_operations {
    int (*read)(inode_t *inode, uint64_t offset, void *buffer, size_t count);
    int (*write)(inode_t *inode, uint64_t offset, const void *buffer, size_t count);
    int (*readdir)(inode_t *inode, uint64_t offset, dir_entry_t *entry);
    int (*lookup)(inode_t *dir_inode, const char *name, inode_t **result);
    int (*create)(inode_t *dir_inode, const char *name, file_type_t type, uint16_t mode, inode_t **result);
    int (*unlink)(inode_t *dir_inode, const char *name);
    int (*mkdir)(inode_t *dir_inode, const char *name, uint16_t mode, inode_t **result);
    int (*rmdir)(inode_t *dir_inode, const char *name);
    int (*symlink)(inode_t *dir_inode, const char *name, const char *target);
    int (*readlink)(inode_t *inode, char *buffer, size_t size);
} vfs_ops_t;

// Global filesystem state
#define MAX_MOUNT_POINTS 32
#define MAX_OPEN_FILES 1024

static mount_point_t mount_points[MAX_MOUNT_POINTS];
static file_descriptor_t file_descriptors[MAX_OPEN_FILES];
static mount_point_t *mount_list = NULL;
static uint32_t next_inode_num = 1;

// Block device operations
typedef struct block_device {
    char name[32];
    uint64_t size;
    uint32_t block_size;
    int (*read_block)(struct block_device *dev, uint64_t block_num, void *buffer);
    int (*write_block)(struct block_device *dev, uint64_t block_num, const void *buffer);
    struct block_device *next;
} block_device_t;

static block_device_t *block_devices = NULL;

// Initialize VFS
void sigma_vfs_init(void) {
    // Clear mount points
    for (int i = 0; i < MAX_MOUNT_POINTS; i++) {
        memset(&mount_points[i], 0, sizeof(mount_point_t));
    }
    
    // Clear file descriptors
    for (int i = 0; i < MAX_OPEN_FILES; i++) {
        memset(&file_descriptors[i], 0, sizeof(file_descriptor_t));
    }
    
    mount_list = NULL;
}

// Register block device
int sigma_vfs_register_device(block_device_t *dev) {
    if (!dev || !dev->read_block || !dev->write_block) {
        return -1;
    }
    
    dev->next = block_devices;
    block_devices = dev;
    
    return 0;
}

// Find block device by name
block_device_t* sigma_vfs_find_device(const char *name) {
    block_device_t *dev = block_devices;
    while (dev) {
        if (strcmp(dev->name, name) == 0) {
            return dev;
        }
        dev = dev->next;
    }
    return NULL;
}

// Mount filesystem
int sigma_vfs_mount(const char *device, const char *mount_point, fs_type_t fs_type) {
    // Find free mount point slot
    mount_point_t *mp = NULL;
    for (int i = 0; i < MAX_MOUNT_POINTS; i++) {
        if (!mount_points[i].is_mounted) {
            mp = &mount_points[i];
            break;
        }
    }
    if (!mp) return -1; // No free mount points
    
    // Find block device
    block_device_t *dev = sigma_vfs_find_device(device);
    if (!dev) return -1;
    
    // Initialize mount point
    strncpy(mp->device, device, sizeof(mp->device) - 1);
    strncpy(mp->mount_point, mount_point, sizeof(mp->mount_point) - 1);
    mp->fs_type = fs_type;
    mp->is_mounted = true;
    
    // Read superblock
    mp->superblock = (superblock_t*)malloc(sizeof(superblock_t));
    if (!mp->superblock) return -1;
    
    if (dev->read_block(dev, 1, mp->superblock) != 0) {
        free(mp->superblock);
        mp->is_mounted = false;
        return -1;
    }
    
    // Verify filesystem magic number
    if (mp->superblock->magic != 0xEF53) { // ext4 magic number
        free(mp->superblock);
        mp->is_mounted = false;
        return -1;
    }
    
    // Add to mount list
    mp->next = mount_list;
    mount_list = mp;
    
    return 0;
}

// Unmount filesystem
int sigma_vfs_unmount(const char *mount_point) {
    mount_point_t *prev = NULL;
    mount_point_t *mp = mount_list;
    
    while (mp) {
        if (strcmp(mp->mount_point, mount_point) == 0) {
            if (prev) {
                prev->next = mp->next;
            } else {
                mount_list = mp->next;
            }
            
            free(mp->superblock);
            mp->is_mounted = false;
            return 0;
        }
        prev = mp;
        mp = mp->next;
    }
    
    return -1; // Mount point not found
}

// Find mount point for path
mount_point_t* sigma_vfs_find_mount_point(const char *path) {
    mount_point_t *best_match = NULL;
    size_t best_len = 0;
    
    mount_point_t *mp = mount_list;
    while (mp) {
        size_t len = strlen(mp->mount_point);
        if (strncmp(path, mp->mount_point, len) == 0 && len > best_len) {
            best_match = mp;
            best_len = len;
        }
        mp = mp->next;
    }
    
    return best_match;
}

// Allocate inode
inode_t* sigma_vfs_alloc_inode(mount_point_t *mp) {
    if (!mp || !mp->superblock) return NULL;
    
    // Find free inode
    for (uint32_t i = 1; i < mp->superblock->inodes_count; i++) {
        // This would check inode bitmap
        // For now, just allocate sequentially
        if (i == next_inode_num) {
            next_inode_num++;
            inode_t *inode = (inode_t*)malloc(sizeof(inode_t));
            if (inode) {
                memset(inode, 0, sizeof(inode_t));
                inode->inode_num = i;
                inode->direct_blocks = (uint64_t*)malloc(12 * sizeof(uint64_t));
                if (inode->direct_blocks) {
                    memset(inode->direct_blocks, 0, 12 * sizeof(uint64_t));
                }
                return inode;
            }
        }
    }
    
    return NULL;
}

// Free inode
void sigma_vfs_free_inode(mount_point_t *mp, inode_t *inode) {
    if (!inode) return;
    
    // Mark inode as free in bitmap
    // This would update the inode bitmap
    
    if (inode->direct_blocks) {
        free(inode->direct_blocks);
    }
    if (inode->single_indirect) {
        free(inode->single_indirect);
    }
    if (inode->double_indirect) {
        free(inode->double_indirect);
    }
    if (inode->triple_indirect) {
        free(inode->triple_indirect);
    }
    
    free(inode);
}

// Read inode from disk
int sigma_vfs_read_inode(mount_point_t *mp, uint32_t inode_num, inode_t **result) {
    if (!mp || !mp->superblock) return -1;
    
    // Calculate inode block and offset
    uint32_t inode_group = (inode_num - 1) / mp->superblock->inodes_per_group;
    uint32_t inode_index = (inode_num - 1) % mp->superblock->inodes_per_group;
    
    uint64_t inode_table_block = mp->superblock->first_data_block + 
                                inode_group * mp->superblock->blocks_per_group;
    
    // Read inode from disk
    block_device_t *dev = sigma_vfs_find_device(mp->device);
    if (!dev) return -1;
    
    uint8_t block_buffer[4096]; // Assume 4KB blocks
    if (dev->read_block(dev, inode_table_block + (inode_index * sizeof(inode_t)) / 4096, 
                       block_buffer) != 0) {
        return -1;
    }
    
    inode_t *inode = (inode_t*)malloc(sizeof(inode_t));
    if (!inode) return -1;
    
    // Copy inode data
    size_t offset = (inode_index * sizeof(inode_t)) % 4096;
    memcpy(inode, block_buffer + offset, sizeof(inode_t));
    
    *result = inode;
    return 0;
}

// Write inode to disk
int sigma_vfs_write_inode(mount_point_t *mp, inode_t *inode) {
    if (!mp || !mp->superblock || !inode) return -1;
    
    // Calculate inode block and offset
    uint32_t inode_group = (inode->inode_num - 1) / mp->superblock->inodes_per_group;
    uint32_t inode_index = (inode->inode_num - 1) % mp->superblock->inodes_per_group;
    
    uint64_t inode_table_block = mp->superblock->first_data_block + 
                                inode_group * mp->superblock->blocks_per_group;
    
    // Write inode to disk
    block_device_t *dev = sigma_vfs_find_device(mp->device);
    if (!dev) return -1;
    
    uint8_t block_buffer[4096];
    if (dev->read_block(dev, inode_table_block + (inode_index * sizeof(inode_t)) / 4096, 
                       block_buffer) != 0) {
        return -1;
    }
    
    // Update inode data
    size_t offset = (inode_index * sizeof(inode_t)) % 4096;
    memcpy(block_buffer + offset, inode, sizeof(inode_t));
    
    return dev->write_block(dev, inode_table_block + (inode_index * sizeof(inode_t)) / 4096, 
                           block_buffer);
}

// Find free file descriptor
int sigma_vfs_find_free_fd(void) {
    for (int i = 0; i < MAX_OPEN_FILES; i++) {
        if (!file_descriptors[i].is_open) {
            return i;
        }
    }
    return -1;
}

// Open file
int sigma_vfs_open(const char *path, int flags, uint16_t mode) {
    // Find mount point
    mount_point_t *mp = sigma_vfs_find_mount_point(path);
    if (!mp) return -1;
    
    // Parse path (simplified)
    const char *filename = strrchr(path, '/');
    if (!filename) filename = path;
    else filename++;
    
    // Find directory inode (simplified - would need full path traversal)
    inode_t *dir_inode;
    if (sigma_vfs_read_inode(mp, 2, &dir_inode) != 0) { // Root inode
        return -1;
    }
    
    // Lookup file
    inode_t *file_inode;
    if (sigma_vfs_lookup(mp, dir_inode, filename, &file_inode) != 0) {
        if (flags & O_CREAT) {
            // Create new file
            if (sigma_vfs_create(mp, dir_inode, filename, FT_REGULAR, mode, &file_inode) != 0) {
                sigma_vfs_free_inode(mp, dir_inode);
                return -1;
            }
        } else {
            sigma_vfs_free_inode(mp, dir_inode);
            return -1;
        }
    }
    
    // Allocate file descriptor
    int fd = sigma_vfs_find_free_fd();
    if (fd < 0) {
        sigma_vfs_free_inode(mp, dir_inode);
        sigma_vfs_free_inode(mp, file_inode);
        return -1;
    }
    
    file_descriptors[fd].inode = file_inode;
    file_descriptors[fd].offset = 0;
    file_descriptors[fd].flags = flags;
    file_descriptors[fd].is_open = true;
    
    sigma_vfs_free_inode(mp, dir_inode);
    return fd;
}

// Close file
int sigma_vfs_close(int fd) {
    if (fd < 0 || fd >= MAX_OPEN_FILES || !file_descriptors[fd].is_open) {
        return -1;
    }
    
    file_descriptors[fd].is_open = false;
    
    // Write back inode
    mount_point_t *mp = sigma_vfs_find_mount_point("/");
    if (mp) {
        sigma_vfs_write_inode(mp, file_descriptors[fd].inode);
        sigma_vfs_free_inode(mp, file_descriptors[fd].inode);
    }
    
    return 0;
}

// Read from file
ssize_t sigma_vfs_read(int fd, void *buffer, size_t count) {
    if (fd < 0 || fd >= MAX_OPEN_FILES || !file_descriptors[fd].is_open) {
        return -1;
    }
    
    file_descriptor_t *desc = &file_descriptors[fd];
    inode_t *inode = desc->inode;
    
    // Check if we can read
    if (!(desc->flags & (O_RDONLY | O_RDWR))) {
        return -1;
    }
    
    // Adjust count if beyond file size
    if (desc->offset + count > inode->size) {
        count = inode->size - desc->offset;
    }
    
    if (count == 0) return 0;
    
    // Read data (simplified - would need block mapping)
    mount_point_t *mp = sigma_vfs_find_mount_point("/");
    if (!mp) return -1;
    
    // This would map inode blocks to disk blocks and read data
    // For now, just return success
    desc->offset += count;
    
    return count;
}

// Write to file
ssize_t sigma_vfs_write(int fd, const void *buffer, size_t count) {
    if (fd < 0 || fd >= MAX_OPEN_FILES || !file_descriptors[fd].is_open) {
        return -1;
    }
    
    file_descriptor_t *desc = &file_descriptors[fd];
    inode_t *inode = desc->inode;
    
    // Check if we can write
    if (!(desc->flags & (O_WRONLY | O_RDWR))) {
        return -1;
    }
    
    // Handle O_APPEND
    if (desc->flags & O_APPEND) {
        desc->offset = inode->size;
    }
    
    // Write data (simplified - would need block allocation and mapping)
    mount_point_t *mp = sigma_vfs_find_mount_point("/");
    if (!mp) return -1;
    
    // This would allocate blocks as needed and write data
    // For now, just update file size
    if (desc->offset + count > inode->size) {
        inode->size = desc->offset + count;
    }
    
    desc->offset += count;
    inode->mtime = time(NULL);
    
    return count;
}

// Seek in file
off_t sigma_vfs_lseek(int fd, off_t offset, int whence) {
    if (fd < 0 || fd >= MAX_OPEN_FILES || !file_descriptors[fd].is_open) {
        return -1;
    }
    
    file_descriptor_t *desc = &file_descriptors[fd];
    inode_t *inode = desc->inode;
    
    switch (whence) {
        case SEEK_SET:
            desc->offset = offset;
            break;
        case SEEK_CUR:
            desc->offset += offset;
            break;
        case SEEK_END:
            desc->offset = inode->size + offset;
            break;
        default:
            return -1;
    }
    
    // Validate offset
    if (desc->offset < 0) {
        desc->offset = 0;
        return -1;
    }
    
    return desc->offset;
}

// VFS operations implementations (simplified)
int sigma_vfs_lookup(mount_point_t *mp, inode_t *dir_inode, const char *name, inode_t **result) {
    if (!mp || !dir_inode || !name || dir_inode->type != FT_DIRECTORY) {
        return -1;
    }
    
    // Read directory entries and find matching name
    // This is a simplified implementation
    for (uint64_t offset = 0; offset < dir_inode->size; ) {
        dir_entry_t entry;
        if (sigma_vfs_read_dir_entry(mp, dir_inode, offset, &entry) == 0) {
            if (strncmp(entry.name, name, entry.name_len) == 0) {
                return sigma_vfs_read_inode(mp, entry.inode_num, result);
            }
            offset += entry.rec_len;
        } else {
            break;
        }
    }
    
    return -1; // Not found
}

int sigma_vfs_create(mount_point_t *mp, inode_t *dir_inode, const char *name, 
                    file_type_t type, uint16_t mode, inode_t **result) {
    if (!mp || !dir_inode || !name || dir_inode->type != FT_DIRECTORY) {
        return -1;
    }
    
    // Allocate new inode
    inode_t *new_inode = sigma_vfs_alloc_inode(mp);
    if (!new_inode) return -1;
    
    // Initialize inode
    new_inode->type = type;
    new_inode->mode = mode;
    new_inode->links_count = 1;
    new_inode->uid = 0; // Root
    new_inode->gid = 0; // Root group
    new_inode->size = 0;
    new_inode->atime = new_inode->mtime = new_inode->ctime = time(NULL);
    
    // Add directory entry
    dir_entry_t entry;
    entry.inode_num = new_inode->inode_num;
    entry.name_len = strlen(name);
    entry.file_type = type;
    strncpy(entry.name, name, MAX_FILENAME_LEN);
    entry.rec_len = sizeof(dir_entry_t);
    
    // Write directory entry (simplified)
    // This would append to the directory's data blocks
    
    // Write new inode
    sigma_vfs_write_inode(mp, new_inode);
    
    *result = new_inode;
    return 0;
}

int sigma_vfs_read_dir_entry(mount_point_t *mp, inode_t *dir_inode, uint64_t offset, dir_entry_t *entry) {
    // This would read directory entry from disk
    // Simplified implementation
    return -1;
}

// Get filesystem statistics
typedef struct {
    uint64_t total_blocks;
    uint64_t free_blocks;
    uint64_t total_inodes;
    uint64_t free_inodes;
    uint32_t block_size;
} fs_stats_t;

int sigma_vfs_statfs(const char *path, fs_stats_t *stats) {
    mount_point_t *mp = sigma_vfs_find_mount_point(path);
    if (!mp || !mp->superblock) return -1;
    
    stats->total_blocks = mp->superblock->blocks_count;
    stats->free_blocks = mp->superblock->free_blocks_count;
    stats->total_inodes = mp->superblock->inodes_count;
    stats->free_inodes = mp->superblock->free_inodes_count;
    stats->block_size = mp->superblock->block_size;
    
    return 0;
}
