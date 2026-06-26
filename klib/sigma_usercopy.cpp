// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_usercopy.cpp — safe kernel↔userspace memory copy (Redox UserSlice-inspired)
 *
 * On the bare-metal kernel, USERSPACE_BASE and USERSPACE_LIMIT define the
 * valid user virtual address range. Any attempt to copy from/to kernel
 * addresses via these functions returns -EFAULT.
 */
#include "include/sigma_usercopy.h"
#include "sigma_log.h"
#include <string.h>

/* User address space bounds (x86_64 canonical user range) */
#define SIGMA_USER_BASE  0x0000000000001000ULL   /* first valid user page    */
#define SIGMA_USER_LIMIT 0x00007FFFFFFFFFFFULL   /* last valid user address  */

static bool is_user_range(const void* ptr, sigma_size_t len) {
    uintptr_t start = (uintptr_t)ptr;
    uintptr_t end   = start + len;
    if (end < start) return false;  /* overflow */
    return (start >= SIGMA_USER_BASE && end <= SIGMA_USER_LIMIT);
}

int sigma_user_ro_create(sigma_user_ro_t* out, const void* ptr, sigma_size_t len) {
    if (!is_user_range(ptr, len)) {
        sigma_log_err("[sigma-usercopy] ro create: bad user range ptr=%p len=%zu\n",
                      ptr, (size_t)len);
        out->validated = false;
        return -1; /* -EFAULT */
    }
    out->ptr       = ptr;
    out->len       = len;
    out->validated = true;
    return 0;
}

int sigma_user_wo_create(sigma_user_wo_t* out, void* ptr, sigma_size_t len) {
    if (!is_user_range(ptr, len)) {
        sigma_log_err("[sigma-usercopy] wo create: bad user range ptr=%p len=%zu\n",
                      ptr, (size_t)len);
        out->validated = false;
        return -1;
    }
    out->ptr       = ptr;
    out->len       = len;
    out->validated = true;
    return 0;
}

sigma_ssize_t sigma_copy_from_user(void* dst_kernel,
                                    const sigma_user_ro_t* src,
                                    sigma_size_t len) {
    if (!src->validated) return -1; /* -EFAULT */
    sigma_size_t copy_len = (len < src->len) ? len : src->len;
    memcpy(dst_kernel, src->ptr, copy_len);
    return (sigma_ssize_t)copy_len;
}

sigma_ssize_t sigma_copy_to_user(sigma_user_wo_t* dst,
                                  const void* src_kernel,
                                  sigma_size_t len) {
    if (!dst->validated) return -1;
    sigma_size_t copy_len = (len < dst->len) ? len : dst->len;
    memcpy(dst->ptr, src_kernel, copy_len);
    return (sigma_ssize_t)copy_len;
}

sigma_ssize_t sigma_strncpy_from_user(char* dst_kernel,
                                       const sigma_user_ro_t* src,
                                       sigma_size_t max_len) {
    if (!src->validated || max_len == 0) return -1;
    sigma_size_t copy_len = (max_len - 1 < src->len) ? max_len - 1 : src->len;
    memcpy(dst_kernel, src->ptr, copy_len);
    dst_kernel[copy_len] = '\0';

    /* Check if the source string was longer than max_len */
    if (copy_len == max_len - 1 && copy_len < src->len) {
        sigma_log_err("[sigma-usercopy] strncpy_from_user: string truncated at %zu bytes\n",
                      max_len);
        return -2; /* -ENAMETOOLONG */
    }
    return (sigma_ssize_t)copy_len;
}
