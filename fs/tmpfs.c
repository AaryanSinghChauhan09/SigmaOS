#include "sigma_kernel_types.h"
#include "sigma_slab.h"

// Basic stub implementation of tmpfs
// In a real VFS, this would register fs operations.

typedef struct tmpfs_file {
    char name[32];
    uint8_t* data;
    size_t size;
    size_t capacity;
    struct tmpfs_file* next;
} tmpfs_file_t;

static tmpfs_file_t* tmpfs_root = NULL;

void tmpfs_init(void) {
    tmpfs_root = NULL;
}

int tmpfs_create(const char* name) {
    // Allocation from slab/kmalloc
    tmpfs_file_t* file = (tmpfs_file_t*)kmalloc(sizeof(tmpfs_file_t));
    if (!file) return -1;
    
    // Copy name
    int i = 0;
    while (name[i] && i < 31) {
        file->name[i] = name[i];
        i++;
    }
    file->name[i] = '\0';
    
    file->data = NULL;
    file->size = 0;
    file->capacity = 0;
    
    file->next = tmpfs_root;
    tmpfs_root = file;
    
    return 0;
}

// In a real OS, this integrates with VFS open, read, write
