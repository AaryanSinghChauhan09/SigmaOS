// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_pkg_verify — dual-hash verification correctness
 *
 * Verifies that:
 *   1. Matching SHA-256 + BLAKE2b passes.
 *   2. SHA-256 mismatch alone fails (even if BLAKE2b matches).
 *   3. BLAKE2b mismatch alone fails (even if SHA-256 matches).
 *   4. Both mismatching fails.
 *
 * Uses mock hash values — the real crypto is tested separately via
 * SovereignPQC tests. This test covers the verification LOGIC only.
 */
#include <cassert>
#include <cstdio>
#include <cstring>

/* ── Minimal package descriptor (mirrors sigma_acq_item_t) ─────────────── */
typedef struct {
    char sha256[65];
    char blake2b[129];
    char actual_sha256[65];   /* what was computed from the downloaded file */
    char actual_blake2b[129];
} pkg_hashes_t;

/* Verification logic extracted from sigma_acquire_verify() */
static int verify(const pkg_hashes_t* h) {
    if (strcmp(h->actual_sha256, h->sha256) != 0)   return -1; /* SHA mismatch */
    if (strcmp(h->actual_blake2b, h->blake2b) != 0) return -2; /* B2 mismatch  */
    return 0;
}

static const char* GOOD_SHA   = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
static const char* GOOD_B2    = "aabbccdd11223344aabbccdd11223344aabbccdd11223344aabbccdd11223344"
                                 "aabbccdd11223344aabbccdd11223344aabbccdd11223344aabbccdd11223344";
static const char* BAD_SHA    = "0000000000000000000000000000000000000000000000000000000000000000";
static const char* BAD_B2     = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                                 "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

int main(void) {
    pkg_hashes_t h = {};

    /* ── Test 1: both hashes match → PASS ───────────────────────────── */
    strncpy(h.sha256,        GOOD_SHA, 64);
    strncpy(h.blake2b,       GOOD_B2,  128);
    strncpy(h.actual_sha256, GOOD_SHA, 64);
    strncpy(h.actual_blake2b,GOOD_B2,  128);
    assert(verify(&h) == 0 && "matching hashes must pass");

    /* ── Test 2: SHA mismatch → FAIL (-1) ───────────────────────────── */
    strncpy(h.actual_sha256, BAD_SHA, 64);
    assert(verify(&h) == -1 && "SHA mismatch must fail");

    /* ── Test 3: SHA ok, BLAKE2b mismatch → FAIL (-2) ───────────────── */
    strncpy(h.actual_sha256,  GOOD_SHA, 64);
    strncpy(h.actual_blake2b, BAD_B2,   128);
    assert(verify(&h) == -2 && "BLAKE2b mismatch must fail");

    /* ── Test 4: both mismatching → returns -1 (SHA checked first) ──── */
    strncpy(h.actual_sha256,  BAD_SHA, 64);
    strncpy(h.actual_blake2b, BAD_B2,  128);
    assert(verify(&h) < 0 && "both hashes mismatching must fail");

    /* ── Test 5: empty expected hash → mismatch with any real hash ───── */
    memset(h.sha256,  0, sizeof(h.sha256));
    memset(h.blake2b, 0, sizeof(h.blake2b));
    strncpy(h.actual_sha256,  GOOD_SHA, 64);
    strncpy(h.actual_blake2b, GOOD_B2,  128);
    assert(verify(&h) != 0 && "empty expected hash must not match a real hash");

    printf("test_pkg_verify: PASS\n");
    return 0;
}
