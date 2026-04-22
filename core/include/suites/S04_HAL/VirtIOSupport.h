#ifndef SIGMA_VIRTIO_SUPPORT_H
#define SIGMA_VIRTIO_SUPPORT_H

#include <stdint.h>

// VirtIO Device configuration structure
typedef struct {
    uint16_t device_id;
    uint16_t vendor_id;
    uint32_t status;
} VirtIODevice;

// Initialize the VirtIO subsystem
int init_virtio_subsystem(void);

// Register a VirtIO driver (Block, Net, Console)
int virtio_register_driver(uint16_t device_id, void* driver_callbacks);

#endif // SIGMA_VIRTIO_SUPPORT_H
