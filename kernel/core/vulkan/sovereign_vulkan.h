#ifndef SOVEREIGN_VULKAN_H
#define SOVEREIGN_VULKAN_H

#include "../../../sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

void vk_init(void);
void vk_submit_shader(const void *spirv_blob, sigma_u32 size);

#ifdef __cplusplus
}
#endif

#endif // SOVEREIGN_VULKAN_H
