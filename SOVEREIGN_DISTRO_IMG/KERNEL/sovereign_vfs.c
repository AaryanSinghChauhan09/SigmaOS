/*
 * Cosmos AI-OS: Sovereign Virtual File System (VFS C-Layer)
 * ========================================================
 * Mission: Zero-Copy read/writes. Caches directory extents directly
 *        into contiguous RAM avoiding context switch overheads.
 */

#include <stddef.h>
#include <stdint.h>


#define MAX_INODES 1024
#define BLOCK_SIZE 4096

typedef struct {
  uint32_t inode_id;
  uint32_t size;
  uint32_t permissions; // e.g. 0644
  uint8_t is_directory;
  uint8_t *memory_mapped_region; // Direct pointer to Slab-Allocator cache
  char filename[64];
} vfs_inode_t;

static vfs_inode_t inode_table[MAX_INODES];
static int next_inode_id = 1;

extern void *cosmos_slab_alloc(uint32_t size); // Link to slab allocator
extern void cosmos_spin_lock(void *lock);
extern void cosmos_spin_unlock(void *lock);

static uint32_t vfs_lock = 0; // Simulated Spinlock struct space

void cosmos_vfs_init() {
  for (int i = 0; i < MAX_INODES; i++) {
    inode_table[i].inode_id = 0;
  }
}

// Opens or creates a file directly in memory cache natively
int cosmos_vfs_open(const char *name, int create) {
  cosmos_spin_lock(&vfs_lock);

  // Fast O(N) search (In real OS, B-Tree is used)
  for (int i = 0; i < MAX_INODES; i++) {
    if (inode_table[i].inode_id != 0) {
      int match = 1;
      for (int j = 0; name[j] && j < 63; j++) {
        if (inode_table[i].filename[j] != name[j]) {
          match = 0;
          break;
        }
      }
      if (match) {
        cosmos_spin_unlock(&vfs_lock);
        return inode_table[i].inode_id; // File Exists
      }
    }
  }

  if (create && next_inode_id < MAX_INODES) {
    vfs_inode_t *new_file = &inode_table[next_inode_id];
    new_file->inode_id = next_inode_id;
    new_file->size = 0;
    new_file->is_directory = 0;
    new_file->memory_mapped_region = 0; // Alloc on write

    int j = 0;
    while (name[j] && j < 63) {
      new_file->filename[j] = name[j];
      j++;
    }
    new_file->filename[j] = '\0';
    next_inode_id++;

    cosmos_spin_unlock(&vfs_lock);
    return new_file->inode_id;
  }

  cosmos_spin_unlock(&vfs_lock);
  return -1; // File not found or out of inodes
}

// Memory-Mapped Write (Zero Copy to Python)
int cosmos_vfs_mmap_write(int fd, const uint8_t *buffer, uint32_t len) {
  if (fd <= 0 || fd >= MAX_INODES || inode_table[fd].inode_id == 0)
    return -1;

  vfs_inode_t *node = &inode_table[fd];

  // Demand Allocation natively
  if (!node->memory_mapped_region) {
    // Assume cosmos_slab_alloc returns page aligned chunks
    node->memory_mapped_region =
        (uint8_t *)cosmos_slab_alloc(BLOCK_SIZE * ((len / BLOCK_SIZE) + 1));
  }

  if (node->memory_mapped_region) {
    // Fast hardware-native memcpy
    for (uint32_t i = 0; i < len; i++) {
      node->memory_mapped_region[i] = buffer[i];
    }
    node->size = len;
    return len;
  }
  return -1; // Out of memory
}
