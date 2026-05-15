#include "../../include/sigma_package_registry.h"

// Internal string copy
static void sigma_internal_strncpy(char* dest, const char* src, uint32_t n) {
    uint32_t i;
    for (i = 0; i < n && src[i] != '\0'; i++) {
        dest[i] = src[i];
    }
    for ( ; i < n; i++) {
        dest[i] = '\0';
    }
}

// Internal zeroing
static void sigma_internal_memzero(void* dest, uint32_t len) {
    uint8_t* d = (uint8_t*)dest;
    while(len--) { *d++ = 0; }
}

// Internal comparison
static int sigma_internal_memcmp(const void* ptr1, const void* ptr2, uint32_t num) {
    const uint8_t* p1 = (const uint8_t*)ptr1;
    const uint8_t* p2 = (const uint8_t*)ptr2;
    for (uint32_t i = 0; i < num; i++) {
        if (p1[i] != p2[i]) return p1[i] - p2[i];
    }
    return 0;
}

void sigma_registry_init(sigma_package_registry_t* reg) {
    if (!reg) return;
    reg->total_packages = 0;
    sigma_internal_memzero(reg->registry_root_hash, 32);
    sigma_internal_memzero(reg->entries, sizeof(reg->entries));
}

int sigma_registry_add_package(sigma_package_registry_t* reg, const sigma_package_manifest_t* pkg) {
    if (!reg || !pkg) return -1;
    if (reg->total_packages >= SIGMA_PKG_REGISTRY_CAPACITY) return -1;
    
    // Check if package hash already exists to prevent duplicates
    if (sigma_registry_resolve_by_hash(reg, pkg->hash) != -1) {
        return -2; // Already exists
    }
    
    uint32_t idx = reg->total_packages;
    
    // Copy the manifest over
    for(int i=0; i<32; i++) reg->entries[idx].hash[i] = pkg->hash[i];
    sigma_internal_strncpy(reg->entries[idx].name, pkg->name, SIGMA_PKG_MAX_NAME);
    reg->entries[idx].version_major = pkg->version_major;
    reg->entries[idx].version_minor = pkg->version_minor;
    reg->entries[idx].size_bytes = pkg->size_bytes;
    
    reg->entries[idx].dep_count = pkg->dep_count > SIGMA_PKG_MAX_DEPS ? SIGMA_PKG_MAX_DEPS : pkg->dep_count;
    for(uint32_t i=0; i < reg->entries[idx].dep_count; i++) {
        reg->entries[idx].dependencies[i] = pkg->dependencies[i];
    }
    
    reg->entries[idx].is_verified = pkg->is_verified;
    reg->total_packages++;
    
    sigma_registry_compute_root(reg); // Update global state
    
    return idx;
}

int sigma_registry_resolve_by_hash(const sigma_package_registry_t* reg, const uint8_t* hash) {
    if (!reg || !hash) return -1;
    for (uint32_t i = 0; i < reg->total_packages; i++) {
        if (sigma_internal_memcmp(reg->entries[i].hash, hash, 32) == 0) {
            return (int)i;
        }
    }
    return -1;
}

void sigma_registry_compute_root(sigma_package_registry_t* reg) {
    if (!reg) return;
    // Simple iterative XOR hash simulation for 0-dependency root generation
    sigma_internal_memzero(reg->registry_root_hash, 32);
    for (uint32_t i = 0; i < reg->total_packages; i++) {
        for (uint32_t j = 0; j < 32; j++) {
            reg->registry_root_hash[j] ^= reg->entries[i].hash[j];
            // Mix with index for positional dependence
            reg->registry_root_hash[(j + i) % 32] ^= (uint8_t)(reg->entries[i].size_bytes & 0xFF);
        }
    }
}
