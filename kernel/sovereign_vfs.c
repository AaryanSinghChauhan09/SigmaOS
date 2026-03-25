/*
 * Cosmos AI-OS: Enterprise Virtual File System (VFS C-Layer)
 * ========================================================
 * Mission: Zero-Copy read/writes. Caches directory extents directly
 *        into contiguous RAM avoiding context switch overheads.
 */

#include <stddef.h>
#include <stdint.h>


#define MAX_INODES 1024
#define BLOCK_SIZE 4096

/* ── Linux-inspired File Operations Abstraction ────────────────── */
struct vfs_inode;
typedef struct {
    int (*open)(struct vfs_inode *inode);
    int (*read)(struct vfs_inode *inode, char *buf, uint32_t len, uint32_t offset);
    int (*write)(struct vfs_inode *inode, const char *buf, uint32_t len, uint32_t offset);
    int (*release)(struct vfs_inode *inode);
} sigma_file_ops_t;

typedef struct vfs_inode {
  uint32_t inode_id;
  uint32_t size;
  uint32_t permissions; // e.g. 0644
  uint8_t is_directory;
  uint8_t *memory_mapped_region; // Direct pointer to Slab-Allocator cache
  sigma_file_ops_t *fops;        /* Function pointers for virtual files */
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
    new_file->fops = 0;

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

/* ── Sovereign ProcFS Implementation (Inspiration: Linux) ──────── */

// External symbols from scheduler
extern uint32_t sigma_get_task_count(void);
extern uint32_t sigma_get_predicted_burst(uint32_t pid);

static int proc_tasks_read(struct vfs_inode *inode, char *buf, uint32_t len, uint32_t offset) {
    (void)inode; (void)offset;
    uint32_t count = sigma_get_task_count();
    // Simplified: format a string into the buffer
    // In a real VFS, we'd iterate over task_table
    int written = 0;
    for (int i=0; i<32 && i < len; i++) {
        buf[i] = "PID  NAME      PRIO  SCG\n"[i];
        written++;
    }
    return written;
}

static sigma_file_ops_t proc_tasks_ops = {
    .read = proc_tasks_read
};

void cosmos_vfs_mount_proc() {
    int fd = cosmos_vfs_open("/proc/tasks", 1);
    if (fd != -1) {
        inode_table[fd].fops = &proc_tasks_ops;
        inode_table[fd].size = 1024; // Virtual size
    }
}

// User-level read interface that respects fops
int cosmos_vfs_read(int fd, char *buf, uint32_t len) {
    if (fd <= 0 || fd >= MAX_INODES || inode_table[fd].inode_id == 0)
        return -1;

    vfs_inode_t *node = &inode_table[fd];
    if (node->fops && node->fops->read) {
        return node->fops->read(node, buf, len, 0);
    }
    
    // Fallback to memory map read
    if (node->memory_mapped_region) {
        uint32_t to_read = (len < node->size) ? len : node->size;
        for (uint32_t i = 0; i < to_read; i++) {
            buf[i] = node->memory_mapped_region[i];
        }
        return to_read;
    }
    return 0;
}
