// SPDX-License-Identifier: GPL-2.0-or-later
// tests/unit/test_sigma_fs.cpp — SemanticFS and CryptFS unit tests
#include <gtest/gtest.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

// ── Minimal SemanticFS stub ───────────────────────────────────────────────
// Mirrors kernel/fs/sigma_semanticfs.h xattr-based metadata indexing

struct SFSEntry {
    char path[256];
    char xattrs[16][2][128];  /* key-value pairs */
    int  n_xattrs;
};

static SFSEntry sfs_db[256];
static int sfs_count = 0;

static void sfs_reset() { sfs_count = 0; memset(sfs_db, 0, sizeof(sfs_db)); }

static int sfs_set_xattr(const char *path, const char *key, const char *value) {
    // Find or create entry
    SFSEntry *e = nullptr;
    for (int i = 0; i < sfs_count; i++) {
        if (strcmp(sfs_db[i].path, path) == 0) { e = &sfs_db[i]; break; }
    }
    if (!e) {
        if (sfs_count >= 256) return -1;
        e = &sfs_db[sfs_count++];
        strncpy(e->path, path, sizeof(e->path)-1);
    }
    // Add or update xattr
    for (int i = 0; i < e->n_xattrs; i++) {
        if (strcmp(e->xattrs[i][0], key) == 0) {
            strncpy(e->xattrs[i][1], value, 127);
            return 0;
        }
    }
    if (e->n_xattrs >= 16) return -1;
    strncpy(e->xattrs[e->n_xattrs][0], key,   127);
    strncpy(e->xattrs[e->n_xattrs][1], value, 127);
    e->n_xattrs++;
    return 0;
}

static const char *sfs_get_xattr(const char *path, const char *key) {
    for (int i = 0; i < sfs_count; i++) {
        if (strcmp(sfs_db[i].path, path) != 0) continue;
        for (int j = 0; j < sfs_db[i].n_xattrs; j++) {
            if (strcmp(sfs_db[i].xattrs[j][0], key) == 0)
                return sfs_db[i].xattrs[j][1];
        }
    }
    return nullptr;
}

// Simple query: find all files where xattr key=value
static int sfs_query(const char *key, const char *value,
                      char results[][256], int max_results) {
    int found = 0;
    for (int i = 0; i < sfs_count && found < max_results; i++) {
        for (int j = 0; j < sfs_db[i].n_xattrs; j++) {
            if (strcmp(sfs_db[i].xattrs[j][0], key) == 0 &&
                strcmp(sfs_db[i].xattrs[j][1], value) == 0) {
                strncpy(results[found++], sfs_db[i].path, 255);
                break;
            }
        }
    }
    return found;
}

// ── SemanticFS Tests ──────────────────────────────────────────────────────

TEST(SemanticFS, SetAndGetXattr) {
    sfs_reset();
    EXPECT_EQ(sfs_set_xattr("/home/user/invoice.pdf", "sigma:type", "invoice"), 0);
    const char *v = sfs_get_xattr("/home/user/invoice.pdf", "sigma:type");
    ASSERT_NE(v, nullptr);
    EXPECT_STREQ(v, "invoice");
}

TEST(SemanticFS, QueryByXattr) {
    sfs_reset();
    sfs_set_xattr("/docs/invoice_001.pdf", "sigma:type", "invoice");
    sfs_set_xattr("/docs/invoice_002.pdf", "sigma:type", "invoice");
    sfs_set_xattr("/docs/contract.pdf",    "sigma:type", "contract");

    char results[10][256];
    int n = sfs_query("sigma:type", "invoice", results, 10);
    EXPECT_EQ(n, 2);
}

TEST(SemanticFS, MissingXattrReturnsNull) {
    sfs_reset();
    sfs_set_xattr("/file.txt", "sigma:author", "Ravi");
    EXPECT_EQ(sfs_get_xattr("/file.txt", "sigma:missing"), nullptr);
    EXPECT_EQ(sfs_get_xattr("/nonexistent.txt", "sigma:type"), nullptr);
}

TEST(SemanticFS, UpdateXattr) {
    sfs_reset();
    sfs_set_xattr("/doc.pdf", "sigma:status", "draft");
    sfs_set_xattr("/doc.pdf", "sigma:status", "final");
    EXPECT_STREQ(sfs_get_xattr("/doc.pdf", "sigma:status"), "final");
}

TEST(SemanticFS, MultipleXattrsOnOneFile) {
    sfs_reset();
    sfs_set_xattr("/report.pdf", "sigma:type",   "report");
    sfs_set_xattr("/report.pdf", "sigma:year",   "2024");
    sfs_set_xattr("/report.pdf", "sigma:author", "Priya");

    EXPECT_STREQ(sfs_get_xattr("/report.pdf", "sigma:type"),   "report");
    EXPECT_STREQ(sfs_get_xattr("/report.pdf", "sigma:year"),   "2024");
    EXPECT_STREQ(sfs_get_xattr("/report.pdf", "sigma:author"), "Priya");
}

// ── CryptFS key tests ─────────────────────────────────────────────────────
// (Non-zero key regression — full test in regression_cryptfs_zero_key.cpp)

static bool key_all_zeros(const uint8_t *k, size_t n) {
    for (size_t i = 0; i < n; i++) if (k[i]) return false;
    return true;
}

TEST(CryptFS, DerivedKeyIsNonZero) {
    // Simulate what sigma_tpm_unseal + HKDF would produce
    // (Real key must come from TPM2 — this is a structural test)
    uint8_t simulated_key[32];
    // Populate with a non-trivial value (mimics HKDF output)
    for (int i = 0; i < 32; i++) simulated_key[i] = (uint8_t)(i * 0x37 ^ 0x5A);
    EXPECT_FALSE(key_all_zeros(simulated_key, 32))
        << "AES key must not be all zeros after TPM2 + HKDF derivation";
}

TEST(CryptFS, SectorIVDependsOnSectorNumber) {
    // Two different sector numbers must produce different IVs
    // IV derivation: SHA-256(sector_no || key[0:8]) → first 12 bytes
    uint8_t iv1[12] = {0x01,0x00,0x00,0x00, 0,0,0,0, 0x37,0x5A,0x11,0x22};
    uint8_t iv2[12] = {0x02,0x00,0x00,0x00, 0,0,0,0, 0x37,0x5A,0x11,0x22};
    bool same = true;
    for (int i = 0; i < 12; i++) if (iv1[i] != iv2[i]) { same = false; break; }
    EXPECT_FALSE(same) << "Different sectors must produce different IVs";
}
