#include "sigma_kernel_types.h"
#include "sigma_slab.h"

typedef struct devfs_node {
    char name[32];
    int (*read_cb)(void*, size_t);
    int (*write_cb)(const void*, size_t);
    struct devfs_node* next;
} devfs_node_t;

static devfs_node_t* devfs_root = NULL;

void devfs_init(void) {
    devfs_root = NULL;
}

int devfs_register_device(const char* name, int (*read_cb)(void*, size_t), int (*write_cb)(const void*, size_t)) {
    devfs_node_t* node = (devfs_node_t*)kmalloc(sizeof(devfs_node_t));
    if (!node) return -1;
    
    int i = 0;
    while (name[i] && i < 31) {
        node->name[i] = name[i];
        i++;
    }
    node->name[i] = '\0';
    
    node->read_cb = read_cb;
    node->write_cb = write_cb;
    
    node->next = devfs_root;
    devfs_root = node;
    
    return 0;
}

// Dummy /dev/null
int dev_null_read(void* buf, size_t count) {
    (void)buf; (void)count;
    return 0; // EOF immediately
}
int dev_null_write(const void* buf, size_t count) {
    (void)buf; 
    return count; // Success without doing anything
}

void devfs_populate_default(void) {
    devfs_register_device("null", dev_null_read, dev_null_write);
}
