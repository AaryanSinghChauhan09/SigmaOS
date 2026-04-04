/* 
 Σ SIGMAOS ZENITH: SOVEREIGN BLOCK DEVICE DRIVER (v2100.0)
 Mission: Direct Silicon Disk Interaction & DMA Orchestration.
*/

#ifndef SIGMA_DISK_H
#define SIGMA_DISK_H

#include "../sigma_kernel_types.h"

#define DISK_BLOCK_SIZE 512

// Σ DISK DEVICE STRUCTURE
typedef struct {
    uint32_t device_id;
    uint64_t total_blocks;
    bool is_ready;
} sigma_disk_device;

// Σ DISK I/O API (BARE-METAL)
bool sigma_disk_read(uint32_t lba, uint8_t* buffer, uint32_t count);
bool sigma_disk_write(uint32_t lba, const uint8_t* buffer, uint32_t count);

#endif
