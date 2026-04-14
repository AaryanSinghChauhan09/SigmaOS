/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ANDROID BINDER IPC (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: Android Open Source Project (AOSP) - Binder
 *   https://android.googlesource.com/kernel/common/+/refs/heads/android-mainline/drivers/android/binder.c
 *
 * Features implemented:
 *   ✓ Binder Nodes and Refs 
 *   ✓ Parcels (marshalling/unmarshalling)
 *   ✓ Transactions (Sync / Async one-way)
 *   ✓ Context Manager (ServiceManager / init)
 * =========================================================================
 */

#ifndef SOVEREIGN_ANDROID_BINDER_H
#define SOVEREIGN_ANDROID_BINDER_H

#include "sigma_types.h"

typedef sigma_u32 sigma_binder_handle_t;

typedef struct {
    sigma_u8 *data;
    sigma_size_t data_size;
    sigma_size_t data_pos;
} SigmaParcel_t;

typedef struct {
    sigma_binder_handle_t target_handle;
    sigma_u32 code;
    sigma_u32 flags; /* 0 = Sync, 1 = OneWay */
    SigmaParcel_t data;
    SigmaParcel_t reply;
    sigma_i32 sender_pid;
    sigma_i32 sender_euid;
} SigmaBinderTransaction_t;

void sigma_parcel_init(SigmaParcel_t *p);
void sigma_parcel_write_int32(SigmaParcel_t *p, sigma_i32 val);
sigma_i32 sigma_parcel_read_int32(SigmaParcel_t *p);
void sigma_parcel_write_string(SigmaParcel_t *p, const char *str);
const char* sigma_parcel_read_string(SigmaParcel_t *p);

sigma_err_t sigma_binder_transact(SigmaBinderTransaction_t *tr);
sigma_err_t sigma_binder_become_context_manager(void);

void SovereignAndroidBinder_Init(void);

#endif /* SOVEREIGN_ANDROID_BINDER_H */
