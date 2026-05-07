#ifndef SIGMA_NVME_H
#define SIGMA_NVME_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint64_t base_addr;
    uint32_t irq;
    bool initialized;
} sigma_nvme_device_t;

/* --- NVMe Primitives --- */
void nvme_init(void);
int nvme_read_blocks(uint64_t lba, uint32_t count, void* buffer);
int nvme_write_blocks(uint64_t lba, uint32_t count, const void* buffer);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NVME_H */
