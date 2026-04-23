#include <stdint.h>
#include <stddef.h>
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Dynamic Module Loader Prototype
// ---------------------------------------------------------

#define MAX_MODULES 64
#define MODULE_NAME_LEN 32

typedef enum {
    MOD_DRIVER,
    MOD_SERVICE,
    MOD_FILESYSTEM,
    MOD_NETWORK
} module_type_t;

typedef struct {
    char name[MODULE_NAME_LEN];
    module_type_t type;
    void* base_address;
    uint32_t size;
    int loaded;
    void (*init_fn)(void);
    void (*cleanup_fn)(void);
} module_t;

static module_t module_table[MAX_MODULES];
static uint32_t module_count = 0;

// Register a module (done at compile time for built-in modules)
int module_register(const char* name, module_type_t type, void(*init)(void), void(*cleanup)(void)) {
    if (module_count >= MAX_MODULES) return -1;
    module_t* m = &module_table[module_count++];
    strncpy(m->name, name, MODULE_NAME_LEN - 1);
    m->type = type;
    m->loaded = 0;
    m->init_fn = init;
    m->cleanup_fn = cleanup;
    return module_count - 1;
}

// Hot-load a module by name
int module_load(const char* name) {
    for (int i = 0; i < module_count; i++) {
        if (strncmp(module_table[i].name, name, MODULE_NAME_LEN) == 0) {
            if (module_table[i].loaded) return 0; // Already loaded
            if (module_table[i].init_fn) module_table[i].init_fn();
            module_table[i].loaded = 1;
            return 0;
        }
    }
    return -1; // Module not found
}

// Hot-unload a module by name (without rebooting)
int module_unload(const char* name) {
    for (int i = 0; i < module_count; i++) {
        if (strncmp(module_table[i].name, name, MODULE_NAME_LEN) == 0) {
            if (!module_table[i].loaded) return 0;
            if (module_table[i].cleanup_fn) module_table[i].cleanup_fn();
            module_table[i].loaded = 0;
            return 0;
        }
    }
    return -1;
}

// List all loaded modules
void module_list() {
    for (int i = 0; i < module_count; i++) {
        if (module_table[i].loaded) {
            // In real OS: print name + type + address
        }
    }
}
