/* SPDX-License-Identifier: MIT
 * tests/unit/kernel/cryptfs_key_determinism_property_test.c
 *
 * Property 17: CryptFS Key Derivation Determinism
 * Validates: Requirement 37.4 — calling derive_key(passphrase, salt) twice
 * with identical inputs MUST produce bit-for-bit identical 256-bit output keys.
 *
 * Exercises: PBKDF2-HMAC-SHA256 implementation in fs/sigma_cryptfs.rs
 * via its C-ABI export: cryptfs_derive_key(pass, pass_len, salt, salt_len, out, out_len)
 *
 * Build (host runner linking sigma_cryptfs):
 *   cc -DTEST_HOST_RUNNER cryptfs_key_determinism_property_test.c \
 *      -L../../.. -lsigma_cryptfs -o cryptfs_det_test
 *
 * Freestanding build (runs inside SigmaOS test harness):
 *   rustc --edition 2021 --crate-type staticlib ...sigma_cryptfs.rs... &&
 *   cc -nostdlib -ffreestanding ... -lsigma_cryptfs cryptfs_key_determinism_property_test.c
 */

#include <stdint.h>
#include <stddef.h>

#ifdef TEST_HOST_RUNNER
#  include <stdio.h>
#  include <stdlib.h>
#  include <string.h>
#endif

/* ── C-ABI declaration for PBKDF2 key derivation (from sigma_cryptfs.rs) ── */
extern void cryptfs_derive_key(
    const uint8_t *password, size_t pass_len,
    const uint8_t *salt,     size_t salt_len,
    uint8_t       *out,      size_t out_len
);

#define KEY_LEN  32  /* AES-256: 32 bytes */

/* ── Test vectors (deterministic; computed offline) ─────────────────────── */
typedef struct {
    const char *label;
    const uint8_t *password;
    size_t         pass_len;
    const uint8_t *salt;
    size_t         salt_len;
} test_vector_t;

static const uint8_t PW1[]   = "correct-horse-battery-staple";
static const uint8_t PW2[]   = "a";
static const uint8_t PW3[]   = { 0x00 };  /* one null byte */
static const uint8_t PW4[]   = {
    0xDE,0xAD,0xBE,0xEF,0xCA,0xFE,0xBA,0xBE,
    0x01,0x23,0x45,0x67,0x89,0xAB,0xCD,0xEF,
    0xFE,0xDC,0xBA,0x98,0x76,0x54,0x32,0x10,
    0x00,0xFF,0x0F,0xF0,0xAA,0x55,0xA5,0x5A
}; /* 32 binary bytes */

static const uint8_t SALT1[] = "sigmaos-cryptfs-salt-v1";
static const uint8_t SALT2[] = { 0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,
                                   0x08,0x09,0x0A,0x0B,0x0C,0x0D,0x0E,0x0F };
static const uint8_t SALT3[] = { 0xFF };

static const test_vector_t VECTORS[] = {
    { "printable/printable_16",  PW1,  sizeof(PW1)-1, SALT1, sizeof(SALT1)-1 },
    { "single_char/salt16",      PW2,  sizeof(PW2)-1, SALT1, sizeof(SALT1)-1 },
    { "null_byte/binary_salt16", PW3,  1,             SALT2, sizeof(SALT2)   },
    { "binary32/salt16",         PW4,  sizeof(PW4),   SALT2, sizeof(SALT2)   },
    { "printable/salt1",         PW1,  sizeof(PW1)-1, SALT3, 1               },
    { "single_char/salt1",       PW2,  sizeof(PW2)-1, SALT3, 1               },
};

#define N_VECTORS (sizeof(VECTORS)/sizeof(VECTORS[0]))

/* ── Memory comparison (no memcmp in freestanding) ───────────────────────── */
static int mem_eq(const uint8_t *a, const uint8_t *b, size_t n)
{
    for (size_t i = 0; i < n; i++) {
        if (a[i] != b[i]) return 0;
    }
    return 1;
}

static int mem_all_zero(const uint8_t *p, size_t n)
{
    for (size_t i = 0; i < n; i++) {
        if (p[i] != 0) return 0;
    }
    return 1;
}

/* ── Property 17: determinism — two calls with same inputs yield same output */
static int prop_determinism(const test_vector_t *v)
{
    uint8_t key1[KEY_LEN];
    uint8_t key2[KEY_LEN];

    /* Zero-initialise to catch non-write bugs */
    for (int i = 0; i < KEY_LEN; i++) { key1[i] = 0xAA; key2[i] = 0xBB; }

    cryptfs_derive_key(v->password, v->pass_len, v->salt, v->salt_len,
                       key1, KEY_LEN);
    cryptfs_derive_key(v->password, v->pass_len, v->salt, v->salt_len,
                       key2, KEY_LEN);

    if (!mem_eq(key1, key2, KEY_LEN)) {
#ifdef TEST_HOST_RUNNER
        printf("  [FAIL] Non-deterministic output for '%s'\n", v->label);
        printf("    key1: "); for(int i=0;i<KEY_LEN;i++) printf("%02X",key1[i]); printf("\n");
        printf("    key2: "); for(int i=0;i<KEY_LEN;i++) printf("%02X",key2[i]); printf("\n");
#endif
        return 0;
    }
    return 1;
}

