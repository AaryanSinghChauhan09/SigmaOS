// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_usercopy.h — type-safe kernel↔userspace memory API (Redox UserSlice-inspired)
 *
 * Replaces raw strcpy/memcpy on userspace pointers in the kernel.
 * Every operation validates bounds before touching user memory.
 * Returns -EFAULT on bad address, -ENAMETOOLONG if string too long.
 *
 * Usage — replacing Bug C-2 (strcpy overflow):
 *   // BEFORE:  strcpy(w->shard_id, shard_id);   // overflow!
 *   // AFTER:
 *   sigma_user_ro_t usr;
 *   sigma_user_ro_create(&usr, shard_id, SIGMA_ZT_SHARD_ID_LEN);
 *   sigma_strncpy_from_user(w->shard_id, &usr, sizeof(w->shard_id));
 */
#include <sigma_kernel_types.h>

/* __user annotation — documents that the pointer comes from userspace */
#define __user

typedef struct {
    const void __user* ptr;
    sigma_size_t       len;
    bool               validated;
} sigma_user_ro_t;

typedef struct {
    void __user*   ptr;
    sigma_size_t   len;
    bool           validated;
} sigma_user_wo_t;

/* Create validated user slices — checks that ptr+len is within user address space */
int sigma_user_ro_create(sigma_user_ro_t* out, const void __user* ptr, sigma_size_t len);
int sigma_user_wo_create(sigma_user_wo_t* out,       void __user* ptr, sigma_size_t len);

/* Copy from/to user — bounded, -EFAULT on invalid address */
sigma_ssize_t sigma_copy_from_user(void* dst_kernel,
                                    const sigma_user_ro_t* src,
                                    sigma_size_t len);
sigma_ssize_t sigma_copy_to_user(sigma_user_wo_t* dst,
                                  const void* src_kernel,
                                  sigma_size_t len);

/* Safe string copy from user — bounded, always NUL-terminates */
sigma_ssize_t sigma_strncpy_from_user(char* dst_kernel,
                                       const sigma_user_ro_t* src,
                                       sigma_size_t max_len);
