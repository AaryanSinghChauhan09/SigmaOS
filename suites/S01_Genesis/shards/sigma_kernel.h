#ifndef SIGMA_KERNEL_H
#define SIGMA_KERNEL_H
#include "sigma_kernel_types.h"
#include "sigma_log.h"
#ifdef __cplusplus
extern "C" {
#endif
void sigma_kernel_init(void);
void sigma_kernel_panic(const char* msg);
#ifdef __cplusplus
}
#endif
#endif