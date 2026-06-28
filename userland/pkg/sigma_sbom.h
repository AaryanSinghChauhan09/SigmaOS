// SPDX-License-Identifier: GPL-2.0-only
// sigma_sbom.h — SigmaOS Software Bill of Materials (SBOM)
// Format: CycloneDX 1.6 JSON + SPDX 2.3 SPDX-compatible output
// Purpose: Every sigma-pkg release has a complete, Dilithium3-signed SBOM
//          listing all components, licenses, vulnerabilities, and provenance.
//          Enables supply chain security audit for enterprise/government users.

#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

#define SIGMA_SBOM_FORMAT_CYCLONEDX  "cyclonedx-1.6"
#define SIGMA_SBOM_FORMAT_SPDX       "spdx-2.3"

typedef enum {
    SIGMA_SBOM_COMP_LIBRARY     = 1,
    SIGMA_SBOM_COMP_APPLICATION = 2,
    SIGMA_SBOM_COMP_FIRMWARE    = 3,
    SIGMA_SBOM_COMP_OS          = 4,
    SIGMA_SBOM_COMP_DEVICE      = 5,
} sigma_sbom_comp_type_t;

typedef struct {
    char   name[128];
    char   version[32];
    char   purl[256];              // Package URL: pkg:sigma/sigma-accounts@1.2.3
    char   cpe[128];               // CPE 2.3 identifier
    sigma_sbom_comp_type_t type;
    char   supplier[128];
    char   license_spdx[64];       // e.g. "GPL-2.0-only"
    char   hash_sha256[65];
    char   hash_sha512[129];
    char   source_url[256];        // Source code location
    char   download_url[256];
    bool   modified;               // If modified from upstream
    // Vulnerability data (from NVD/OSV)
    struct {
        char   cve_id[16];
        char   severity[8];        // "CRITICAL", "HIGH", "MEDIUM", "LOW"
        double cvss_score;
        bool   patch_available;
        char   fixed_version[32];
    } vulns[8];
    int    vuln_count;
} sigma_sbom_component_t;

typedef struct {
    char   package_name[64];
    char   package_version[32];
    char   build_host[64];
    time_t build_timestamp;
    char   source_date_epoch[32];  // For reproducible builds
    char   build_hash[65];         // Hash of build environment
    char   sigma_os_version[32];
    char   compiler_version[32];
    char   dilithium3_signature[512]; // Signature of entire SBOM
    char   signer_did[128];
    sigma_sbom_component_t components[256];
    int    component_count;
    uint64_t sbom_generation_ns;   // nanoseconds timestamp
} sigma_sbom_t;

// Generate SBOM for a package from its build metadata
int sigma_sbom_generate(const char *package_name, const char *version,
                          sigma_sbom_t *sbom_out);
int sigma_sbom_export_cyclonedx(const sigma_sbom_t *sbom,
                                 const char *output_json);
int sigma_sbom_export_spdx(const sigma_sbom_t *sbom,
                             const char *output_spdx);
int sigma_sbom_sign(sigma_sbom_t *sbom, const char *signer_did);
int sigma_sbom_verify(const sigma_sbom_t *sbom, bool *valid);

// Vulnerability scan against OSV (Open Source Vulnerabilities) database
int sigma_sbom_vuln_scan(sigma_sbom_t *sbom, int *critical_count,
                          int *high_count);

// Public transparency log verification
// submit hash to transparency.sigmaos.dev — append-only log
int sigma_sbom_transparency_log_submit(const sigma_sbom_t *sbom,
                                        char *log_entry_id_out);
int sigma_sbom_transparency_log_verify(const char *log_entry_id,
                                        bool *present);
