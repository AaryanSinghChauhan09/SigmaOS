/**
 * =========================================================================
 * Σ SIGMAOS: OFFLINE SYNC ENGINE PUBLIC HEADER
 * =========================================================================
 */
#pragma once
#include "../../include/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

sigma_status sigma_offline_sync_init(sigma_u32 peer_id);
sigma_status sigma_offline_sync_put(const char* key, const sigma_u8* value, sigma_u32 len);
sigma_status sigma_offline_sync_get(const char* key, sigma_u8* buf, sigma_u32 len, sigma_u32* out_len);
sigma_status sigma_offline_sync_delete(const char* key);
sigma_status sigma_offline_sync_merge(const char* key,
                                       const sigma_u8* remote_value, sigma_u32 remote_len,
                                       const sigma_u64* remote_vc, sigma_u32 remote_vc_peers,
                                       bool remote_deleted);
void         sigma_offline_sync_set_online(bool online);
sigma_status sigma_offline_sync_flush_dirty(void);
void         sigma_offline_sync_stats(void);

#ifdef __cplusplus
}
#endif
