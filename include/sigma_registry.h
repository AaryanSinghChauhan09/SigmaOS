#ifndef SIGMA_REGISTRY_H
#define SIGMA_REGISTRY_H

#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_types.h"

/* =========================================================================
 * SIGMA OS: SOVEREIGN REGISTRY (S10)
 * Replaces Linux's fragmented /proc, /sys, sysctl, and udev subsystems
 * with a single unified silicon configuration matrix.
 * ========================================================================= */

#define MAX_REGISTRY_KEYS 512
#define MAX_KEY_LEN       64
#define MAX_VALUE_LEN     256

typedef enum {
    REG_TYPE_INT    = 0,
    REG_TYPE_STRING = 1,
    REG_TYPE_BYTES  = 2,
} reg_value_type_t;

typedef struct {
    char             key[MAX_KEY_LEN];
    reg_value_type_t type;
    union {
        int64_t  as_int;
        char     as_str[MAX_VALUE_LEN];
        uint8_t  as_bytes[MAX_VALUE_LEN];
    } value;
    uint8_t active;
} __attribute__((packed)) sigma_reg_entry_t;

void sigma_registry_init(void);
int  sigma_registry_set_int(const char* key, int64_t val);
int  sigma_registry_set_str(const char* key, const char* val);
int64_t sigma_registry_get_int(const char* key, int64_t default_val);
const char* sigma_registry_get_str(const char* key, const char* default_val);
void sigma_registry_dump(void);

#endif
