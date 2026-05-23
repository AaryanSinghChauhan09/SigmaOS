/*
 * Σ SigmaOS Zenith — Registry Manager Shard
 * Zero-Dependency Implementation. No predefined libraries.
 */

typedef unsigned int uint32_t;
typedef unsigned char uint8_t;
typedef unsigned long long uint64_t;

/* Sovereign string utility (No libc) */
static uint32_t sovereign_strlen(const char* str) {
    uint32_t len = 0;
    while (str[len]) len++;
    return len;
}

static bool sovereign_streq(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *s1 == *s2;
}

static void sovereign_strncpy(char* dest, const char* src, uint32_t n) {
    uint32_t i;
    for (i = 0; i < n && src[i] != '\0'; i++)
        dest[i] = src[i];
    for ( ; i < n; i++)
        dest[i] = '\0';
}

#define MAX_REGISTRY_KEYS 1024
#define KEY_NAME_MAX 64
#define KEY_VALUE_MAX 256

struct SigmaRegistryEntry {
    char key[KEY_NAME_MAX];
    char value[KEY_VALUE_MAX];
    bool in_use;
};

static struct SigmaRegistryEntry registry_store[MAX_REGISTRY_KEYS];

/* API: Initialize Registry */
extern "C" void sigma_registry_init() {
    for (uint32_t i = 0; i < MAX_REGISTRY_KEYS; i++) {
        registry_store[i].in_use = false;
        registry_store[i].key[0] = '\0';
        registry_store[i].value[0] = '\0';
    }
}

/* API: Set Registry Key */
extern "C" bool sigma_registry_set(const char* key, const char* value) {
    if (!key || !value) return false;

    /* Check if exists to overwrite */
    for (uint32_t i = 0; i < MAX_REGISTRY_KEYS; i++) {
        if (registry_store[i].in_use && sovereign_streq(registry_store[i].key, key)) {
            sovereign_strncpy(registry_store[i].value, value, KEY_VALUE_MAX - 1);
            return true;
        }
    }

    /* Find empty slot */
    for (uint32_t i = 0; i < MAX_REGISTRY_KEYS; i++) {
        if (!registry_store[i].in_use) {
            sovereign_strncpy(registry_store[i].key, key, KEY_NAME_MAX - 1);
            sovereign_strncpy(registry_store[i].value, value, KEY_VALUE_MAX - 1);
            registry_store[i].in_use = true;
            return true;
        }
    }

    return false; /* Registry Full */
}

/* API: Get Registry Key */
extern "C" bool sigma_registry_get(const char* key, char* out_value, uint32_t max_len) {
    if (!key || !out_value || max_len == 0) return false;

    for (uint32_t i = 0; i < MAX_REGISTRY_KEYS; i++) {
        if (registry_store[i].in_use && sovereign_streq(registry_store[i].key, key)) {
            sovereign_strncpy(out_value, registry_store[i].value, max_len - 1);
            out_value[max_len - 1] = '\0'; /* Ensure null-termination */
            return true;
        }
    }
    return false; /* Not found */
}
