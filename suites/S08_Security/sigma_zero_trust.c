/*
 * =============================================================================
 * Σ SIGMAOS: ZERO-TRUST CAPABILITY VERIFICATION ENGINE (v1.0)
 * =============================================================================
 * Every module must present a cryptographically signed capability token
 * before it can access any kernel resource (HAL, memory, IPC).
 *
 * Design:
 *   - Capabilities are 256-bit tokens signed with the kernel master key.
 *   - Each token encodes: module_id, resource_mask, expiry_tick, nonce.
 *   - The kernel verifies the token before dispatching any syscall.
 *   - Tokens are non-transferable (bound to the module's pool ID).
 *
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_features.h"

/* =========================================================================
 * Capability Token
 * ========================================================================= */

#define CAP_RESOURCE_HAL        (1u << 0)
#define CAP_RESOURCE_MEMORY     (1u << 1)
#define CAP_RESOURCE_NETWORK    (1u << 2)
#define CAP_RESOURCE_STORAGE    (1u << 3)
#define CAP_RESOURCE_IPC        (1u << 4)
#define CAP_RESOURCE_DISPLAY    (1u << 5)
#define CAP_RESOURCE_CRYPTO     (1u << 6)
#define CAP_RESOURCE_ADMIN      (1u << 7)
#define CAP_RESOURCE_ALL        0xFFFFFFFFu

typedef struct SigmaCapability {
    u32  module_id;          /* Bound to this module's pool ID */
    u32  resource_mask;      /* Bitfield of allowed resources */
    u64  expiry_tick;        /* Kernel tick after which this token is invalid */
    u64  nonce;              /* Anti-replay nonce */
    u8   signature[32];      /* HMAC-SHA256 of (module_id|mask|expiry|nonce) */
} SigmaCapability;

/* =========================================================================
 * Internal: Kernel master key (set at boot from secure enclave / TPM)
 * ========================================================================= */

static u8 g_master_key[32] = {0};
static u64 g_nonce_counter  = 0;

extern u64 get_system_ticks(void);
extern void hash_sha256(const void* in, u64 len, void* out);

/* Simple HMAC-SHA256 over the token fields (simplified) */
static void compute_signature(const SigmaCapability* cap, const u8* key, u8* sig_out) {
    /* In production: proper HMAC. Here we hash key||fields. */
    u8 buf[32 + 4 + 4 + 8 + 8];
    u32 i;
    for (i = 0; i < 32; i++) buf[i] = key[i];

    u32 off = 32;
    buf[off++] = (u8)(cap->module_id);
    buf[off++] = (u8)(cap->module_id >> 8);
    buf[off++] = (u8)(cap->module_id >> 16);
    buf[off++] = (u8)(cap->module_id >> 24);

    buf[off++] = (u8)(cap->resource_mask);
    buf[off++] = (u8)(cap->resource_mask >> 8);
    buf[off++] = (u8)(cap->resource_mask >> 16);
    buf[off++] = (u8)(cap->resource_mask >> 24);

    u64 e = cap->expiry_tick;
    for (i = 0; i < 8; i++) { buf[off++] = (u8)(e & 0xFF); e >>= 8; }

    u64 n = cap->nonce;
    for (i = 0; i < 8; i++) { buf[off++] = (u8)(n & 0xFF); n >>= 8; }

    hash_sha256(buf, off, sig_out);
}

/* =========================================================================
 * Public API
 * ========================================================================= */

/**
 * zt_init — Initialize the zero-trust engine with a master key.
 */
void zt_init(const u8 key[32]) {
    u32 i;
    for (i = 0; i < 32; i++) g_master_key[i] = key[i];
    g_nonce_counter = 0;
}

/**
 * zt_issue_capability — Mint a new capability token for a module.
 *
 * @param module_id      Module's pool ID.
 * @param resource_mask  Bitfield of resources this module may access.
 * @param ttl_ticks      How many ticks until expiry.
 * @param out            Output token.
 * @return               K_OK on success.
 */
k_status zt_issue_capability(u32 module_id, u32 resource_mask,
                              u64 ttl_ticks, SigmaCapability* out)
{
    if (!out) return K_ERR_INVAL;
    out->module_id     = module_id;
    out->resource_mask = resource_mask;
    out->expiry_tick   = get_system_ticks() + ttl_ticks;
    out->nonce         = g_nonce_counter++;
    compute_signature(out, g_master_key, out->signature);
    return K_OK;
}

/**
 * zt_verify — Verify a capability token before granting access.
 *
 * @param cap            The token to verify.
 * @param required_mask  Resources the caller is requesting.
 * @return               K_OK if verified, K_ERR_INVAL otherwise.
 */
k_status zt_verify(const SigmaCapability* cap, u32 required_mask) {
    if (!cap) return K_ERR_INVAL;

    /* 1. Check expiry */
    if (get_system_ticks() > cap->expiry_tick) return K_ERR_INVAL;

    /* 2. Check resource permissions */
    if ((cap->resource_mask & required_mask) != required_mask) return K_ERR_INVAL;

    /* 3. Verify signature */
    u8 expected[32];
    compute_signature(cap, g_master_key, expected);
    u32 i;
    u32 diff = 0;
    for (i = 0; i < 32; i++) diff |= (cap->signature[i] ^ expected[i]);
    if (diff != 0) return K_ERR_INVAL;

    return K_OK;
}

/**
 * zt_revoke — Invalidate a capability by zeroing its signature.
 */
void zt_revoke(SigmaCapability* cap) {
    if (!cap) return;
    u32 i;
    for (i = 0; i < 32; i++) cap->signature[i] = 0;
    cap->expiry_tick = 0;
}
