#include "../../../../include/libc/SovereignLibC.h"
#include "../../../../include/sigma_registry.h"
#include "../../../../include/sigma_libc.h"
#include "../../../../include/sigma_libc.h"

/* =========================================================================
 * SIGMA OS: SOVEREIGN REGISTRY (S10)
 * Unified OS config store replacing /proc, /sys, sysctl and udev.
 * ========================================================================= */

static sigma_reg_entry_t registry[MAX_REGISTRY_KEYS];
static uint32_t reg_count = 0;

void sigma_registry_init(void) {
    sigma_sigma_memset(registry, 0, sizeof(registry));
    // Seed sovereign defaults
    sigma_registry_set_str("os.name",          "SigmaOS Zenith");
    sigma_registry_set_str("os.version",       "1.0.0");
    sigma_registry_set_str("os.arch",          "x86_64");
    sigma_registry_set_int("kernel.pages",     0);
    sigma_registry_set_int("net.mtu",          1500);
    sigma_registry_set_int("sched.quantum_ms", 10);
    sigma_registry_set_str("vault.status",     "active");
    sigma_registry_set_str("linux.status",     "IRRELEVANT");
    sigma_sigma_printf("[REG] Sovereign Registry initialized. Linux /proc and /sys containerized.\n");
}

static sigma_reg_entry_t* find_entry(const char* key) {
    for (uint32_t i = 0; i < reg_count; i++)
        if (registry[i].active && strncmp(registry[i].key, key, MAX_KEY_LEN - 1) == 0)
            return &registry[i];
    return SIGMA_NULL;
}

int sigma_registry_set_int(const char* key, int64_t val) {
    sigma_reg_entry_t* e = find_entry(key);
    if (!e) {
        if (reg_count >= MAX_REGISTRY_KEYS) return -1;
        e = &registry[reg_count++];
    }
    strncpy(e->key, key, MAX_KEY_LEN - 1);
    e->type = REG_TYPE_INT;
    e->value.as_int = val;
    e->active = 1;
    return 0;
}

int sigma_registry_set_str(const char* key, const char* val) {
    sigma_reg_entry_t* e = find_entry(key);
    if (!e) {
        if (reg_count >= MAX_REGISTRY_KEYS) return -1;
        e = &registry[reg_count++];
    }
    strncpy(e->key, key, MAX_KEY_LEN - 1);
    strncpy(e->value.as_str, val, MAX_VALUE_LEN - 1);
    e->type = REG_TYPE_STRING;
    e->active = 1;
    return 0;
}

int64_t sigma_registry_get_int(const char* key, int64_t default_val) {
    sigma_reg_entry_t* e = find_entry(key);
    return (e && e->type == REG_TYPE_INT) ? e->value.as_int : default_val;
}

const char* sigma_registry_get_str(const char* key, const char* default_val) {
    sigma_reg_entry_t* e = find_entry(key);
    return (e && e->type == REG_TYPE_STRING) ? e->value.as_str : default_val;
}

void sigma_registry_dump(void) {
    sigma_sigma_printf("[REG] --- Sovereign Registry Dump ---\n");
    for (uint32_t i = 0; i < reg_count; i++) {
        if (!registry[i].active) continue;
        if (registry[i].type == REG_TYPE_INT)
            sigma_sigma_printf("[REG]   %s = %lld\n", registry[i].key, (long long)registry[i].value.as_int);
        else
            sigma_sigma_printf("[REG]   %s = \"%s\"\n", registry[i].key, registry[i].value.as_str);
    }
}
