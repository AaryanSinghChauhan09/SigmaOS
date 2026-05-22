#ifndef SIGMA_SLAB_H
#define SIGMA_SLAB_H

#include <stddef.h>

// Initialize the slab allocator system
void sigma_slab_init(void);

// Allocate memory
void* kmalloc(size_t size);

// Free memory
void kfree(void* ptr);

#endif // SIGMA_SLAB_H