/* ── Property: output is NOT all-zero (real key was written) ─────────────── */
static int prop_non_zero_output(const test_vector_t *v)
{
    uint8_t key[KEY_LEN];
    for (int i = 0; i < KEY_LEN; i++) key[i] = 0;

    cryptfs_derive_key(v->password, v->pass_len, v->salt, v->salt_len,
                       key, KEY_LEN);

    if (mem_all_zero(key, KEY_LEN)) {
#ifdef TEST_HOST_RUNNER
        printf("  [FAIL] All-zero key for '%s' — derivation likely broken\n", v->label);
#endif
        return 0;
    }
    return 1;
}

/* ── Property: different passwords → different keys ─────────────────────── */
static int prop_different_passwords_differ(void)
{
    uint8_t key1[KEY_LEN], key2[KEY_LEN];

    cryptfs_derive_key(PW1, sizeof(PW1)-1, SALT2, sizeof(SALT2), key1, KEY_LEN);
    cryptfs_derive_key(PW2, sizeof(PW2)-1, SALT2, sizeof(SALT2), key2, KEY_LEN);

    if (mem_eq(key1, key2, KEY_LEN)) {
#ifdef TEST_HOST_RUNNER
        printf("  [FAIL] Different passwords produced identical keys\n");
#endif
        return 0;
    }
    return 1;
}

/* ── Property: different salts → different keys ──────────────────────────── */
static int prop_different_salts_differ(void)
{
    uint8_t key1[KEY_LEN], key2[KEY_LEN];

    cryptfs_derive_key(PW1, sizeof(PW1)-1, SALT1, sizeof(SALT1)-1, key1, KEY_LEN);
    cryptfs_derive_key(PW1, sizeof(PW1)-1, SALT2, sizeof(SALT2),   key2, KEY_LEN);

    if (mem_eq(key1, key2, KEY_LEN)) {
#ifdef TEST_HOST_RUNNER
        printf("  [FAIL] Different salts produced identical keys\n");
#endif
        return 0;
    }
    return 1;
}

/* ── Property: output length is exactly KEY_LEN bytes written ─────────────── */
static int prop_full_key_written(const test_vector_t *v)
{
    /* Fill with sentinel value, verify all bytes are overwritten */
    uint8_t key[KEY_LEN + 2];
    key[0] = 0xDE; key[KEY_LEN + 1] = 0xAD;
    for (int i = 1; i <= KEY_LEN; i++) key[i] = 0xFF;

    cryptfs_derive_key(v->password, v->pass_len, v->salt, v->salt_len,
                       &key[1], KEY_LEN);

    /* Sentinel bytes must be untouched */
    if (key[0] != 0xDE || key[KEY_LEN + 1] != 0xAD) {
#ifdef TEST_HOST_RUNNER
        printf("  [FAIL] '%s': derive_key wrote outside output buffer\n", v->label);
#endif
        return 0;
    }
    /* Actual key bytes must differ from 0xFF (were overwritten) */
    int all_ff = 1;
    for (int i = 1; i <= KEY_LEN; i++) {
        if (key[i] != 0xFF) { all_ff = 0; break; }
    }
    if (all_ff) {
#ifdef TEST_HOST_RUNNER
        printf("  [FAIL] '%s': derive_key wrote nothing (all 0xFF sentinel)\n", v->label);
#endif
        return 0;
    }
    return 1;
}

/* ── Test runner ─────────────────────────────────────────────────────────── */
#ifdef TEST_HOST_RUNNER
int main(void)
{
    int passed = 0, failed = 0;

    printf("=== CryptFS Key Derivation Determinism Property Tests ===\n");

    for (size_t i = 0; i < N_VECTORS; i++) {
        const test_vector_t *v = &VECTORS[i];
        char name[128];

#define RUNV(propname, call)                                                \
        do {                                                                \
            int r = (call);                                                 \
            if (r) { printf("[PASS] %s / %s\n", propname, v->label); passed++; } \
            else   { printf("[FAIL] %s / %s\n", propname, v->label); failed++; } \
        } while(0)

        RUNV("determinism",    prop_determinism(v));
        RUNV("non_zero",       prop_non_zero_output(v));
        RUNV("full_key_written", prop_full_key_written(v));
    }

    int r;
    r = prop_different_passwords_differ();
    if (r) { printf("[PASS] different_passwords_differ\n"); passed++; }
    else   { printf("[FAIL] different_passwords_differ\n"); failed++; }

    r = prop_different_salts_differ();
    if (r) { printf("[PASS] different_salts_differ\n"); passed++; }
    else   { printf("[FAIL] different_salts_differ\n"); failed++; }

    printf("\nResults: %d passed, %d failed\n", passed, failed);
    return failed == 0 ? 0 : 1;
}
#endif
