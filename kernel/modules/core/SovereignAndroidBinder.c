/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ANDROID BINDER IPC — IMPL (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignAndroidBinder.h"

static sigma_u8 s_parcel_buffers[16][1024]; /* Mock allocation */
static sigma_u32 s_parcel_idx = 0;
static sigma_bool s_has_service_manager = SIGMA_FALSE;

void sigma_parcel_init(SigmaParcel_t *p) {
    if (s_parcel_idx >= 16) s_parcel_idx = 0;
    p->data = s_parcel_buffers[s_parcel_idx++];
    p->data_size = 1024;
    p->data_pos = 0;
}

void sigma_parcel_write_int32(SigmaParcel_t *p, sigma_i32 val) {
    if (p->data_pos + 4 <= p->data_size) {
        sigma_memcpy(p->data + p->data_pos, &val, 4);
        p->data_pos += 4;
    }
}

sigma_i32 sigma_parcel_read_int32(SigmaParcel_t *p) {
    sigma_i32 val = 0;
    if (p->data_pos + 4 <= p->data_size) {
        sigma_memcpy(&val, p->data + p->data_pos, 4);
        p->data_pos += 4;
    }
    return val;
}

void sigma_parcel_write_string(SigmaParcel_t *p, const char *str) {
    sigma_size_t len = sigma_strlen(str) + 1;
    if (p->data_pos + len <= p->data_size) {
        sigma_memcpy(p->data + p->data_pos, str, len);
        p->data_pos += len;
    }
}

const char* sigma_parcel_read_string(SigmaParcel_t *p) {
    const char *str = (const char*)(p->data + p->data_pos);
    sigma_size_t len = sigma_strlen(str) + 1;
    if (p->data_pos + len <= p->data_size) {
        p->data_pos += len;
        return str;
    }
    return SIGMA_NULL;
}

sigma_err_t sigma_binder_transact(SigmaBinderTransaction_t *tr) {
    sigma_printf("Σ [BINDER]: Transaction -> target:%u code:%u sender:%d\n", tr->target_handle, tr->code, tr->sender_pid);
    if (tr->target_handle == 0 && tr->code == 1 && s_has_service_manager) {
        /* Add Service */
        sigma_parcel_init(&tr->reply);
        sigma_parcel_write_int32(&tr->reply, 0); /* Success */
        sigma_printf("Σ [BINDER]: ServiceManager registered new service.\n");
    }
    return SIGMA_OK;
}

sigma_err_t sigma_binder_become_context_manager(void) {
    if (s_has_service_manager) return SIGMA_EBUSY;
    s_has_service_manager = SIGMA_TRUE;
    sigma_printf("Σ [BINDER]: Process registered as Context Manager (ServiceManager).\n");
    return SIGMA_OK;
}

void SovereignAndroidBinder_Init(void) {
    sigma_printf("Σ [BINDER]: Initialising Sovereign Android Binder IPC parity...\n");
    sigma_binder_become_context_manager();

    SigmaBinderTransaction_t tr;
    sigma_memset(&tr, 0, sizeof(tr));
    tr.target_handle = 0; /* Service Manager */
    tr.code = 1; /* ADD_SERVICE_TRANSACTION */
    tr.sender_pid = 100;
    sigma_parcel_init(&tr.data);
    sigma_parcel_write_string(&tr.data, "sigma.hardware.audio");
    
    sigma_binder_transact(&tr);
}
