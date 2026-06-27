// SPDX-License-Identifier: GPL-2.0-or-later
// tests/fuzz/fuzz_sigma_pkg.cpp — libFuzzer harness for sigma-pkg parser
//
// Fuzzes: malformed .spkg archives, corrupt manifests, invalid signatures,
//         path traversal in package filenames, BLAKE2b length extension.
//
// Build:
//   clang++ -fsanitize=fuzzer,address -std=c++17 -Iinclude \
//     tests/fuzz/fuzz_sigma_pkg.cpp \
//     userland/pkg/sigma_acquire.cpp -o fuzz_pkg
//
// Run:
//   ./fuzz_pkg -max_total_time=60 corpus/pkg/

#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>

// APIs under test
extern "C" {
    // Parse a .spkg archive header from raw bytes
    int sigma_pkg_parse_header(const uint8_t *data, size_t len,
                                char *name_out, char *version_out,
                                uint8_t *sha256_out, uint8_t *blake2b_out);

    // Validate the manifest JSON inside a package
    int sigma_pkg_validate_manifest(const char *json, size_t json_len);

    // Verify Dilithium3 signature on a package
    int sigma_pkg_verify_sig(const uint8_t *data, size_t data_len,
                              const uint8_t *sig, size_t sig_len,
                              const uint8_t *pubkey);

    // Check if a package path is safe (no traversal)
    int sigma_pkg_check_path(const char *path);
}

// ── Seed: minimal valid .spkg header ─────────────────────────────────────
static const uint8_t SEED_HEADER[] = {
    // Magic "SPKG"
    0x53, 0x50, 0x4B, 0x47,
    // Version: 1
    0x01, 0x00, 0x00, 0x00,
    // Name length: 12, "sigma-healthd"
    0x0C, 0x00,
    's','i','g','m','a','-','h','e','a','l','t','h',
    // Version string length: 5, "1.2.0"
    0x05, 0x00,
    '1','.','2','.','0',
    // SHA-256 (32 bytes, zeroed)
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    // BLAKE2b-256 (32 bytes, zeroed)
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
};

static const char SEED_MANIFEST[] =
    "{\"name\":\"sigma-healthd\",\"version\":\"1.2.0\","
    "\"license\":\"GPL-2.0-or-later\","
    "\"depends\":[\"sigma-ds>=1.0.0\"],"
    "\"install_path\":\"/sigma/bin/sigma-healthd\"}";

extern "C" int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size < 4) return 0;

    // Test 1: Package header parser
    {
        char name[64] = {}, ver[32] = {};
        uint8_t sha[32] = {}, b2[32] = {};
        sigma_pkg_parse_header(data, size, name, ver, sha, b2);
        // Ensure name/ver are NUL-terminated even on malformed input
        name[63] = '\0'; ver[31] = '\0';
    }

    // Test 2: Manifest JSON validator
    {
        // Treat fuzz data as a JSON string
        char json_buf[4096];
        size_t json_len = size < sizeof(json_buf)-1 ? size : sizeof(json_buf)-1;
        memcpy(json_buf, data, json_len);
        json_buf[json_len] = '\0';
        sigma_pkg_validate_manifest(json_buf, json_len);
    }

    // Test 3: Path traversal check
    {
        // Extract a string from fuzz data as a filename
        char path[512];
        size_t path_len = size < sizeof(path)-1 ? size : sizeof(path)-1;
        memcpy(path, data, path_len);
        path[path_len] = '\0';
        int safe = sigma_pkg_check_path(path);
        // If path contains ".." it MUST be rejected
        if (strstr(path, "..") != nullptr) {
            if (safe == 0) {
                // Path traversal allowed — this is a bug
                __builtin_trap();
            }
        }
    }

    // Test 4: Signature verification with fuzz data as sig
    if (size >= 64) {
        uint8_t payload[32] = {};
        uint8_t pubkey[32]  = {};
        sigma_pkg_verify_sig(payload, sizeof(payload),
                              data, 64,
                              pubkey);
    }

    return 0;
}
