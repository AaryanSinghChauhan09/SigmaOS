#ifndef HAL_H
#define HAL_H

#include "../../../sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

void hal_init(void);
void hal_write_io(sigma_u16 port, sigma_u8 value);
sigma_u8 hal_read_io(sigma_u16 port);
void *hal_alloc_pages(sigma_u32 count);
void hal_free_pages(void *addr, sigma_u32 count);

#ifdef __cplusplus
}
#endif

#endif // HAL_H
